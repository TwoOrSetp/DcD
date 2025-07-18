use crate::clickpack::{LoadClickpackFor, Pitch, Timings, VolumeSettings};
use egui::{Key, KeyboardShortcut, Modifiers};
use egui_keybind::Shortcut;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

const UI_SCALE_RANGE: RangeInclusive<f32> = 0.3..=5.0;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Shortcuts {
    pub toggle_menu: Shortcut,
    pub toggle_bot: Shortcut,
    pub toggle_noise: Shortcut,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            toggle_menu: Shortcut::new(
                Some(KeyboardShortcut::new(Modifiers::NONE, Key::Num1)),
                None,
            ),
            toggle_bot: Shortcut::new(
                Some(KeyboardShortcut::new(Modifiers::NONE, Key::Num2)),
                None,
            ),
            toggle_noise: Shortcut::NONE,
        }
    }
}

const fn true_value() -> bool {
    true
}

const fn default_buffer_size() -> u32 {
    512
}

#[inline]
fn float_one<Num: egui::emath::Numeric>() -> Num {
    Num::from_f64(1.0)
}

// clickpack, options, audio
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Copy)]
pub enum Stage {
    #[default]
    Clickpack,
    Audio,
    Options,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct IgnoredClickTypes {
    pub hardclicks: bool,
    pub hardreleases: bool,
    pub softclicks: bool,
    pub softreleases: bool,
    pub clicks: bool,
    pub releases: bool,
    pub microclicks: bool,
    pub microreleases: bool,
}

impl IgnoredClickTypes {
    #[inline]
    pub const fn is_ignored(&self, typ: crate::clickpack::ClickType) -> bool {
        use crate::clickpack::ClickType;
        match typ {
            ClickType::HardClick => self.hardclicks,
            ClickType::HardRelease => self.hardreleases,
            ClickType::SoftClick => self.softclicks,
            ClickType::SoftRelease => self.softreleases,
            ClickType::Click => self.clicks,
            ClickType::Release => self.releases,
            ClickType::MicroClick => self.microclicks,
            ClickType::MicroRelease => self.microreleases,
            ClickType::None => true,
        }
    }

