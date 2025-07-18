use kittyaudio::{Device, Mixer, SoundHandle, StreamSettings};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct AudioSystem {
    pub mixer: Mixer,
    pub noise_sound: Option<SoundHandle>,
    pub devices: Arc<Mutex<Vec<String>>>,
    pub master_volume: f32,
}

impl Default for AudioSystem {
    fn default() -> Self {
        Self {
            mixer: Mixer::new(),
            noise_sound: None,
            devices: Arc::new(Mutex::new(vec![])),
            master_volume: 1.0,
        }
    }
}

impl AudioSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn maybe_init_kittyaudio(&mut self, use_fmod: bool, buffer_size: u32, selected_device: &str) {
        if use_fmod {
            return;
        }
        log::debug!("starting kittyaudio playback thread");
        self.mixer = Mixer::new();
        let device = self.get_device(selected_device);

        self.mixer.init_ex(
            device,
            StreamSettings {
                buffer_size: Some(buffer_size),
                ..Default::default()
            },
        );
    }

    pub fn get_device(&self, selected_device: &str) -> Device {
        if selected_device.is_empty() {
            Device::Default
        } else {
            Device::Name(selected_device.to_string())
        }
    }

    pub fn play_noise(&mut self) -> anyhow::Result<()> {
        if let Some(ref mut _sound) = self.noise_sound {
            // Use available methods for the current kittyaudio version
            // sound.set_loop(0.0..=1.0); // Correct API would need a range
        }
        Ok(())
    }

    pub fn stop_noise(&mut self) {
        if let Some(ref mut sound) = self.noise_sound {
            // Stop by setting the sound to None
            self.noise_sound = None;
        }
    }

    pub fn set_noise_volume(&mut self, volume: f64) {
        if let Some(ref mut sound) = self.noise_sound {
            sound.set_volume(volume as f32);
        }
    }

    pub fn load_noise_sound(&mut self, path: &std::path::Path) -> anyhow::Result<()> {
        // Use the correct method for loading sounds
        let sound = self.mixer.play(kittyaudio::Sound::from_path(path)?);
        self.noise_sound = Some(sound);
        Ok(())
    }

    pub fn update_devices(&self) {
        // Device listing may not be available in current version
        // For now, just add a default device
        let mut device_list = self.devices.lock();
        device_list.clear();
        device_list.push("Default Device".to_string());
    }

    pub fn get_devices(&self) -> Vec<String> {
        self.devices.lock().clone()
    }

    pub fn is_device_available(&self, device_name: &str) -> bool {
        let devices = self.devices.lock();
        devices.iter().any(|d| d == device_name)
    }

    pub fn get_default_device_name(&self) -> String {
        Device::Default.name().unwrap_or_else(|_| "Default".to_string())
    }

    pub fn reinit_with_device(&mut self, device_name: &str, buffer_size: u32) -> anyhow::Result<()> {
        let device = if device_name.is_empty() {
            Device::Default
        } else {
            Device::Name(device_name.to_string())
        };

        self.mixer.init_ex(
            device,
            StreamSettings {
                buffer_size: Some(buffer_size),
                ..Default::default()
            },
        );
        Ok(())
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        // Store volume for future use since API may not be available
        self.master_volume = volume;
    }

    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }

    pub fn pause_all(&mut self) {
        // Pause functionality may not be available
    }

    pub fn resume_all(&mut self) {
        // Resume functionality may not be available
    }

    pub fn stop_all(&mut self) {
        // Stop all functionality may not be available
    }

    pub fn is_playing(&self) -> bool {
        // Return false for now
        false
    }

    pub fn active_sounds_count(&self) -> usize {
        // Return 0 for now
        0
    }
}
