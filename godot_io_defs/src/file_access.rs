use std::{
    cmp,
    io::{BufRead, Error as IoError, ErrorKind, Read, Seek, Write},
};

use godot::{
    engine::{global::Error, FileAccess},
    prelude::Gd,
};

pub struct FaWrapper {
    fa: Gd<FileAccess>,
    pos: u64,
    buffer: Vec<u8>,
}

impl FaWrapper {
    const BUFFER_SIZE: usize = 4096;

    pub fn new(fa: Gd<FileAccess>) -> Self {
        let pos = fa.get_position();
        Self {
            fa,
            pos,
            buffer: Vec::new(),
        }
    }

    /// Gets the reference to inner [FileAccess]
    pub fn get(&self) -> &Gd<FileAccess> {
        &self.fa
    }

    /// Gets the mutable reference to inner [FileAccess]. After making some operations
    /// on it, call `update_pos` to keep the wrappers position aligned with [FileAccess] one
    pub fn get_mut(&mut self) -> &mut Gd<FileAccess> {
        &mut self.fa
    }

    /// Deconstructs the wrapper and retrieves the inner [FileAccess]
    pub fn to_owned(self) -> Gd<FileAccess> {
        self.fa
    }

    /// Updates the wrapper internal position with [FileAccess] position. Call after
    /// making manual writes or reads on inner [FileAccess] through `get_mut()` reference
    pub fn update_pos(&mut self) {
        self.pos = self.fa.get_position()
    }

    fn check_error(&self) -> Result<(), IoError> {
        if self.fa.get_error() == Error::OK {
            return Ok(());
        }
        Err(IoError::new(
            ErrorKind::Other,
            format!("GodotError: {:?}", self.fa.get_error()),
        ))
    }
}

impl From<Gd<FileAccess>> for FaWrapper {
    fn from(value: Gd<FileAccess>) -> Self {
        Self::new(value)
    }
}

impl Read for FaWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let length = self.fa.get_length();

        if self.pos >= length {
            return Ok(0);
        }

        let remaining_bytes = (length - self.pos) as usize;
        let bytes_to_read = cmp::min(buf.len(), remaining_bytes);

        if bytes_to_read == 0 {
            return Ok(0);
        }
        let mut readen = 0;
        while readen < bytes_to_read {
            buf[readen] = self.fa.get_8();
            readen += 1;
            self.pos += 1;
        }
        Ok(readen)
    }
}

impl Write for FaWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let bytes_to_write = buf.len();
        let mut bytes_written = 0;

        while bytes_written < bytes_to_write {
            self.fa.store_8(buf[bytes_written]);
            bytes_written += 1;
            self.pos += 1;
            self.check_error()?;
        }
        Ok(bytes_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Seek for FaWrapper {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match pos {
            std::io::SeekFrom::Start(position) => {
                self.fa.seek(position);
                self.pos = position;
                Ok(position)
            }
            std::io::SeekFrom::End(position) => {
                if (self.fa.get_length() as i64) < position {
                    return Err(IoError::new(
                        ErrorKind::InvalidInput,
                        "Position set would be negative",
                    ));
                }
                self.fa.seek_end_ex().position(position).done();
                self.update_pos();
                Ok(self.pos)
            }
            std::io::SeekFrom::Current(position) => {
                self.update_pos();
                let new_pos = self.pos as i64 + position;
                if new_pos < 0 {
                    return Err(IoError::new(
                        ErrorKind::InvalidInput,
                        "Position set would be negative",
                    ));
                }
                let new_pos = new_pos as u64;
                self.fa.seek(new_pos);
                self.pos = new_pos;
                Ok(self.pos)
            }
        }
    }
}

impl BufRead for FaWrapper {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.buffer = vec![0; Self::BUFFER_SIZE];

        let gd_buffer = self.fa.get_buffer(Self::BUFFER_SIZE as i64);
        self.check_error()?;

        for i in 0..gd_buffer.len() {
            self.buffer[i] = gd_buffer.get(i);
        }

        Ok(&self.buffer[0..gd_buffer.len()])
    }

    fn consume(&mut self, amt: usize) {
        _ = self.seek(std::io::SeekFrom::Current(amt as i64));
    }
}
