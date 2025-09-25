use log::Level;

pub mod logger {
    pub fn init_log() {
        simple_logger::init_with_level(Level::Trace).expect("logger init failed");
    }
}