use env_logger::fmt::style::{
  AnsiColor::{Blue, Green, Magenta, Red, Yellow},
  Color,
};
use log::Level;
use std::io::Write;

pub fn configure_logger() {
  env_logger::builder()
    .parse_default_env()
    .format(|buf, record| {
      let trace_style = buf
        .default_level_style(Level::Trace)
        .fg_color(Some(Color::Ansi(Magenta)));
      let debug_style = buf
        .default_level_style(Level::Debug)
        .fg_color(Some(Color::Ansi(Blue)));
      let info_style = buf
        .default_level_style(Level::Info)
        .fg_color(Some(Color::Ansi(Green)));
      let warn_style = buf
        .default_level_style(Level::Warn)
        .fg_color(Some(Color::Ansi(Yellow)));
      let error_style = buf
        .default_level_style(Level::Error)
        .fg_color(Some(Color::Ansi(Red)));

      let level_color = match record.level() {
        Level::Trace => trace_style,
        Level::Debug => debug_style,
        Level::Info => info_style,
        Level::Warn => warn_style,
        Level::Error => error_style,
      };

      writeln!(
        buf,
        "{:<17}:{} {level_color}{:<5}{level_color:#} > {}",
        record.file().unwrap_or("unknown"),
        record.line().unwrap_or(0),
        record.level(),
        record.args()
      )
    })
    .init();
}
