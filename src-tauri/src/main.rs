// Winfluencers masaustu kabugu: tek pencere, canli winvestour.com'u app-modu ile acar.
// 19 Tem: tepsi/close-to-tray KALDIRILDI (Tutku "cikis yapamiyorum, tepside
// calismaya devam ediyor") — X = gercek cikis (beklenen davranis, Chrome/VSCode
// gibi). Tek-ornek + pencere boyut/konum hatirlama korundu. Is mantigi yok.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // Ikinci acilis = mevcut pencereyi one getir (yeni pencere acma).
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.set_focus();
            }
        }))
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("Tauri baslatilamadi");
}
