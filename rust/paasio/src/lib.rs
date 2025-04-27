use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrap: R,
    bytes_read: usize,
    read_calls: usize
}

impl<R: Read> ReadStats<R> {
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
        // NOTE: Took me forever to get this, and had to look at community
        // solutions to finally get.

        // self.wrap.read() is NOT calling itself! The read() I define here is
        // what I want added to the self.wrap.read() which uses the basic
        // read() supplied by std::io::read(), which my struct gets by using
        // the Read trait (use std::io::{Read, Result, Write}; at the top) 
        // and implementing it here: impl<R: Read> ReadStats<R>

        // The only REQUIRED thing that Read asks us to implement is the
        // read() function, but the only thing I'd have to return is
        // self.wrap.read(buf)?
        // This would have prevented me from getting the stats of 
        // read_calls and bytes_read
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
