pub struct Application {}

impl Application {
    pub fn new() -> Self {
        winui_sys::bootstrap::initialize().unwrap();

        Self {  }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        winui_sys::bootstrap::shutdown().unwrap();
    }
}