use std::fs;
use std::path::PathBuf;
use test_files::TestFiles;

pub fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

pub fn simple_testfile_setup(temp_dir: &mut TestFiles) -> (PathBuf, PathBuf) {
    let (file1, contents1) = ("a.txt", "this is a base file");
    let (file2, contents2) = ("b.txt", "this is a modified file");

    temp_dir.file(file1, contents1).file(file2, contents2);

    let fp1 = temp_dir.path().join(file1);
    let fp2 = temp_dir.path().join(file2);

    log::info!("file 2 start: {}", fs::read_to_string(fp2.clone()).unwrap());
    assert_ne!(
        fs::read_to_string(fp1.clone()).unwrap(),
        fs::read_to_string(fp2.clone()).unwrap()
    );

    (fp1, fp2)
}
