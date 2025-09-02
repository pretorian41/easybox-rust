use gelf::Logger as GelfLogger;
use gelf::UdpBackend;
use log::{LevelFilter, Log, Metadata, Record, Level};
use simple_logger::SimpleLogger;
use multi_log::MultiLogger;

struct GraylogAdapter {
    inner: GelfLogger,
    level: LevelFilter,
}

impl Log for GraylogAdapter {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let _ = self.inner.log(
                record
            );
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(console_level: LevelFilter, graylog_level: LevelFilter, graylog_addr: Option<&str>) {
    let console = SimpleLogger::new().with_level(console_level);

    let mut loggers: Vec<Box<dyn Log>> = vec![Box::new(console)];

    if let Some(addr) = graylog_addr {
        let leaked: &'static str = Box::leak(addr.to_string().into_boxed_str());
        let backend = UdpBackend::new(leaked).expect("Failed to create UDP backend");
        let gelf = GelfLogger::new(Box::new(backend)).expect("Failed to create logger");
        // let gelf = GelfLogger::new(addr, socket).expect("Failed to init Graylog logger");
        loggers.push(Box::new(GraylogAdapter { inner: gelf, level: graylog_level }));
    }

    MultiLogger::init(loggers, Level::Trace).expect("Failed to init multi logger");
}
