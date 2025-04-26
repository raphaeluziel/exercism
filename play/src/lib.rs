use std::io::{self, Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    wrap: R,
    readed: usize,
    i: usize
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrap: wrapped,
            readed: 0,
            i: 0
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrap
    }

    pub fn bytes_through(&self) -> usize {
        self.readed
    }

    pub fn reads(&self) -> usize {
        self.i
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        println!("BUF = {:?}", buf);
        self.i += 1;
        if self.i > 1 { return Ok(0); }
        let mut gg = Vec::new();
        self.readed = self.wrap.read_to_end(&mut gg)?;
        for g in 0..gg.len() {
            buf[g] = gg[g];
        }
        
        Ok(self.readed)
    }
}

pub struct WriteStats<W> {
    wrap: W,
    i: usize
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats { wrap: wrapped, i: 0 }
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
