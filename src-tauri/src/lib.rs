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
use serde_json::{json, Number};
use std::sync::{Arc, Mutex};
use tauri::image::Image;
use tauri::{
    menu::{Menu, MenuBuilder, MenuItem, MenuItemBuilder, SubmenuBuilder},
    tray::TrayIconBuilder,
};
use tauri::{AppHandle, Emitter};
use tauri::{LogicalPosition, LogicalSize, Manager, Webview, WebviewUrl};
use tauri_plugin_store::StoreExt;
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tauri::command]
fn submit_width(app_handle: AppHandle, width_: f64) {
    app_handle.emit("new-width", &width_).unwrap();
}

#[tauri::command]
async fn new_left_width(_app_handle: tauri::AppHandle, width: Number) {
    println!("Successfully changed Left WIDTH to: {}", width);

    // Ensure that the value of `width` is converted to f64 for calculations.
    let new_width = width.as_f64().unwrap_or(0.0);

    // Lock the WEBVIEW1 and WEBVIEW2 to make sure they are safely accessed and updated.
    let mut webview1 = WEBVIEW1.lock().unwrap();
    let mut webview2 = WEBVIEW2.lock().unwrap();
    let origin_width = *WEBVIEW_WIDTH.lock().unwrap();
    let height = *WEBVIEW_HEIGHT.lock().unwrap();
    if let Some(webview1_instance) = &mut *webview1 {
        // Update the width of WEBVIEW1
        let window = webview1_instance.lock().unwrap();
        let new_webview1_width = new_width;

        if let Err(e) = window.set_size(tauri::LogicalSize::new(new_webview1_width, height), ) {
            eprintln!("Failed to set new width for WEBVIEW1: {}", e);
        } else {
            println!("Successfully updated WEBVIEW1 width to: {}", new_webview1_width);
        }
    } else {
        eprintln!("Error: Webview 'main2' not found!");
    }
    if let Some(webview2_instance) = &mut *webview2 {
        // Update the width and position of WEBVIEW2
        let window = webview2_instance.lock().unwrap();
        let new_webview2_width = origin_width - new_width; // Ensure the total width is maintained
    
        // Calculate the new position for WEBVIEW2
        let new_position_x = new_width; // Position the second webview at the end of the first one
    
        if let Err(e) = window.set_size(tauri::LogicalSize::new(new_webview2_width, height)) {
            eprintln!("Failed to set new width for WEBVIEW2: {}", e);
        } else {
            println!("Successfully updated WEBVIEW2 width to: {}", new_webview2_width);
        }
    
        if let Err(e) = window.set_position(tauri::LogicalPosition::new(new_position_x + 10.0, 40.0)) {
            eprintln!("Failed to set new position for WEBVIEW2: {}", e);
        } else {
            println!("Successfully updated WEBVIEW2 position to: ({}, 0)", new_position_x + 10.0);
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
        .setup(|app| {
            use tauri_plugin_notification::NotificationExt;
            app.notification()
                .builder()
                .title("Tauri")
                .body("This is webview appication.")
                .show()
                .unwrap();

            // Create menu items
            let guide = MenuItemBuilder::with_id("Guide", "Guide").build(app)?;
            let menu = MenuBuilder::new(app).items(&[&guide]).build()?;
            let submenu = SubmenuBuilder::new(app, "Resize")
                .item(&MenuItem::with_id(app, "8:2", "8:2", true, None::<&str>)?)
                .item(&MenuItem::with_id(app, "7:3", "7:3", true, None::<&str>)?)
                .item(&MenuItem::with_id(app, "6:4", "6:4", true, None::<&str>)?)
                .item(&MenuItem::with_id(app, "5:5", "5:5", true, None::<&str>)?)
                .build()?;
            menu.append(&submenu)?;
            app.set_menu(menu)?;

            let alwaysontop_i =
                MenuItem::with_id(app, "always_on_top", "Always On Top", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu1 = Menu::with_items(app, &[&alwaysontop_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(Image::from_path("./icons/icon.png")?)
                .menu(&menu1)
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "always_on_top" => {
                            // Get the main window
                            if let Some(window) = app.get_window("main") {
                                // Set the window to always on top or remove the setting based on the current state
                                let mut always_on_top = ALWAYS_ON_TOP.lock().unwrap();
                                *always_on_top = !*always_on_top; // Toggle the value

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
                    }
                })
                .build(app)?;

            // Initialize the main window (hidden)
            let window = app.get_window("main").unwrap();

            let width: f64;
            let height: f64;
            let mut scale = 1.0;

            if let Ok(Some(monitor)) = window.current_monitor() {
                scale = monitor.scale_factor();
            } else {
                println!("Failed to retrieve current monitor information");
            }
            if let Ok(physical_size) = window.inner_size() {
                width = (physical_size.width as f64) / scale;
                height = (physical_size.height as f64) / scale;
            } else {
                todo!();
            }

            *WEBVIEW_WIDTH.lock().unwrap() = width;
            *WEBVIEW_HEIGHT.lock().unwrap() = height;

            // Open the store to retrieve data
            let store = app.store("store.json")?;
            let ratio_value: f64 = store
                .get("left_width")
                .and_then(|v| v.get("value").cloned()) // Clone the value to own it
                .and_then(|v| v.as_f64()) // Convert the cloned value to f64
                .unwrap_or(width * 4.0 / 5.0);
            *LEFT_WIDTH.lock().unwrap() = ratio_value;

            // Calculate the webview widths based on the ratio
            let left_width = *LEFT_WIDTH.lock().unwrap();
            let new_webview_width1 = left_width;
            let new_webview_width2 = (width as f64) - left_width;

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
                LogicalSize::new(new_webview_width1 - 5.0, (height as f64) - 77.0),
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
                LogicalPosition::new(new_webview_width1 + 10.0, 40.0), // Position the second webview
                LogicalSize::new(new_webview_width2 - 10.0 , (height as f64) - 40.0), // Width based on the ratio
            )?;

            *WEBVIEW2.lock().unwrap() = Some(Arc::new(Mutex::new(webview2.clone())));

            // Event handler for menu items
            app.on_menu_event(move |app, event| {
                let app_handle = app.app_handle().clone();
                let mut left_width = LEFT_WIDTH.lock().unwrap(); // Access the shared ratio
                if event.id() == "8:2" {
                    *left_width = 5.0; // Set ratio to 80:20
                    submit_width(
                        app_handle.clone(),
                        &(width as f64) - &(width as f64) / *left_width,
                    );
                    store.set("left_width", json!({ "value": 5.0 }));
                } else if event.id() == "7:3" {
                    *left_width = 3.0; // Set ratio to 70:30
                    submit_width(
                        app_handle.clone(),
                        &(width as f64) - &(width as f64) / *left_width,
                    );
                    store.set("left_width", json!({ "value": 3.0 }));
                } else if event.id() == "6:4" {
                    *left_width = 2.5; // Set ratio to 60:40
                    submit_width(
                        app_handle.clone(),
                        &(width as f64) - &(width as f64) / *left_width,
                    );
                    store.set("left_width", json!({ "value": 2.5 }));
                } else if event.id() == "5:5" {
                    *left_width = 2.0; // Set ratio to 50:50
                    submit_width(
                        app_handle.clone(),
                        &(width as f64) - &(width as f64) / *left_width,
                    );
                    store.set("left_width", json!({ "value": 2.0 }));
                }
                let new_webview_width2 = (width as f64) / *left_width;
                let new_webview_width1 = (width as f64) - new_webview_width2;

                // Update the size of both webviews
                webview1
                    .set_size(LogicalSize::new(new_webview_width1, height as f64 - 77.0))
                    .unwrap();
                webview2
                    .set_size(LogicalSize::new(new_webview_width2, height as f64 - 40.0))
                    .unwrap();

                webview2
                    .set_position(LogicalPosition::new(new_webview_width1, 40.0))
                    .unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            submit_width,
            new_left_url,
            new_right_url,
            new_left_width
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
