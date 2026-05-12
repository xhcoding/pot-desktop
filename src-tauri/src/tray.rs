use crate::clipboard::start_clipboard_monitor;
use crate::clipboard::ClipboardMonitorEnableWrapper;
use crate::config::{get, set};
use crate::window::config_window;
use crate::window::input_translate;
use crate::window::ocr_recognize;
use crate::window::ocr_translate;
use crate::window::updater_window;
use log::info;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
    AppHandle, Emitter, Manager,
};
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub fn update_tray(app_handle: AppHandle, mut language: String, mut copy_mode: String) {
    if language.is_empty() {
        language = match get("app_language") {
            Some(v) => v.as_str().unwrap().to_string(),
            None => {
                set("app_language", "en");
                "en".to_string()
            }
        };
    }
    if copy_mode.is_empty() {
        copy_mode = match get("translate_auto_copy") {
            Some(v) => v.as_str().unwrap().to_string(),
            None => {
                set("translate_auto_copy", "disable");
                "disable".to_string()
            }
        };
    }

    info!(
        "Update tray with language: {}, copy mode: {}",
        language, copy_mode
    );

    let enable_clipboard_monitor = match get("clipboard_monitor") {
        Some(v) => v.as_bool().unwrap(),
        None => {
            set("clipboard_monitor", false);
            false
        }
    };

    let menu = match language.as_str() {
        "en" => tray_menu_en(enable_clipboard_monitor, copy_mode),
        "zh_cn" => tray_menu_zh_cn(enable_clipboard_monitor, copy_mode),
        "zh_tw" => tray_menu_zh_tw(enable_clipboard_monitor, copy_mode),
        "ja" => tray_menu_ja(enable_clipboard_monitor, copy_mode),
        "ko" => tray_menu_ko(enable_clipboard_monitor, copy_mode),
        "fr" => tray_menu_fr(enable_clipboard_monitor, copy_mode),
        "de" => tray_menu_de(enable_clipboard_monitor, copy_mode),
        "ru" => tray_menu_ru(enable_clipboard_monitor, copy_mode),
        "pt_br" => tray_menu_pt_br(enable_clipboard_monitor, copy_mode),
        "fa" => tray_menu_fa(enable_clipboard_monitor, copy_mode),
        "uk" => tray_menu_uk(enable_clipboard_monitor, copy_mode),
        _ => tray_menu_en(enable_clipboard_monitor, copy_mode),
    };

    if let Some(tray) = app_handle.tray_by_id("main") {
        tray.set_menu(Some(menu)).unwrap();
        #[cfg(not(target_os = "linux"))]
        tray.set_tooltip(Some(&format!("pot {}", env!("CARGO_PKG_VERSION"))))
            .unwrap();
    }
}

pub fn tray_event_handler(_app: &AppHandle, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            #[cfg(target_os = "windows")]
            on_tray_click();
        }
        _ => {}
    }
}

pub fn menu_event_handler(app: &AppHandle, event: MenuEvent) {
    match event.id().0.as_str() {
        "input_translate" => on_input_translate_click(),
        "copy_source" => on_auto_copy_click(app, "source"),
        "clipboard_monitor" => on_clipboard_monitor_click(app),
        "copy_target" => on_auto_copy_click(app, "target"),
        "copy_source_target" => on_auto_copy_click(app, "source_target"),
        "copy_disable" => on_auto_copy_click(app, "disable"),
        "ocr_recognize" => on_ocr_recognize_click(),
        "ocr_translate" => on_ocr_translate_click(),
        "config" => on_config_click(),
        "check_update" => on_check_update_click(),
        "view_log" => on_view_log_click(app),
        "restart" => on_restart_click(app),
        "quit" => on_quit_click(app),
        _ => {}
    }
}

#[cfg(target_os = "windows")]
fn on_tray_click() {
    let event = match get("tray_click_event") {
        Some(v) => v.as_str().unwrap().to_string(),
        None => {
            set("tray_click_event", "config");
            "config".to_string()
        }
    };
    match event.as_str() {
        "config" => config_window(),
        "translate" => input_translate(),
        "ocr_recognize" => ocr_recognize(),
        "ocr_translate" => ocr_translate(),
        "disable" => {}
        _ => config_window(),
    }
}

