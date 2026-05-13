use gtk::prelude::*;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    platform::unix::WindowExtUnix,
    window::WindowBuilder,
};
use wry::{
    dpi::{LogicalPosition, LogicalSize},
    Rect, WebViewBuilder, WebViewBuilderExtUnix,
};

#[derive(Debug, Clone)]
enum UserEvent {
    SwitchTab(usize),
}

fn code_placeholder_html() -> String {
    r#"<!DOCTYPE html>
<html>
<body style="background:#1a1a2e;color:#e0e0e0;display:flex;align-items:center;justify-content:center;height:100vh;margin:0;font-family:sans-serif">
<div style="text-align:center">
<h2 style="color:#e8a87c">Claude Code</h2>
<p style="color:#8892a8">Terminal integration coming next</p>
</div>
</body>
</html>"#.to_string()
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("Claude Desktop")
        .with_inner_size(tao::dpi::LogicalSize::new(1200u32, 800u32))
        .build(&event_loop)
        .unwrap();

    let vbox = window.default_vbox().unwrap();

    // --- Native GTK tab bar ---
    let tab_bar = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    tab_bar.set_widget_name("tab-bar");

    // Style the tab bar with CSS
    let css = gtk::CssProvider::new();
    css.load_from_data(
        b"
        #tab-bar {
            background-color: #16213e;
            border-bottom: 1px solid #0f3460;
        }
        #tab-bar button {
            background: none;
            border: none;
            border-bottom: 3px solid transparent;
            color: #8892a8;
            font-size: 14px;
            font-weight: 600;
            padding: 12px 24px;
            letter-spacing: 0.3px;
            border-radius: 0;
            box-shadow: none;
            outline: none;
        }
        #tab-bar button:hover {
            color: #c4c9d6;
            background-color: rgba(255,255,255,0.05);
        }
        #tab-bar button.active {
            color: #e8a87c;
            border-bottom-color: #e8a87c;
        }
        ",
    )
    .unwrap();
    gtk::StyleContext::add_provider_for_screen(
        &gtk::gdk::Screen::default().unwrap(),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let tab_names = ["Chat", "Cowork", "Code"];
    let buttons: Vec<gtk::Button> = tab_names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let btn = gtk::Button::with_label(name);
            btn.set_widget_name(&format!("tab-{}", i));
            if i == 0 {
                btn.style_context().add_class("active");
            }
            let proxy_clone = proxy.clone();
            btn.connect_clicked(move |_| {
                let _ = proxy_clone.send_event(UserEvent::SwitchTab(i));
            });
            tab_bar.pack_start(&btn, false, false, 0);
            btn
        })
        .collect();

    // Add tab bar to vbox FIRST (non-expanding)
    vbox.pack_start(&tab_bar, false, false, 0);

    // --- Content area with GtkFixed for webviews ---
    let fixed = gtk::Fixed::new();
    vbox.pack_start(&fixed, true, true, 0);
    vbox.show_all();

    let build = |builder: WebViewBuilder<'_>| -> wry::Result<wry::WebView> {
        builder.build_gtk(&fixed)
    };

    // Chat — claude.ai
    let chat = build(
        WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0u32, 0u32).into(),
                size: LogicalSize::new(1200u32, 750u32).into(),
            })
            .with_url("https://claude.ai"),
    )?;

    // Cowork — loads claude.ai upfront, just hidden
    let cowork = build(
        WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0u32, 0u32).into(),
                size: LogicalSize::new(1200u32, 750u32).into(),
            })
            .with_url("https://claude.ai"),
    )?;
    let _ = cowork.set_visible(false);

    // Code — placeholder
    let code = build(
        WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0u32, 0u32).into(),
                size: LogicalSize::new(1u32, 1u32).into(),
            })
            .with_html(code_placeholder_html()),
    )?;
    let _ = code.set_visible(false);

    let mut active_tab: usize = 0;
    let mut last_w: u32 = 0;
    let mut last_h: u32 = 0;

    fn get_view<'a>(i: usize, chat: &'a wry::WebView, cowork: &'a wry::WebView, code: &'a wry::WebView) -> &'a wry::WebView {
        match i {
            0 => chat,
            1 => cowork,
            _ => code,
        }
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::SwitchTab(idx)) => {
                if idx == active_tab {
                    return;
                }

                // Update GTK button styles
                for (i, btn) in buttons.iter().enumerate() {
                    if i == idx {
                        btn.style_context().add_class("active");
                    } else {
                        btn.style_context().remove_class("active");
                    }
                }

                // Get content area size from the fixed container
                let alloc = fixed.allocation();
                let w = alloc.width() as u32;
                let h = alloc.height() as u32;

                // Hide old
                let old = get_view(active_tab, &chat, &cowork, &code);
                let _ = old.set_visible(false);

                // Show new
                let new = get_view(idx, &chat, &cowork, &code);
                let _ = new.set_bounds(Rect {
                    position: LogicalPosition::new(0u32, 0u32).into(),
                    size: LogicalSize::new(w, h).into(),
                });
                let _ = new.set_visible(true);

                active_tab = idx;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                // Get actual content area size
                let alloc = fixed.allocation();
                let w = alloc.width() as u32;
                let h = alloc.height() as u32;

                if w == last_w && h == last_h {
                    return;
                }
                last_w = w;
                last_h = h;

                // Resize active webview to fill content area
                let active = get_view(active_tab, &chat, &cowork, &code);
                let _ = active.set_bounds(Rect {
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
