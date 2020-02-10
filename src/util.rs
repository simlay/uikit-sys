
use crate::{
    struct_UIColor,
    interface_UIColor,
};
pub fn uicolor_test()  {
    unsafe {

        let white = struct_UIColor(*struct_UIColor::whiteColor());
        let _ : () = msg_send!(white.0, setFill);
        /*
        let white = struct_UIColor(*struct_UIColor::whiteColor());
        let color = struct_UIColor::alloc();
        let foo = color.initWithRed_green_blue_alpha_(0.1, 0.2, 0.3, 0.4);
        let color = struct_UIColor(*foo);
        */
        //let foo = color.setFill();
        println!("RAN THE UIN TESTS FOR  UICOLOR!");
        //assert_eq!(color.CGColor(), 0.4);
    }
}
