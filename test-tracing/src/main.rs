use chrono::Local;
use color_eyre::{eyre::eyre, Result};
use tracing::{error, info, instrument};
use tracing_appender::{non_blocking, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::{self, time::FormatTime, writer::MakeWriterExt},
    prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt,
    Registry,
};
use wen_layer::WenLayer;
/// https://burgers.io/custom-logging-in-rust-using-tracing
mod wen_layer;

fn custom_layer() {
    tracing_subscriber::registry().with(WenLayer).init();
    info!(a_bool = true, answer = 42, message = "first example");
}

fn main() {
    // custom_layer();
    // common().unwrap();
    // appender();
    // console_and_file();
    console_and_multiple_file();
}

struct LocalTimer;
impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut fmt::format::Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%FT%T%.3f"))
    }
}
fn console_and_file() {
    // 输出到控制台中
    let formatting_layer = fmt::layer()
        .pretty()
        .with_writer(std::io::stdout)
        .with_timer(LocalTimer);

    // 输出到文件中
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_line_number(true)
        .with_timer(LocalTimer)
        .with_writer(non_blocking_appender);
    // 注册
    Registry::default()
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        .with(ErrorLayer::default())
        .with(formatting_layer)
        .with(file_layer)
        .init();
    tracing::warn!("sleeping for a minute...");

    std::thread::sleep(std::time::Duration::from_secs(60));

    tracing::info!("okay, time to shave some more yaks!");
}
fn console_and_multiple_file() {
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stdout);
    let (debug_file, _guard) = non_blocking(rolling::daily("logs", "debug"));
    let (warn_file, _guard) = non_blocking(rolling::daily("logs", "warning"));
    let (info_file, _guard) = non_blocking(rolling::daily("logs", "info"));
    let all_files = debug_file
        .and(warn_file.with_max_level(tracing::Level::WARN).with_min_level(tracing::Level::ERROR))
        .and(info_file.with_max_level(tracing::Level::INFO).with_min_level(tracing::Level::INFO));

    let file_layer = fmt::layer().with_ansi(false).with_writer(all_files);

    Registry::default()
        .with(formatting_layer)
        .with(file_layer)
        .init();
    tracing::warn!("sleeping for a minute...");
    tracing::info!("sleeping for a minute...");
    tracing::debug!("sleeping for a minute...");
    tracing::error!("sleeping for a minute...");

    std::thread::sleep(std::time::Duration::from_secs(60));

    tracing::info!("okay, time to shave some more yaks!");
}
fn appender() {
    // Log all `tracing` events to files prefixed with `debug`. Since these
    // files will be written to very frequently, roll the log file every minute.
    let debug_file = rolling::minutely("./logs", "debug");
    // Log warnings and errors to a separate file. Since we expect these events
    // to occur less frequently, roll that file on a daily basis instead.
    let warn_file = rolling::daily("./logs", "warning").with_max_level(tracing::Level::WARN);
    let all_files = debug_file.and(warn_file);

    tracing_subscriber::fmt()
        .with_writer(all_files)
        .with_ansi(false)
        .with_max_level(tracing::Level::TRACE)
        .with_writer(std::io::stderr) // 输出到控制台中
        .init();

    tracing::info!("sleeping for a minute...");

    std::thread::sleep(std::time::Duration::from_secs(60));

    tracing::info!("okay, time to shave some more yaks!");
}

fn common() -> Result<()> {
    // 输出到控制台中
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);

    // 输出到文件中
    let file_appender = rolling::never("logs", "app.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);

    // 注册
    Registry::default()
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        .with(ErrorLayer::default())
        .with(formatting_layer)
        .with(file_layer)
        .init();

    // 安裝 color-eyre 的 panic 处理句柄
    color_eyre::install()?;

    call_return_err();

    Ok(())
}

#[instrument]
fn return_err() -> Result<()> {
    Err(eyre!("Something went wrong"))
}

#[instrument]
fn call_return_err() {
    info!("going to log error");
    if let Err(err) = return_err() {
        // 推荐大家运行下，看看这里的输出效果
        error!(?err, "error");
    }
}