fn on_input_translate_click() {
    input_translate();
}

fn on_clipboard_monitor_click(app: &AppHandle) {
    let enable_clipboard_monitor = match get("clipboard_monitor") {
        Some(v) => v.as_bool().unwrap(),
        None => {
            set("clipboard_monitor", false);
            false
        }
    };
    let current = !enable_clipboard_monitor;
    set("clipboard_monitor", current);
    let state = app.state::<ClipboardMonitorEnableWrapper>();
    state
        .0
        .lock()
        .unwrap()
        .replace_range(.., &current.to_string());
    if current {
        start_clipboard_monitor(app.clone());
    }
    update_tray(app.clone(), "".to_string(), "".to_string());
}

fn on_auto_copy_click(app: &AppHandle, mode: &str) {
    info!("Set copy mode to: {}", mode);
    set("translate_auto_copy", mode);
    app.emit("translate_auto_copy_changed", mode).unwrap();
    update_tray(app.clone(), "".to_string(), mode.to_string());
}

fn on_ocr_recognize_click() {
    ocr_recognize();
}

fn on_ocr_translate_click() {
    ocr_translate();
}

fn on_config_click() {
    config_window();
}

fn on_check_update_click() {
    updater_window();
}

fn on_view_log_click(app: &AppHandle) {
    let log_dir = app
        .path()
        .app_log_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    let _ = app.opener().open_url(log_dir.to_str().unwrap(), None::<String>);
}

fn on_restart_click(app: &AppHandle) {
    info!("============== Restart App ==============");
    app.restart();
}

fn on_quit_click(app: &AppHandle) {
    let _ = app.global_shortcut().unregister_all();
    info!("============== Quit App ==============");
    app.exit(0);
}

