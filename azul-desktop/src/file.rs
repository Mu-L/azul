use std::fs;
use azul_css::{U8Vec, AzString};
use std::io::{Read, Write};
use alloc::sync::Arc;
use std::sync::Mutex;

#[repr(C)]
#[derive(Clone)]
pub struct File {
    ptr: Arc<Mutex<fs::File>>,
}

impl_option!(File, OptionFile, copy = false, [Clone]);

impl File {
    fn new(f: fs::File) -> Self { Self { ptr: Arc::new(Mutex::new(f)) } }
    pub fn open(path: &str) -> Option<Self> {
        Some(Self::new(fs::File::open(path).ok()?))
    }
    pub fn create(path: &str) -> Option<Self> {
        Some(Self::new(fs::File::create(path).ok()?))
    }
    pub fn read_to_string(&mut self) -> Option<AzString> {
        let mut contents = String::new();
        Arc::get_mut(&mut &mut self.ptr)?.get_mut().ok()?.read_to_string(&mut contents).ok()?;
        Some(contents.into())
    }
    pub fn read_to_bytes(&mut self) -> Option<U8Vec> {
        let mut contents = Vec::new();
        Arc::get_mut(&mut self.ptr)?.get_mut().ok()?.read(&mut contents).ok()?;
        Some(contents.into())
    }
    pub fn write_string(&mut self, string: &str) -> Option<()> {
        self.write_bytes(string.as_bytes())
    }
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Option<()> {
        let mut lock = Arc::get_mut(&mut self.ptr)?.get_mut().ok()?;
        lock.write_all(bytes).ok()?;
        lock.sync_all().ok()?;
        Some(())
    }
    pub fn close(self) { }
}