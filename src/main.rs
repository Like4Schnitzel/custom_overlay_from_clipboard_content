use std::fs;
use eframe::egui;
use inputbot::KeybdKey::*;
use str_distance::*;

struct ClipboardKeyValueDisplay {
    pairs: Vec<(String, String)>,
    key: String,
    value: String,
}

impl Default for ClipboardKeyValueDisplay {
    fn default() -> Self {
        Self {
            pairs: Vec::new(),
            key: String::new(),
            value: "Fortnite".to_string(),
        }
    }
}

impl eframe::App for ClipboardKeyValueDisplay {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.key = cli_clipboard::get_contents().unwrap_or(self.key.clone());

        let mut min_levenshtein_distance = f64::MAX;
        let mut min_levenshtein_value = String::new();
        for (k, v) in &self.pairs {
            let distance = str_distance_normalized(&self.key, k, Levenshtein::default());
            if distance < min_levenshtein_distance {
                min_levenshtein_distance = distance;
                min_levenshtein_value = v.clone();
                if distance == 0.0 {
                    break;
                }
            }
        }
        self.value = min_levenshtein_value;

        egui::CentralPanel::default().frame(egui::Frame::NONE).show(ctx, |ui| {
            ui.label(self.value.clone());
        });
    }

    // fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    //     egui::Rgba::TRANSPARENT.to_array()
    // }
}

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("data.txt")
        .expect("Something went wrong data.txt");
    let lines = contents.lines();
    let mut pairs: Vec<(String, String)> = Vec::new();

    for line in lines {
        let penis = line.replace("\\n", "\n");
        let mut parts = penis.split(';');
        let key = parts.next().unwrap();
        let mut value = parts.fold(String::new(), |a, b| a + b + ";");
        value.pop();
        pairs.push((key.to_string(), value.to_string()));
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true)
            .with_decorations(false)
            .with_drag_and_drop(false)
            .with_always_on_top()
            .with_mouse_passthrough(true),
        ..Default::default()
    };

    let clipboard_object = ClipboardKeyValueDisplay {
        pairs,
        key: String::new(),
        value: String::new(),
    };
    
    FKey.bind(move || {
        println!("FKey pressed");
    });

    std::thread::spawn(inputbot::handle_input_events);

    let _ = eframe::run_native(
        "Clipboard Key-Value Display",
        options,
        Box::new(|_cc| {
            Ok(Box::new(clipboard_object))
        }),
    );
}
