use std::sync::{Arc, Mutex};

use super::error::Error;
use super::file::File;
pub enum Action {
    TermSave,
    Load,
}
mod inner {
    use signal_hook::consts::{SIGHUP, TERM_SIGNALS};
    use std::sync::{atomic::AtomicBool, Arc};
    #[derive(Clone)]
    pub struct Saver {
        flag: Arc<AtomicBool>,
    }
    impl Saver {
        pub fn new() -> Self {
            let term = Arc::new(AtomicBool::new(false));
            for sig in TERM_SIGNALS {
                signal_hook::flag::register(*sig, Arc::clone(&term)).unwrap();
            }
            Self { flag: term }
        }
        pub fn swap(&self, flag: bool) -> bool {
            self.flag.swap(flag, std::sync::atomic::Ordering::Relaxed)
        }
    }
    #[derive(Clone)]
    pub struct Loader {
        flag: Arc<AtomicBool>,
    }
    impl Loader {
        pub fn new() -> Self {
            let hup = Arc::new(AtomicBool::new(false));
            signal_hook::flag::register(SIGHUP, Arc::clone(&hup)).unwrap();
            Self { flag: hup }
        }
        pub fn swap(&self, flag: bool) -> bool {
            self.flag.swap(flag, std::sync::atomic::Ordering::Relaxed)
        }
    }
}
#[derive(Clone)]
pub struct Saver<T> {
    inner: inner::Saver,
    file: Arc<Mutex<File<T>>>,
}
impl<T> Saver<T> {
    pub fn new(file: Arc<Mutex<File<T>>>) -> Self {
        Self {
            inner: inner::Saver::new(),
            file,
        }
    }
}
impl<T> Saver<T>
where
    T: serde::Serialize,
{
    pub fn run(&self) -> Result<Action, Error> {
        loop {
            if self.inner.swap(false) {
                self.file.lock().unwrap().save()?;
                return Ok(Action::TermSave);
            }
        }
    }
    pub fn run_with<F>(&self, f: F) -> Result<Action, Error>
    where
        F: FnOnce() -> (),
    {
        loop {
            if self.inner.swap(false) {
                f();
                self.file.lock().unwrap().save()?;
                return Ok(Action::TermSave);
            }
        }
    }
}
#[derive(Clone)]
pub struct Loader<T> {
    inner: inner::Loader,
    file: Arc<Mutex<File<T>>>,
}
impl<T: Default> Loader<T> {
    pub fn new(file: Arc<Mutex<File<T>>>) -> Self {
        Self {
            inner: inner::Loader::new(),
            file,
        }
    }
}
impl<T: serde::de::DeserializeOwned> Loader<T> {
    pub fn run(&mut self) -> Result<Action, Error> {
        loop {
            if self.inner.swap(false) {
                self.file.lock().unwrap().load()?;
                return Ok(Action::Load);
            }
        }
    }
}

pub struct Keeper<T> {
    saver: inner::Saver,
    loader: inner::Loader,
    file: Arc<Mutex<File<T>>>,
}
impl<T: Default> Keeper<T> {
    pub fn new(file: Arc<Mutex<File<T>>>) -> Self {
        Self {
            saver: inner::Saver::new(),
            loader: inner::Loader::new(),
            file,
        }
    }
}
impl<T: serde::Serialize + serde::de::DeserializeOwned> Keeper<T> {
    pub fn run(&mut self) -> Result<Action, Error> {
        loop {
            if self.saver.swap(false) {
                self.file.lock().unwrap().save()?;
                return Ok(Action::TermSave);
            }
            if self.loader.swap(false) {
                self.file.lock().unwrap().load()?;
                return Ok(Action::Load);
            }
        }
    }
}
