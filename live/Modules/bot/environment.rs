use crate::clickpack::LoadClickpackFor;
use kittyaudio::Device;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn skip_serializing_selected_device(device: &str) -> bool {
    let is_default = if let Ok(name) = Device::Default.name() {
        name == device
    } else {
        false
    };
    device.is_empty() || is_default
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub enum ClickpackEnv {
    #[default]
    None,
    Name(String),
    Path(PathBuf),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Env {
    version: String,
    pub clickpack_ord: Vec<(ClickpackEnv, LoadClickpackFor)>,
    is_first_launch: bool,
    #[serde(
        default = "String::new",
        skip_serializing_if = "skip_serializing_selected_device"
    )]
    pub selected_device: String,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            version: "0.1.42".to_string(), // Use hardcoded version for now
            clickpack_ord: vec![(ClickpackEnv::None, LoadClickpackFor::All)],
            is_first_launch: true, // overriden later
            selected_device: String::new(),
        }
    }
}

impl Env {
    pub fn load() -> Self {
        let _ = std::fs::create_dir_all(".dcd")
            .map_err(|e| log::error!("failed to create .dcd directory: {e}"));

        let path = std::path::Path::new(".dcd/env.json");
        if let Ok(f) = std::fs::File::open(path) {
            let env = serde_json::from_reader(f);
            if let Ok(env) = env {
                return env;
            } else if let Err(e) = env {
                log::error!("failed to deserialize env: {e}");
            }
        }

        // if we're here we failed
        log::warn!("failed to deserialize env, writing defaults");
        if let Ok(f) = std::fs::File::create(path) {
            let _ = serde_json::to_writer_pretty(f, &Self::default())
                .map_err(|e| log::error!("failed to write env: {e}"));
        }

        Self::default()
    }

    pub fn save(&self) {
        log::info!("writing .dcd/env.json");
        let mut env = self.clone();
        env.version = "0.1.42".to_string(); // Use hardcoded version for now
        if let Ok(f) = std::fs::File::create(".dcd/env.json") {
            let _ = serde_json::to_writer_pretty(f, &env)
                .map_err(|e| log::error!("failed to write env: {e}"));
        }
    }

    pub fn update(&mut self, clickpack_env: ClickpackEnv, load_for: LoadClickpackFor) {
        match load_for {
            LoadClickpackFor::All => self.clickpack_ord = vec![(clickpack_env, load_for)],
            _ => {
                self.clickpack_ord.retain(|ord| ord.1 != load_for);
                log::info!("pushing to ord: ({clickpack_env:?}, {load_for:?})");
                self.clickpack_ord.push((clickpack_env, load_for));
            }
        }
        self.save();
    }

    pub fn get_clickpack_env(&self) -> &(ClickpackEnv, LoadClickpackFor) {
        self.clickpack_ord.first().unwrap_or(&(ClickpackEnv::None, LoadClickpackFor::All))
    }

    pub fn is_first_launch(&self) -> bool {
        self.is_first_launch
    }

    pub fn set_first_launch(&mut self, first_launch: bool) {
        self.is_first_launch = first_launch;
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

}
