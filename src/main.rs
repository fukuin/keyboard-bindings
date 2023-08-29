pub mod components;
pub mod ui;

use crate::ui::ActionItem;
use eframe::IconData;
use once_cell::sync::Lazy;
use rayon::{ThreadPool, ThreadPoolBuilder};
use rdev::{simulate, EventType, Key};
use std::{sync::Arc, thread, time::Duration};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

pub static POOL: Lazy<Arc<ThreadPool>> = Lazy::new(|| {
    let pool = ThreadPoolBuilder::new().num_threads(2).build().unwrap();
    Arc::new(pool)
});
pub static mut LISTEN: bool = false; //是否监听键盘事件的全局变量
pub static mut ALREADY_LISTEN: bool = false;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        always_on_top: true,
        centered: true,
        icon_data: Some(
            IconData::try_from_png_bytes(&include_bytes!("../logo/rustacean-flat-happy.png")[..])
                .unwrap(),
        ),
        ..Default::default()
    };
    eframe::run_native(
        "Keyboard Bind",
        options,
        Box::new(|_cc| Box::<ui::App>::default()),
    )
}

//监听输入按键并执行按键组合
pub fn listen_active(app: ui::App) {
    let mut hkm = HotkeyManager::new();
    for item in app.combination {
        match hkm.register(item.input, &[], move || {
            for key in &item.output {
                match *key {
                    ActionItem::Press(k) => press(k),
                    ActionItem::Delay(t) => {
                        thread::sleep(Duration::from_millis(t));
                    }
                };
            }
        }) {
            Ok(_) => {}
            Err(err) => {
                dbg!(err);
            }
        }
    }

    let handle = hkm.interrupt_handle();

    POOL.spawn(move || {
        hkm.event_loop();
        unsafe { ALREADY_LISTEN = false }
    });

    POOL.spawn(move || loop {
        if !unsafe { LISTEN } {
            handle.interrupt();
            break;
        }
    });
}

#[inline(always)]
fn press(key: Key) {
    simulate(&EventType::KeyPress(key)).unwrap();
    simulate(&EventType::KeyRelease(key)).unwrap();
}
