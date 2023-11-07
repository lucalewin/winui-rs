// use windows_app::Microsoft::Windows::System::Power::PowerManager;

pub struct Application {}

impl Application {
    pub fn new() -> Self {
        winui_sys::bootstrap::initialize().unwrap();

        // let charge = PowerManager::RemainingChargePercent().unwrap();
        // println!("Remaining charge: {charge}%");

        Self {  }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        winui_sys::bootstrap::shutdown().unwrap();
    }
}