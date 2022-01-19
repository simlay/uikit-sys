.PHONEY: test

RUNTIME_ID=$(shell xcrun simctl list runtimes | grep iOS | cut -d ' ' -f 7 | tail -1)

DEVICE_ID=$(shell xcrun simctl list devices 'iOS 15.2' | grep -v '^--' | grep -v '==' | head -n 1 | awk '{print $$3}' | sed 's/[()]//g')

TARGET=x86_64-apple-ios

boot-sim:
	xcrun simctl list devices booted | grep iPhone || xcrun simctl boot $(DEVICE_ID)

test: boot-sim
	cargo dinghy --platform auto-ios-x86_64 test

bundle:
	cargo bundle --example rect --format ios --target $(TARGET)

bundle-install: bundle
	xcrun simctl install booted target/$(TARGET)/debug/examples/bundle/ios/rect.app

bundle-run: bundle-install
	RUST_LOG=debug xcrun simctl launch booted com.github.simlay.uikit-sys.rect

.EXPORT_ALL_VARIABLES:
LLVM_CONFIG_PATH=$(shell brew --prefix llvm)/bin/llvm-config