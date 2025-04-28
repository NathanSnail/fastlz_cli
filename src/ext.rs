use bytemuck::AnyBitPattern;
use std::io::Read;

pub trait ByteReaderExt {
    fn read_le<T: AnyBitPattern>(&mut self) -> T;
}

impl<R: Read> ByteReaderExt for R {
    fn read_le<T: AnyBitPattern>(&mut self) -> T {
        let mut buffer = vec![0; size_of::<T>()];
        self.read_exact(&mut buffer)
            .expect("Buffer should always contain enough space for a T");
        *(bytemuck::try_from_bytes::<T>(&buffer).expect("Casting from bytes should never fail"))
    }
}
