#[test]
fn uicolor()  {
    use uikit_sys::{
        UIColor,
        IUIColor,
    };
    unsafe {
        let white = UIColor::whiteColor();
        white.setFill();
    }
}
#[test]
fn uiview_downcasting_tests() {
    use uikit_sys::{
        UIButton,
        INSObject,
        UIView,
        UILabel,
    };
    use std::convert::TryFrom;
    let button = UIButton(unsafe {UIButton::alloc().init()});
    let as_view : UIView = button.into();
    let as_button = UIButton::try_from(as_view.clone());
    assert!(as_button.is_ok());
    let as_uilabel = UILabel::try_from(as_view);
    assert!(as_uilabel.is_err());
    println!("AS UILABEL: {:?}", as_uilabel.err());
}
