use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    platform::unix::WindowExtUnix,
    window::WindowBuilder,
};
use wry::{
    NewWindowResponse, WebContext, WebViewBuilder, WebViewBuilderExtUnix, WebViewExtUnix,
};

#[derive(Debug)]
enum UserEvent {
    Navigate(String),
    OpenPopup(String),
    ClosePopup,
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

    // Persistent cookie and session storage
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/home/isaulysses/.local/share"))
        .join("claude-native")
        .join("webdata");
    std::fs::create_dir_all(&data_dir).ok();
    let mut web_context = WebContext::new(Some(data_dir));

    // Navigation bar — Chat and Code buttons
    let nav_bar = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    gtk::prelude::WidgetExt::set_margin_start(&nav_bar, 8);
    gtk::prelude::WidgetExt::set_margin_top(&nav_bar, 4);
    gtk::prelude::WidgetExt::set_margin_bottom(&nav_bar, 4);

    let chat_btn = gtk::Button::with_label("Chat");
    let code_btn = gtk::Button::with_label("Code");

    let chat_proxy = proxy.clone();
    gtk::prelude::ButtonExt::connect_clicked(&chat_btn, move |_| {
        let _ = chat_proxy.send_event(UserEvent::Navigate("https://claude.ai".into()));
    });
    let code_proxy = proxy.clone();
    gtk::prelude::ButtonExt::connect_clicked(&code_btn, move |_| {
        let _ = code_proxy.send_event(UserEvent::Navigate("https://claude.ai/code".into()));
    });

    gtk::prelude::BoxExt::pack_start(&nav_bar, &chat_btn, false, false, 0);
    gtk::prelude::BoxExt::pack_start(&nav_bar, &code_btn, false, false, 0);
    gtk::prelude::BoxExt::pack_start(vbox, &nav_bar, false, false, 0);

    let popup_proxy = proxy.clone();
    let webview = WebViewBuilder::new_with_web_context(&mut web_context)
        .with_url("https://claude.ai")
        .with_new_window_req_handler(move |url, _features| {
            eprintln!("[new-window] {}", url);
            if url.starts_with("https://claude.ai") {
                let _ = popup_proxy.send_event(UserEvent::Navigate(url));
            } else {
                let _ = popup_proxy.send_event(UserEvent::OpenPopup(url));
            }
            NewWindowResponse::Deny
        })
        .build_gtk(vbox)?;

    gtk::prelude::WidgetExt::show_all(vbox);

    // Grab the underlying webkit2gtk view so popup can share the web process
    let main_webkit_view = webview.webview();

    // Popup state — window + webview kept alive until closed
    let popup_window: Arc<Mutex<Option<tao::window::Window>>> = Arc::new(Mutex::new(None));
    let popup_webview: Arc<Mutex<Option<wry::WebView>>> = Arc::new(Mutex::new(None));

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;
        let _ = &web_context;

        match event {
            Event::UserEvent(UserEvent::Navigate(url)) => {
                let _ = webview.load_url(&url);
            }
            Event::UserEvent(UserEvent::OpenPopup(url)) => {
                popup_webview.lock().unwrap().take();
                popup_window.lock().unwrap().take();

                let popup_win = WindowBuilder::new()
                    .with_title("Sign In")
                    .with_inner_size(tao::dpi::LogicalSize::new(500u32, 700u32))
                    .build(event_loop)
                    .unwrap();

                let popup_vbox = popup_win.default_vbox().unwrap();

                let close_proxy = proxy.clone();
                let popup_wv = WebViewBuilder::new()
                    .with_url(&url)
                    .with_navigation_handler(move |nav_url| {
                        if nav_url.starts_with("https://claude.ai")
                            && !nav_url.contains("accounts.google")
                        {
                            let _ = close_proxy.send_event(UserEvent::ClosePopup);
                        }
                        true
                    })
                    .with_related_view(main_webkit_view.clone())
                    .build_gtk(popup_vbox);

                if let Ok(wv) = popup_wv {
                    gtk::prelude::WidgetExt::show_all(popup_vbox);
                    *popup_webview.lock().unwrap() = Some(wv);
                    *popup_window.lock().unwrap() = Some(popup_win);
                }
            }
            Event::UserEvent(UserEvent::ClosePopup) => {
                popup_webview.lock().unwrap().take();
                popup_window.lock().unwrap().take();
                let _ = webview.load_url("https://claude.ai");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                if window_id == window.id() {
                    *control_flow = ControlFlow::Exit;
                } else {
                    popup_webview.lock().unwrap().take();
                    popup_window.lock().unwrap().take();
                }
            }
            _ => {}
        }
    });
}
