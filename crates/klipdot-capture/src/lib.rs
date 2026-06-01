pub mod util;
pub mod clipboard;
pub mod config;
pub mod error;
pub mod image_processor;
pub mod image_preview;
pub mod installer;
pub mod interceptor;
pub mod service;
pub mod shell_hooks;
pub mod stdout_monitor;

pub use error::{Error, Result};
pub use util::{
    DisplayServer, detect_display_server, detect_wayland_compositor, is_command_available,
    get_available_clipboard_tools, get_available_screenshot_tools, is_image_file,
    generate_screenshot_filename, format_file_size, format_duration,
    get_app_dir, get_config_dir, get_home_dir,
};

// Re-export constants
pub use util::{
    VERSION, APP_NAME, SCREENSHOT_DIR, CONFIG_FILE, PID_FILE, LOG_FILE, HOOKS_DIR, TEMP_DIR,
    DEFAULT_POLL_INTERVAL, DEFAULT_CLEANUP_DAYS, MAX_FILE_SIZE, IMAGE_QUALITY,
    MAX_RECENT_SCREENSHOTS, SERVICE_CHECK_INTERVAL, SUPPORTED_FORMATS,
    IMAGE_COMMAND_PATTERNS, IMAGE_PROCESS_NAMES,
    WAYLAND_SCREENSHOT_TOOLS, X11_SCREENSHOT_TOOLS, MACOS_SCREENSHOT_TOOLS,
    WAYLAND_CLIPBOARD_TOOLS, X11_CLIPBOARD_TOOLS, MACOS_CLIPBOARD_TOOLS,
};
