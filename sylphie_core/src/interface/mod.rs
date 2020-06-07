//! Handles logging, terminal input, error reporting and related concerns.

use crate::errors::*;
use crate::module::CrateMetadata;
use parking_lot::Mutex;
use static_events::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

//mod error_report;
mod logger;
mod terminal;

pub use logger::SetupLoggerEvent;
pub use terminal::TerminalCommandEvent;

pub(crate) struct InterfaceInfo {
    pub bot_name: String,
    pub root_path: PathBuf,
    pub loaded_crates: Arc<[CrateMetadata]>,
}

struct InterfaceShared {
    info: InterfaceInfo,
    is_shutdown: AtomicBool,
}

struct InterfaceData {
    shared: Arc<InterfaceShared>,
    terminal: Arc<terminal::Terminal>,
    guard: Arc<Mutex<Option<logger::Logger>>>,
}
struct LoggerLockGuard<'a>(&'a InterfaceData);
impl <'a> Drop for LoggerLockGuard<'a> {
    fn drop(&mut self) {
        *self.0.guard.lock() = None;
    }
}

/// A handle to services related to logging, the user interface, and error reporting.
#[derive(Clone)]
pub struct Interface(Arc<InterfaceData>);
impl Interface {
    pub(crate) fn new(info: InterfaceInfo) -> Result<Interface> {
        let shared = Arc::new(InterfaceShared {
            info,
            is_shutdown: AtomicBool::new(false),
        });
        let terminal = Arc::new(terminal::Terminal::new(shared.clone())?);
        Ok(Interface(Arc::new(InterfaceData {
            shared,
            terminal,
            guard: Arc::new(Mutex::new(None)),
        })))
    }

    pub(crate) fn start(&self, target: &Handler<impl Events>) -> Result<()> {
        let _lock_guard = {
            let mut lock = self.0.guard.lock();
            let logger = logger::activate(target, self.0.shared.clone(), self.0.terminal.clone())?;
            *lock = Some(logger);
            LoggerLockGuard(&self.0)
        };
        self.0.terminal.start_terminal(target)?;
        Ok(())
    }

    pub(crate) fn shutdown(&self) {
        self.0.shared.is_shutdown.store(true, Ordering::Relaxed)
    }

    /// Reloads the logger, to reflect any configuration changes that may have occurred since.
    ///
    /// If no logger is currently active, this method will return an error.
    pub fn reload_logger(&self, target: &Handler<impl Events>) -> Result<()> {
        let mut lock = self.0.guard.lock();
        let handle = lock.as_mut().internal_err(|| "Logger is not running.")?;
        logger::reload(target, handle)
    }
}

pub(crate) fn init_early_logging() {
    logger::activate_log_compat();
    logger::activate_fallback();
}