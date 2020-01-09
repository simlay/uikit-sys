extern crate uikit_sys;
#[macro_use]
extern crate log;
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
    UIView,
    UIViewController,
    UIColor,
};

use objc::runtime::Object as id;
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

    /*
    let root_vc: id = unsafe {
        *(window.ui_view_controller() as *mut id)
    };
    let root_vc: ShareId<UIViewController> = unsafe {
        Id::from_retained_ptr(&mut *(window.ui_view_controller() as * mut UIViewController))
    }.share();
    */
    //root_vc.view().set_background_color(UIColor::from_rgba(0., 1., 0., 1.).share());

    //root_vc.view().set_background_color(UIColor::colorWithDisplayP3Red_green_blue_alpha_(0., 1., 0., 1.));

    event_loop.run(move |event: Event<()>, _, control_flow: &mut ControlFlow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                //add_views(&root_vc);
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
