.PHONY: xcode cargo run install simulator-logs simulator

LIB = libuikit_sys_example.a
run: install
	xcrun simctl launch booted com.iced.UIKitSysExample

install: xcode
	xcrun simctl install booted build/Build/Products/Debug-iphonesimulator/UIKitSysExample.app

xcode: $(LIB)
	xcrun xcodebuild -scheme UIKitSysExample -project UIKitSysExample.xcodeproj/ -configuration Debug -destination 'platform=iOS Simulator,name=iPhone 11 Pro,OS=13.2.2' -derivedDataPath build

simulator-logs:
	xcrun simctl spawn booted log stream --level=debug --predicate 'processImagePath endswith "UIKitSysExample"'
ci: simulator
	xcrun xcodebuild -scheme UIKitSysExample -project UIKitSysExample.xcodeproj/ -configuration Debug -destination 'platform=iOS Simulator,name=iPhone 11 Pro,OS=13.2.2' -derivedDataPath build

cargo:
	cargo lipo
	cp target/universal/debug/$(LIB) ./

simulator:
	cargo build --target x86_64-apple-ios
	cp target/x86_64-apple-ios/debug/$(LIB) ./

release:
	cargo lipo --release
	cp target/universal/release/$(LIB) ./

iphone:
	cargo build --target aarch64-apple-ios --release
	cp target/aarch64-apple-ios/release/$(LIB) ./

doc:
	cargo doc --target aarch64-apple-ios --open
