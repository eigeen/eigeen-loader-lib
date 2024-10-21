use std::{
    collections::HashMap,
    path::Path,
    sync::{LazyLock, Mutex},
};

use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::utility;

static CACHE: LazyLock<Mutex<HashMap<String, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static mut ADDRESS_FILE: Option<AddressFile> = None;

pub struct AddressRepository;

impl AddressRepository {
    /// Load address file.
    pub fn initialize<P: AsRef<Path>>(address_file_path: P) -> Result<()> {
        let file = std::fs::File::open(address_file_path)?;
        let reader = std::io::BufReader::new(file);

        let address_file: AddressFile = serde_json::from_reader(reader)?;

        unsafe {
            ADDRESS_FILE = Some(address_file);
        }

        Ok(())
    }

    /// Get address by address file name.
    pub fn get_address(name: &str) -> Result<usize> {
        if let Some(cached) = CACHE.lock().unwrap().get(name) {
            return Ok(*cached);
        }

        let Some(record) = Self::lookup_record(name) else {
            return Err(Error::PatternUnmanaged(name.to_string()));
        };

        let Some(addr_without_offset) = Self::pattern_scan(&record.pattern) else {
            return Err(Error::PatternMismatch(name.to_string()));
        };

        // add offset
        let addr = (addr_without_offset as isize + record.offset) as usize;
        CACHE.lock().unwrap().insert(name.to_string(), addr);

        debug!("{} found at 0x{:x}", name, addr);

        Ok(addr)
    }

    /// Get pointer by address file name.
    pub fn get_ptr<T>(name: &str) -> Result<*mut T> {
        Self::get_address(name).map(|addr| addr as *mut T)
    }

    /// 从已加载的地址文件中获取特征码
    fn lookup_record(name: &str) -> Option<&AddressRecord> {
        unsafe {
            if let Some(address_file) = ADDRESS_FILE.as_ref() {
                if let Some(record) = address_file.records.get(name) {
                    return Some(record);
                }
            }
        }

        None
    }

    /// Pattern scan.
    fn pattern_scan(pattern: &str) -> Option<usize> {
        match utility::memory::auto_scan_first(pattern) {
            Ok(addr) => Some(addr),
            Err(e) => {
                error!("Pattern scan failed: {}", e);
                None
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AddressFile {
    records: HashMap<String, AddressRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AddressRecord {
    pattern: String,
    offset: isize,
}
