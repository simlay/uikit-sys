.PHONY: xcode cargo run install simulator-logs simulator

run: install
	xcrun simctl launch --console booted com.rust.use-uikit-example

install: xcodebuild
	 xcrun simctl install booted build/Build/Products/Debug-iphonesimulator/use_uikit_example.app

xcodebuild: generate
	xcodebuild -scheme use_uikit_example -configuration Debug -destination 'platform=iOS Simulator,name=iPhone SE (2nd generation),OS=13.6' -derivedDataPath build

generate: project.yml ios-src/Info.plist ios-src/main.m ios-src/bindings.h
	xcodegen

clean:
	rm -r build
	cargo clean
