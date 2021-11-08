use chrono::Utc;

pub struct Console;

impl log::Log for Console {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        match record.level() {
            log::Level::Error => {
                eprintln!(
                    "{} {}: {}",
                    Utc::now().to_rfc2822(),
                    record.level(),
                    record.args()
                );
            }
            _ => println!(
                "{} {}: {}",
                Utc::now().to_rfc2822(),
                record.level(),
                record.args()
            ),
        }
    }

    fn flush(&self) {}
}
