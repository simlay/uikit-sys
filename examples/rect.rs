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
    struct_NSTextContainer,
    interface_NSTextContainer,
    struct_UITextView,
    interface_UITextView,
    struct_UIView,
    interface_UIView,
    interface_UIColor,
    struct_UIColor,
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
        .build(&event_loop)
        .expect("Failed to build window");

    let root_vc: id = unsafe {
        *(window.ui_view_controller() as *mut id)
    };

    let root_view: struct_UIView = struct_UIView(window.ui_view() as id );
    unsafe {
        let color = struct_UIColor::alloc();
        let background = struct_UIColor(color.initWithRed_green_blue_alpha_(0.1, 1.0, 2.0, 2.0));
        root_view.setBackgroundColor_(background.0);
    }

    event_loop.run(move |event: Event<()>, _, control_flow: &mut ControlFlow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                let root_view: struct_UIView = struct_UIView(window.ui_view() as id );
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

fn add_views(root_view: struct_UIView) {
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
        let foo : struct_UIView = struct_UIView(struct_UIView::alloc().initWithFrame_(rect));
        let background = struct_UIColor(struct_UIColor::yellowColor());
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
            height: 200.0,
            width: 200.0,
        }
    };
    let input = unsafe {
        let text_container = struct_NSTextContainer(
            struct_NSTextContainer::alloc().initWithSize_(
                CGSize {
                    height: 100.0,
                    width: 200.0,
                }
            )
        );
        let foo = struct_UITextView(
            struct_UITextView::alloc().initWithFrame_textContainer_(
                input_rect,
                text_container.0
            )
        );
        foo
    };
    unsafe {
        root_view.addSubview_(input.0);
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

