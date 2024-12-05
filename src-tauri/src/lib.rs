#[macro_use]
extern crate lazy_static;
#[cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
lazy_static! {
    static ref AUTO_START_SELECTED: Mutex<bool> = Mutex::new(false);
    static ref ALWAYS_ON_TOP: Mutex<bool> = Mutex::new(false);
    static ref DESIRED_WIDTH_RATIO: Mutex<f64> = Mutex::new(5.0);
    static ref WEBVIEW1: Mutex<Option<Arc<Mutex<Webview>>>> = Mutex::new(None);
    static ref WEBVIEW2: Mutex<Option<Arc<Mutex<Webview>>>> = Mutex::new(None);
}
use tauri_plugin_store::StoreExt;
use serde_json::json;
use std::sync::{ Arc, Mutex };
use tauri::{ LogicalPosition, LogicalSize, Manager, Webview, WebviewUrl };
use tauri::image::Image;
use tauri::{
    menu::{ Menu, MenuItem, MenuBuilder, SubmenuBuilder, MenuItemBuilder },
    tray::{ TrayIconBuilder },
};
use tauri::{ AppHandle, Emitter };
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn submit_width(app_handle: AppHandle, width_: f64) {
    app_handle.emit("new-width", &width_).unwrap();
}

#[tauri::command]
fn new_left_url(_app_handle: tauri::AppHandle, url: String) {
    if let Some(webview) = &*WEBVIEW1.lock().unwrap() {
        if let Err(e) = webview.lock().unwrap().eval(&format!("window.location.href = '{}';", url)) {
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
        if let Err(e) = webview.lock().unwrap().eval(&format!("window.location.href = '{}';", url)) {
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
    tauri::Builder
        ::default()
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

            let alwaysontop_i = MenuItem::with_id(
                app,
                "always_on_top",
                "Always On Top",
                true,
                None::<&str>
            )?;
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

            // Open the store to retrieve data
            let store = app.store("store.json")?;
            let ratio_value: f64 = store
                .get("ratio_of_screen")
                .and_then(|v| v.get("value").cloned()) // Clone the value to own it
                .and_then(|v| v.as_f64()) // Convert the cloned value to f64
                .unwrap_or(5.0);
            *DESIRED_WIDTH_RATIO.lock().unwrap() = ratio_value;

            // Calculate the webview widths based on the ratio
            let desired_width_ratio = *DESIRED_WIDTH_RATIO.lock().unwrap();
            let new_webview_width2 = (width as f64) / desired_width_ratio;
            let new_webview_width1 = (width as f64) - new_webview_width2;
            // submit_width(app.handle().clone(), new_webview_width1);

            window
                .set_min_size(Some(tauri::LogicalSize::new(width, height)))
                .expect("Failed to set window minimum size");

            // Add the first webview (left side)
            let webview1 = window.add_child(
                tauri::webview::WebviewBuilder
                    ::new(
                        "main2",
                        WebviewUrl::External(
                            "https://google.com/".parse().unwrap()
                        )
                    )
                    .devtools(true)
                    .auto_resize(),
                LogicalPosition::new(0.0, 70.0),
                LogicalSize::new(new_webview_width1, (height as f64) - 70.0)
            )?;

            *WEBVIEW1.lock().unwrap() = Some(Arc::new(Mutex::new(webview1.clone())));

            // Add the second webview (right side)
            let webview2 = window.add_child(
                tauri::webview::WebviewBuilder
                    ::new("main3", WebviewUrl::External("https://google.com/".parse().unwrap()))
                    .devtools(true)
                    .auto_resize(),
                LogicalPosition::new(new_webview_width1, 34.0), // Position the second webview
                LogicalSize::new(new_webview_width2, (height as f64) - 34.0) // Width based on the ratio
            )?;

            *WEBVIEW2.lock().unwrap() = Some(Arc::new(Mutex::new(webview2.clone())));

            // Event handler for menu items
            app.on_menu_event(move |app, event| {
                let app_handle = app.app_handle().clone();
                let mut desired_width_ratio = DESIRED_WIDTH_RATIO.lock().unwrap(); // Access the shared ratio
                if event.id() == "8:2" {
                    *desired_width_ratio = 5.0; // Set ratio to 80:20
                    submit_width(
                        app_handle.clone(),
                        &(width as f64) - &(width as f64) / *desired_width_ratio
                    );
                    store.set("ratio_of_screen", json!({ "value": 5.0 }));
                } else if event.id() == "7:3" {
                    *desired_width_ratio = 3.0; // Set ratio to 70:30
                    submit_width(
                        app_handle.clone(),
                        &(width as f64) - &(width as f64) / *desired_width_ratio
                    );
                    store.set("ratio_of_screen", json!({ "value": 3.0 }));
                } else if event.id() == "6:4" {
                    *desired_width_ratio = 2.5; // Set ratio to 60:40
                    submit_width(
                        app_handle.clone(),
                        &(width as f64) - &(width as f64) / *desired_width_ratio
                    );
                    store.set("ratio_of_screen", json!({ "value": 2.5 }));
                } else if event.id() == "5:5" {
                    *desired_width_ratio = 2.0; // Set ratio to 50:50
                    submit_width(
                        app_handle.clone(),
                        &(width as f64) - &(width as f64) / *desired_width_ratio
                    );
                    store.set("ratio_of_screen", json!({ "value": 2.0 }));
                }
                let new_webview_width2 = (width as f64) / *desired_width_ratio;
                let new_webview_width1 = (width as f64) - new_webview_width2;

                // Update the size of both webviews
                webview1.set_size(LogicalSize::new(new_webview_width1, height as f64 - 70.0)).unwrap();
                webview2.set_size(LogicalSize::new(new_webview_width2, height as f64 - 34.0)).unwrap();

                webview2.set_position(LogicalPosition::new(new_webview_width1, 34.0)).unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![submit_width, new_left_url, new_right_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
