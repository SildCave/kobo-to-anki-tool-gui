use eframe::{egui::{self, text::LayoutJob, Color32, FontFamily, FontId, RichText, TextFormat}, epaint::color};

use crate::AppState;

pub fn draw_header(
    ui: &mut egui::Ui
) {
    ui.vertical_centered(|ui| {
        ui.heading(RichText::new("Kobo To Anki Sync Tool")
            .color(Color32::WHITE)
            .font(FontId::new(50.0, FontFamily::Monospace))
            .monospace()
        );
    });
}


pub fn draw_kobo_connection_information_message(app_state: &AppState, ui: &mut egui::Ui) {

    ui.vertical_centered(|ui| {
        let text: String;
        if app_state.kobo_path.is_none() {
            text = "Please connect your Kobo eReader, If for some reason it is not detected or you want to select the path to your reader manually, please click the button below.".to_string();
        } else {
            text = "Kobo Reader detected! If for some reason you want to change the path to your reader, please click the button below.".to_string();
        }
        ui.label(RichText::new(text)
            .color(Color32::LIGHT_GRAY)
            .font(FontId::new(20.0, FontFamily::Proportional)));

    });
}

pub fn draw_kobo_connection_status_message_when_no_device_is_detected(app_state: &mut AppState, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("Kobo Reader not detected!")
            .color(Color32::LIGHT_RED)
            .font(FontId::new(20.0, FontFamily::Proportional)));
        draw_file_picker_button(app_state, ui);
    });
}

pub fn draw_kobo_connection_status_message_when_device_is_detected(app_state: &mut AppState, ui: &mut egui::Ui) {
    let mut job = LayoutJob::default();
    job.append(
        "Kobo path: ",
        0.0,
        TextFormat {
            font_id: FontId::new(20.0, FontFamily::Proportional),
            color: Color32::LIGHT_BLUE,
            ..Default::default()
        },
    );
    job.append(
        format!("{}", app_state.kobo_path.as_ref().unwrap().to_str().unwrap()).as_str(),
        0.0,
        TextFormat {
            font_id: FontId::new(20.0, FontFamily::Proportional),
            color: Color32::GREEN,
            italics: false,
            ..Default::default()
        },
    );
    ui.vertical_centered(|ui| {
        ui.label(job);

        draw_file_picker_button(app_state, ui);
    });
}

fn draw_file_picker_button(app_state: &mut AppState, ui: &mut egui::Ui) {
    if ui.button(
        RichText::new("Select/Change Path")
            .color(Color32::LIGHT_BLUE)
            .font(FontId::new(20.0, FontFamily::Proportional))
        ).clicked() {
        // Open the file dialog to select a file.
        app_state.file_dialog.select_directory();
    }
}

pub fn notify_user_about_invalid_kobo_path(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("The path you just selected is not a valid Kobo eReader path!")
            .color(Color32::LIGHT_RED)
            .font(FontId::new(20.0, FontFamily::Proportional)));
    });
}

pub fn display_anki_connection_status_message(app_state: &AppState, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        let text: String;
        let color: Color32;
        match app_state.anki_connection_status {
            crate::anki::AnkiConnectionStatus::Connected => {
                text = "Connected to Anki!".to_string();
                color = Color32::GREEN;
            }
            crate::anki::AnkiConnectionStatus::Connecting => {
                if app_state.first_attempt_at_connecting_to_anki {
                    text = "Connecting to Anki...".to_string();
                    color = Color32::WHITE;
                } else {
                    text = "Could not connect to Anki! Retrying...".to_string();
                    color = Color32::LIGHT_RED;
                }
            }
            crate::anki::AnkiConnectionStatus::Disconnected => {
                text = "Anki disconnected!".to_string();
                color = Color32::LIGHT_RED;
            }
            crate::anki::AnkiConnectionStatus::CouldNotConnect => {
                text = "Could not connect to Anki! Retrying...".to_string();
                color = Color32::LIGHT_RED;
            }
        }
        if (app_state.anki_connection_status == crate::anki::AnkiConnectionStatus::Connecting) || app_state.anki_connection_status == crate::anki::AnkiConnectionStatus::CouldNotConnect {

            ui.vertical_centered_justified(|ui| {
                ui.label(RichText::new(text)
                    .color(color)
                    .font(FontId::new(20.0, FontFamily::Proportional)));
                ui.add(egui::Spinner::new());
            });

        } else {
            ui.label(RichText::new(text)
                .color(color)
                .font(FontId::new(20.0, FontFamily::Proportional)));
        }

    });
}

pub fn draw_horizontal_line(ui: &mut egui::Ui) {
    ui.separator();
}

pub fn draw_anki_connection_guide(ui: &mut egui::Ui) {

    ui.vertical_centered(|ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("To connect to Anki, please make sure that Anki is running and that")
                .color(Color32::WHITE)
                .font(FontId::new(20.0, FontFamily::Proportional)));
            ui.hyperlink_to(RichText::new("AnkiConnect")
                .color(Color32::LIGHT_BLUE)
                .font(FontId::new(20.0, FontFamily::Proportional)), "https://ankiweb.net/shared/info/2055492159");
            ui.label(RichText::new("is installed.")
                .color(Color32::WHITE)
                .font(FontId::new(20.0, FontFamily::Proportional)));
        });
    });
}

pub fn display_new_words_count(
    app_state: &AppState,
    ui: &mut egui::Ui,
    new_words_count: usize,
    already_added_words_count: usize,
    all_words_in_kobo_count: usize,

) {

    let new_words_count_layout = generate_layout_for_display_new_words_count("New words to add: ", new_words_count.to_string().as_str(), Color32::GREEN);
    let already_added_words_count_layout = generate_layout_for_display_new_words_count("Words already added: ", already_added_words_count.to_string().as_str(), Color32::LIGHT_BLUE);
    let all_words_in_kobo_count_layout = generate_layout_for_display_new_words_count("All words in Kobo: ", all_words_in_kobo_count.to_string().as_str(), Color32::RED);

    ui.vertical_centered(|ui| {
        ui.label(new_words_count_layout);
        ui.label(already_added_words_count_layout);
        ui.label(all_words_in_kobo_count_layout);
    });
}


fn generate_layout_for_display_new_words_count(
    string_left: &str,
    string_right: &str,
    right_color: Color32,
) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.append(
        string_left,
        0.0,
        TextFormat {
            font_id: FontId::new(20.0, FontFamily::Proportional),
            color: Color32::WHITE,
            ..Default::default()
        },
    );
    job.append(
        format!("{}", string_right).as_str(),
        0.0,
        TextFormat {
            font_id: FontId::new(22.0, FontFamily::Proportional),
            italics: true,
            color: right_color,
            ..Default::default()
        },
    );

    job
}