fn tray_menu_en(
    enable_clipboard_monitor: bool,
    copy_mode: String,
) -> Menu<tauri::Wry> {
    let input_translate = MenuItem::with_id(&app_handle(), "input_translate", "Input Translate", true, None::<&str>).unwrap();
    let clipboard_monitor =
        CheckMenuItem::with_id(&app_handle(), "clipboard_monitor", "Clipboard Monitor", true, enable_clipboard_monitor, None::<&str>).unwrap();
    let copy_source =
        CheckMenuItem::with_id(&app_handle(), "copy_source", "Source", true, copy_mode == "source", None::<&str>).unwrap();
    let copy_target =
        CheckMenuItem::with_id(&app_handle(), "copy_target", "Target", true, copy_mode == "target", None::<&str>).unwrap();
    let copy_source_target =
        CheckMenuItem::with_id(&app_handle(), "copy_source_target", "Source+Target", true, copy_mode == "source_target", None::<&str>).unwrap();
    let copy_disable =
        CheckMenuItem::with_id(&app_handle(), "copy_disable", "Disable", true, copy_mode == "disable", None::<&str>).unwrap();
    let ocr_recognize = MenuItem::with_id(&app_handle(), "ocr_recognize", "OCR Recognize", true, None::<&str>).unwrap();
    let ocr_translate = MenuItem::with_id(&app_handle(), "ocr_translate", "OCR Translate", true, None::<&str>).unwrap();
    let config = MenuItem::with_id(&app_handle(), "config", "Config", true, None::<&str>).unwrap();
    let check_update = MenuItem::with_id(&app_handle(), "check_update", "Check Update", true, None::<&str>).unwrap();
    let view_log = MenuItem::with_id(&app_handle(), "view_log", "View Log", true, None::<&str>).unwrap();
    let restart = MenuItem::with_id(&app_handle(), "restart", "Restart", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(&app_handle(), "quit", "Quit", true, None::<&str>).unwrap();


    let auto_copy_submenu = Submenu::with_id_and_items(&app_handle(), "Auto Copy", "Auto Copy", true, &[&copy_source, &copy_target, &copy_source_target, &copy_disable])
    .unwrap();

    Menu::with_items(
        &app_handle(),
        &[
            &input_translate,
            &clipboard_monitor,
            &auto_copy_submenu,
            &ocr_recognize,
            &ocr_translate,
            &config,
            &check_update,
            &view_log,
            &restart,
            &quit,
        ],
    )
    .unwrap()
}

fn tray_menu_zh_cn(
    enable_clipboard_monitor: bool,
    copy_mode: String,
) -> Menu<tauri::Wry> {
    let input_translate = MenuItem::with_id(&app_handle(), "input_translate", "输入翻译", true, None::<&str>).unwrap();
    let clipboard_monitor =
        CheckMenuItem::with_id(&app_handle(), "clipboard_monitor", "监听剪切板", true, enable_clipboard_monitor, None::<&str>).unwrap();
    let copy_source =
        CheckMenuItem::with_id(&app_handle(), "copy_source", "原文", true, copy_mode == "source", None::<&str>).unwrap();
    let copy_target =
        CheckMenuItem::with_id(&app_handle(), "copy_target", "译文", true, copy_mode == "target", None::<&str>).unwrap();
    let copy_source_target =
        CheckMenuItem::with_id(&app_handle(), "copy_source_target", "原文+译文", true, copy_mode == "source_target", None::<&str>).unwrap();
    let copy_disable =
        CheckMenuItem::with_id(&app_handle(), "copy_disable", "关闭", true, copy_mode == "disable", None::<&str>).unwrap();
    let ocr_recognize = MenuItem::with_id(&app_handle(), "ocr_recognize", "文字识别", true, None::<&str>).unwrap();
    let ocr_translate = MenuItem::with_id(&app_handle(), "ocr_translate", "截图翻译", true, None::<&str>).unwrap();
    let config = MenuItem::with_id(&app_handle(), "config", "偏好设置", true, None::<&str>).unwrap();
    let check_update = MenuItem::with_id(&app_handle(), "check_update", "检查更新", true, None::<&str>).unwrap();
    let view_log = MenuItem::with_id(&app_handle(), "view_log", "查看日志", true, None::<&str>).unwrap();
    let restart = MenuItem::with_id(&app_handle(), "restart", "重启应用", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(&app_handle(), "quit", "退出", true, None::<&str>).unwrap();


    let auto_copy_submenu = Submenu::with_id_and_items(&app_handle(), "自动复制", "自动复制", true, &[&copy_source, &copy_target, &copy_source_target, &copy_disable])
    .unwrap();

    Menu::with_items(
        &app_handle(),
        &[
            &input_translate,
            &clipboard_monitor,
            &auto_copy_submenu,
            &ocr_recognize,
            &ocr_translate,
            &config,
            &check_update,
            &view_log,
            &restart,
            &quit,
        ],
    )
    .unwrap()
}

fn tray_menu_zh_tw(
    enable_clipboard_monitor: bool,
    copy_mode: String,
) -> Menu<tauri::Wry> {
    let input_translate = MenuItem::with_id(&app_handle(), "input_translate", "輸入翻譯", true, None::<&str>).unwrap();
    let clipboard_monitor =
        CheckMenuItem::with_id(&app_handle(), "clipboard_monitor", "偵聽剪貼簿", true, enable_clipboard_monitor, None::<&str>).unwrap();
    let copy_source =
        CheckMenuItem::with_id(&app_handle(), "copy_source", "原文", true, copy_mode == "source", None::<&str>).unwrap();
    let copy_target =
        CheckMenuItem::with_id(&app_handle(), "copy_target", "譯文", true, copy_mode == "target", None::<&str>).unwrap();
    let copy_source_target =
        CheckMenuItem::with_id(&app_handle(), "copy_source_target", "原文+譯文", true, copy_mode == "source_target", None::<&str>).unwrap();
    let copy_disable =
        CheckMenuItem::with_id(&app_handle(), "copy_disable", "關閉", true, copy_mode == "disable", None::<&str>).unwrap();
    let ocr_recognize = MenuItem::with_id(&app_handle(), "ocr_recognize", "文字識別", true, None::<&str>).unwrap();
    let ocr_translate = MenuItem::with_id(&app_handle(), "ocr_translate", "截圖翻譯", true, None::<&str>).unwrap();
    let config = MenuItem::with_id(&app_handle(), "config", "偏好設定", true, None::<&str>).unwrap();
    let check_update = MenuItem::with_id(&app_handle(), "check_update", "檢查更新", true, None::<&str>).unwrap();
    let view_log = MenuItem::with_id(&app_handle(), "view_log", "查看日誌", true, None::<&str>).unwrap();
    let restart = MenuItem::with_id(&app_handle(), "restart", "重啓程式", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(&app_handle(), "quit", "退出", true, None::<&str>).unwrap();


    let auto_copy_submenu = Submenu::with_id_and_items(&app_handle(), "自動複製", "自動複製", true, &[&copy_source, &copy_target, &copy_source_target, &copy_disable])
    .unwrap();

    Menu::with_items(
        &app_handle(),
        &[
            &input_translate,
            &clipboard_monitor,
            &auto_copy_submenu,
            &ocr_recognize,
            &ocr_translate,
            &config,
            &check_update,
            &view_log,
            &restart,
            &quit,
        ],
    )
    .unwrap()
}

fn tray_menu_ja(
    enable_clipboard_monitor: bool,
    copy_mode: String,
) -> Menu<tauri::Wry> {
    let input_translate = MenuItem::with_id(&app_handle(), "input_translate", "翻訳を入力", true, None::<&str>).unwrap();
    let clipboard_monitor =
        CheckMenuItem::with_id(&app_handle(), "clipboard_monitor", "クリップボードを監視する", true, enable_clipboard_monitor, None::<&str>).unwrap();
    let copy_source =
        CheckMenuItem::with_id(&app_handle(), "copy_source", "原文", true, copy_mode == "source", None::<&str>).unwrap();
    let copy_target =
        CheckMenuItem::with_id(&app_handle(), "copy_target", "訳文", true, copy_mode == "target", None::<&str>).unwrap();
    let copy_source_target =
        CheckMenuItem::with_id(&app_handle(), "copy_source_target", "原文+訳文", true, copy_mode == "source_target", None::<&str>).unwrap();
    let copy_disable =
        CheckMenuItem::with_id(&app_handle(), "copy_disable", "閉じる", true, copy_mode == "disable", None::<&str>).unwrap();
    let ocr_recognize = MenuItem::with_id(&app_handle(), "ocr_recognize", "テキスト認識", true, None::<&str>).unwrap();
    let ocr_translate = MenuItem::with_id(&app_handle(), "ocr_translate", "スクリーンショットの翻訳", true, None::<&str>).unwrap();
    let config = MenuItem::with_id(&app_handle(), "config", "プリファレンス設定", true, None::<&str>).unwrap();
    let check_update = MenuItem::with_id(&app_handle(), "check_update", "更新を確認する", true, None::<&str>).unwrap();
    let view_log = MenuItem::with_id(&app_handle(), "view_log", "ログを見る", true, None::<&str>).unwrap();
    let restart = MenuItem::with_id(&app_handle(), "restart", "アプリの再起動", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(&app_handle(), "quit", "退出する", true, None::<&str>).unwrap();


    let auto_copy_submenu = Submenu::with_id_and_items(&app_handle(), "自動コピー", "自動コピー", true, &[&copy_source, &copy_target, &copy_source_target, &copy_disable])
    .unwrap();

    Menu::with_items(
        &app_handle(),
        &[
            &input_translate,
            &clipboard_monitor,
            &auto_copy_submenu,
            &ocr_recognize,
            &ocr_translate,
            &config,
            &check_update,
            &view_log,
            &restart,
            &quit,
        ],
    )
    .unwrap()
}

fn tray_menu_ko(
    enable_clipboard_monitor: bool,
    copy_mode: String,
) -> Menu<tauri::Wry> {
    let input_translate = MenuItem::with_id(&app_handle(), "input_translate", "입력 번역", true, None::<&str>).unwrap();
    let clipboard_monitor =
        CheckMenuItem::with_id(&app_handle(), "clipboard_monitor", "감청 전단판", true, enable_clipboard_monitor, None::<&str>).unwrap();
    let copy_source =
        CheckMenuItem::with_id(&app_handle(), "copy_source", "원문", true, copy_mode == "source", None::<&str>).unwrap();
    let copy_target =
        CheckMenuItem::with_id(&app_handle(), "copy_target", "번역문", true, copy_mode == "target", None::<&str>).unwrap();
    let copy_source_target =
        CheckMenuItem::with_id(&app_handle(), "copy_source_target", "원문+번역문", true, copy_mode == "source_target", None::<&str>).unwrap();
    let copy_disable =
        CheckMenuItem::with_id(&app_handle(), "copy_disable", "닫기", true, copy_mode == "disable", None::<&str>).unwrap();
    let ocr_recognize = MenuItem::with_id(&app_handle(), "ocr_recognize", "문자인식", true, None::<&str>).unwrap();
    let ocr_translate = MenuItem::with_id(&app_handle(), "ocr_translate", "스크린샷 번역", true, None::<&str>).unwrap();
    let config = MenuItem::with_id(&app_handle(), "config", "기본 설정", true, None::<&str>).unwrap();
    let check_update = MenuItem::with_id(&app_handle(), "check_update", "업데이트 확인", true, None::<&str>).unwrap();
    let view_log = MenuItem::with_id(&app_handle(), "view_log", "로그 보기", true, None::<&str>).unwrap();
    let restart = MenuItem::with_id(&app_handle(), "restart", "응용 프로그램 다시 시작", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(&app_handle(), "quit", "퇴출", true, None::<&str>).unwrap();


    let auto_copy_submenu = Submenu::with_id_and_items(&app_handle(), "자동 복사", "자동 복사", true, &[&copy_source, &copy_target, &copy_source_target, &copy_disable])
    .unwrap();

    Menu::with_items(
        &app_handle(),
        &[
            &input_translate,
            &clipboard_monitor,
            &auto_copy_submenu,
            &ocr_recognize,
            &ocr_translate,
            &config,
            &check_update,
            &view_log,
            &restart,
            &quit,
        ],
    )
    .unwrap()
}

fn tray_menu_fr(
    enable_clipboard_monitor: bool,
    copy_mode: String,
) -> Menu<tauri::Wry> {
    let input_translate = MenuItem::with_id(&app_handle(), "input_translate", "Traduction d'entrée", true, None::<&str>).unwrap();
    let clipboard_monitor =
        CheckMenuItem::with_id(&app_handle(), "clipboard_monitor", "Surveiller le presse-papiers", true, enable_clipboard_monitor, None::<&str>).unwrap();
    let copy_source =
        CheckMenuItem::with_id(&app_handle(), "copy_source", "Source", true, copy_mode == "source", None::<&str>).unwrap();
    let copy_target =
        CheckMenuItem::with_id(&app_handle(), "copy_target", "Cible", true, copy_mode == "target", None::<&str>).unwrap();
    let copy_source_target =
        CheckMenuItem::with_id(&app_handle(), "copy_source_target", "Source+Cible", true, copy_mode == "source_target", None::<&str>).unwrap();
    let copy_disable =
        CheckMenuItem::with_id(&app_handle(), "copy_disable", "Désactiver", true, copy_mode == "disable", None::<&str>).unwrap();
    let ocr_recognize = MenuItem::with_id(&app_handle(), "ocr_recognize", "Reconnaissance de texte", true, None::<&str>).unwrap();
    let ocr_translate = MenuItem::with_id(&app_handle(), "ocr_translate", "Traduction d'image", true, None::<&str>).unwrap();
    let config = MenuItem::with_id(&app_handle(), "config", "Paramètres", true, None::<&str>).unwrap();
    let check_update = MenuItem::with_id(&app_handle(), "check_update", "Vérifier les mises à jour", true, None::<&str>).unwrap();
    let view_log = MenuItem::with_id(&app_handle(), "view_log", "Voir le journal", true, None::<&str>).unwrap();
    let restart = MenuItem::with_id(&app_handle(), "restart", "Redémarrer l'application", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(&app_handle(), "quit", "Quitter", true, None::<&str>).unwrap();


    let auto_copy_submenu = Submenu::with_id_and_items(&app_handle(), "Copier automatiquement", "Copier automatiquement", true, &[&copy_source, &copy_target, &copy_source_target, &copy_disable])
    .unwrap();

    Menu::with_items(
        &app_handle(),
        &[
            &input_translate,
            &clipboard_monitor,
            &auto_copy_submenu,
            &ocr_recognize,
            &ocr_translate,
            &config,
            &check_update,
            &view_log,
            &restart,
            &quit,
        ],
    )
    .unwrap()
}

fn tray_menu_de(
    enable_clipboard_monitor: bool,
    copy_mode: String,
) -> Menu<tauri::Wry> {
    let input_translate = MenuItem::with_id(&app_handle(), "input_translate", "Eingabeübersetzung", true, None::<&str>).unwrap();
    let clipboard_monitor =
        CheckMenuItem::with_id(&app_handle(), "clipboard_monitor", "Zwischenablage überwachen", true, enable_clipboard_monitor, None::<&str>).unwrap();
    let copy_source =
        CheckMenuItem::with_id(&app_handle(), "copy_source", "Quelle", true, copy_mode == "source", None::<&str>).unwrap();
    let copy_target =
        CheckMenuItem::with_id(&app_handle(), "copy_target", "Ziel", true, copy_mode == "target", None::<&str>).unwrap();
    let copy_source_target =
        CheckMenuItem::with_id(&app_handle(), "copy_source_target", "Quelle+Ziel", true, copy_mode == "source_target", None::<&str>).unwrap();
    let copy_disable =
        CheckMenuItem::with_id(&app_handle(), "copy_disable", "Deaktivieren", true, copy_mode == "disable", None::<&str>).unwrap();
    let ocr_recognize = MenuItem::with_id(&app_handle(), "ocr_recognize", "Texterkennung", true, None::<&str>).unwrap();
    let ocr_translate = MenuItem::with_id(&app_handle(), "ocr_translate", "Bildübersetzung", true, None::<&str>).unwrap();
    let config = MenuItem::with_id(&app_handle(), "config", "Einstellungen", true, None::<&str>).unwrap();
    let check_update = MenuItem::with_id(&app_handle(), "check_update", "Auf Updates prüfen", true, None::<&str>).unwrap();
    let view_log = MenuItem::with_id(&app_handle(), "view_log", "Protokoll anzeigen", true, None::<&str>).unwrap();
    let restart = MenuItem::with_id(&app_handle(), "restart", "Anwendung neu starten", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(&app_handle(), "quit", "Beenden", true, None::<&str>).unwrap();


    let auto_copy_submenu = Submenu::with_id_and_items(&app_handle(), "Automatisch kopieren", "Automatisch kopieren", true, &[&copy_source, &copy_target, &copy_source_target, &copy_disable])
    .unwrap();

    Menu::with_items(
        &app_handle(),
        &[
            &input_translate,
            &clipboard_monitor,
            &auto_copy_submenu,
            &ocr_recognize,
            &ocr_translate,
            &config,
            &check_update,
            &view_log,
            &restart,
            &quit,
        ],
    )
    .unwrap()
}

fn tray_menu_ru(
    enable_clipboard_monitor: bool,
    copy_mode: String,
) -> Menu<tauri::Wry> {
    let input_translate = MenuItem::with_id(&app_handle(), "input_translate", "Ввод перевода", true, None::<&str>).unwrap();
    let clipboard_monitor =
        CheckMenuItem::with_id(&app_handle(), "clipboard_monitor", "Следить за буфером обмена", true, enable_clipboard_monitor, None::<&str>).unwrap();
    let copy_source =
        CheckMenuItem::with_id(&app_handle(), "copy_source", "Источник", true, copy_mode == "source", None::<&str>).unwrap();
    let copy_target =
        CheckMenuItem::with_id(&app_handle(), "copy_target", "Цель", true, copy_mode == "target", None::<&str>).unwrap();
    let copy_source_target =
        CheckMenuItem::with_id(&app_handle(), "copy_source_target", "Источник+Цель", true, copy_mode == "source_target", None::<&str>).unwrap();
    let copy_disable =
        CheckMenuItem::with_id(&app_handle(), "copy_disable", "Отключить", true, copy_mode == "disable", None::<&str>).unwrap();
    let ocr_recognize = MenuItem::with_id(&app_handle(), "ocr_recognize", "Распознавание текста", true, None::<&str>).unwrap();
    let ocr_translate = MenuItem::with_id(&app_handle(), "ocr_translate", "Перевод изображения", true, None::<&str>).unwrap();
    let config = MenuItem::with_id(&app_handle(), "config", "Настройки", true, None::<&str>).unwrap();
    let check_update = MenuItem::with_id(&app_handle(), "check_update", "Проверить обновления", true, None::<&str>).unwrap();
    let view_log = MenuItem::with_id(&app_handle(), "view_log", "Просмотр журнала", true, None::<&str>).unwrap();
    let restart = MenuItem::with_id(&app_handle(), "restart", "Перезапустить приложение", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(&app_handle(), "quit", "Выйти", true, None::<&str>).unwrap();


    let auto_copy_submenu = Submenu::with_id_and_items(&app_handle(), "Автоматически копировать", "Автоматически копировать", true, &[&copy_source, &copy_target, &copy_source_target, &copy_disable])
    .unwrap();

    Menu::with_items(
        &app_handle(),
        &[
            &input_translate,
            &clipboard_monitor,
            &auto_copy_submenu,
            &ocr_recognize,
            &ocr_translate,
            &config,
            &check_update,
            &view_log,
            &restart,
            &quit,
        ],
    )
    .unwrap()
}

fn tray_menu_pt_br(
    enable_clipboard_monitor: bool,
    copy_mode: String,
) -> Menu<tauri::Wry> {
    let input_translate = MenuItem::with_id(&app_handle(), "input_translate", "Tradução de entrada", true, None::<&str>).unwrap();
    let clipboard_monitor =
        CheckMenuItem::with_id(&app_handle(), "clipboard_monitor", "Monitorar área de transferência", true, enable_clipboard_monitor, None::<&str>).unwrap();
    let copy_source =
        CheckMenuItem::with_id(&app_handle(), "copy_source", "Fonte", true, copy_mode == "source", None::<&str>).unwrap();
    let copy_target =
        CheckMenuItem::with_id(&app_handle(), "copy_target", "Alvo", true, copy_mode == "target", None::<&str>).unwrap();
    let copy_source_target =
        CheckMenuItem::with_id(&app_handle(), "copy_source_target", "Fonte+Alvo", true, copy_mode == "source_target", None::<&str>).unwrap();
    let copy_disable =
        CheckMenuItem::with_id(&app_handle(), "copy_disable", "Desativar", true, copy_mode == "disable", None::<&str>).unwrap();
    let ocr_recognize = MenuItem::with_id(&app_handle(), "ocr_recognize", "Reconhecimento de texto", true, None::<&str>).unwrap();
    let ocr_translate = MenuItem::with_id(&app_handle(), "ocr_translate", "Tradução de imagem", true, None::<&str>).unwrap();
    let config = MenuItem::with_id(&app_handle(), "config", "Configurações", true, None::<&str>).unwrap();
    let check_update = MenuItem::with_id(&app_handle(), "check_update", "Verificar atualizações", true, None::<&str>).unwrap();
    let view_log = MenuItem::with_id(&app_handle(), "view_log", "Ver registro", true, None::<&str>).unwrap();
    let restart = MenuItem::with_id(&app_handle(), "restart", "Reiniciar aplicativo", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(&app_handle(), "quit", "Sair", true, None::<&str>).unwrap();


    let auto_copy_submenu = Submenu::with_id_and_items(&app_handle(), "Copiar automaticamente", "Copiar automaticamente", true, &[&copy_source, &copy_target, &copy_source_target, &copy_disable])
    .unwrap();

    Menu::with_items(
        &app_handle(),
        &[
            &input_translate,
            &clipboard_monitor,
            &auto_copy_submenu,
            &ocr_recognize,
            &ocr_translate,
            &config,
            &check_update,
            &view_log,
            &restart,
            &quit,
        ],
    )
    .unwrap()
}

fn tray_menu_fa(
    enable_clipboard_monitor: bool,
    copy_mode: String,
) -> Menu<tauri::Wry> {
    let input_translate = MenuItem::with_id(&app_handle(), "input_translate", "ترجمه ورودی", true, None::<&str>).unwrap();
    let clipboard_monitor =
        CheckMenuItem::with_id(&app_handle(), "clipboard_monitor", "نظارت بر کلیپ بورد", true, enable_clipboard_monitor, None::<&str>).unwrap();
    let copy_source =
        CheckMenuItem::with_id(&app_handle(), "copy_source", "منبع", true, copy_mode == "source", None::<&str>).unwrap();
    let copy_target =
        CheckMenuItem::with_id(&app_handle(), "copy_target", "هدف", true, copy_mode == "target", None::<&str>).unwrap();
    let copy_source_target =
        CheckMenuItem::with_id(&app_handle(), "copy_source_target", "منبع+هدف", true, copy_mode == "source_target", None::<&str>).unwrap();
    let copy_disable =
        CheckMenuItem::with_id(&app_handle(), "copy_disable", "غیرفعال", true, copy_mode == "disable", None::<&str>).unwrap();
    let ocr_recognize = MenuItem::with_id(&app_handle(), "ocr_recognize", "تشخیص متن", true, None::<&str>).unwrap();
    let ocr_translate = MenuItem::with_id(&app_handle(), "ocr_translate", "ترجمه تصویر", true, None::<&str>).unwrap();
    let config = MenuItem::with_id(&app_handle(), "config", "تنظیمات", true, None::<&str>).unwrap();
    let check_update = MenuItem::with_id(&app_handle(), "check_update", "بررسی به روزرسانی", true, None::<&str>).unwrap();
    let view_log = MenuItem::with_id(&app_handle(), "view_log", "مشاهده گزارش", true, None::<&str>).unwrap();
    let restart = MenuItem::with_id(&app_handle(), "restart", "راه اندازی مجدد", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(&app_handle(), "quit", "خروج", true, None::<&str>).unwrap();


    let auto_copy_submenu = Submenu::with_id_and_items(&app_handle(), "کپی خودکار", "کپی خودکار", true, &[&copy_source, &copy_target, &copy_source_target, &copy_disable])
    .unwrap();

    Menu::with_items(
        &app_handle(),
        &[
            &input_translate,
            &clipboard_monitor,
            &auto_copy_submenu,
            &ocr_recognize,
            &ocr_translate,
            &config,
            &check_update,
            &view_log,
            &restart,
            &quit,
        ],
    )
    .unwrap()
}

fn tray_menu_uk(
    enable_clipboard_monitor: bool,
    copy_mode: String,
) -> Menu<tauri::Wry> {
    let input_translate = MenuItem::with_id(&app_handle(), "input_translate", "Введення перекладу", true, None::<&str>).unwrap();
    let clipboard_monitor =
        CheckMenuItem::with_id(&app_handle(), "clipboard_monitor", "Спостерігати за буфером обміну", true, enable_clipboard_monitor, None::<&str>).unwrap();
    let copy_source =
        CheckMenuItem::with_id(&app_handle(), "copy_source", "Джерело", true, copy_mode == "source", None::<&str>).unwrap();
    let copy_target =
        CheckMenuItem::with_id(&app_handle(), "copy_target", "Ціль", true, copy_mode == "target", None::<&str>).unwrap();
    let copy_source_target =
        CheckMenuItem::with_id(&app_handle(), "copy_source_target", "Джерело+Ціль", true, copy_mode == "source_target", None::<&str>).unwrap();
    let copy_disable =
        CheckMenuItem::with_id(&app_handle(), "copy_disable", "Вимкнути", true, copy_mode == "disable", None::<&str>).unwrap();
    let ocr_recognize = MenuItem::with_id(&app_handle(), "ocr_recognize", "Розпізнавання тексту", true, None::<&str>).unwrap();
    let ocr_translate = MenuItem::with_id(&app_handle(), "ocr_translate", "Переклад зображення", true, None::<&str>).unwrap();
    let config = MenuItem::with_id(&app_handle(), "config", "Налаштування", true, None::<&str>).unwrap();
    let check_update = MenuItem::with_id(&app_handle(), "check_update", "Перевірити оновлення", true, None::<&str>).unwrap();
    let view_log = MenuItem::with_id(&app_handle(), "view_log", "Переглянути журнал", true, None::<&str>).unwrap();
    let restart = MenuItem::with_id(&app_handle(), "restart", "Перезапустити додаток", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(&app_handle(), "quit", "Вийти", true, None::<&str>).unwrap();


    let auto_copy_submenu = Submenu::with_id_and_items(&app_handle(), "Автоматично копіювати", "Автоматично копіювати", true, &[&copy_source, &copy_target, &copy_source_target, &copy_disable])
    .unwrap();

    Menu::with_items(
        &app_handle(),
        &[
            &input_translate,
            &clipboard_monitor,
            &auto_copy_submenu,
            &ocr_recognize,
            &ocr_translate,
            &config,
            &check_update,
            &view_log,
            &restart,
            &quit,
        ],
    )
    .unwrap()
}

fn app_handle() -> AppHandle {
    crate::APP.get().unwrap().clone()
}
