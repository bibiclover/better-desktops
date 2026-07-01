#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

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

use winvd::{create_desktop, get_desktop_count, move_window_to_desktop, switch_desktop};

use windows::Win32::UI::WindowsAndMessaging::{
    GetDesktopWindow, GetForegroundWindow, GetShellWindow,
};

fn main() {
    let manager = GlobalHotKeyManager::new().expect("Failed to intialise GlobalHotKeyManager.");

    let mut map: HashMap<u32, Action> = HashMap::new();

    for number_key in 0..10 {
        let code = match number_key {
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

        let travel_hotkey = HotKey::new(Some(Modifiers::ALT), code);
        map.insert(
            travel_hotkey.id,
            Action::Travel(Travel {
                desktop_num: number_key,
            }),
        );
        manager.register(travel_hotkey).unwrap();

        let move_hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), code);
        map.insert(
            move_hotkey.id,
            Action::Move(Move {
                desktop_num: number_key,
            }),
        );
        manager.register(move_hotkey).unwrap();
    }

    let map = map;

    let event_loop = EventLoop::<AppEvent>::with_user_event().build().unwrap();
    let proxy = event_loop.create_proxy();

    GlobalHotKeyEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(AppEvent::HotKey(event));
    }));

    let mut app = App { manager, map };

    event_loop.run_app(&mut app).unwrap()
}

#[derive(Debug)]
enum AppEvent {
    HotKey(GlobalHotKeyEvent),
}

struct App {
    #[allow(dead_code)]
    manager: GlobalHotKeyManager,
    map: HashMap<u32, Action>,
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
                println!("{:?}", event);
                if event.state != HotKeyState::Pressed {
                    return;
                }

                let action = self.map.get(&event.id).unwrap();

                match action {
                    Action::Move(x) => x.execute(),
                    Action::Travel(x) => x.execute(),
                }
            }
        }
    }
}

// struct Desktop {
//     num: u32,
//     travel_hotkey: HotKey,
//     move_hotkey: HotKey,
// }

enum Action {
    Travel(Travel),
    Move(Move),
}

trait ActionBehaviour {
    fn execute(&self);

    /// Creates desktops until the required number of desktops exists.
    fn create_desktops(&self, desktop_num: u32) {
        let desktop_count = get_desktop_count().expect("Failed to get desktop count.");

        if desktop_count < desktop_num + 1 {
            for _ in 0..=(desktop_num - desktop_count) {
                create_desktop().expect("Failed to create required desktops.");
            }
        }
    }
}

struct Travel {
    desktop_num: u32,
}

impl ActionBehaviour for Travel {
    fn execute(&self) {
        self.create_desktops(self.desktop_num);
        switch_desktop(self.desktop_num).unwrap_or_else(|err| {
            panic!(
                "Failed to switch to desktop {}: {:?}",
                self.desktop_num + 1,
                err
            )
        });
    }
}

struct Move {
    desktop_num: u32,
}

impl ActionBehaviour for Move {
    fn execute(&self) {
        self.create_desktops(self.desktop_num);
        let hwnd = unsafe { GetForegroundWindow() };

        if hwnd.is_invalid() {
            eprintln!("Foreground window handle is not valid.");
            return;
        }

        if hwnd == unsafe { GetDesktopWindow() } || hwnd == unsafe { GetShellWindow() } {
            eprintln!("Desktop is in focus. Can't move.");
            return;
        }

        if let Err(e) = move_window_to_desktop(self.desktop_num, &hwnd) {
            eprintln!("Failed to move window {:?}: {:?}", &hwnd, e);
            return;
        }
    }
}
