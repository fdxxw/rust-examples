use eframe::{egui, epi};
// mod log;
struct App {
  name: String,
  age: u8,
}

impl Default for App {
  fn default() -> Self {
    Self {
      name: "fdxxw".to_string(),
      age: 26,
    }
  }
}

impl epi::App for App {
  fn name(&self) -> &str {
    "My egui App"
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
    let Self { name, age } = self;

    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Log Record");
      ui.horizontal(|ui| {
        ui.label("开始计划: ");
        ui.text_edit_singleline(name);
        if ui.button("开始").clicked() {
          *age += 1;
        }
      });
      
      ui.add(egui::Slider::new(age, 0..=120).text("age"));
      
      ui.label(format!("Hello '{}', age {}", name, age));
    });

    // Resize the native window to be just the size we need it to be:
    frame.set_window_size(ctx.used_size());
  }
}

pub fn run() {
  let options = eframe::NativeOptions::default();
  eframe::run_native(Box::new(App::default()), options);
}
