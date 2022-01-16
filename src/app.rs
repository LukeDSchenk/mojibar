use clipboard::{ClipboardContext, ClipboardProvider};
use eframe::{egui, epi};
use lazy_static::lazy_static;
use std::fs::File;
use std::io::Read;

use crate::emoji::{self, Emoji};

lazy_static! {
    static ref EMOJIS: String = {
        let mut f = File::open("./emojis/emojis.txt").expect("Could not open font file: ./emojis/emojis.txt");
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).expect("Could not read font file to a string");
        buffer
    };
    static ref EMOS: Vec<Emoji> = emoji::load_emoji_data("./emojis/emoji.json");
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    search: String,
    cb: ClipboardContext,

    // this how you opt-out of serialization of a member
    //#[cfg_attr(feature = "persistence", serde(skip))]
    //value: i32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            search: "Coming soon!".to_owned(),
            cb: ClipboardProvider::new().unwrap(),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Mojibar 🥴"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        // Set up custom font to support Unicode Chars.
        // THIS CODE DOESN'T FUCKING WORK.
        let mut fonts = egui::FontDefinitions::default();
        fonts.family_and_size.insert(
            egui::TextStyle::Button,
            (egui::FontFamily::Proportional, 36.0)
        );

        // Try and set up custom fonts here. Not currently working.
        fonts.font_data.insert("OpenMoji".to_owned(), std::borrow::Cow::Borrowed(include_bytes!("../fonts/OpenMoji-Color.ttf")));
        fonts.fonts_for_family.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "OpenMoji".to_owned());

        _ctx.set_fonts(fonts);
        println!("{}", EMOS[1700]);
    }

    /// Called by the framework to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your wilet search_bar = ui.text_edit_singleline(&mut "".to_string());dgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self { search, cb } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Search 🔍");
            let _search_bar = ui.text_edit_singleline(search); // remove underscore when ready
        });

        // The central panel is the region left after adding TopPanel's and SidePanel's.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Emojis 🌵");
            egui::warn_if_debug_build(ui);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for c in EMOJIS.chars() {
                        if ui.button(c).clicked() {
                            cb.set_contents(c.to_string()).unwrap();
                            println!("{}", c);
                        }
                    }
                });
            });
        });
    }
}
