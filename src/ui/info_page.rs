use egui::{RichText, ScrollArea};
use crate::app::{AppState, Page};
use super::theme;

pub fn show(state: &mut AppState, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.heading(RichText::new("domectrl — Help").color(theme::TEXT_HEADING));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Back").clicked() {
                state.page = Page::Main;
            }
        });
    });
    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        ui.spacing_mut().item_spacing.y = 6.0;

        section(ui, "Overview");
        body(ui, "domectrl sends ASCII characters over a serial interface to control telescope domes, created specifically for the Robbins and Helio domes at Pine Mountain Observatory.");

        ui.add_space(8.0);
        section(ui, "Connecting");
        body(ui, "Set the serial port and baud rate in Settings, then click Connect. \
The status dot turns green when the port is open. \
On Linux, the port is usually /dev/ttyUSB0 or /dev/ttyACM0 — \
your user must be in the dialout group (sudo usermod -aG dialout $USER).");

        ui.add_space(8.0);
        section(ui, "Motor Buttons");
        kv(ui, "Lower A / Lower B", "Hold to run the corresponding lowering motor. \
Releases when you lift the mouse button or the cursor leaves the button.");
        kv(ui, "Raise A / Raise B",  "Hold to run the corresponding raising motor. \
Same hold-and-release behaviour as the lower buttons.");
        body(ui, "Only one motor command is active at a time. Clicking a second button \
while one is held will not stack.");

        ui.add_space(8.0);
        section(ui, "Controller Responses");
        body(ui, "The controller sends single bytes back to report dome position. \
domectrl watches the incoming stream and updates the response display:");
        kv(ui, "Y  →  OPEN",   "The dome reports it is fully open.");
        kv(ui, "X  →  CLOSED", "The dome reports it is fully closed.");
        body(ui, "If both Y and X appear in the same chunk the last one in byte order wins. \
Click Clear to reset the display.");

        ui.add_space(8.0);
        section(ui, "ABORT");
        body(ui, "Immediately stops all motor output. Use this if a button gets stuck or \
you need to halt movement fast. It does not disconnect the serial port.");

        ui.add_space(8.0);
        section(ui, "Auto-Reconnect");
        body(ui, "If the serial port drops unexpectedly (cable fault, etc), \
domectrl will automatically attempt to reconnect at the configured interval. \
The status bar shows a countdown while retrying. Click Cancel to stop retrying \
and return to the Disconnected state. Auto-reconnect does not trigger on a \
manual Disconnect.");

        ui.add_space(8.0);
        section(ui, "Settings");
        kv(ui, "Port / Baud rate",    "Serial port path and speed.");
        kv(ui, "Send interval",       "How often (in ms) the command byte is repeated while \
a button is held. Range 10–2000 ms. Lower = faster response, \
higher = less serial traffic.");
        kv(ui, "Auto-reconnect",      "Toggle automatic reconnect on error. When enabled, \
set the retry interval (500–30000 ms).");
        kv(ui, "Command characters",  "The ASCII byte sent for each motor direction. \
Must be a single printable ASCII character. \
Defaults: a / b (lower), A / B (raise).");
        kv(ui, "Config directory",    "Where config.json is saved and loaded from. \
Defaults to ~/.config/domectrl/ on Linux/Mac \
and %APPDATA%\\domectrl\\ on Windows.");
        body(ui, "Click Apply to apply changes without saving to disk. \
Click Save to File to write a JSON config file. ");

        ui.add_space(8.0);
        section(ui, "Config File");
        body(ui, "Load Config and Save Config in the main toolbar read and write the \
JSON file at the configured config directory. If the file is missing or \
unparseable, defaults are used instead.");

        ui.add_space(12.0);
        ui.separator();
        ui.label(
            RichText::new("domectrl v1.0.0 - built by Graeme Kieran - MIT License")
                .color(theme::RESPONSE_UNKNOWN)
                .size(11.0),
        );
        use egui::special_emojis::GITHUB;
        ui.hyperlink_to(
        format!("{GITHUB} github.com/bean-frog/domectrl"),
            "https://github.com/bean-frog/domectrl",
        );
    });
}

fn section(ui: &mut egui::Ui, title: &str) {
    ui.label(RichText::new(title).color(theme::TEXT_HEADING).size(15.0).strong());
}

fn body(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).color(theme::TEXT_NORMAL));
}

fn kv(ui: &mut egui::Ui, key: &str, value: &str) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(format!("{key}:")).color(theme::TEXT_STRONG).strong());
        ui.label(RichText::new(value).color(theme::TEXT_NORMAL));
    });
}
