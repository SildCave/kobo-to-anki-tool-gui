use std::path::PathBuf;

use anki_bridge::prelude::CardsInfoResponse;
use eframe::egui;
use egui_file_dialog::FileDialog;
use tokio::runtime;

mod layout;
mod events;
mod kobo;
mod anki;
mod cards;

struct Channels {
    anki_connection_status_rc: Option<tokio::sync::mpsc::Receiver<anki::AnkiConnectionStatus>>,
}

struct AppState {
    file_dialog: FileDialog,
    async_rt: runtime::Runtime,
    kobo_path: Option<PathBuf>,
    invalid_kobo_path: bool,
    anki_client: anki::AnkiClient<'static>,
    anki_connection_status: anki::AnkiConnectionStatus,
    channels: Channels,
    custom_path: bool,
    first_attempt_at_connecting_to_anki: bool,
    last_connection_attempt_time: std::time::Instant,
    prepared_words_from_kobo: Option<Vec<String>>,
    prepared_words_from_anki: Option<Vec<String>>,
    raw_cards_from_anki: Option<Vec<CardsInfoResponse>>,
    deck_name: Option<String>,
}

impl AppState {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        layout::setup_custom_fonts(&cc.egui_ctx);
        Self {
            file_dialog: FileDialog::new(),
            async_rt: runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            kobo_path: None,
            invalid_kobo_path: false,
            anki_client: anki::AnkiClient::new(),
            anki_connection_status: anki::AnkiConnectionStatus::Disconnected,
            channels: Channels {
                anki_connection_status_rc: None,
            },
            custom_path: false,
            first_attempt_at_connecting_to_anki: true,
            last_connection_attempt_time: std::time::Instant::now() - std::time::Duration::from_secs(5),
            prepared_words_from_kobo: None,
            prepared_words_from_anki: None,
            deck_name: None,
            raw_cards_from_anki: None,
        }
    }
}


impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        events::update_ui(self, ctx);
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let mut native_options = eframe::NativeOptions::default();

    native_options.viewport.min_inner_size = Some(egui::vec2(800.0, 600.0));

    eframe::run_native(
        "Kobo To Anki Sync Tool",
        native_options,
        Box::new(|ctx| Ok(Box::new(AppState::new(ctx)))),
    )
}



