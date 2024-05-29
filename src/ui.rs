use std::fmt::Debug;

use crate::{
    components::{load_fonts, select_table_key, select_table_vkey, toggle_ui},
    listen_active,
    record::record_ui,
    ALREADY_LISTEN, LISTEN,
};
use eframe::egui;
use egui_dnd::dnd;
use nanoid::nanoid;
use rdev::Key;
use windows_hotkeys::keys::VKey;

#[derive(Debug, Clone, Default)]
pub struct App {
    pub keys_item: Vec<(ListItem, ListItem)>, //(本身,上一次保存的状态)
    pub combination: Vec<Combo>,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct Combo {
    pub input: VKey,
    pub output: Vec<ActionItem>,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub id: String,
    pub name: String,
    pub input_key: VKey,
    pub output: Vec<(ActionItem, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionItem {
    Press(Key),
    Delay(u64),
}

impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.input_key == other.input_key
            && self.output.iter().eq(other.output.iter())
    }
}

impl ToString for ActionItem {
    fn to_string(&self) -> String {
        match self {
            ActionItem::Press(key) => match key {
                Key::Alt => String::from("Alt"),
                Key::AltGr => String::from("AltGr"),
                Key::Backspace => String::from("Backspace"),
                Key::CapsLock => String::from("CapsLock"),
                Key::ControlLeft => String::from("ControlLeft"),
                Key::ControlRight => String::from("ControlRight"),
                Key::Delete => String::from("Delete"),
                Key::DownArrow => String::from("DownArrow"),
                Key::End => String::from("End"),
                Key::Escape => String::from("Escape"),
                Key::F1 => String::from("F1"),
                Key::F10 => String::from("F10"),
                Key::F11 => String::from("F11"),
                Key::F12 => String::from("F12"),
                Key::F2 => String::from("F2"),
                Key::F3 => String::from("F3"),
                Key::F4 => String::from("F4"),
                Key::F5 => String::from("F5"),
                Key::F6 => String::from("F6"),
                Key::F7 => String::from("F7"),
                Key::F8 => String::from("F8"),
                Key::F9 => String::from("F9"),
                Key::Home => String::from("Home"),
                Key::LeftArrow => String::from("LeftArrow"),
                Key::MetaLeft => String::from("MetaLeft"),
                Key::MetaRight => String::from("MetaRight"),
                Key::PageDown => String::from("PageDown"),
                Key::PageUp => String::from("PageUp"),
                Key::Return => String::from("Return"),
                Key::RightArrow => String::from("RightArrow"),
                Key::ShiftLeft => String::from("ShiftLeft"),
                Key::ShiftRight => String::from("ShiftRight"),
                Key::Space => String::from("Space"),
                Key::Tab => String::from("Tab"),
                Key::UpArrow => String::from("UpArrow"),
                Key::PrintScreen => String::from("PrintScreen"),
                Key::ScrollLock => String::from("ScrollLock"),
                Key::Pause => String::from("Pause"),
                Key::NumLock => String::from("NumLock"),
                Key::BackQuote => String::from("BackQuote"),
                Key::Num1 => String::from("Num1"),
                Key::Num2 => String::from("Num2"),
                Key::Num3 => String::from("Num3"),
                Key::Num4 => String::from("Num4"),
                Key::Num5 => String::from("Num5"),
                Key::Num6 => String::from("Num6"),
                Key::Num7 => String::from("Num7"),
                Key::Num8 => String::from("Num8"),
                Key::Num9 => String::from("Num9"),
                Key::Num0 => String::from("Num0"),
                Key::Minus => String::from("Minus"),
                Key::Equal => String::from("Equal"),
                Key::KeyQ => String::from("KeyQ"),
                Key::KeyW => String::from("KeyW"),
                Key::KeyE => String::from("KeyE"),
                Key::KeyR => String::from("KeyR"),
                Key::KeyT => String::from("KeyT"),
                Key::KeyY => String::from("KeyY"),
                Key::KeyU => String::from("KeyU"),
                Key::KeyI => String::from("KeyI"),
                Key::KeyO => String::from("KeyO"),
                Key::KeyP => String::from("KeyP"),
                Key::LeftBracket => String::from("LeftBracket"),
                Key::RightBracket => String::from("RightBracket"),
                Key::KeyA => String::from("KeyA"),
                Key::KeyS => String::from("KeyS"),
                Key::KeyD => String::from("KeyD"),
                Key::KeyF => String::from("KeyF"),
                Key::KeyG => String::from("KeyG"),
                Key::KeyH => String::from("KeyH"),
                Key::KeyJ => String::from("KeyJ"),
                Key::KeyK => String::from("KeyK"),
                Key::KeyL => String::from("KeyL"),
                Key::SemiColon => String::from("SemiColon"),
                Key::Quote => String::from("Quote"),
                Key::BackSlash => String::from("BackSlash"),
                Key::IntlBackslash => String::from("IntlBackslash"),
                Key::KeyZ => String::from("KeyZ"),
                Key::KeyX => String::from("KeyX"),
                Key::KeyC => String::from("KeyC"),
                Key::KeyV => String::from("KeyV"),
                Key::KeyB => String::from("KeyB"),
                Key::KeyN => String::from("KeyN"),
                Key::KeyM => String::from("KeyM"),
                Key::Comma => String::from("Comma"),
                Key::Dot => String::from("Dot"),
                Key::Slash => String::from("Slash"),
                Key::Insert => String::from("Insert"),
                Key::KpReturn => String::from("KpReturn"),
                Key::KpMinus => String::from("KpMinus"),
                Key::KpPlus => String::from("KpPlus"),
                Key::KpMultiply => String::from("KpMultiply"),
                Key::KpDivide => String::from("KpDivide"),
                Key::Kp0 => String::from("Kp0"),
                Key::Kp1 => String::from("Kp1"),
                Key::Kp2 => String::from("Kp2"),
                Key::Kp3 => String::from("Kp3"),
                Key::Kp4 => String::from("Kp4"),
                Key::Kp5 => String::from("Kp5"),
                Key::Kp6 => String::from("Kp6"),
                Key::Kp7 => String::from("Kp7"),
                Key::Kp8 => String::from("Kp8"),
                Key::Kp9 => String::from("Kp9"),
                Key::KpDelete => String::from("KpDelete"),
                Key::Function => String::from("Function"),
                Key::Unknown(key_code) => key_code.to_string(),
            },
            ActionItem::Delay(time) => time.to_string(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        load_fonts(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            self.enabled = !self.keys_item.is_empty();
            ui.horizontal_top(|ui| {
                ui.add(egui::Label::new("按键绑定").selectable(false));
                if ui
                    .button("添加")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    self.keys_item.push((
                        ListItem {
                            id: nanoid!(),
                            name: Default::default(),
                            input_key: VKey::A,
                            output: vec![],
                        },
                        ListItem {
                            id: nanoid!(),
                            name: Default::default(),
                            input_key: VKey::A,
                            output: vec![],
                        },
                    ));
                }
                if ui
                    .button("清空")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    self.keys_item = vec![];
                    self.combination = vec![];
                    self.enabled = false;
                }
            });
            ui.separator(); //分割线
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_enabled_ui(!unsafe { LISTEN }, |ui| {
                    let mut is_delete = false;
                    let mut delede_id = String::new();
                    egui::Grid::new("list").striped(true).show(ui, |ui| {
                        let mut deleted_index = 0;
                        for (index, item) in self.keys_item.clone().iter_mut().enumerate() {
                            // 每一组
                            ui.vertical(|ui| {
                                // 操作区域
                                ui.horizontal(|ui| {
                                    ui.label(index.to_string() + ":");
                                    ui.add(
                                        egui::TextEdit::singleline(
                                            &mut self.keys_item[index].0.name,
                                        )
                                        .desired_width(60.0)
                                        .hint_text("组名"),
                                    );
                                    ui.label("监听按钮:");
                                    select_table_vkey(
                                        ui,
                                        index.to_string(),
                                        &mut self.keys_item[index].0.input_key,
                                    );

                                    if ui
                                        .button("添加执行按钮")
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .clicked()
                                    {
                                        self.keys_item[index]
                                            .0
                                            .output
                                            .push((ActionItem::Press(Key::KeyA), nanoid!()));
                                        //默认值
                                    };
                                    if ui
                                        .button("添加延时(ms)")
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .clicked()
                                    {
                                        self.keys_item[index]
                                            .0
                                            .output
                                            .push((ActionItem::Delay(0), nanoid!()));
                                    };
                                    let mut del_delay_index = 0;
                                    let mut can_del = false;
                                    for (i, o) in
                                        self.keys_item[index].0.output.iter_mut().enumerate()
                                    {
                                        match o.0 {
                                            ActionItem::Press(ref mut k) => {
                                                select_table_key(ui, format!("{index}-{i}"), k)
                                                    .context_menu(|ui| {
                                                        if ui
                                                            .add(egui::Button::new("删除"))
                                                            .on_hover_cursor(
                                                                egui::CursorIcon::PointingHand,
                                                            )
                                                            .clicked()
                                                        {
                                                            del_delay_index = i;
                                                            can_del = true;
                                                            ui.close_menu();
                                                        }
                                                    });
                                            }
                                            ActionItem::Delay(ref mut value) => {
                                                ui.add(egui::DragValue::new(value)).context_menu(
                                                    |ui| {
                                                        if ui
                                                            .add(egui::Button::new("删除"))
                                                            .on_hover_cursor(
                                                                egui::CursorIcon::PointingHand,
                                                            )
                                                            .clicked()
                                                        {
                                                            del_delay_index = i;
                                                            can_del = true;
                                                            ui.close_menu();
                                                        }
                                                    },
                                                );
                                            }
                                        }
                                    }
                                    if can_del {
                                        self.keys_item[index].0.output.remove(del_delay_index);
                                        can_del = false;
                                    }

                                    ui.add_enabled_ui(
                                        !self.keys_item[index].0.eq(&self.keys_item[index].1),
                                        |ui| {
                                            if ui
                                                .button("保存")
                                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                                .clicked()
                                            {
                                                let c = Combo {
                                                    input: self.keys_item[index].0.input_key,
                                                    output: self.keys_item[index]
                                                        .0
                                                        .output
                                                        .iter()
                                                        .map(|i| i.0)
                                                        .collect(),
                                                    id: item.0.id.to_string(),
                                                };
                                                let mut has = 0;
                                                if self.combination.is_empty() {
                                                    has = 1;
                                                } else {
                                                    for item in self.combination.iter_mut() {
                                                        if item.id == c.id {
                                                            *item = c.clone();
                                                        } else {
                                                            has = 1;
                                                        }
                                                    }
                                                }
                                                if has == 1 {
                                                    self.combination.push(c);
                                                }
                                                self.keys_item[index].1 =
                                                    self.keys_item[index].0.clone();
                                                self.enabled = true;
                                            };
                                        },
                                    );

                                    if ui
                                        .button("删除")
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .clicked()
                                    {
                                        deleted_index = index;
                                        delede_id = item.0.id.clone();
                                        is_delete = true;
                                    };
                                });
                                // 拖拽区域
                                ui.horizontal(|ui| {
                                    dnd(ui, index).show_vec_sized(
                                        &mut self.keys_item[index].0.output,
                                        egui::Vec2 { x: 30.0, y: 10.0 },
                                        |ui, item, handle, _state| {
                                            egui::Frame::none()
                                                .fill(egui::Color32::from_rgb(200, 200, 200))
                                                .show(ui, |ui| {
                                                    let mut block = handle.ui_sized(
                                                        ui,
                                                        egui::Vec2 { x: 30.0, y: 10.0 },
                                                        |ui| {
                                                            ui.centered_and_justified(|ui| {
                                                                egui::Widget::ui(
                                                                    egui::Label::new(
                                                                        item.0.to_string(),
                                                                    ),
                                                                    ui,
                                                                );
                                                            });
                                                        },
                                                    );
                                                    block.id = egui::Id::new(item.1.clone());
                                                });
                                        },
                                    );
                                });
                                ui.separator();
                            });
                            ui.end_row();
                        }
                        if is_delete {
                            self.keys_item.remove(deleted_index);
                            is_delete = false;
                            self.combination.retain(|i| i.id != delede_id);
                            if self.combination.is_empty() {
                                self.enabled = false;
                            }
                        }
                    });
                });
            });
        });
        egui::SidePanel::right("Right")
            .min_width(100.0)
            .max_width(100.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    //监听开关
                    toggle_ui(ui, unsafe { &mut LISTEN }, self.enabled)
                        .on_hover_cursor(if self.enabled {
                            egui::CursorIcon::PointingHand
                        } else {
                            egui::CursorIcon::NotAllowed
                        })
                        .on_hover_text(if self.enabled {
                            "修改配置后需要关闭再开启才能生效"
                        } else {
                            "未添加按键组合时无法开启"
                        });
                    ui.add(
                        egui::Label::new(if unsafe { LISTEN } {
                            "开启中"
                        } else {
                            "关闭中"
                        })
                        .selectable(false),
                    );
                });

                if unsafe { LISTEN } && !unsafe { ALREADY_LISTEN } {
                    listen_active(self.clone());
                    unsafe { ALREADY_LISTEN = true };
                }
            });
        record_ui(ctx);
    }
}
