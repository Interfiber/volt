use kas::class::HasString;
use kas::event::{Handler, Manager, Response, VoidMsg, Event};
use kas::macros::make_widget;
use kas::widget::*;
use kas::class::HasStr;
use serde_json::Value;
// Modules
mod plugins;
mod http;
mod util;
mod minecraft;
mod config;

// Browse plugins will allow the user to browse curseforge plugins
fn browse_plugins() -> Box<dyn kas::Window> {
    // Create window content
    let content = make_widget! {
        #[layout(column)]
        #[widget(config=noauto)]
        struct {
            #[widget] display: impl HasString = Label::new("Enter Project ID for mod to install\nOr enter mod name to uninstall".to_string()),
            #[widget] project_id: EditBox = EditBox::new(""),
            #[widget(handler = switch_mode)] switch_mode: impl HasStr = TextButton::new_msg("Show Mods", ()),
            #[widget(handler = install)] search_button = TextButton::new_msg("Install Mod", ()),
            #[widget(handler = uninstall)] rm_button = TextButton::new_msg("Uninstall Mod", ()),
            #[widget] output: EditBox = EditBox::new("Installation output will appear here").editable(false).multi_line(true)
        }
        impl {
            fn switch_mode(&mut self, _mgr: &mut Manager, _: ()) -> Response<VoidMsg>{
                println!(":: finding mod list...");
                let mods = minecraft::get_mods();
                let _ = self.output.set_string(format!("Installed Mods:\n{}", mods));
                Response::None
            }
            fn uninstall(&mut self, _mgr: &mut Manager, _: ()) -> Response<VoidMsg> {
                let _ = self.output.set_string("== Mod Uninstall log ==".to_string());
                println!(":: prepping for uninstall...");
                let name = self.project_id.get_string();
                println!(":: mod name {}", name);
                let out = plugins::remove_mod(name);
                let _ = self.output.set_string(format!("{}\n{}", self.output.get_string(), out));
                Response::None
            }
            fn install(&mut self, _mgr: &mut Manager, _: ()) -> Response<VoidMsg> {
                // Set the output
                let _ = self.output.set_string("== Mod Install log ==".to_string());
                // Check if the string is a number
                if !util::is_number(self.project_id.get_string()){
                    let _ = self.output.set_string(format!("{}\nPlease enter a valid project ID", self.output.get_string()));
                }else {
                    let _ = self.output.set_string(format!("{}\nGetting latest download URL...", self.output.get_string()));
                    println!(":: finding latest download URL...");
                    let latest = plugins::get_latest_plugin(self.project_id.get_string());
                    if latest == "PLUGIN_NOT_FOUND" {
                        println!(":: plugin not found");
                        let _ = self.output.set_string(format!("{}\nFailed to find latest version: {}", self.output.get_string(), latest));
                    }else {
                        let _ = self.output.set_string(format!("{}\nGot latest download URL: {}", self.output.get_string(), latest));
                        println!(":: got latest download URL: {}", latest);
                        let _ = self.output.set_string(format!("{}\nDownloading mod...", self.output.get_string()));
                        // Download from the URL
                        plugins::download(latest);
                        let _ = self.output.set_string(format!("{}\nInstalling mod...", self.output.get_string()));
                        let name = plugins::get_plugin_name(self.project_id.get_string());
                        plugins::install_mod(name);
                        let _ = self.output.set_string(format!("{}\nMod Installed! Now open minecraft!", self.output.get_string()));
                    }
                }
               Response::None
            }
        }
        impl kas::WidgetConfig {
            fn configure(&mut self, mgr: &mut Manager) {
                mgr.enable_alt_bypass(true);
            }
        }
        impl Handler {
            type Msg = VoidMsg;
            fn handle(&mut self, _mgr: &mut Manager, event: Event) -> Response<VoidMsg> {
                println!("{:#?}", event);
                return Response::Unhandled
            }
        }
    };
    let window = Window::new("Volt - Install Plugins", content);
    Box::new(window)
}

fn main() -> Result<(), kas_wgpu::Error> {
    println!(":: volt has started");
    println!(":: checking for config file...");
    if !config::config_exists(){
        println!(":: config not found, launching with default config");
        let theme = kas_theme::ShadedTheme::new()
            .with_font_size(18.0);
            kas_wgpu::Toolkit::new(theme)?
            .with_boxed(browse_plugins())?
            .run()
    }else {
        println!(":: config found, apply theme data.");
        let config_raw = config::get_config();
        println!(":: parsing config...");
        let config: Value = serde_json::from_str(&config_raw).expect("Failed to parse config!");
        println!(":: parsed config");
        let theme = kas_theme::ShadedTheme::new()
            .with_font_size(18.0)
            .with_colours(&config["theme"]["default_colors"].to_string().replace("\"", ""));
            kas_wgpu::Toolkit::new(theme)?
            .with_boxed(browse_plugins())?
        .run()
    }
}
