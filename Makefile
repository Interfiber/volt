build:
	sh build.sh
pkg:
	mkdir Volt.app
	mkdir -p Volt.app/Contents/MacOS
	mkdir -p Volt.app/Contents/Resources
	cp ./assets/Info.plist Volt.app/Contents/Info.plist
	cp ./target/release/volt Volt.app/Contents/MacOS/volt
	cp ./assets/icon.icns Volt.app/Contents/Resources/volt_icon.icns
