use eframe::egui;
use eframe::egui::CentralPanel;

use std::sync::mpsc::SyncSender;
use std::collections::BTreeMap;

use crate::config::Config;
use crate::command::Command;

pub fn run(song_name: String, track_names: BTreeMap<usize, String>, send: SyncSender<Command>) {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Stemmefiløving", native_options, Box::new(|cc| Box::new(StemmefilApp::new(cc, song_name, track_names, send))));
}

struct StemmefilApp {
    song_name: String,
    tracks: BTreeMap<usize, (String, Config)>,
    progress: f64,
    paused: bool,
    send: SyncSender<Command>,
}

impl StemmefilApp {
    fn new(_cc: &eframe::CreationContext<'_>, song_name: String, track_names: BTreeMap<usize, String>, send: SyncSender<Command>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            song_name,
            tracks: track_names.into_iter().map(|t| (t.0, (t.1, Config::default()))).collect(),
            progress: 0.0,
            paused: false,
            send,
        }
    }
}

impl eframe::App for StemmefilApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Playing {}!", self.song_name));
            
            if ui.toggle_value(&mut self.paused, "Pause").clicked() {
                if self.paused {
                    let _ = self.send.send(Command::Pause);
                } else {
                    let _ = self.send.send(Command::Play);
                }
            }

            if ui.add(egui::Slider::new(&mut self.progress, 0.0..=1.0)).changed() {
                let _ = self.send.send(Command::Jump(self.progress));
            }

            for (id, (name, config)) in self.tracks.iter_mut().skip(1) { // Skip dummy track
                ui.label(name.as_str());
                if ui.toggle_value(&mut config.muted, "Mute").clicked() {
                    let _ = self.send.send(Command::Muted(*id, config.muted));
                }
                if ui.add(egui::Slider::new(&mut config.gain, 0.0..=1.0).text("Volume")).changed() {
                    let _ = self.send.send(Command::Gain(*id, config.gain));
                }
            }
        });
    }
}