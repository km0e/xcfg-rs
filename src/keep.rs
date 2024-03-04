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
pub trait TFile {
    fn save(&self) -> Result<(), Error>;
    fn load(&mut self) -> Result<(), Error>;
}

pub type MSFile<T> = Arc<Mutex<File<T>>>;

impl<T> TFile for MSFile<T>
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
{
    fn save(&self) -> Result<(), Error> {
        let binding = self.lock().unwrap();
        binding.save()
    }
    fn load(&mut self) -> Result<(), Error> {
        let mut binding = self.lock().unwrap();
        binding.load()
    }
}
#[derive(Clone)]
pub struct Saver<T>
where
    T: TFile,
{
    inner: inner::Saver,
    file: T,
}
impl<T> Saver<T>
where
    T: TFile,
{
    pub fn new(file: T) -> Self {
        Self {
            inner: inner::Saver::new(),
            file,
        }
    }
}
impl<T> Saver<T>
where
    T: TFile,
{
    pub fn run(&self) -> Result<Action, Error> {
        loop {
            if self.inner.swap(false) {
                self.file.save()?;
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
                self.file.save()?;
                return Ok(Action::TermSave);
            }
        }
    }
}
#[derive(Clone)]
pub struct Loader<T>
where
    T: TFile,
{
    inner: inner::Loader,
    file: T,
}
impl<T: Default> Loader<T>
where
    T: TFile,
{
    pub fn new(file: T) -> Self {
        Self {
            inner: inner::Loader::new(),
            file,
        }
    }
}
impl<T> Loader<T>
where
    T: TFile,
{
    pub fn run(&mut self) -> Result<Action, Error> {
        loop {
            if self.inner.swap(false) {
                self.file.load()?;
                return Ok(Action::Load);
            }
        }
    }
}

pub struct Keeper<T>
where
    T: TFile,
{
    saver: inner::Saver,
    loader: inner::Loader,
    file: T,
}
impl<T> Keeper<T>
where
    T: TFile,
{
    pub fn new(file: T) -> Self {
        Self {
            saver: inner::Saver::new(),
            loader: inner::Loader::new(),
            file,
        }
    }
}
impl<T> Keeper<T>
where
    T: TFile,
{
    pub fn run(&mut self) -> Result<Action, Error> {
        loop {
            if self.saver.swap(false) {
                self.file.save()?;
                return Ok(Action::TermSave);
            }
            if self.loader.swap(false) {
                self.file.load()?;
                return Ok(Action::Load);
            }
        }
    }
}
