#[test]
fn uicolor() {
    use crate::{IUIColor, UIColor};
    unsafe {
        let white = UIColor::whiteColor();
        white.setFill();
    }
}
