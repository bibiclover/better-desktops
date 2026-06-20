use global_hotkey::{
    GlobalHotKeyManager, 
    hotkey::{HotKey, Modifiers, Code},
};

use winit::{
    applications::ApplicationHandler,
    event::WindowEvent,
    event_loop::{self, ActiveEventLoop, EventLoop},
    window::WindowId,
};

fn main() {
    
    println!("Hello, world!");

    let manager = GlobalHotKeyManager::new().expect("Failed to intialise GlobalHotKeyManager.");

    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyB);
    
    manager.register(hotkey).unwrap();

    let event_loop = EventLoop::<AppEvent>::with_user_event().build().unwrap();
}


