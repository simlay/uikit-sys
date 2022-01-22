#[macro_use]
extern crate objc;

use log::trace;
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
};
use winit::event_loop::EventLoopProxy;
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::ios::{EventLoopExtIOS, WindowBuilderExtIOS, WindowExtIOS},
    window::{Window, WindowBuilder},
};

use objc::msg_send;
use uikit_sys::CFGetRetainCount;
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

pub fn main() -> ! {
    debug_init();
    log::debug!("STARTING THE APP!");
    let event_loop: EventLoop<WidgetEvent> = EventLoop::with_user_event();
    let proxy = event_loop.create_proxy();
    EventHandler::init(proxy.clone());
    let window: Window = WindowBuilder::new()
        .with_title("UIKit Rust App")
        .with_maximized(true)
        .build(&event_loop)
        .expect("Failed to build window");

    let root_vc: id = unsafe { *(window.ui_view_controller() as *mut id) };

    let root_view: UIView = UIView(window.ui_view() as id);
    unsafe {
        let background = UIColor::redColor();
        root_view.setBackgroundColor_(background);
    }
    let mut count = 0;
    let mut label = add_counte_label(count);
    event_loop.run(
        move |event: winit::event::Event<WidgetEvent>, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => {
                    let root_view: UIView = UIView(window.ui_view() as id);
                    let views = get_views();
                    for i in &views {
                        unsafe {
                            root_view.addSubview_(i.clone());
                        }
                    }
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
                    WindowEvent::Touch(winit::event::Touch { phase, .. }) => {
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
                Event::UserEvent(widget_event) => {}
                _ => {}
            }
        },
    )
}

#[derive(PartialEq, Clone, Debug)]
pub struct WidgetEvent {
    pub widget_id: u64,
    pub id: usize,
}

#[derive(Debug)]
pub struct EventHandler {
    pub id: id,
    pub widget_id: u64,
}

static mut PROXY: Option<EventLoopProxy<WidgetEvent>> = None;
static mut COUNTER: Option<u64> = None;
impl EventHandler {
    pub fn init(proxy: EventLoopProxy<WidgetEvent>) {
        unsafe {
            COUNTER = Some(0);
            PROXY = Some(proxy);
        }
    }
    pub fn new(objc_id: id) -> Self {
        let mut widget_id = 0;
        // TODO: Figure out how to make this unsafe block much smaller.
        use objc::sel;
        let obj = unsafe {
            let obj: id = objc::msg_send![Self::class(), alloc];
            let obj: id = objc::msg_send![obj, init];

            if let Some(mut counter) = COUNTER {
                counter += 1;
                COUNTER = Some(counter);
                widget_id = counter;
                (*obj).set_ivar::<u64>("widget_id", widget_id);
            }
            (*obj).set_ivar::<id>("objc_id", objc_id);
            obj
        };
        trace!("NEW EVENTHANDLER WITH WIDGET ID :{:?}", widget_id);
        Self { id: obj, widget_id }
    }
    extern "C" fn event(this: &Object, _cmd: objc::runtime::Sel) {
        // TODO: Figure out how to make this unsafe block smaller.
        unsafe {
            if let Some(ref proxy) = PROXY {
                let widget_id = *this.get_ivar::<u64>("widget_id");
                let id = *this.get_ivar::<id>("objc_id");
                let _ = proxy.send_event(WidgetEvent {
                    widget_id,
                    id: id as usize,
                });
            }
        }
    }

    fn class() -> &'static Class {
        let cls_name = "RustEventHandler";
        match Class::get(cls_name) {
            Some(cls) => cls,
            None => {
                let superclass = objc::class!(NSObject);
                let mut decl = ClassDecl::new(cls_name, superclass).unwrap();
                unsafe {
                    decl.add_method(
                        objc::sel!(sendEvent),
                        Self::event as extern "C" fn(&Object, Sel),
                    );
                }
                decl.add_ivar::<u64>("widget_id");
                decl.add_ivar::<id>("objc_id");
                decl.register()
            }
        }
    }
}

fn get_views() -> Vec<UIView> {
    //{{{
    let mut views = Vec::new();
    let rect = CGRect {
        origin: CGPoint { x: 10.0, y: 20.0 },
        size: CGSize {
            height: 20.0,
            width: 20.0,
        },
    };
    let rect = unsafe {
        let foo: UIView = UIView(UIView::alloc().initWithFrame_(rect));
        let background = UIColor::yellowColor();
        foo.setBackgroundColor_(background);
        foo
    };
    views.push(rect);
    let input_rect = CGRect {
        origin: CGPoint { x: 10.0, y: 50.0 },
        size: CGSize {
            width: 200.0,
            height: 20.0,
        },
    };
    let input = unsafe {
        let text_container = NSTextContainer(NSTextContainer::alloc().initWithSize_(CGSize {
            height: 100.0,
            width: 200.0,
        }));
        let foo = UITextView(
            UITextView::alloc().initWithFrame_textContainer_(input_rect, text_container),
        );
        foo
    };
    views.push(UIView(input.0));
    let switch = unsafe {
        UISwitch(uikit_sys::IUISwitch::initWithFrame_(
            &UISwitch::alloc(),
            CGRect {
                origin: CGPoint { x: 10.0, y: 80.0 },
                size: CGSize {
                    height: 200.0,
                    width: 200.0,
                },
            },
        ))
    };
    views.push(UIView(switch.0));
    views
} //}}}

fn add_counte_label(count: i64) -> UIView {
    //{{{
    use uikit_sys::{
        CGPoint, CGRect, CGSize, INSObject, IUILabel, NSString, NSString_NSStringExtensionMethods,
        NSUTF8StringEncoding, UILabel, UIView_UIViewGeometry, UIView_UIViewHierarchy,
    };

    use std::{convert::TryInto, ffi::CString};
    let label_text = format!("{:?} COUNT COUNT", count);

    let text = CString::new(label_text.as_str()).expect("CString::new failed");
    let text_ptr = text.as_ptr();
    let text_length: u64 = label_text.len().try_into().unwrap();
    println!("WE MAKIN A LABEL");

    let label = unsafe {
        //let alloc = UILabel::alloc();
        //printlnalloc.print_retain_count();
        let label = UILabel(UILabel::alloc().init());

        label.setFrame_(CGRect {
            origin: CGPoint { x: 20.0, y: 100.0 },
            size: CGSize {
                width: 400.0,
                height: 40.0,
            },
        });

        let text_alloc = NSString::alloc();
        let text = NSString(text_alloc.initWithBytes_length_encoding_(
            text_ptr as *mut std::ffi::c_void,
            text_length,
            NSUTF8StringEncoding,
        ));
        label.setText_(text);
        label
    };
    //label
    UIView(label.0)
} //}}}

fn debug_init() {
    color_backtrace::install_with_settings(
        color_backtrace::Settings::new().verbosity(color_backtrace::Verbosity::Full),
    );
    pretty_env_logger::init();
    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "full");
}
