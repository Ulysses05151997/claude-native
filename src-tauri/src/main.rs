use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowExtUnix,
    window::WindowBuilder,
};
use wry::{
    dpi::{LogicalPosition, LogicalSize},
    Rect, WebViewBuilder, WebViewBuilderExtUnix,
};

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Claude Desktop")
        .with_inner_size(tao::dpi::LogicalSize::new(1200u32, 800u32))
        .build(&event_loop)
        .unwrap();

    let vbox = window.default_vbox().unwrap();

    // Single content container — webview fills entire window
    let fixed = gtk::Fixed::new();
    gtk::prelude::BoxExt::pack_start(vbox, &fixed, true, true, 0);
    gtk::prelude::WidgetExt::show_all(vbox);

    // One webview, one session — claude.ai handles its own tabs
    let webview = WebViewBuilder::new()
        .with_bounds(Rect {
            position: LogicalPosition::new(0u32, 0u32).into(),
            size: LogicalSize::new(1200u32, 800u32).into(),
        })
        .with_url("https://claude.ai")
        .build_gtk(&fixed)?;

    let mut last_w: u32 = 0;
    let mut last_h: u32 = 0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                let alloc = gtk::prelude::WidgetExt::allocation(&fixed);
                let w = alloc.width() as u32;
                let h = alloc.height() as u32;

                if w == last_w && h == last_h {
                    return;
                }
                last_w = w;
                last_h = h;

                let _ = webview.set_bounds(Rect {
                    position: LogicalPosition::new(0u32, 0u32).into(),
                    size: LogicalSize::new(w, h).into(),
                });
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
