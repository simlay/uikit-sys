extern crate uikit_sys;
#[macro_use]
extern crate log;
#[macro_use]
extern crate objc;
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
    struct_UIView,
    interface_UIColor,
    struct_UIColor,
    CGRect,
    CGPoint,
    CGSize,
    id,
};

use objc::runtime::{
    Object,
    Class,
};
use objc_id::{
    ShareId,
    Id,
};

fn run_winit() -> ! {
    let event_loop = EventLoop::new();
    let window : Window = WindowBuilder::new()
        .with_title("UIKit Rust App")
        .build(&event_loop)
        .expect("Failed to build window");

    let root_vc: id = unsafe {
        *(window.ui_view_controller() as *mut id)
    };
    /*
    let root_view: Id<Object> = unsafe {
        Id::from_ptr(*(window.ui_view() as *mut id))
    };
    */
    let root_view: struct_UIView = struct_UIView(unsafe { *(window.ui_view() as *mut id) });
    unsafe {
        let color = struct_UIColor::alloc();
        let foo = color.initWithRed_green_blue_alpha_(0.1, 1.0, 2.0, 2.0);
        let color = struct_UIColor(*foo);

        //let color: id = msg_send![color, initWithRed: 0.1 green: 0.1 blue: 1.0 alpha: 0.0];
        //let _ : id = msg_send![root_view, setBackgroundColor: color];
        root_view.drawRect_(
            CGRect {
                origin: CGPoint {
                    x: 10.0,
                    y: 20.0
                },
                size: CGSize {
                    height: 200.0,
                    width: 200.0,
                }
            }
        );
        //let _ : id = msg_send![root_view, setBackgroundColor: color];
    }
    //root_vc.view().set_background_color(UIColor::from_rgba(0., 1., 0., 1.).share());

    event_loop.run(move |event: Event<()>, _, control_flow: &mut ControlFlow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                //add_views(root_view);
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

fn add_views(root_view: Id<Object>) {

    //let view : id = unsafe { root_vc.view() as id };
    unsafe {
        //let cls = Class::get("UIColor").unwrap();
        //let color: id = msg_send![cls, alloc];

        //let color: id = msg_send![color, initWithRed: 0.1 green: 0.1 blue: 1.0 alpha: 0.0];
        //let color: id = *color.initWithRed_green_blue_alpha_(0.1, 1.0, 2.0, 2.0);
        //let _ : id = msg_send![root_view, setBackgroundColor: color];
        //(&mut (*root_view)).setBackgroundColor_(color);
        /*
        root_view.drawRect_(
            CGRect {
                origin: CGPoint {
                    x: 10.0,
                    y: 20.0
                },
                size: CGSize {
                    height: 200.0,
                    width: 200.0,
                }
            }
        );
        */
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

fn main() -> ! {
    debug_init();
    debug!("STARTING THE APP!");
    run_winit()
    //uikit_impl::application_main(ExampleAppDelegate)
}

#[no_mangle]
pub extern "C" fn run_app() {
    color_backtrace::install();
    main();
}
