
use crate::{
    struct_UIColor,
    interface_UIColor,
};
#[test]
fn uicolor()  {
    use crate::util::{
        uicolor_test,
    };
    unsafe {
        let white = struct_UIColor(*struct_UIColor::whiteColor());
        let _ : () = msg_send!(white.0, setFill);
    }
    //uicolor_test();
}
