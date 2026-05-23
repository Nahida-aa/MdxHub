use std::{
    env,
    ops::AddAssign,
    panic::Location,
    pin::Pin,
    sync::OnceLock,
    task::{Context, Poll},
    time::Instant,
};

pub trait ResultExt<E> {
    type Ok;

    fn log_err(self) -> Option<Self::Ok>;
    // /// Like [`ResultExt::log_err`], but uses `{:?}` formatting so `anyhow::Error` values emit their
    // /// full backtrace. Reach for this only when a backtrace is genuinely wanted — most call sites
    // /// should stick with `log_err` / `warn_on_err`, whose output is a single chained error message.
    // fn log_err_with_backtrace(self) -> Option<Self::Ok>
    // where
    //     E: std::fmt::Debug;
    // /// Assert that this result should never be an error in development or tests.
    // fn debug_assert_ok(self, reason: &str) -> Self;
    // fn warn_on_err(self) -> Option<Self::Ok>;
    fn log_with_level(self, level: log::Level) -> Option<Self::Ok>;
    // fn anyhow(self) -> anyhow::Result<Self::Ok>
    // where
    //     E: Into<anyhow::Error>;
}

#[macro_export]
macro_rules! debug_panic {
    ( $($fmt_arg:tt)* ) => {
        if cfg!(debug_assertions) {
            panic!( $($fmt_arg)* );
        } else {
            let backtrace = std::backtrace::Backtrace::capture();
            log::error!("{}\n{:?}", format_args!($($fmt_arg)*), backtrace);
        }
    };
}

impl<T, E> ResultExt<E> for Result<T, E>
where
    E: std::fmt::Display,
{
    type Ok = T;
    #[track_caller]
    fn log_err(self) -> Option<T> {
        self.log_with_level(log::Level::Error)
    }
    #[track_caller]
    fn log_with_level(self, level: log::Level) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                log_error_with_caller(*Location::caller(), error, level);
                None
            }
        }
    }
}
fn log_error_with_caller<E>(caller: core::panic::Location<'_>, error: E, level: log::Level)
where
    E: std::fmt::Display,
{
    #[cfg(not(windows))]
    let file = caller.file();
    #[cfg(windows)]
    let file = caller.file().replace('\\', "/");
    // In this codebase all crates reside in a `crates` directory,
    // so discard the prefix up to that segment to find the crate name
    let file = file.split_once("crates/");
    let target = file.as_ref().and_then(|(_, s)| s.split_once("/src/"));

    let module_path = target.map(|(krate, module)| {
        if module.starts_with(krate) {
            module.trim_end_matches(".rs").replace('/', "::")
        } else {
            krate.to_owned() + "::" + &module.trim_end_matches(".rs").replace('/', "::")
        }
    });
    let file = file.map(|(_, file)| format!("crates/{file}"));
    log::logger().log(
        &log::Record::builder()
            .target(module_path.as_deref().unwrap_or(""))
            .module_path(file.as_deref())
            .args(format_args!("{:#}", error))
            .file(Some(caller.file()))
            .line(Some(caller.line()))
            .level(level)
            .build(),
    );
}
