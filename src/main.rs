use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager,
    hotkey::{Code, HotKey, Modifiers},
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

use winvd::{create_desktop, get_desktop_count, switch_desktop};

fn main() {
    // desktop (winvd) setup ---
    create_desktops(10);

    // ---

    // hotkey (global_hotkey) setup ---
    let manager = GlobalHotKeyManager::new().expect("Failed to intialise GlobalHotKeyManager.");

    let hotkey = HotKey::new(Some(Modifiers::ALT), Code::Digit0);

    manager.register(hotkey).unwrap();

    let event_loop = EventLoop::<AppEvent>::with_user_event().build().unwrap();
    let proxy = event_loop.create_proxy();

    GlobalHotKeyEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(AppEvent::HotKey(event));
    }));

    let mut app = App {
        hotkeys_manager: manager,
        hotkey,
    };

    event_loop.run_app(&mut app).unwrap()
    // ---
}

/// Creates desktops until a certain number of desktops exists.
/// i.e. if to_create is 10, and there currently is 8 desktops, 2 more will be created.
/// if to_create is 10, and there are currently 12 desktops, nothing will happen.
fn create_desktops(to_create: u32) {
    let desktop_count = get_desktop_count().expect("Failed to get desktop count.");

    if desktop_count < to_create {
        for _ in 1..(desktop_count - to_create) {
            create_desktop().expect("Failed to create required desktops.");
        }
    }
}

#[derive(Debug)]
enum AppEvent {
    HotKey(GlobalHotKeyEvent),
}

struct App {
    hotkeys_manager: GlobalHotKeyManager,
    hotkey: HotKey,
}

impl ApplicationHandler<AppEvent> for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::HotKey(event) => {
                println!("{event:?}");
            }
        }
    }
}

struct Desktop {
    num: i32,
    hotkey: HotKey,
}

impl Desktop {
    fn switch_to(self) {
        switch_desktop(self.num)
            .unwrap_or_else(|err| panic!("Failed to switch to destkop {}: {:?}", self.num, err));
    }
}
