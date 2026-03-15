use fltk::{prelude::*, *};
use fltk::enums::CallbackTrigger;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QgisSettings {
    pub profile: String,
    pub project_path: String,
    pub reearth_url: Option<String>,
    pub box_url: Option<String>,
    pub settings_dir: Option<String>,
}

impl Default for QgisSettings {
    fn default() -> Self {
        Self {
            profile: "default".to_string(),
            project_path: "ProjectFile.qgs".to_string(),
            reearth_url: None,
            box_url: None,
            settings_dir: Some(r"C:\qgis_launcher".to_string()),
        }
    }
}

fn get_settings_path(custom_dir: &str) -> PathBuf {
    let target_dir = PathBuf::from(custom_dir);
    let target_path = target_dir.join("qgis_settings.json");

    if target_dir.exists() {
        return target_path;
    }

    let mut path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    path.pop();
    path.join("qgis_settings.json")
}

fn load_settings(custom_dir: &str) -> QgisSettings {
    let p = get_settings_path(custom_dir);
    if let Ok(s) = fs::read_to_string(&p) {
        serde_json::from_str(&s).unwrap_or_else(|_| QgisSettings::default())
    } else {
        QgisSettings::default()
    }
}

fn save_settings(custom_dir: &str, s: &QgisSettings) -> Result<(), String> {
    let p = get_settings_path(custom_dir);
    if let Some(parent) = p.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("dir create error: {}", e))?;
        }
    }
    let data = serde_json::to_string_pretty(s).map_err(|e| e.to_string())?;
    fs::write(&p, data).map_err(|e| e.to_string())
}

fn find_qgis_path() -> Option<String> {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let prog_id = match hkcr.open_subkey(r".qgs") {
        Ok(k) => match k.get_value::<String, _>("") {
            Ok(v) => v,
            Err(_) => return None,
        },
        Err(_) => return None,
    };
    let cmd_path = format!(r"{}\\shell\\open\\command", prog_id);
    let command_string = match hkcr.open_subkey(&cmd_path) {
        Ok(k) => match k.get_value::<String, _>("") {
            Ok(v) => v,
            Err(_) => return None,
        },
        Err(_) => return None,
    };
    let exe_path = if command_string.starts_with('"') {
        command_string.split('"').nth(1).unwrap_or(&command_string)
    } else {
        command_string.split_whitespace().next().unwrap_or(&command_string)
    };
    if exe_path.is_empty() {
        None
    } else {
        Some(exe_path.to_string())
    }
}

fn try_launch(qgis_exe: &str, profile: &str, project: Option<&str>) -> Result<(), String> {
    let mut cmd = Command::new(qgis_exe);
    cmd.arg("--profile").arg(profile);
    if let Some(p) = project {
        if !p.trim().is_empty() {
            cmd.arg(p);
        }
    }
    cmd.spawn().map(|_| ()).map_err(|e| e.to_string())
}

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::new(100, 100, 520, 260, "QGIS Launcher GUI");

    let mut pack = group::Pack::new(10, 10, 500, 220, "");
    pack.set_spacing(8);

    let mut profile_in = input::Input::new(0, 0, 480, 30, "Profile:");
    profile_in.set_trigger(CallbackTrigger::Changed);
    let mut project_in = input::Input::new(0, 0, 480, 30, "Project Path:");
    let mut settings_dir_in = input::Input::new(0, 0, 480, 30, "Settings Dir:");

    let mut status = frame::Frame::new(0, 0, 480, 30, "");

    let mut row = group::Pack::new(0, 0, 480, 40, "");
    row.set_type(group::PackType::Horizontal);
    row.set_spacing(8);
    let mut load_btn = button::Button::new(0, 0, 120, 30, "Load");
    let mut save_btn = button::Button::new(0, 0, 120, 30, "Save");
    let mut launch_btn = button::Button::new(0, 0, 120, 30, "Launch QGIS");
    row.end();

    pack.end();
    wind.end();
    wind.show();

    // initial values
    let mut settings_dir = r"C:\qgis_launcher".to_string();
    let mut settings = load_settings(&settings_dir);
    profile_in.set_value(&settings.profile);
    project_in.set_value(&settings.project_path);
    if let Some(sd) = &settings.settings_dir {
        settings_dir = sd.clone();
        settings_dir_in.set_value(sd);
    } else {
        settings_dir_in.set_value(&settings_dir);
    }

    // Load
    {
        let mut profile_in = profile_in.clone();
        let mut project_in = project_in.clone();
        let mut settings_dir_in = settings_dir_in.clone();
        let mut status = status.clone();
        load_btn.set_callback(move |_| {
            let sd = settings_dir_in.value();
            let s = load_settings(&sd);
            profile_in.set_value(&s.profile);
            project_in.set_value(&s.project_path);
            status.set_label("Loaded settings.");
        });
    }

    // Save
    {
        let mut profile_in = profile_in.clone();
        let mut project_in = project_in.clone();
        let mut settings_dir_in = settings_dir_in.clone();
        let mut status = status.clone();
        save_btn.set_callback(move |_| {
            let sd = settings_dir_in.value();
            let s = QgisSettings {
                profile: profile_in.value(),
                project_path: project_in.value(),
                reearth_url: None,
                box_url: None,
                settings_dir: Some(sd.clone()),
            };
            match save_settings(&sd, &s) {
                Ok(_) => status.set_label("Settings saved."),
                Err(e) => status.set_label(&format!("Save failed: {}", e)),
            }
        });
    }

    // Launch
    {
        let mut profile_in = profile_in.clone();
        let mut project_in = project_in.clone();
        let mut settings_dir_in = settings_dir_in.clone();
        let mut status = status.clone();
        launch_btn.set_callback(move |_| {
            // try to find QGIS exe via registry
            match find_qgis_path() {
                Some(qgis_exe) => {
                    let project_val = project_in.value();
                    let project_opt = if project_val.trim().is_empty() { None } else { Some(project_val.as_str()) };
                    match try_launch(&qgis_exe, &profile_in.value(), project_opt) {
                        Ok(_) => status.set_label("Launch requested."),
                        Err(e) => status.set_label(&format!("Launch failed: {}", e)),
                    }
                }
                None => {
                    status.set_label("QGIS exe not found in registry.");
                }
            }
        });
    }

    app.run().unwrap();
}
