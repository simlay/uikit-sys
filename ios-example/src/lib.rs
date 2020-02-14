extern crate uikit_sys;
#[macro_use]
extern crate log;
#[macro_use]
extern crate objc;

#[path = "../../examples/rect.rs"]
mod example;
#[no_mangle]
pub extern "C" fn run_app() {
    color_backtrace::install();
    example::main();
}
