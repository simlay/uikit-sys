use winit::{
    event::{StartCause, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{
        WindowBuilder,
        Window
    },
    platform::ios::{
        WindowExtIOS,
        EventLoopExtIOS,
        WindowBuilderExtIOS,
    },
};

use uikit_sys::{
    //UIView,
    //UIViewController,
    UIView_UIViewRendering,
    UIView_UIViewHierarchy,
    UIView_UIViewGeometry,
    NSTextContainer,
    INSTextContainer,
    UITextView,
    IUITextView,
    UISwitch,
    IUISwitch,
    UIView,
    IUIView,
    IUIColor,
    UIColor,
    CGRect,
    CGPoint,
    CGSize,
    id,
};

pub fn main() -> ! {
    debug_init();
    log::debug!("STARTING THE APP!");
    let event_loop = EventLoop::new();
    let window : Window = WindowBuilder::new()
        .with_title("UIKit Rust App")
        .with_maximized(true)
        .build(&event_loop)
        .expect("Failed to build window");

    let root_vc: id = unsafe {
        *(window.ui_view_controller() as *mut id)
    };

    let root_view: UIView = UIView(window.ui_view() as id );
    unsafe {
        let color = UIColor::alloc();
        let background = UIColor(color.initWithRed_green_blue_alpha_(0.1, 1.0, 2.0, 2.0));
        root_view.setBackgroundColor_(background.0);
    }

    event_loop.run(move |event: Event<()>, _, control_flow: &mut ControlFlow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                let root_view: UIView = UIView(window.ui_view() as id );
                add_views(root_view);
            }
            Event::LoopDestroyed => return,
            Event::RedrawRequested(_) => {
            }
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(_logical_size) => {
                    //window.request_redraw();
                }
                WindowEvent::Touch(_touch) => {
                },
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            _ => (),
        }
    })
}

fn add_views(root_view: UIView) {
    let rect = CGRect {
        origin: CGPoint {
            x: 10.0,
            y: 20.0
        },
        size: CGSize {
            height: 20.0,
            width: 20.0,
        }
    };
    let rect =  unsafe {
        let foo : UIView = UIView(UIView::alloc().initWithFrame_(rect));
        let background = UIColor(UIColor::yellowColor());
        foo.setBackgroundColor_(background.0);
        foo
    };
    unsafe {
        root_view.addSubview_(rect.0);
    }
    let input_rect = CGRect {
        origin: CGPoint {
            x: 10.0,
            y: 50.0
        },
        size: CGSize {
            width: 200.0,
            height: 20.0,
        }
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
                text_container.0
            )
        );
        foo
    };
    unsafe {
        root_view.addSubview_(input.0);
    }
    unsafe {
        let switch = UISwitch(UISwitch::alloc().initWithFrame_(
            CGRect {
                origin: CGPoint {
                    x: 10.0,
                    y: 80.0
                },
                size: CGSize {
                    height: 200.0,
                    width: 200.0,
                }
            }
        ));
        root_view.addSubview_(switch.0);
    }
}
fn debug_init() {
    color_backtrace::install_with_settings(
        color_backtrace::Settings::new().verbosity(color_backtrace::Verbosity::Full),
    );
    pretty_env_logger::init();
    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "full");
}

