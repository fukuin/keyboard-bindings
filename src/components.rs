use eframe::egui;
use egui::{InnerResponse, Response};
use font_kit::source::SystemSource;
use rdev::Key;
use windows_hotkeys::keys::VKey;

//switch按钮组件
pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool, enabled: bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() && enabled {
        *on = !*on;
        response.mark_changed(); // report back that the value changed
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));
    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(0.0);
        let radius = 0.5 * rect.height();
        if enabled {
            ui.painter()
                .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        } else {
            ui.painter().rect(
                rect,
                radius,
                egui::Color32::from_gray(220),
                egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(60, 60, 60, 120)),
            );
        }
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        if enabled {
            ui.painter()
                .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        } else {
            ui.painter().circle(
                center,
                0.75 * radius,
                egui::Color32::from_gray(220),
                egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(60, 60, 60, 120)),
            );
        }
    }
    response
}

//select下拉菜单
pub fn select_table_vkey(
    ui: &mut egui::Ui,
    id: String,
    item: &mut VKey,
) -> InnerResponse<Option<()>> {
    egui::ComboBox::from_id_source(id)
        .width(65.0)
        .selected_text(format!("{:?}", item))
        .show_ui(ui, |ui| {
            ui.selectable_value(item, VKey::A, "A");
            ui.selectable_value(item, VKey::B, "B");
            ui.selectable_value(item, VKey::C, "C");
            ui.selectable_value(item, VKey::D, "D");
            ui.selectable_value(item, VKey::E, "E");
            ui.selectable_value(item, VKey::F, "F");
            ui.selectable_value(item, VKey::G, "G");
            ui.selectable_value(item, VKey::H, "H");
            ui.selectable_value(item, VKey::I, "I");
            ui.selectable_value(item, VKey::J, "J");
            ui.selectable_value(item, VKey::K, "K");
            ui.selectable_value(item, VKey::L, "L");
            ui.selectable_value(item, VKey::M, "M");
            ui.selectable_value(item, VKey::N, "N");
            ui.selectable_value(item, VKey::O, "O");
            ui.selectable_value(item, VKey::P, "P");
            ui.selectable_value(item, VKey::Q, "Q");
            ui.selectable_value(item, VKey::R, "R");
            ui.selectable_value(item, VKey::S, "S");
            ui.selectable_value(item, VKey::T, "T");
            ui.selectable_value(item, VKey::U, "U");
            ui.selectable_value(item, VKey::V, "V");
            ui.selectable_value(item, VKey::W, "W");
            ui.selectable_value(item, VKey::X, "X");
            ui.selectable_value(item, VKey::Y, "Y");
            ui.selectable_value(item, VKey::Z, "Z");
            ui.selectable_value(item, VKey::Vk0, "0");
            ui.selectable_value(item, VKey::Vk1, "1");
            ui.selectable_value(item, VKey::Vk2, "2");
            ui.selectable_value(item, VKey::Vk3, "3");
            ui.selectable_value(item, VKey::Vk4, "4");
            ui.selectable_value(item, VKey::Vk5, "5");
            ui.selectable_value(item, VKey::Vk6, "6");
            ui.selectable_value(item, VKey::Vk7, "7");
            ui.selectable_value(item, VKey::Vk8, "8");
            ui.selectable_value(item, VKey::Vk9, "9");
            ui.selectable_value(item, VKey::F1, "F1");
            ui.selectable_value(item, VKey::F2, "F2");
            ui.selectable_value(item, VKey::F3, "F3");
            ui.selectable_value(item, VKey::F4, "F4");
            ui.selectable_value(item, VKey::F5, "F5");
            ui.selectable_value(item, VKey::F6, "F6");
            ui.selectable_value(item, VKey::F7, "F7");
            ui.selectable_value(item, VKey::F8, "F8");
            ui.selectable_value(item, VKey::F9, "F9");
            ui.selectable_value(item, VKey::F10, "F10");
            ui.selectable_value(item, VKey::F11, "F11");
            ui.selectable_value(item, VKey::F12, "F12");
            ui.selectable_value(item, VKey::Numpad0, "Num0");
            ui.selectable_value(item, VKey::Numpad1, "Num1");
            ui.selectable_value(item, VKey::Numpad2, "Num2");
            ui.selectable_value(item, VKey::Numpad3, "Num3");
            ui.selectable_value(item, VKey::Numpad4, "Num4");
            ui.selectable_value(item, VKey::Numpad5, "Num5");
            ui.selectable_value(item, VKey::Numpad6, "Num6");
            ui.selectable_value(item, VKey::Numpad7, "Num7");
            ui.selectable_value(item, VKey::Numpad8, "Num8");
            ui.selectable_value(item, VKey::Numpad9, "Num9");
            ui.selectable_value(item, VKey::Up, "Up");
            ui.selectable_value(item, VKey::Down, "Down");
            ui.selectable_value(item, VKey::Left, "Left");
            ui.selectable_value(item, VKey::Right, "Right");
            ui.selectable_value(item, VKey::Space, "Space");
        })
}

