use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    wrap: R,
    bytes_read: usize,
    read_calls: usize
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrap: wrapped,
            bytes_read: 0,
            read_calls: 0
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrap
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.read_calls
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = std::cmp::min(self.wrap.read(buf)?, buf.len());
        self.read_calls += 1;
        self.bytes_read += bytes_read;
        Ok(bytes_read)
    }
}

pub struct WriteStats<W> {
    wrap: W,
    bytes_written: usize,
    write_calls: usize
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrap: wrapped,
            bytes_written: 0,
            write_calls: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrap
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    pub fn writes(&self) -> usize {
        self.write_calls
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = std::cmp::min(self.wrap.write(buf)?, buf.len());
        self.write_calls += 1;
        self.bytes_written += bytes_written;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrap.flush()
    }
}
