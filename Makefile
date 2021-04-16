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
	rm -rf dist
pkg_linux:
	# Generate tarball
	mkdir dist
	mkdir dist/config
	mkdir dist/bin
	cp ./target/release/volt dist/bin/volt
	touch dist/config/basic_config.json
	echo "{" >> dist/config/basic_config.json
	echo "'_comment': 'Basic config file, put this in ~/.config/volt/volt.json'," >> dist/config/basic_config.json
	echo "}" >> dist/config/basic_config.json
	printf "Installation:\nTo install volt copy the executable located in bin. Into your PATH.\nThen create the folder ~/.config/volt/ and place the example config located in\nconfig/basic_config.json into ~/.config/volt/volt.json" >> dist/INSTALLATION.txt
	tar cvf volt_linux.tar.gz dist
	rm -rf dist
	echo "output file: volt_linux.tar.gz"
