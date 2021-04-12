use kas::class::HasString;
use kas::event::{Handler, Manager, Response, VoidMsg, Event};
use kas::macros::make_widget;
use kas::widget::{Label, TextButton, Window, EditBox};
use kas::class::HasStr;
// Modules
mod plugins;
mod keys;

// Ask keys will ask the user to enter their CurseForge api key
fn ask_keys() -> Box<dyn kas::Window> {
    // Construct a row widget, with state and children
    let content = make_widget! {
        #[layout(column)]
        #[widget(config=noauto)]
        struct {
            #[widget] display: impl HasString = Label::new("Enter CurseForge API key:".to_string()),
            #[widget] editor: EditBox = EditBox::new(""),
            #[widget(handler = save_keys)] _ = TextButton::new_msg("Confirm Keys", ()),
        }
        impl {
            fn save_keys(&mut self, _mgr: &mut Manager, _: ()) -> Response<VoidMsg> {
                println!(":: saving api keys to .volt_keys");
                std::fs::write(".volt_keys", self.editor.get_str());
                println!(":: keys added, exiting volt...");
                std::process::exit(1);
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

    let mut window = Window::new("Volt - Setup API keys", content);
    window.set_restrict_dimensions(true, true);
    Box::new(window)
}
// Browse plugins will allow the user to browse curseforge plugins
fn browse_plugins() -> Box<dyn kas::Window> {
    // Construct a row widget, with state and children
    let content = make_widget! {
        #[layout(column)]
        #[widget(config=noauto)]
        struct {
            #[widget] display: impl HasString = Label::new("Search Plugins:   ".to_string()),
            #[widget] searchbox: EditBox = EditBox::new(""),
            #[widget(handler = search)] search_button = TextButton::new_msg("Search", ()),
            #[widget] list: EditBox = EditBox::new("Search results will appear here").multi_line(true).editable(false),
        }
        impl {
            fn search(&mut self, _mgr: &mut Manager, _: ()) -> Response<VoidMsg> {
                plugins::search_plugins(self.searchbox.get_str().to_string());
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

    let mut window = Window::new("Volt - Browse Plugins", content);
    window.set_restrict_dimensions(true, true);
    Box::new(window)
}
fn main() -> Result<(), kas_wgpu::Error> {
    println!(":: volt has started");
    println!(":: checking for CurseForge api keys...");
    if !std::path::Path::new(".volt_keys").exists(){
        println!(":: keys not present.");
        let theme = kas_theme::ShadedTheme::new()
            .with_font_size(18.0);
        kas_wgpu::Toolkit::new(theme)?
            .with_boxed(ask_keys())?
            .run()
    }else {
        println!(":: keys present.");
        let theme = kas_theme::ShadedTheme::new()
            .with_font_size(18.0);
        kas_wgpu::Toolkit::new(theme)?
            .with_boxed(browse_plugins())?
            .run()
    }
}
