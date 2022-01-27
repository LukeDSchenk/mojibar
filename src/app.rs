use clipboard::{ClipboardContext, ClipboardProvider};
use eframe::{egui, epi};
use lazy_static::lazy_static;
use radix_tree::{Node, Radix};

use crate::emoji::{self, Emoji};

lazy_static! {
    static ref EMOJIS: Vec<Emoji> = emoji::load_emoji_data("./emojis/emoji-min.json"); // this should actually return a result possibly
    static ref TREE: Node<char, &'static Emoji> = create_radix_tree();
}

fn create_radix_tree() -> Node<char, &'static Emoji> {
    let mut tree = Node::<char, &Emoji>::new("", None);
    for emoji in EMOJIS.iter() {
        for word in emoji.keywords.iter() {
            tree.insert(word.as_str(), &emoji);
        }
    }
    tree
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct MojiApp {
    search: String,
    selected: String,
    cb: ClipboardContext,
}

impl Default for MojiApp {
    fn default() -> Self {
        Self {
            search: String::from(""),
            selected: String::from(" "),
            cb: ClipboardProvider::new().unwrap(),
        }
    }
}

impl epi::App for MojiApp {
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
        fonts.family_and_size.insert(
            egui::TextStyle::Body,
            (egui::FontFamily::Proportional, 16.0)
        );
        fonts.family_and_size.insert(
            egui::TextStyle::Heading,
            (egui::FontFamily::Proportional, 30.0)
        );

        // Try and set up custom fonts here. Not currently working.
        fonts.font_data.insert("OpenMoji".to_owned(), std::borrow::Cow::Borrowed(include_bytes!("../fonts/OpenMoji-Color.ttf")));
        fonts.fonts_for_family.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "OpenMoji".to_owned());
        _ctx.set_fonts(fonts);
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
        let Self { search, selected, cb } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.heading("Search 🔍");
            ui.text_edit_singleline(search);
            //let node = TREE.find(search.as_str());
            //println!("{:?}", node)
        });

        // The central panel is the region left after adding TopPanel's and SidePanel's.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Selected: ");
                ui.heading(&selected);
                egui::warn_if_debug_build(ui);
            });
            ui.add_space(15.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for emoji in EMOJIS.iter() {
                        if ui.button(&emoji.ch).on_hover_text(&emoji.name).clicked() {
                            cb.set_contents(emoji.ch.clone()).unwrap();
                            *selected = emoji.ch.clone();
                            println!("{}", emoji.ch);
                        }
                    }
                });
            });
        });
    }
}
