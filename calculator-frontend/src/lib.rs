/// ./src/gui.rs
/// This file contains the GUI implementation for the calculator application.

use calculator_backend::{calculate_expression, History};
use eframe::egui;
use egui::{Frame, Margin, Color32, TextEdit, Vec2, FontId, CornerRadius, RichText};

#[derive(PartialEq)]
enum Mode {
    Basic,
    Scientific,
    Trigonometry,
    History,
}

pub struct CalcGUI {
    input_value: String,
    derived_number: Option<f64>,
    //history_string: String,
    selected_mode: Mode,
    history: History,
}

impl CalcGUI {
    pub fn new() -> Self {
        Self {
            input_value: String::new(),
            derived_number: None,
            selected_mode: Mode::Basic,
            history: History::new(),

        }
    }
    
    fn process_input(&mut self) {
        let result = calculate_expression(&self.input_value, &mut self.history);
        self.input_value = result.result.to_string();
        self.derived_number = Some(result.result as f64);
        //self.history_string = Some(result.history as String);
    }
}

const MENU_INDENT: i8 = 35;

impl eframe::App for CalcGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(120.0);
            //Form, Enter Button, Styling
            ui.horizontal(|ui| {
                let response = ui.add_sized(
                    Vec2::new(300.0, 20.0), // width, height
                    TextEdit::singleline(&mut self.input_value)
                        .font(FontId::proportional(20.0))
                );
            
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.process_input();
                }

                if ui.add_sized(Vec2::new(25.0, 25.0), egui::Button::new("C")).clicked() {
                    self.input_value.clear();
                }
            
                if ui.add_sized(Vec2::new(50.0, 25.0), egui::Button::new("Enter")).clicked() {
                    self.process_input();
                }
            });

            ui.separator();
            
            
            //radio buttons
            Frame::new()
                .fill(Color32::from_rgb(0, 0, 0)) // background color
                .inner_margin(Margin {
                    left: 40,
                    top: 10,
                    right: 73,
                    bottom: 10,
                })
                .outer_margin(Margin{
                    left: MENU_INDENT +1,
                    top: 0,
                    right: 0,
                    bottom: 0,
                })
                .stroke(egui::Stroke::new(1.0, Color32::WHITE)) //border
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.selected_mode, Mode::Basic, "Basic");
                        ui.radio_value(&mut self.selected_mode, Mode::Scientific, "Sci");
                        ui.radio_value(&mut self.selected_mode, Mode::Trigonometry, "Trig");
                        ui.radio_value(&mut self.selected_mode, Mode::History, "Hist");
                    });
                });
            

                //Basic Scientific and Trig options
                match self.selected_mode {
                    Mode::Basic => {
                    
                        Frame::group(ui.style())
                            .fill(Color32::from_rgb(20, 20, 20))
                            .inner_margin(Margin {
                                left: 100,
                                top: 25,
                                right: 100,
                                bottom: 25,
                            })
                            .outer_margin(Margin{
                                left: MENU_INDENT,
                                top: 0,
                                right: 0,
                                bottom: 0,
                            })
                            .corner_radius(CornerRadius {
                                nw: 0, 
                                ne: 0, 
                                sw: 10,  
                                se: 10,  
                            })
                            .stroke(egui::Stroke::new(1.0, Color32::LIGHT_BLUE))
                            .show(ui, |ui| {
                                //ui.label("Basic Mode Panel");
                                ui.horizontal(|ui| {
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("+")).clicked() {
                                        self.input_value.push('+');
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("-")).clicked() {
                                        self.input_value.push('-');
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("*")).clicked() {
                                        self.input_value.push('*');
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("/")).clicked() {
                                        self.input_value.push('/');
                                    }
                                });
                                ui.horizontal(|ui| {
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("^")).clicked() {
                                        self.input_value.push('^');
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("√")).clicked() {
                                        self.input_value.push('√');
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("!")).clicked() {
                                        self.input_value.push('!');
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("ans")).clicked() {
                                        self.input_value.push_str("ans");
                                    }
                                });
                            });
                    }

                    Mode::Scientific => {
                        Frame::group(ui.style())
                            .fill(Color32::from_rgb(20, 20, 20))
                            .inner_margin(Margin {
                                left: 100,
                                top: 25,
                                right: 100,
                                bottom: 25,
                            })
                            .outer_margin(Margin{
                                left: MENU_INDENT,
                                top: 0,
                                right: 0,
                                bottom: 0,
                            })
                            .corner_radius(CornerRadius {
                                nw: 0, 
                                ne: 0, 
                                sw: 10,  
                                se: 10,  
                            })
                            .stroke(egui::Stroke::new(1.0, Color32::LIGHT_GREEN))
                            .show(ui, |ui| {
                                //ui.label("Basic Mode Panel");
                                ui.horizontal(|ui| {
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("z")).clicked() {
                                        //self.input_value.push()
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("y")).clicked() {
                                        //self.input_value.push()
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("x")).clicked() {
                                        //self.input_value.push()
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("w")).clicked() {
                                        //self.input_value.push()
                                    }
                                });
                                ui.horizontal(|ui| {
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("v")).clicked() {
                                        //self.input_value.push()
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("u")).clicked() {
                                        //self.input_value.push()
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("t")).clicked() {
                                        //self.input_value.push()
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("s")).clicked() {
                                        //self.input_value.push()
                                    }
                                });
                            });
                    }

                    Mode::Trigonometry => {
                        Frame::group(ui.style())
                            .fill(Color32::from_rgb(20, 20, 20))
                            .inner_margin(Margin {
                                left: 100,
                                top: 25,
                                right: 100,
                                bottom: 25,
                            })
                            .outer_margin(Margin{
                                left: MENU_INDENT,
                                top: 0,
                                right: 0,
                                bottom: 0,
                            })
                            .corner_radius(CornerRadius {
                                nw: 0, 
                                ne: 0, 
                                sw: 10,  
                                se: 10,  
                            })
                            .stroke(egui::Stroke::new(1.0, Color32::ORANGE))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("sin")).clicked() {
                                        self.input_value.push_str("sin ")
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("cos")).clicked() {
                                        self.input_value.push_str("cos ")
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("tan")).clicked() {
                                        self.input_value.push_str("tan ")
                                    }
                                });
                                ui.horizontal(|ui| {
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("arcsin")).clicked() {
                                        self.input_value.push_str("arcsin ")
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("arccos")).clicked() {
                                        self.input_value.push_str("arccos ")
                                    }
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("arctan")).clicked() {
                                        self.input_value.push_str("arctan ")
                                    }
                                });
                            });
                    }
                    Mode::History => {
                        Frame::group(ui.style())
                            
                            .inner_margin(Margin {
                                left: 100,
                                top: 25,
                                right: 60,
                                bottom: 25,
                            })
                            .outer_margin(Margin{
                                left: MENU_INDENT,
                                top: 0,
                                right: 0,
                                bottom: 0,
                            })
                            .corner_radius(CornerRadius {
                                nw: 0, 
                                ne: 0, 
                                sw: 10,  
                                se: 10,  
                            })
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    if ui.add_sized(Vec2::new(20.0, 20.0), egui::Button::new("Clear")).clicked() {
                                        self.history.entries.clear();
                                    }
                                    
                                });

                                ui.separator();


                                ui.vertical(|ui| {
                                    egui::ScrollArea::vertical()
                                        .auto_shrink([false; 2])
                                        .show(ui, |ui| {
                                            for (i, entry) in self.history.entries.iter().enumerate() {
                                                if let Some(result) = entry.result {
                                                    ui.label(
                                                        RichText::new(format!("{}: {} = {}", i + 1, entry.input, result))
                                                            .font(FontId::proportional(15.0))
                                                    );
                                                } else if let Some(error) = &entry.error_message {
                                                    ui.label(
                                                        RichText::new(format!("{}: {} = Error: {}", i + 1, entry.input, error))
                                                            .font(FontId::proportional(15.0))
                                                            .color(Color32::RED)
                                                    );
                                                }
                                            }
                                            
                                    });
                                });
                            });

                            


                    }


                }
        });
    }
}

