#[test]
fn uicolor()  {
    use crate::{
        UIColor,
        IUIColor,
    };
    unsafe {
        let white = UIColor::whiteColor();
        white.setFill();
    }
}
