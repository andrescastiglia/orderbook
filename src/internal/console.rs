use chrono::Utc;

///
/// The console is a singleton that is used to log messages to the console.
/// It is used by the `log` macro.
///
pub struct Console;

impl log::Log for Console {
    /// Determines if a log message with the specified metadata would be logged
    /// to the console.
    /// # Arguments
    /// * `metadata` - The metadata of the log message.
    /// # Returns
    /// `true` if the message would be logged to the console, `false` otherwise.
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    /// Logs the `Record`
    /// # Arguments
    /// * `record` - The log message to log.
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

    /// Flushes any buffered records
    fn flush(&self) {}
}
