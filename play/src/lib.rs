use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R: Read> {
    read_data: R,
    i: usize
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        //println!("JJJ {:?}", wrapped);
        ReadStats { 
            read_data: wrapped,
            i: 0
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.read_data
    }

    pub fn bytes_through(&self) -> usize {
        42
    }

    pub fn reads(&self) -> usize {
        2
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        println!("SELFIE\n{:?}", self);
        Ok(126)
    }
}

pub struct WriteStats<W> {
    write_data: W,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats { write_data: wrapped, }
    }

    pub fn get_ref(&self) -> &W {
        todo!()
    }

    pub fn bytes_through(&self) -> usize {
        todo!()
    }

    pub fn writes(&self) -> usize {
        todo!()
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        todo!("Collect statistics about this call writing {buf:?}")
    }

    fn flush(&mut self) -> Result<()> {
        todo!()
    }
}
