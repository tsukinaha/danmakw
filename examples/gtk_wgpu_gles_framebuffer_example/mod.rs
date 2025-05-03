// Modules
mod area;

// Re-Exports
use area::WgpuArea;

// Imports
use gtk::{glib, prelude::*};

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("GtkWgpuArea")
        .default_width(800)
        .default_height(600)
        .build();

    let wgpu_area = WgpuArea::default();
    window.set_child(Some(&wgpu_area));
    window.present();
}

/// Load GL pointers from epoxy (GL context management library used by GTK).
pub fn init_epoxy() {
    // check if ANGLE is present on macos
    #[cfg(target_os = "macos")]
    {
        let is_angle_loaded = unsafe {
            [
                libloading::os::unix::Library::new("libEGL.dylib"),
                libloading::os::unix::Library::new("libGLESv2.dylib"),
            ]
        }
        .iter()
        .fold(true, |acc, new| acc && new.is_ok());
        if !is_angle_loaded {
            panic!("The ANGLE library must be loaded for this example to work");
        }
    }

    #[cfg(target_os = "macos")]
    let library = unsafe { libloading::os::unix::Library::new("libepoxy.0.dylib") }.unwrap();
    #[cfg(all(unix, not(target_os = "macos")))]
    let library = unsafe { libloading::os::unix::Library::new("libepoxy.so.0") }.unwrap();
    #[cfg(windows)]
    let library = libloading::os::windows::Library::open_already_loaded("libepoxy-0.dll")
        .or_else(|_| libloading::os::windows::Library::open_already_loaded("epoxy-0.dll"))
        .unwrap();

    epoxy::load_with(|name| {
        unsafe { library.get(name.as_bytes()) }
            .map(|symbol| *symbol)
            .unwrap_or_else(|e| {
                eprintln!("failed to init epoxy, Err: {e:?}");
                std::ptr::null()
            })
    });
}
