mod client;
mod comms;
mod server;
mod servicer;
mod shm;
mod ssh;
mod sync;

pub use sync::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, Clone, Copy)]
pub struct Args {
    verbose: bool,
    quiet: bool,
    backup: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            verbose: false,
            quiet: false,
            backup: false,
        }
    }
}

impl Args {
    fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    fn quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }

    fn backup(mut self, backup: bool) -> Self {
        self.backup = backup;
        self
    }
}

#[cfg(test)]
mod tests {
    use test_files::TestFiles;
    use std::fs;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_shm_simple() {
        init();

        let (file1, contents1) = ("a.txt", "this is a base file");
        let (file2, contents2) = ("b.txt", "this is a modified file");

        let temp_dir = TestFiles::new();
        temp_dir.file(file1, contents1).file(file2, contents2);

        let fp1 = temp_dir.path().join(file1);
        let fp2 = temp_dir.path().join(file2);

        log::info!("file 2 start: {}", fs::read_to_string(fp2.clone()).unwrap());
        assert_ne!(
            fs::read_to_string(fp1.clone()).unwrap(),
            fs::read_to_string(fp2.clone()).unwrap()
        );

        let mut snc = crate::Sshync::from_shm();
        snc.sync(fp1.to_str().unwrap(), fp2.to_str().unwrap(), None).unwrap();

        log::info!("file 2 end: {}", fs::read_to_string(fp2.clone()).unwrap());
        assert_eq!(
            fs::read_to_string(fp1).unwrap(),
            fs::read_to_string(fp2).unwrap()
        );
    }

    #[test]
    fn create_client_cant_req2() {
        let mut snc = crate::Sshync::from_ssh();
        let sync_args = crate::Args::default().verbose(true).backup(true);

        let file1 = "/home/knara/dev/rust/sshync/tests/test_files/a.txt";
        let file2 = "/home/knara/dev/rust/sshync/tests/test_files/b.txt";

        snc.sync(file1, file2, Some(sync_args)).unwrap();
    }
}
