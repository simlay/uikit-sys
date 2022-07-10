use std::env;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let target = std::env::var("TARGET").unwrap();
    if !target.contains("apple-ios") {
        panic!("uikit-sys requires the ios target");
    }
    use std::process::Command;
	let sdk = match target.as_str() {
        "x86_64-apple-ios" | "i386-apple-ios" | "aarch64-apple-ios-sim" => "iphonesimulator",
		"aarch64-apple-ios" | "armv7-apple-ios" | "armv7s-apple-ios" => "iphoneos",
		"x86_64-apple-ios-macabi" | "aarch64-apple-ios-macabi" => "macosx",
		_ => unreachable!()
	};
    let output = Command::new("xcrun")
        .args(&["--sdk", sdk, "--show-sdk-path"])
        .output()?
        .stdout;
    let prefix_str = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
	let sys_root = prefix_str.trim_end().to_string();
	
    // Generate one large set of bindings for all frameworks.
    //
    // We do this rather than generating a module per framework as some frameworks depend on other
    // frameworks and in turn share types. To ensure all types are compatible across each
    // framework, we feed all headers to bindgen at once.
    //
    // Only link to each framework and include their headers if their features are enabled and they
    // are available on the target os.
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    println!("cargo:rustc-link-lib=framework=UIKit");

    // See https://github.com/rust-lang/rust-bindgen/issues/1211
    // Technically according to the llvm mailing list, the argument to clang here should be
    // -arch arm64 but it looks cleaner to just change the target.
	let target = match target.as_str() {
		"aarch64-apple-ios"  => "aarch64-apple-ios",
		_ => &target
	};
    // Begin building the bindgen params.
    let mut builder = bindgen::Builder::default();

    let target_arg = format!("--target={}", target);
	// Set clang's -isysroot dir for all targets.
	// Set -isystem and -iframework For Mac catalyst only per https://stackoverflow.com/a/59939450
	let isystem = format!("{}/System/iOSSupport/usr/include",  &sys_root);
	let iframework = format!("{}/System/iOSSupport/System/Library/Frameworks",  &sys_root);
    let mut clang_args = vec!["-x", "objective-c", "-fblocks", &target_arg];
	clang_args.extend(&["-isysroot", &sys_root]);

	match target {
		"x86_64-apple-ios-macabi" | "aarch64-apple-ios-macabi" => {
			clang_args.extend(&["-isystem", isystem.as_str()]);
			clang_args.extend(&["-iframework", iframework.as_str()]);
		},
		_ => ()
	};

    builder = builder
        .clang_args(&clang_args)
        .objc_extern_crate(true)
        //.block_extern_crate(true)
        //.generate_block(true)
        .rustfmt_bindings(true)
        // time.h as has a variable called timezone that conflicts with some of the objective-c
        // calls from NSCalendar.h in the Foundation framework. This removes that one variable.
        .blocklist_item("timezone")
        // https://github.com/rust-lang/rust-bindgen/issues/1705
        .blocklist_item("IUIStepper")
        .blocklist_function("dividerImageForLeftSegmentState_rightSegmentState_")
        .blocklist_item("objc_object")
        .header_contents("UIKit.h", "#include<UIKit/UIKit.h>");

    // Generate the bindings.
    let bindings = builder.generate().expect("unable to generate bindings");

    // Get the cargo out directory.
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("env variable OUT_DIR not found"));

    // Write them to the crate root.
    bindings
        .write_to_file(out_dir.join("uikit.rs"))
        .expect("could not write bindings");

	Ok(())
}
