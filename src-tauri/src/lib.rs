#[macro_use]
extern crate lazy_static;
#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
lazy_static! {
    static ref AUTO_START_SELECTED: Mutex<bool> = Mutex::new(false);
    static ref ALWAYS_ON_TOP: Mutex<bool> = Mutex::new(false);
    static ref LEFT_WIDTH: Mutex<f64> = Mutex::new(1000.0);
    static ref WEBVIEW_HEIGHT: Mutex<f64> = Mutex::new(1000.0);
    static ref WEBVIEW_WIDTH: Mutex<f64> = Mutex::new(1000.0);
    static ref WEBVIEW1: Mutex<Option<Arc<Mutex<Webview>>>> = Mutex::new(None);
    static ref WEBVIEW2: Mutex<Option<Arc<Mutex<Webview>>>> = Mutex::new(None);
}
use serde_json::Number;
use std::sync::{Arc, Mutex};
use tauri::image::Image;
use tauri::{
    menu::{Menu, MenuBuilder, MenuItem, MenuItemBuilder, SubmenuBuilder},
    tray::TrayIconBuilder,
};
use tauri::{AppHandle, Emitter};
use tauri::{LogicalPosition, LogicalSize, Manager, Webview, WebviewUrl};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tauri::command]
fn submit_width(app_handle: AppHandle, width_: f64) {
    app_handle.emit("new-width", &width_).unwrap();
}

#[tauri::command]
async fn new_size(_app_handle: tauri::AppHandle, leftWidth: Number, width: Number, height: Number) {
    println!("Successfully changed Left WIDTH to: {}", leftWidth);

    // Ensure that the value of `width` is converted to f64 for calculations.
    let new_left_width = leftWidth.as_f64().unwrap_or(0.0);
    let new_width = width.as_f64().unwrap_or(0.0);
    let new_height = height.as_f64().unwrap_or(0.0);

    // Lock the WEBVIEW1 and WEBVIEW2 to make sure they are safely accessed and updated.
    let mut webview1 = WEBVIEW1.lock().unwrap();
    let mut webview2 = WEBVIEW2.lock().unwrap();
    *WEBVIEW_WIDTH.lock().unwrap() = width.as_f64().unwrap_or(0.0);
    *WEBVIEW_HEIGHT.lock().unwrap() = height.as_f64().unwrap_or(0.0);
    if let Some(webview1_instance) = &mut *webview1 {
        // Update the width of WEBVIEW1
        let window = webview1_instance.lock().unwrap();
        let new_webview1_width = new_left_width;

        if let Err(e) = window.set_size(tauri::LogicalSize::new(
            new_webview1_width,
            new_height - 77.0,
        )) {
            eprintln!("Failed to set new width for WEBVIEW1: {}", e);
        } else {
            println!(
                "Successfully updated WEBVIEW1 width to: {}",
                new_webview1_width
            );
        }
    } else {
        eprintln!("Error: Webview 'main2' not found!");
    }
    if let Some(webview2_instance) = &mut *webview2 {
        // Update the width and position of WEBVIEW2
        let window = webview2_instance.lock().unwrap();
        let new_webview2_width = new_width - new_left_width; // Ensure the total width is maintained

        // Calculate the new position for WEBVIEW2
        let new_position_x = new_left_width; // Position the second webview at the end of the first one

        if let Err(e) = window.set_size(tauri::LogicalSize::new(
            new_webview2_width,
            new_height - 40.0,
        )) {
            eprintln!("Failed to set new width for WEBVIEW2: {}", e);
        } else {
            println!(
                "Successfully updated WEBVIEW2 width to: {}",
                new_webview2_width
            );
        }

        if let Err(e) =
            window.set_position(tauri::LogicalPosition::new(new_position_x + 10.0, 40.0))
        {
            eprintln!("Failed to set new position for WEBVIEW2: {}", e);
        } else {
            println!(
                "Successfully updated WEBVIEW2 position to: ({}, 0)",
                new_position_x + 10.0
            );
        }
    } else {
        eprintln!("Error: Webview 'main3' not found!");
    }
}

