#[cfg(not(feature = "geode"))]
use windows::Win32::System::Console::{AllocConsole, FreeConsole};

pub struct ConsoleManager {
    console_allocated: bool,
}

impl Default for ConsoleManager {
    fn default() -> Self {
        Self {
            console_allocated: false,
        }
    }
}

impl ConsoleManager {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(not(feature = "geode"))]
    pub fn maybe_alloc_console(&mut self, show_console: bool) {
        if show_console && !self.console_allocated {
            unsafe {
                let _ = AllocConsole();
            }
            self.console_allocated = true;
            log::info!("Console allocated");
        } else if !show_console && self.console_allocated {
            self.free_console();
        }
    }

    #[cfg(feature = "geode")]
    pub fn maybe_alloc_console(&mut self, _show_console: bool) {
        // Console not supported in Geode builds
    }

    #[cfg(not(feature = "geode"))]
    pub fn free_console(&mut self) {
        if self.console_allocated {
            unsafe {
                let _ = FreeConsole();
            }
            self.console_allocated = false;
            log::info!("Console freed");
        }
    }

    #[cfg(feature = "geode")]
    pub fn free_console(&mut self) {
        // Console not supported in Geode builds
    }

    pub fn is_console_allocated(&self) -> bool {
        self.console_allocated
    }

    pub fn toggle_console(&mut self) {
        #[cfg(not(feature = "geode"))]
        {
            if self.console_allocated {
                self.free_console();
            } else {
                self.maybe_alloc_console(true);
            }
        }
    }
}

impl Drop for ConsoleManager {
    fn drop(&mut self) {
        self.free_console();
    }
}
