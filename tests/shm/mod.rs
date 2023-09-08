mod client_server;

use super::common;
use sshync::Sshync;
use client_server::ShmClient;
use std::fs;
use test_files::TestFiles;

#[test]
fn test_shm_simple() {
    common::init();

    let mut test_files = TestFiles::new();
    let (fp1, fp2) = common::simple_testfile_setup(&mut test_files);

    let client = Box::new(ShmClient::init());

    let mut snc = Sshync::init(client, None);
    snc.sync(fp1.to_str().unwrap(), fp2.to_str().unwrap(), None)
        .unwrap();

    log::info!("file 2 end: {}", fs::read_to_string(fp2.clone()).unwrap());
    assert_eq!(
        fs::read_to_string(fp1).unwrap(),
        fs::read_to_string(fp2).unwrap()
    );
}
