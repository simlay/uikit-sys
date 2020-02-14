#[test]
fn uicolor()  {
    use crate::{
        struct_UIColor,
        interface_UIColor,
    };
    unsafe {
        let white = struct_UIColor(struct_UIColor::whiteColor());
        white.setFill();
    }
}
