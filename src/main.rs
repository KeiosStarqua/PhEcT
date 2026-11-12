// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::process::Command;
use std::rc::Rc;
use std::time::Duration;

use slint::{ModelRc, Timer, TimerMode, VecModel};

slint::include_modules!();

fn short_cwd(cwd: &str) -> String {
    if let Some(home) = std::env::var_os("HOME").and_then(|h| h.into_string().ok()) {
        if let Some(rest) = cwd.strip_prefix(&home) {
            return format!("~{rest}");
        }
    }
    cwd.to_string()
}

fn fetch_terminals() -> Result<Vec<Terminal>, ()> {
    let out = Command::new("hc").arg("ls").output().map_err(|_| ())?;
    if !out.status.success() {
        return Err(());
    }
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).map_err(|_| ())?;
    let list = json
        .get("terminals")
        .and_then(|t| t.as_array())
        .cloned()
        .unwrap_or_default();

    Ok(list
        .iter()
        .map(|t| Terminal {
            address: t.get("address").and_then(|v| v.as_str()).unwrap_or("?").into(),
            name: t.get("name").and_then(|v| v.as_str()).unwrap_or("").into(),
            agent: t.get("agent").and_then(|v| v.as_str()).unwrap_or("").into(),
            cwd: short_cwd(t.get("cwd").and_then(|v| v.as_str()).unwrap_or("")).into(),
            state: t.get("state").and_then(|v| v.as_str()).unwrap_or("").into(),
            status: t.get("status").and_then(|v| v.as_str()).unwrap_or("idle").into(),
            turn: t.get("turn").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
        })
        .collect())
}

fn refresh(ui: &AppWindow) {
    match fetch_terminals() {
        Ok(terms) => {
            let working = terms.iter().filter(|t| t.status == "working").count();
            ui.set_summary(format!("{} live · {} working", terms.len(), working).into());
            ui.set_terminals(ModelRc::new(Rc::new(VecModel::from(terms))));
            ui.set_hc_error(false);
        }
        Err(()) => {
            ui.set_hc_error(true);
            ui.set_summary("hc ls failed".into());
        }
    }
    ui.set_updated(chrono::Local::now().format("%H:%M:%S").to_string().into());
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    refresh(&ui);

    let ui_handle = ui.as_weak();
    ui.on_refresh(move || {
        if let Some(ui) = ui_handle.upgrade() {
            refresh(&ui);
        }
    });

    // poll every 2 seconds
    let poll_handle = ui.as_weak();
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_secs(2), move || {
        if let Some(ui) = poll_handle.upgrade() {
            refresh(&ui);
        }
    });

    ui.run()?;
    drop(timer);

    Ok(())
}