#[tauri::command]
fn new_left_url(_app_handle: tauri::AppHandle, url: String) {
    if let Some(webview) = &*WEBVIEW1.lock().unwrap() {
        if let Err(e) = webview
            .lock()
            .unwrap()
            .eval(&format!("window.location.href = '{}';", url))
        {
            eprintln!("Error while evaluating script: {}", e);
        } else {
            println!("Successfully changed Left URL to: {}", url);
        }
    } else {
        eprintln!("Error: Webview 'main2' not found!");
    }
}
#[tauri::command]
fn new_right_url(_app_handle: tauri::AppHandle, url: String) {
    if let Some(webview) = &*WEBVIEW2.lock().unwrap() {
        if let Err(e) = webview
            .lock()
            .unwrap()
            .eval(&format!("window.location.href = '{}';", url))
        {
            eprintln!("Error while evaluating script: {}", e);
        } else {
            println!("Successfully changed Riht URL to: {}", url);
        }
    } else {
        eprintln!("Error: Webview 'main2' not found!");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            use tauri_plugin_notification::NotificationExt;
            app.notification()
                .builder()
                .title("Web-Viewer")
                .body("This is webview appication.")
                .show()
                .unwrap();

            let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let alwaysontop_i =
                MenuItem::with_id(app, "always_on_top", "Always On Top", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu1 = Menu::with_items(app, &[&settings_i, &alwaysontop_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(Image::from_path("./icons/icon.png")?)
                .menu(&menu1)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "settings" => {
                        let handle = app.clone();
                        std::thread::spawn(move || {
                            let webview_window = tauri::WebviewWindowBuilder::new(
                                &handle,
                                "setting_window",
                                tauri::WebviewUrl::App("index_setting.html".into()),
                            )
                            .inner_size(480.0, 250.0)
                            .build()
                            .unwrap();
                        });
                    }
                    "always_on_top" => {
                        if let Some(window) = app.get_window("main") {
                            let mut always_on_top = ALWAYS_ON_TOP.lock().unwrap();
                            *always_on_top = !*always_on_top;

                            if *always_on_top {
                                window.set_always_on_top(true).unwrap();
                            } else {
                                window.set_always_on_top(false).unwrap();
                            }
                        }
                    }
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            // Initialize the main window (hidden)
            let window = app.get_window("main").unwrap();

            *LEFT_WIDTH.lock().unwrap() = 800.0;
            let left_width = *LEFT_WIDTH.lock().unwrap();
            let new_webview_width1 = *WEBVIEW_WIDTH.lock().unwrap();
            let new_webview_width2 = *WEBVIEW_WIDTH.lock().unwrap() - left_width;
            let new_webview_height1 = *WEBVIEW_HEIGHT.lock().unwrap();
            let new_webview_height2 = *WEBVIEW_HEIGHT.lock().unwrap();

            // window
            // .set_min_size(Some(tauri::LogicalSize::new(width, height)))
            // .expect("Failed to set window minimum size");

            // Add the first webview (left side)
            let webview1 = window.add_child(
                tauri::webview::WebviewBuilder::new(
                    "main2",
                    WebviewUrl::External("https://google.com/".parse().unwrap()),
                )
                .devtools(true)
                .auto_resize(),
                LogicalPosition::new(0.0, 77.0),
                LogicalSize::new(
                    new_webview_width1 - 5.0,
                    (new_webview_height1 as f64) - 77.0,
                ),
            )?;

            *WEBVIEW1.lock().unwrap() = Some(Arc::new(Mutex::new(webview1.clone())));

            // Add the second webview (right side)
            let webview2 = window.add_child(
                tauri::webview::WebviewBuilder::new(
                    "main3",
                    WebviewUrl::External("https://google.com/".parse().unwrap()),
                )
                .devtools(true)
                .auto_resize(),
                LogicalPosition::new(new_webview_width1 + 5.0, 40.0), // Position the second webview
                LogicalSize::new(
                    new_webview_width2 - 5.0,
                    (new_webview_height2 as f64) - 40.0,
                ), // Width based on the ratio
            )?;

            *WEBVIEW2.lock().unwrap() = Some(Arc::new(Mutex::new(webview2.clone())));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            submit_width,
            new_left_url,
            new_right_url,
            new_size
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
