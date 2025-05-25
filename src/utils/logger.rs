use anyhow::Result;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init() -> Result<()> {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {} - {}",
                record.level(),
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    Ok(())
}
