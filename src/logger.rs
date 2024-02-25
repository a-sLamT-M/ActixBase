use chrono::Local;
use fern::Dispatch;

pub fn build() -> anyhow::Result<()> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} | {} | {}:{} | {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
    Ok(())
}