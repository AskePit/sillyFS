use std::io;
use std::io::Read;
use std::path::Path;

type Version = u32;
type Size = u64;
type Count = u16;
type NameSize = u8;
type Id = Count;

pub struct FileSystem {
    header: Header,
}

struct Dir {
    id: Id,
    name: String,
    subdirs: Vec<Id>,
    files: Vec<Id>,
}

struct ClusterLoc {
    cluster: Id,
    offset: Size,
    chunk_size: Size,
}

struct File {
    id: Id,
    name: String,
    size: Size,
    clusters: Vec<ClusterLoc>,
}

pub struct Header {
    fs_version: Version,
    cluster_size: Size,
    dirs: Vec<Dir>,
    files: Vec<File>,
}

impl Header {
    pub fn load(file_name: &Path) -> Result<Header, io::Error> {
        let data = std::fs::read(file_name)?;
        let mut data_slice = data.as_slice();

        let fs_version = Header::read_u32(data_slice)?;
        data_slice = &data_slice[4..];

        let cluster_size = Header::read_u64(data_slice)?;
        data_slice = &data_slice[8..];

        let dirs_size = Header::read_u16(data_slice)? as usize;
        data_slice = &data_slice[2..];

        let dirs = Vec::new();
        for i in 0..dirs_size {
            todo!()
        }

        Ok(Header {
            fs_version,
            cluster_size,
            dirs,
            files: vec![],
        })
    }

    pub fn save(header: &Header, file_name: &Path) {
        unimplemented!();
    }

    fn read_u16(data: &[u8]) -> io::Result<u16> {
        Ok(u16::from_le_bytes(data[..2].try_into().unwrap()))
    }

    fn read_u32(data: &[u8]) -> io::Result<u32> {
        Ok(u32::from_le_bytes(data[..4].try_into().unwrap()))
    }

    fn read_u64(data: &[u8]) -> io::Result<u64> {
        Ok(u64::from_le_bytes(data[..8].try_into().unwrap()))
    }
}
