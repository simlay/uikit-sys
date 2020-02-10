use uikit_sys::{
    interface_UIColor,
    struct_UIColor,

};
fn main() {
    unsafe {
        let color = struct_UIColor::alloc();
        let foo = color.initWithRed_green_blue_alpha_(0.1, 1.0, 2.0, 2.0);
        let color = struct_UIColor(*foo);
    }
}


