build:
	sh build.sh
pkg_mac:
	# Create app file
	mkdir Volt.app
	mkdir -p Volt.app/Contents/MacOS
	mkdir -p Volt.app/Contents/Resources
	cp ./assets/Info.plist Volt.app/Contents/Info.plist
	cp ./target/release/volt Volt.app/Contents/MacOS/volt
	cp ./assets/icon.icns Volt.app/Contents/Resources/volt_icon.icns
	mkdir dist
	mv Volt.app dist/Volt.app
	# Prep for disk image creation
	ln -s /Applications dist/Applications
	# Create disk image
	hdiutil create Volt.dmg -ov -volname "Volt Installer" -fs HFS+ -srcfolder "dist" 