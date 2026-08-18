use std::path::PathBuf;

pub trait DirectoryAsset {
    const DIRECTORY_NAME: &'static str;
}

pub fn dir_path<T: DirectoryAsset>() -> PathBuf {
    let mut path = PathBuf::from("data");
    path.push(<T as DirectoryAsset>::DIRECTORY_NAME);

    path
}
