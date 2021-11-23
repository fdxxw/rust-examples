use chrono::prelude::*;
use eframe::{egui, epi};

struct Log {
  time: DateTime<Local>,
  name: String,
}

impl Log {
  fn new(name: &str) -> Log {
    Log {
      name: name.to_string(),
      time: Local::now(),
    }
  }
}
struct App {
  name: String,
  logs: Vec<Log>,
}

impl Default for App {
  fn default() -> Self {
    Self {
      name: "".to_string(),
      logs: Vec::new(),
    }
  }
}

impl epi::App for App {
  fn name(&self) -> &str {
    "Daily Log"
  }

  fn setup(&mut self, ctx: &egui::CtxRef, _: &mut epi::Frame<'_>, _: Option<&dyn epi::Storage>) {
    //Custom font install
    // # use epaint::text::*;
    // 1. Create a `FontDefinitions` object.
    let mut font = egui::FontDefinitions::default();
    // Install my own font (maybe supporting non-latin characters):
    // 2. register the font content with a name.
    font.font_data.insert(
      "my_font".to_owned(),
      std::borrow::Cow::Borrowed(include_bytes!("../fonts/SIMHEI.TTF")),
    );
    //font.font_data.insert("mPlus".to_string(), Cow::from(&mPlus_font[..]));
    // 3. Set two font families to use the font, font's name must have been
    // Put new font first (highest priority)registered in `font_data`.
    font
      .fonts_for_family
      .get_mut(&egui::FontFamily::Monospace)
      .unwrap()
      .insert(0, "my_font".to_owned());
    font
      .fonts_for_family
      .get_mut(&egui::FontFamily::Proportional)
      .unwrap()
      .insert(0, "my_font".to_owned());
    // 4. Configure context with modified `FontDefinitions`.
    ctx.set_fonts(font);
  }

  fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
    let Self { name, logs } = self;

    egui::CentralPanel::default().show(ctx, |ui| {
      // ui.heading("Log Record");
      ui.horizontal(|ui| {
        ui.label("计划内容: ");
        ui.text_edit_singleline(name);
        if ui.button("开始").clicked() {
          if !name.is_empty() {
            logs.push(Log::new(name));
            name.clear()
          }
        }
      });
      for log in logs.into_iter().rev() {
        ui.horizontal(|ui| {
          let s = log.time.format("%X");
          ui.label(s);
          ui.text_edit_singleline(&mut log.name);
        });
      }
      // ui.add(egui::Slider::new(age, 0..=120).text("age"));
      // ui.label(format!("Hello '{}', age {}", name, age));
    });

    // Resize the native window to be just the size we need it to be:
    frame.set_window_size(ctx.used_size());
  }
}

pub fn run() {
  let options = eframe::NativeOptions {
    always_on_top: false,
    maximized: false,
    decorated: true,
    drag_and_drop_support: false,
    icon_data: None,
    initial_window_size: Some(egui::Vec2::new(450.0, 400.0)),
    resizable: true,
    transparent: false,
  };
  eframe::run_native(Box::new(App::default()), options);
}