    #[inline]
    pub const fn any_ignored(&self) -> bool {
        self.hardclicks
            || self.hardreleases
            || self.softclicks
            || self.softreleases
            || self.clicks
            || self.releases
            || self.microclicks
            || self.microreleases
    }
}

const fn death_release_delay_default() -> f64 {
    0.13
}

const fn death_release_delay_offset_default() -> f64 {
    0.13
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
pub enum ToastVisibility {
    #[default]
    AlwaysVisible,
    VisibleWhenOpen,
    NeverVisible,
}

impl ToastVisibility {
    #[inline]
    pub const fn text(self) -> &'static str {
        match self {
            ToastVisibility::AlwaysVisible => "Always Visible",
            ToastVisibility::VisibleWhenOpen => "Visible in Menu",
            ToastVisibility::NeverVisible => "Never Visible",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub pitch_enabled: bool,
    pub pitch: Pitch,
    pub timings: Timings,
    pub volume_settings: VolumeSettings,
    #[serde(default = "Shortcuts::default")]
    pub shortcuts: Shortcuts,
    #[serde(default = "true_value")]
    pub enabled: bool,
    #[serde(default = "bool::default")]
    pub hidden: bool,
    #[serde(default = "default_buffer_size")]
    pub buffer_size: u32,
    #[serde(default = "bool::default")]
    pub play_noise: bool,
    #[serde(default = "float_one")]
    pub noise_volume: f64,
    #[serde(default = "bool::default")]
    pub force_playing_platformer: bool,
    #[serde(default = "bool::default")]
    pub use_alternate_hook: bool,
    #[serde(default = "bool::default")]
    pub show_console: bool,
    #[serde(default = "Stage::default")]
    pub stage: Stage,
    #[serde(default = "bool::default")]
    pub use_fmod: bool,
    #[serde(default = "bool::default")]
    pub cut_sounds: bool,
    #[serde(default = "bool::default")]
    pub cut_by_releases: bool,
    #[serde(default = "float_one")]
    pub click_speedhack: f64,
    #[serde(default = "float_one")]
    pub noise_speedhack: f64,
    #[serde(default = "LoadClickpackFor::default")]
    pub load_clickpack_for: LoadClickpackFor,
    #[serde(default = "bool::default")]
    pub decouple_platformer: bool,
    #[serde(default = "true_value")]
    pub autosave_config: bool,
    #[serde(default = "true_value")]
    pub release_buttons_on_death: bool,
    #[serde(default = "death_release_delay_default")]
    pub death_release_delay: f64,
    #[serde(default = "death_release_delay_offset_default")]
    pub death_release_delay_offset: f64,
    #[serde(default = "IgnoredClickTypes::default")]
    pub ignored_click_types: IgnoredClickTypes,
    #[serde(default = "float_one")]
    pub ui_scale: f32,
    #[serde(default = "ToastVisibility::default")]
    pub toast_visibility: ToastVisibility,
    #[serde(default = "bool::default")]
    pub show_clickpack_db: bool,
    #[serde(default = "bool::default")]
    pub show_first_launch_dialog: bool,
    #[serde(default = "bool::default")]
    pub show_buffer_size_warning: bool,
    #[serde(default = "bool::default")]
    pub show_fmod_buffersize_warn: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_warning: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_error: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_success: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_loading: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_loaded: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_failed: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_empty: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_offline: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_online: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_update: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_updated: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_update_failed: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_update_success: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_update_loading: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_update_loaded: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_update_empty: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_update_offline: bool,
    #[serde(default = "bool::default")]
    pub show_clickpack_db_update_online: bool,
    #[serde(default = "bool::default")]
    pub death_release_delay_neg: bool,
    #[serde(default = "bool::default")]
    pub force_player2_sounds: bool,
    #[serde(default = "bool::default")]
    pub play_noise_when_disabled: bool,
    #[serde(default = "bool::default")]
    pub use_ingame_time: bool,
}

impl Config {
    #[inline]
    pub fn fixup(mut self) -> Self {
        self.buffer_size = self.buffer_size.max(1);
        #[cfg(feature = "geode")]
        {
            self.show_console = false;
        }
        self.ui_scale = self
            .ui_scale
            .clamp(*UI_SCALE_RANGE.start(), *UI_SCALE_RANGE.end());
        self
    }

    pub fn load() -> anyhow::Result<Self> {
        let mut path = std::path::PathBuf::from(".dcd/");
        log::debug!("creating directory {path:?}");
        std::fs::create_dir_all(&path)?;
        path.push("config.json");

        // try to read config
        log::debug!("trying to read config at {path:?}");
        if let Ok(f) = std::fs::File::open(&path) {
            let config = serde_json::from_reader(f)
                .map_err(|e| log::error!("failed to deserialize config at {path:?}: {e}"));
            if let Ok(config) = config {
                log::debug!("successfully read config at {path:?}");
                return Ok(config);
            }
        }

        // failed to read config, write default config
        let config = Self::default();
        log::debug!("creating file {path:?}");
        let f = std::fs::File::create(&path)?;
        log::debug!("writing default config to {path:?}");
        serde_json::to_writer_pretty(f, &config)?;
        Ok(config)
    }

    pub fn save(&self) {
        let Ok(f) = std::fs::File::create(".dcd/config.json") else {
            log::error!("failed to create config.json!");
            return;
        };
        let _ = serde_json::to_writer_pretty(f, self)
            .map_err(|e| log::error!("failed to write config: {e}"))
            .map(|_| log::debug!("successfully saved config to \".dcd/config.json\""));
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pitch_enabled: true,
            pitch: Pitch::default(),
            timings: Timings::default(),
            volume_settings: VolumeSettings::default(),
            shortcuts: Shortcuts::default(),
            enabled: true,
            hidden: false,
            buffer_size: default_buffer_size(),
            play_noise: false,
            noise_volume: 1.0,
            force_playing_platformer: false,
            use_alternate_hook: false,
            show_console: false,
            stage: Stage::default(),
            use_fmod: false,
            cut_sounds: false,
            cut_by_releases: false,
            click_speedhack: 1.0,
            noise_speedhack: 1.0,
            load_clickpack_for: LoadClickpackFor::All,
            decouple_platformer: false,
            autosave_config: true,
            release_buttons_on_death: true,
            death_release_delay: death_release_delay_default(),
            death_release_delay_offset: death_release_delay_offset_default(),
            ignored_click_types: IgnoredClickTypes::default(),
            ui_scale: 1.0,
            toast_visibility: ToastVisibility::default(),
            show_clickpack_db: false,
            show_first_launch_dialog: false,
            show_buffer_size_warning: false,
            show_fmod_buffersize_warn: false,
            show_clickpack_db_warning: false,
            show_clickpack_db_error: false,
            show_clickpack_db_success: false,
            show_clickpack_db_loading: false,
            show_clickpack_db_loaded: false,
            show_clickpack_db_failed: false,
            show_clickpack_db_empty: false,
            show_clickpack_db_offline: false,
            show_clickpack_db_online: false,
            show_clickpack_db_update: false,
            show_clickpack_db_updated: false,
            show_clickpack_db_update_failed: false,
            show_clickpack_db_update_success: false,
            show_clickpack_db_update_loading: false,
            show_clickpack_db_update_loaded: false,
            show_clickpack_db_update_empty: false,
            show_clickpack_db_update_offline: false,
            show_clickpack_db_update_online: false,
            death_release_delay_neg: false,
            force_player2_sounds: false,
            play_noise_when_disabled: false,
            use_ingame_time: false,
        }
    }
}
