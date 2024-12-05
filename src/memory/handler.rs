use crate::memory::enums::{BufType, BufValue};
use core::panic;
use process_memory::{DataMember, Memory, Pid as WPid, TryIntoProcessHandle};
use read_process_memory::{CopyAddress, ProcessHandle as RProcessHandle};
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::usize;
use sysinfo::{Pid, System};

pub struct MemoryHandler {
    pub pid: Option<Pid>,
    pub base_addr: Option<u64>,
}

impl MemoryHandler {
    pub fn new() -> Self {
        MemoryHandler {
            pid: None,
            base_addr: None,
        }
    }

    pub fn init(&mut self, process_name: &str, perms: &str) -> bool {
        if let Some(pid) = self.get_pid(process_name) {
            let base_addr = match self.fetch_base_address(pid, perms) {
                Ok(Some(addr)) => addr,
                Ok(None) => {
                    println!("Couldn't fetch base address for PID: {}", pid);
                    return false;
                }
                Err(e) => {
                    panic!("Error while fetching base address: {}", e);
                }
            };

            self.base_addr = Some(base_addr);
            self.pid = Some(pid);
            true
        } else {
            println!("No process named \"{}\" was found.", process_name);
            false
        }
    }

    fn get_pid(&self, name: &str) -> Option<Pid> {
        let mut system = System::new_all();
        system.refresh_all();

        for (pid, proc) in system.processes() {
            if proc.name().eq_ignore_ascii_case(name) {
                return Some(*pid);
            }
        }
        None
    }

    fn fetch_base_address(&self, pid: Pid, perms: &str) -> io::Result<Option<u64>> {
        let path = format!("/proc/{}/maps", pid);
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.contains(perms) {
                if let Some(base) = line.split_whitespace().next() {
                    let addr = u64::from_str_radix(&base.split('-').next().unwrap(), 16).unwrap();
                    return Ok(Some(addr));
                }
            }
        }
        Ok(None)
    }

    pub fn read_addr(&self, pid: Pid, addr: u64, typo: &BufType) -> Result<BufValue, Vec<u8>> {
        let handle: RProcessHandle = (pid.as_u32() as i32).try_into().unwrap();
        let bytes = typo.bytes();
        let mut buf = vec![0u8; bytes];

        match handle.copy_address(addr as usize, &mut buf) {
            Ok(_) => match typo {
                BufType::U8 => Ok(BufValue::U8(buf[0])),
                BufType::U16 => Ok(BufValue::U16(u16::from_le_bytes([buf[0], buf[1]]))),
                BufType::U32 => Ok(BufValue::U32(u32::from_le_bytes([
                    buf[0], buf[1], buf[2], buf[3],
                ]))),
                BufType::U64 => {
                    if let Ok(bytes) = buf.as_slice().try_into() {
                        Ok(BufValue::U64(u64::from_le_bytes(bytes)))
                    } else {
                        Err(buf)
                    }
                }
                BufType::I8 => Ok(BufValue::I8(buf[0] as i8)),
                BufType::I16 => Ok(BufValue::I16(i16::from_le_bytes([buf[0], buf[1]]))),
                BufType::I32 => Ok(BufValue::I32(i32::from_le_bytes([
                    buf[0], buf[1], buf[2], buf[3],
                ]))),
                BufType::I64 => {
                    if let Ok(bytes) = buf.as_slice().try_into() {
                        Ok(BufValue::I64(i64::from_le_bytes(bytes)))
                    } else {
                        Err(buf)
                    }
                }
                BufType::Float => {
                    if let Ok(bytes) = buf.as_slice().try_into() {
                        Ok(BufValue::Float(f32::from_le_bytes(bytes)))
                    } else {
                        Err(buf)
                    }
                }
                BufType::Double => {
                    if let Ok(bytes) = buf.as_slice().try_into() {
                        Ok(BufValue::Double(f64::from_le_bytes(bytes)))
                    } else {
                        Err(buf)
                    }
                }
            },
            Err(e) => panic!("Failed reading memory: {:?}", e),
        }
    }

    pub fn write_addr(&self, pid: Pid, addr: u64, value: BufValue) {
        let handle = (pid.as_u32() as WPid).try_into_process_handle().unwrap();
        let member: DataMember<u64> = DataMember::new_offset(handle, vec![addr as usize]);

        match member.write(&value.to_u64().unwrap()) {
            Ok(_) => return,
            Err(e) => panic!("Failed writing memory: {:?}", e),
        }
    }

    pub fn resolve_ptr_chain(&self, pid: Pid, base: u64, offsets: &[u64], typo: &BufType) -> u64 {
        let mut addr = base;

        for (i, offset) in offsets.iter().enumerate() {
            addr = match self.read_addr(pid, addr + offset, typo) {
                Ok(val) => match val {
                    BufValue::U64(v) => v,
                    BufValue::I64(v) => v as u64,
                    _ => {
                        println!("Unexpected type for pointer resolution");
                        return 0;
                    }
                },
                Err(_) => {
                    println!(
                        "Failed to read memory at step {} addr {:x}",
                        i,
                        addr + offset
                    );
                    return 0;
                }
            };
        }

        addr
    }
}
