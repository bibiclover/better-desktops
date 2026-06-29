use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
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

    // ---

    // hotkey (global_hotkey) setup ---
    let manager = GlobalHotKeyManager::new().expect("Failed to intialise GlobalHotKeyManager.");

    let desktops: [Desktop; 10] = std::array::from_fn(|index| {
        let code = match index {
            0 => Code::Digit1,
            1 => Code::Digit2,
            2 => Code::Digit3,
            3 => Code::Digit4,
            4 => Code::Digit5,
            5 => Code::Digit6,
            6 => Code::Digit7,
            7 => Code::Digit8,
            8 => Code::Digit9,
            9 => Code::Digit0,
            _ => unreachable!(),
        };

        Desktop {
            num: u32::try_from(index).unwrap(),
            travel_hotkey: HotKey::new(Some(Modifiers::ALT), code),
            move_hotkey: HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), code),
        }
    });

    for desktop in &desktops {
        manager.register(desktop.travel_hotkey).unwrap();
        manager.register(desktop.move_hotkey).unwrap();
    }

    let event_loop = EventLoop::<AppEvent>::with_user_event().build().unwrap();
    let proxy = event_loop.create_proxy();

    GlobalHotKeyEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(AppEvent::HotKey(event));
    }));

    let mut app = App {
        hotkeys_manager: manager,
        desktops,
    };

    event_loop.run_app(&mut app).unwrap()
    // ---
}

#[derive(Debug)]
enum AppEvent {
    HotKey(GlobalHotKeyEvent),
}

struct App {
    hotkeys_manager: GlobalHotKeyManager,
    desktops: [Desktop; 10],
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
                //println!("{:?}", event);
                if event.state != HotKeyState::Pressed {
                    return;
                }

                for desktop in &self.desktops {
                    if event.id == desktop.travel_hotkey.id {
                        desktop.switch_to();
                    } else if event.id == desktop.move_hotkey.id {
                        desktop.move_to();
                    }
                }
            }
        }
    }
}

struct Desktop {
    num: u32,
    travel_hotkey: HotKey,
    move_hotkey: HotKey,
}

impl Desktop {
    fn switch_to(&self) {
        self.create_desktops();
        switch_desktop(self.num).unwrap_or_else(|err| {
            panic!("Failed to switch to destkop {}: {:?}", self.num + 1, err)
        });
    }

    fn move_to(&self) {
        todo!()
    }

    /// Creates desktops until the required number of desktops exists.
    fn create_desktops(&self) {
        let desktop_count = get_desktop_count().expect("Failed to get desktop count.");

        if desktop_count < self.num + 1 {
            for _ in 0..=(self.num - desktop_count) {
                create_desktop().expect("Failed to create required desktops.");
            }
        }
    }
}
