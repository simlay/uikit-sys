.PHONY: xcode cargo run install simulator-logs simulator

run: install
	xcrun simctl launch --console booted com.rust.use-uikit-example

install: xcodebuild
	 xcrun simctl install booted build/Build/Products/Debug-iphonesimulator/use_uikit_example.app

DEVICE_ID=$(shell xcrun simctl list  devices 'iOS' | grep -v '^--' | grep -v '==' | head -n 1 | cut -d ' ' -f 7 | sed 's/[()]//g')
xcodebuild: generate
	xcodebuild -scheme use_uikit_example -configuration Debug -derivedDataPath build -destination "id=$(DEVICE_ID)"

generate: project.yml ios-src/Info.plist ios-src/main.m ios-src/bindings.h
	xcodegen

clean:
	rm -r build
	cargo clean
