/*
use uikit_sys::{
    UIView,
    UIViewController,
    CGRect,
    CGPoint,
    CGSize,
    id,

};
*/
use uikit_sys::{
    //interface_UIColor,
    //struct_UIColor,
    struct_Foo,
    interface_Foo,
};
fn main() {
    unsafe {
        let foo = struct_Foo::alloc();
        foo.initWithFirstNumber_(10);
    }
}
