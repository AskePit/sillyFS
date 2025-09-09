use super::serialization::{read_num, read_string};
use std::io;
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
    pub fn load(file_name: &Path) -> io::Result<Header> {
        let data = std::fs::read(file_name)?;
        let mut data_slice = data.as_slice();

        let fs_version = read_num::<Version>(&mut data_slice)?;
        let cluster_size = read_num::<Size>(&mut data_slice)?;

        let dirs_count = read_num::<Count>(&mut data_slice)? as usize;
        let mut dirs = Vec::with_capacity(dirs_count);
        for i in 0..dirs_count {
            let id = read_num::<Id>(&mut data_slice)?;
            let name = read_string(&mut data_slice)?;

            let subdirs_count = read_num::<Count>(&mut data_slice)? as usize;
            let mut subdirs = Vec::with_capacity(subdirs_count);
            for i in 0..subdirs_count {
                let subdir_id = read_num::<Id>(&mut data_slice)?;
                subdirs.push(subdir_id);
            }

            let files_count = read_num::<Count>(&mut data_slice)? as usize;
            let mut files = Vec::with_capacity(files_count);
            for i in 0..files_count {
                let file_id = read_num::<Id>(&mut data_slice)?;
                files.push(file_id);
            }

            dirs.push(Dir {
                id,
                name,
                subdirs,
                files,
            });
        }

        let files_count = read_num::<Count>(&mut data_slice)? as usize;
        let mut files = Vec::with_capacity(files_count);
        for i in 0..files_count {
            let id = read_num::<Id>(&mut data_slice)?;
            let name = read_string(&mut data_slice)?;
            let size = read_num::<Size>(&mut data_slice)?;

            let clusters_count = read_num::<Count>(&mut data_slice)? as usize;
            let mut clusters = Vec::with_capacity(clusters_count);
            for i in 0..clusters_count {
                let cluster = read_num::<Id>(&mut data_slice)?;
                let offset = read_num::<Size>(&mut data_slice)?;
                let chunk_size = read_num::<Size>(&mut data_slice)?;

                clusters.push(ClusterLoc {
                    cluster,
                    offset,
                    chunk_size,
                });
            }

            files.push(File {
                id,
                name,
                size,
                clusters,
            });
        }

        Ok(Header {
            fs_version,
            cluster_size,
            dirs,
            files,
        })
    }

    pub fn save(header: &Header, file_name: &Path) {
        unimplemented!();
    }
}
