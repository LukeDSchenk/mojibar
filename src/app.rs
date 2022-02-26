use eframe::{egui, epi};
use lazy_static::lazy_static;
use radix_trie::{Trie, TrieCommon};

use crate::emoji::{self, Emoji};

#[cfg(not(target_arch = "wasm32"))]
use {
    clipboard::{ClipboardContext, ClipboardProvider},
};

#[cfg(target_arch = "wasm32")]
use {
    wasm_bindgen_futures::JsFuture, // https://rustwasm.github.io/docs/wasm-bindgen/reference/js-promises-and-rust-futures.html
    web_sys::{self, Clipboard},
};

// Static references to data structures containing the emoji data.
// EMOJIS is a vector of Emoji structs. This is where the actual data for each emoji resides.
// TREE is a radix tree containing references to Emoji structs inside EMOJIS (used for searching).
lazy_static! {
    static ref EMOJIS: Vec<Emoji> = emoji::load_emoji_data().expect("Could not load emoji data");
    static ref TREE: Trie<&'static [u8], Vec<&'static Emoji>> = create_radix_trie();
}

/// Creates a static radix trie where the keys are arrays of chars (u8) and the optional values
/// are vectors of emoji references.
fn create_radix_trie() -> Trie<&'static [u8], Vec<&'static Emoji>> {
    let mut tree: Trie<&[u8], Vec<&Emoji>> = Trie::new();
    for emoji in EMOJIS.iter() {
        match tree.get_mut(emoji.name.as_bytes()) {
            Some(vector) => vector.push(emoji),
            None => {
                tree.insert(emoji.name.as_bytes(), vec![emoji]);
                ()
            },
        };
        for word in emoji.keywords.iter() {
            match tree.get_mut(word.as_bytes()) {
                Some(vector) => vector.push(emoji),
                None => {
                    tree.insert(word.as_bytes(), vec![emoji]);
                    ()
                },
            };
        }
    }
    tree
}

/// Deriving Deserialize/Serialize allows persisting app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
#[cfg(not(target_arch = "wasm32"))]
pub struct MojiApp<'a> {
    search: String,
    results: Vec<&'a Emoji>, // stores emoji refs matched in the search
    selected: String,
    cb: ClipboardContext,
}

#[cfg(target_arch = "wasm32")]
pub struct MojiApp<'a> {
    search: String,
    results: Vec<&'a Emoji>, // stores emoji refs matched in the search
    selected: String,
    cb: Clipboard,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for MojiApp<'_> {
    fn default() -> Self {
        Self {
            search: String::from(""),
            results: Vec::new(),
            selected: String::from(" "),
            cb: ClipboardProvider::new().unwrap(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for MojiApp<'_> {
    fn default() -> Self {
        Self {
            search: String::from(""),
            results: Vec::new(),
            selected: String::from(" "),
            cb: web_sys::window().expect("could not get web-sys window object")
                .navigator()
                .clipboard().expect("could not get clipboard"), // replace expects with JS alert in browser
        }
    }
}

impl epi::App for MojiApp<'_> {
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

        // Set up custom fonts.
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

        // Set up custom fonts here. All emojis in egui are black/white by default, need to look into why.
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
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self { search, results, selected, cb } = self;

        // Side panel with search bar.
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.heading("Search 🔍");

            if ui.text_edit_singleline(search).changed() {
                results.clear();
                if search != "" {
                    match TREE.subtrie(&search.as_bytes()) {
                        Some(st) => {
                            for vector in st.values() { // iterate over all values of the subtrie
                                for item in vector.iter() {
                                    results.push(item);
                                }
                            }
                        },
                        None => (),
                    };
                }
            }

            // The emoji area for the search bar.
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for emoji in results.iter() {
                        if ui.button(&emoji.ch).on_hover_text(&emoji.name).clicked() {
                            #[cfg(not(target_arch = "wasm32"))] {
                                cb.set_contents(emoji.ch.clone()).unwrap();
                                *selected = emoji.ch.clone();
                                if cfg!(debug_assertions) {
                                    println!("{}", emoji.ch); // only prints on debug builds
                                }
                            }

                            #[cfg(target_arch = "wasm32")] {
                                let promise = JsFuture::from(cb.write_text(&emoji.ch));
                                let _result =  async {
                                    promise.await.expect("could not copy to clipboard");
                                };
                                *selected = emoji.ch.clone();
                            }
                        }
                    }
                });
            });
        });

        // The central panel is the region left after adding TopPanel's and SidePanel's.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Selected: ");
                ui.heading(&selected);
                egui::warn_if_debug_build(ui);
            });
            ui.add_space(15.0);

            // The main emoji scroll area.
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for emoji in EMOJIS.iter() {
                        if ui.button(&emoji.ch).on_hover_text(&emoji.name).clicked() {
                            #[cfg(not(target_arch = "wasm32"))] {
                                cb.set_contents(emoji.ch.clone()).unwrap();
                                *selected = emoji.ch.clone();
                                if cfg!(debug_assertions) {
                                    println!("{}", emoji.ch); // only prints on debug builds
                                }
                            }

                            #[cfg(target_arch = "wasm32")] {
                                let promise = JsFuture::from(cb.write_text(&emoji.ch));
                                let _result =  async {
                                    promise.await.expect("could not copy to clipboard");
                                };
                                *selected = emoji.ch.clone();
                            }
                        }
                    }
                });
            });
        });
    }
}
