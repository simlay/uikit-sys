use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::ios::{EventLoopExtIOS, WindowBuilderExtIOS, WindowExtIOS},
    window::{Window, WindowBuilder},
};

use uikit_sys::{
    id,
    CGPoint,
    CGRect,
    CGSize,
    INSTextContainer,
    IUIColor,
    IUISwitch,
    IUITextView,
    IUIView,
    NSTextContainer,
    UIColor,
    UISwitch,
    UITextView,
    UIView,
    UIView_UIViewGeometry,
    UIView_UIViewHierarchy,
    //UIView,
    //UIViewController,
    UIView_UIViewRendering,
};
use objc::msg_send;
use uikit_sys::CFGetRetainCount;

pub fn main() -> ! {
    debug_init();
    log::debug!("STARTING THE APP!");
    let event_loop = EventLoop::new();
    let window: Window = WindowBuilder::new()
        .with_title("UIKit Rust App")
        .with_maximized(true)
        .build(&event_loop)
        .expect("Failed to build window");

    let root_vc: id = unsafe { *(window.ui_view_controller() as *mut id) };

    let root_view: UIView = UIView::from_id(window.ui_view() as id, true);
    unsafe {

        //let background = UIColor::alloc().initWithRed_green_blue_alpha_(0.1, 1.0, 2.0, 2.0);
        let background = UIColor::redColor();
        root_view.setBackgroundColor_(background);
    }
    /*

    let foo = unsafe {UIColor::redColor()};
    for i in 1..10 {
        let baz = foo.clone();
        let retainCount = baz.retainCount();
        //let retainCount = unsafe { CFGetRetainCount(baz.id() as *const std::ffi::c_void) };
        println!("RETAIN COUNT FOR FOO IS: {:?}", retainCount);
    }
    //let retainCount :u64 = unsafe { msg_send!(foo, retainCount)};
    println!("RETAIN COUNT FOR FOO IS: {:?}", foo.retainCount());
    */
    let mut count = 0;
    let mut label = add_counte_label(count);
    event_loop.run(move |event: Event<()>, _, control_flow: &mut ControlFlow| {
        //println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                let root_view: UIView = UIView::from_id(window.ui_view() as id, true);
                //add_views(&root_view);
                unsafe {
                    root_view.addSubview_(label.clone());
                }
            }
            Event::LoopDestroyed => return,
            Event::RedrawRequested(_) => {}
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(_logical_size) => {
                    //window.request_redraw();
                }
                WindowEvent::Touch(winit::event::Touch{
                    phase, ..
                }) => {
                    if phase == &winit::event::TouchPhase::Started {
                        println!("Removing old label");
                        //for i in 1..100 {
                            unsafe {
                                label.removeFromSuperview();
                            }
                            count = count + 1;
                            label = add_counte_label(count);
                            unsafe {
                                root_view.addSubview_(label.clone());
                            }
                        }
                    //}
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => {
            },
        }
                            println!("LABEL REF COUNTE: {:?}", label.get_retain_count());
    })
}

fn add_views(root_view: &UIView) {
    let rect = CGRect {
        origin: CGPoint { x: 10.0, y: 20.0 },
        size: CGSize {
            height: 20.0,
            width: 20.0,
        },
    };
    let rect =  unsafe {
        let foo : UIView = UIView(UIView::alloc().initWithFrame_(rect));
        let background = UIColor::yellowColor();
        foo.setBackgroundColor_(background);
        foo
    };
    unsafe {
        root_view.addSubview_(rect);
    }
    let input_rect = CGRect {
        origin: CGPoint { x: 10.0, y: 50.0 },
        size: CGSize {
            width: 200.0,
            height: 20.0,
        },
    };
    let input = unsafe {
        let text_container = NSTextContainer(
            NSTextContainer::alloc().initWithSize_(
                CGSize {
                    height: 10.0,
                    width: 200.0,
                }
            )
        );
        let foo = UITextView(
            UITextView::alloc().initWithFrame_textContainer_(
                input_rect,
                text_container
            )
        );
        foo
    };
    unsafe {
        root_view.addSubview_(UIView(input.0));
    }
    unsafe {
        let switch = UISwitch(uikit_sys::IUISwitch::initWithFrame_(UISwitch::alloc(),
            CGRect {
                origin: CGPoint { x: 10.0, y: 80.0 },
                size: CGSize {
                    height: 200.0,
                    width: 200.0,
                }
            }
        ));
        root_view.addSubview_(UIView(switch.0));
    }
}

fn add_counte_label(count: i64) -> UIView {
    use uikit_sys::{
        UILabel,
        IUILabel,
        NSString,
        INSObject,
        UIView_UIViewHierarchy,
        UIView_UIViewGeometry,
        NSString_NSStringExtensionMethods,
        NSUTF8StringEncoding,
        CGRect,
        CGPoint,
        CGSize,
    };

    use std::{
        ffi::CString,
        convert::TryInto,
    };
    let label_text = format!("{:?} COUNT COUNT", count);

    let text = CString::new(label_text.as_str()).expect("CString::new failed");
    let text_ptr = text.as_ptr();
    let text_length : u64 = label_text.len().try_into().unwrap();
    println!("WE MAKIN A LABEL");

    let label = unsafe {
        //let alloc = UILabel::alloc();
        //printlnalloc.print_retain_count();
        let label = UILabel::from_id(UILabel::alloc().init(), true);

        label.setFrame_(CGRect {
            origin: CGPoint {
                x: 20.0,
                y: 100.0,
            },
            size: CGSize {
                width: 400.0,
                height: 40.0,
            },
        });

        /*
        let text_alloc = NSString::alloc();
        let text = NSString::from_id(
            text_alloc.initWithBytes_length_encoding_(
                text_ptr as *mut std::ffi::c_void,
                text_length,
                NSUTF8StringEncoding,
            ),
            true
        );
        label.setText_(text);
        */
        label

    };
    //label
    //UIView::from_id(label.id())
    label.into()
}
fn debug_init() {
    color_backtrace::install_with_settings(
        color_backtrace::Settings::new().verbosity(color_backtrace::Verbosity::Full),
    );
    pretty_env_logger::init();
    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "full");
}
