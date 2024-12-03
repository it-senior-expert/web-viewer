// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg(all(desktop, not(test)))]

use std::sync::atomic::{ AtomicBool, Ordering };
use tauri::{
    menu::{ Menu, MenuItem },
    tray::{ MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent },
    Manager,
    Runtime,
};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    #[cfg(target_os = "macos")]
    let set_title_i = MenuItem::with_id(app, "set-title", "Set Title", true, None::<&str>)?;
    let switch_i = MenuItem::with_id(app, "switch-menu", "Switch Menu", true, None::<&str>)?;
    // let always_on_top_i = MenuItem::with_id(app, "always_on_top", "Always On Top", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let remove_tray_i = MenuItem::with_id(
        app,
        "remove-tray",
        "Remove Tray icon",
        true,
        None::<&str>
    )?;
    let menu1 = Menu::with_items(
        app,
        &[ 
            #[cfg(target_os = "macos")] &set_title_i,
            &quit_i,
            &remove_tray_i,
        ]
    )?;
    let menu2 = Menu::with_items(
        app,
        &[&quit_i, &remove_tray_i]
    )?;

    let _ = TrayIconBuilder::with_id("tray-1")
        .tooltip("Tauri")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu1)
        .menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app);

    Ok(())
}