pub fn select_table_key(ui: &mut egui::Ui, id: String, item: &mut Key) -> Response {
    egui::ComboBox::from_id_source(id)
        .width(65.0)
        .selected_text(format!("{:?}", item))
        .show_ui(ui, |ui| {
            ui.selectable_value(item, Key::KeyA, "A");
            ui.selectable_value(item, Key::KeyB, "B");
            ui.selectable_value(item, Key::KeyC, "C");
            ui.selectable_value(item, Key::KeyD, "D");
            ui.selectable_value(item, Key::KeyE, "E");
            ui.selectable_value(item, Key::KeyF, "F");
            ui.selectable_value(item, Key::KeyG, "G");
            ui.selectable_value(item, Key::KeyH, "H");
            ui.selectable_value(item, Key::KeyI, "I");
            ui.selectable_value(item, Key::KeyJ, "J");
            ui.selectable_value(item, Key::KeyK, "K");
            ui.selectable_value(item, Key::KeyL, "L");
            ui.selectable_value(item, Key::KeyM, "M");
            ui.selectable_value(item, Key::KeyN, "N");
            ui.selectable_value(item, Key::KeyO, "O");
            ui.selectable_value(item, Key::KeyP, "P");
            ui.selectable_value(item, Key::KeyQ, "Q");
            ui.selectable_value(item, Key::KeyR, "R");
            ui.selectable_value(item, Key::KeyS, "S");
            ui.selectable_value(item, Key::KeyT, "T");
            ui.selectable_value(item, Key::KeyU, "U");
            ui.selectable_value(item, Key::KeyV, "V");
            ui.selectable_value(item, Key::KeyW, "W");
            ui.selectable_value(item, Key::KeyX, "X");
            ui.selectable_value(item, Key::KeyY, "Y");
            ui.selectable_value(item, Key::KeyZ, "Z");
            ui.selectable_value(item, Key::Kp0, "0");
            ui.selectable_value(item, Key::Kp1, "1");
            ui.selectable_value(item, Key::Kp2, "2");
            ui.selectable_value(item, Key::Kp3, "3");
            ui.selectable_value(item, Key::Kp4, "4");
            ui.selectable_value(item, Key::Kp5, "5");
            ui.selectable_value(item, Key::Kp6, "6");
            ui.selectable_value(item, Key::Kp7, "7");
            ui.selectable_value(item, Key::Kp8, "8");
            ui.selectable_value(item, Key::Kp9, "9");
            ui.selectable_value(item, Key::F1, "F1");
            ui.selectable_value(item, Key::F2, "F2");
            ui.selectable_value(item, Key::F3, "F3");
            ui.selectable_value(item, Key::F4, "F4");
            ui.selectable_value(item, Key::F5, "F5");
            ui.selectable_value(item, Key::F6, "F6");
            ui.selectable_value(item, Key::F7, "F7");
            ui.selectable_value(item, Key::F8, "F8");
            ui.selectable_value(item, Key::F9, "F9");
            ui.selectable_value(item, Key::F10, "F10");
            ui.selectable_value(item, Key::F11, "F11");
            ui.selectable_value(item, Key::F12, "F12");
            ui.selectable_value(item, Key::Num0, "Num0");
            ui.selectable_value(item, Key::Num1, "Num1");
            ui.selectable_value(item, Key::Num2, "Num2");
            ui.selectable_value(item, Key::Num3, "Num3");
            ui.selectable_value(item, Key::Num4, "Num4");
            ui.selectable_value(item, Key::Num5, "Num5");
            ui.selectable_value(item, Key::Num6, "Num6");
            ui.selectable_value(item, Key::Num7, "Num7");
            ui.selectable_value(item, Key::Num8, "Num8");
            ui.selectable_value(item, Key::Num9, "Num9");
            ui.selectable_value(item, Key::UpArrow, "Up");
            ui.selectable_value(item, Key::DownArrow, "Down");
            ui.selectable_value(item, Key::LeftArrow, "Left");
            ui.selectable_value(item, Key::RightArrow, "Right");
            ui.selectable_value(item, Key::Space, "Space");
        })
        .response
}

//加载中文
pub fn load_fonts(ctx: &egui::Context) {
    let sys = SystemSource::new();
    let font_name = "SimHei".to_string();
    let font = sys.select_family_by_name(&font_name).unwrap().fonts()[0]
        .load()
        .unwrap()
        .copy_font_data()
        .unwrap()
        .to_vec();
    let mut font_defs = egui::FontDefinitions::default();
    font_defs
        .font_data
        .insert(font_name.to_string(), egui::FontData::from_owned(font));
    font_defs
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, font_name);
    ctx.set_fonts(font_defs);
}
