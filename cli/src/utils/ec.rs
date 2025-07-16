use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::thread;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const EC_DATA_PORT: u16 = 0x62;
const EC_COMMAND_PORT: u16 = 0x66;
const EC_READ_COMMAND: u8 = 0x80;
const EC_WRITE_COMMAND: u8 = 0x81;

const IBF_FLAG: u8 = 0x02;
const OBF_FLAG: u8 = 0x01;

pub struct EC {
    port: std::fs::File,
}

impl EC {
    pub fn new() -> Result<Self> {
        let port = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/port")?;
        Ok(EC { port })
    }

    fn wait_for_ibf_clear(&mut self) -> Result<()> {
        for _ in 0..1000 {
            self.port.seek(SeekFrom::Start(EC_COMMAND_PORT as u64))?;

            let mut status = [0u8; 1];
            self.port.read_exact(&mut status)?;

            if status[0] & IBF_FLAG == 0 {
                return Ok(());
            }
            thread::sleep(Duration::from_micros(100));
        }
        Err("Timeout waiting for IBF to clear".into())
    }

    fn wait_for_obf_set(&mut self) -> Result<()> {
        for _ in 0..1000 {
            self.port.seek(SeekFrom::Start(EC_COMMAND_PORT as u64))?;
            let mut status = [0u8; 1];
            self.port.read_exact(&mut status)?;

            if status[0] & OBF_FLAG != 0 {
                return Ok(());
            }
            thread::sleep(Duration::from_micros(100));
        }
        Err("Timeout waiting for OBF set".into())
    }

    pub fn read(&mut self, register: u8) -> Result<u8> {
        self.wait_for_ibf_clear()?;

        self.port.seek(SeekFrom::Start(EC_COMMAND_PORT as u64))?;
        self.port.write_all(&[EC_READ_COMMAND])?;

        self.wait_for_ibf_clear()?;

        self.port.seek(SeekFrom::Start(EC_DATA_PORT as u64))?;
        self.port.write_all(&[register])?;

        self.wait_for_obf_set()?;

        let mut value = [0u8; 1];
        self.port.seek(SeekFrom::Start(EC_DATA_PORT as u64))?;
        self.port.read_exact(&mut value)?;

        Ok(value[0])
    }

    pub fn write(&mut self, register: u8, value: u8) -> Result<()> {
        self.wait_for_ibf_clear()?;

        self.port.seek(SeekFrom::Start(EC_COMMAND_PORT as u64))?;
        self.port.write_all(&[EC_WRITE_COMMAND])?;

        self.wait_for_ibf_clear()?;

        self.port.seek(SeekFrom::Start(EC_DATA_PORT as u64))?;
        self.port.write_all(&[register])?;

        self.wait_for_ibf_clear()?;

        self.port.seek(SeekFrom::Start(EC_DATA_PORT as u64))?;
        self.port.write_all(&[value])?;

        self.wait_for_ibf_clear()?;

        Ok(())
    }
}
