#[test]
fn uicolor()  {
    use crate::{
        UIColor,
        IUIColor,
    };
    unsafe {
        let white = UIColor(UIColor::whiteColor());
        white.setFill();
    }
}
