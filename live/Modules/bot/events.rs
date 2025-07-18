use crate::clickpack::{Button, ClickType};
use std::time::Instant;

#[derive(Default, Clone)]
pub struct ClickTimes {
    pub click: f64,
    pub release: f64,
    pub hardclick: f64,
    pub hardrelease: f64,
    pub softclick: f64,
    pub softrelease: f64,
    pub microclick: f64,
    pub microrelease: f64,
    pub jump: f64,
    pub left: f64,
    pub right: f64,
}

pub struct EventHandler {
    pub prev_times: ClickTimes,
    pub is_in_level: bool,
    pub playlayer_time: f64,
    pub level_start: Instant,
    pub dead_timer: f32,
    pub dead_timer_limit: f32,
}

// Add a ClickTime struct that was referenced in bot.rs
#[derive(Default, Clone, Copy)]
pub struct ClickTime {
    pub time: f64,
    pub typ: crate::clickpack::ClickType,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self {
            prev_times: ClickTimes::default(),
            is_in_level: false,
            playlayer_time: 0.0,
            level_start: Instant::now(),
            dead_timer: f32::NAN,
            dead_timer_limit: 0.0,
        }
    }
}

impl EventHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_action(&mut self, button: Button, player2: bool, push: bool) {
        // Handle button action events
        log::debug!("Action: {:?}, Player2: {}, Push: {}", button, player2, push);
        
        // Update timing information
        let current_time = self.playlayer_time;
        
        match (button, push) {
            (Button::Left, true) => self.prev_times.click = current_time,
            (Button::Left, false) => self.prev_times.release = current_time,
            (Button::Right, true) => self.prev_times.hardclick = current_time,
            (Button::Right, false) => self.prev_times.hardrelease = current_time,
            _ => {}
        }
    }

    pub fn on_reset(&mut self) {
        log::debug!("Level reset");
        self.level_start = Instant::now();
        self.playlayer_time = 0.0;
        self.dead_timer = f32::NAN;
        self.prev_times = ClickTimes::default();
    }

    pub fn on_init(&mut self, playlayer_addr: usize) {
        log::debug!("Level init: 0x{:x}", playlayer_addr);
        self.is_in_level = true;
        self.level_start = Instant::now();
        self.playlayer_time = 0.0;
        self.dead_timer = f32::NAN;
    }

    pub fn on_exit(&mut self) {
        log::debug!("Level exit");
        self.is_in_level = false;
        self.playlayer_time = 0.0;
        self.dead_timer = f32::NAN;
    }

    pub fn on_death(&mut self) {
        log::debug!("Player death");
        self.dead_timer = 0.0;
        self.dead_timer_limit = 0.5; // 500ms death timer
    }

    pub fn on_update(&mut self, dt: f32) {
        if self.is_in_level {
            self.playlayer_time += dt as f64;
        }
        
        if !self.dead_timer.is_nan() {
            self.dead_timer += dt;
            if self.dead_timer >= self.dead_timer_limit {
                self.dead_timer = f32::NAN;
            }
        }
    }

    pub fn is_dead(&self) -> bool {
        !self.dead_timer.is_nan()
    }

    pub fn get_level_time(&self) -> f64 {
        self.playlayer_time
    }

    pub fn get_level_duration(&self) -> std::time::Duration {
        self.level_start.elapsed()
    }

    pub fn should_ignore_click(&self, click_type: ClickType) -> bool {
        // Add logic to determine if a click should be ignored
        // based on current state, timing, etc.
        false
    }

    pub fn get_click_timing(&self, button: Button, push: bool) -> f64 {
        match (button, push) {
            (Button::Left, true) => self.prev_times.click,
            (Button::Left, false) => self.prev_times.release,
            (Button::Right, true) => self.prev_times.hardclick,
            (Button::Right, false) => self.prev_times.hardrelease,
            _ => 0.0,
        }
    }

    pub fn update_timing(&mut self, button: Button, push: bool, time: f64) {
        match (button, push) {
            (Button::Left, true) => self.prev_times.click = time,
            (Button::Left, false) => self.prev_times.release = time,
            (Button::Right, true) => self.prev_times.hardclick = time,
            (Button::Right, false) => self.prev_times.hardrelease = time,
            _ => {}
        }
    }

    pub fn reset_timings(&mut self) {
        self.prev_times = ClickTimes::default();
    }

    pub fn get_death_timer(&self) -> f32 {
        self.dead_timer
    }

    pub fn is_level_active(&self) -> bool {
        self.is_in_level && self.dead_timer.is_nan()
    }
}
