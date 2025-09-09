use std::io;
use std::mem::size_of;

pub(crate) fn read_num<T>(data: &mut &[u8]) -> io::Result<T>
where
    T: Copy + Default,
    T: FromLeBytes,
{
    if data.len() < size_of::<T>() {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "not enough bytes",
        ));
    }
    let (bytes, rest) = data.split_at(size_of::<T>());
    *data = rest;
    Ok(T::from_le_bytes(bytes)?)
}

pub(crate) fn read_string(data: &mut &[u8]) -> io::Result<String> {
    let name_size = read_num::<u8>(data)? as usize;
    if data.len() < name_size {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "not enough bytes",
        ));
    }
    let (bytes, rest) = data.split_at(name_size);
    *data = rest;
    Ok(String::from_le_bytes(bytes)?)
}

pub(crate) trait FromLeBytes: Sized {
    fn from_le_bytes(bytes: &[u8]) -> io::Result<Self>;
}

impl FromLeBytes for u8 {
    fn from_le_bytes(bytes: &[u8]) -> io::Result<Self> {
        let res = Self::from_le_bytes(
            bytes
                .try_into()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
        );
        Ok(res)
    }
}

impl FromLeBytes for u16 {
    fn from_le_bytes(bytes: &[u8]) -> io::Result<Self> {
        let res = Self::from_le_bytes(
            bytes
                .try_into()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
        );
        Ok(res)
    }
}

impl FromLeBytes for u32 {
    fn from_le_bytes(bytes: &[u8]) -> io::Result<Self> {
        let res = Self::from_le_bytes(
            bytes
                .try_into()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
        );
        Ok(res)
    }
}

impl FromLeBytes for u64 {
    fn from_le_bytes(bytes: &[u8]) -> io::Result<Self> {
        let res = Self::from_le_bytes(
            bytes
                .try_into()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
        );
        Ok(res)
    }
}

impl FromLeBytes for String {
    fn from_le_bytes(bytes: &[u8]) -> io::Result<Self> {
        let name = String::from_utf8(bytes.into())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(name)
    }
}
