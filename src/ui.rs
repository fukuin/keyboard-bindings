use crate::{
    components::{load_fonts, select_table_key, select_table_vkey, toggle_ui},
    listen_active, ALREADY_LISTEN, LISTEN,
};
use eframe::egui::{self};
use nanoid::nanoid;
use rdev::Key;
use windows_hotkeys::keys::VKey;

#[derive(Debug, Clone)]
pub struct App {
    pub keys_item: Vec<ListItem>,
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
    pub output: Vec<ActionItem>,
}

#[derive(Debug, Clone, Copy)]
pub enum ActionItem {
    Press(Key),
    Delay(u64),
}

impl Default for App {
    fn default() -> Self {
        Self {
            keys_item: vec![],
            combination: vec![],
            enabled: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        load_fonts(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.keys_item.len() == 0 {
                self.enabled = false;
            } else {
                self.enabled = true;
            }
            ui.horizontal_top(|ui| {
                ui.label("按键绑定");
                if ui
                    .button("添加")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    self.keys_item.push(ListItem {
                        id: nanoid!(),
                        name: Default::default(),
                        input_key: VKey::A,
                        output: vec![],
                    });
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
            ui.add_enabled_ui(!unsafe { LISTEN }, |ui| {
                let mut is_break = false; //是否break当前for循环
                egui::Grid::new("list")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        for (index, item) in self.keys_item.clone().iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(index.to_string() + ":");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.keys_item[index].name)
                                        .desired_width(60.0)
                                        .hint_text("组名"),
                                );
                                ui.label("添加监听按钮:");
                                select_table_vkey(
                                    ui,
                                    index.to_string(),
                                    &mut self.keys_item[index].input_key,
                                );

                                if ui
                                    .button("添加执行按钮")
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    self.keys_item[index]
                                        .output
                                        .push(ActionItem::Press(Key::KeyA)); //默认值
                                };
                                if ui
                                    .button("添加延时(ms)")
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    self.keys_item[index].output.push(ActionItem::Delay(0));
                                };
                                let mut del_delay_index = 0;
                                let mut can_del = false;
                                for (i, o) in self.keys_item[index].output.iter_mut().enumerate() {
                                    match o {
                                        ActionItem::Press(k) => {
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
                                    self.keys_item[index].output.remove(del_delay_index);
                                    can_del = false;
                                }

                                if ui
                                    .button("保存")
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    let c = Combo {
                                        input: self.keys_item[index].input_key,
                                        output: self.keys_item[index].output.clone(),
                                        id: item.id.to_string(),
                                    };
                                    self.combination.push(c);
                                    self.enabled = true;
                                };
                                if ui
                                    .button("删除")
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    self.keys_item.remove(index);
                                    is_break = true;
                                    self.combination.retain(|i| i.id != *item.id);
                                    if self.combination.len() == 0 {
                                        self.enabled = false;
                                    }
                                };
                            });
                            ui.end_row();
                            if is_break {
                                break;
                            }
                        }
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

                    ui.label(if unsafe { LISTEN } {
                        "开启中"
                    } else {
                        "关闭中"
                    });
                });

                if unsafe { LISTEN } && !unsafe { ALREADY_LISTEN } {
                    listen_active(self.clone());
                    unsafe { ALREADY_LISTEN = true };
                }
            });
    }
}
