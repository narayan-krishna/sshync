mod comms;
mod client;
mod server;
mod ssh;
mod shm;
mod sync;
mod servicer;

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

/// thinking about not_rsync as a library
// fn remote_sshync () -> anyhow::Result<()> {
//     let snc = sshync::SshClient::new(); // returns a sync ssh connection
//
//     let sync_args = sshync::Args::default().verbose().backups().quiet();
//
//     snc.sync(file1, file2)?; // should be blocking or async?
//     snc.sync_dir(dir1, dir2)?;
// }
//
// fn sync_some_directories () -> anyhow::Result<()> {
//     let sshync_ssh = sshync::ShmClient::new();
//
//     let file_sync_args = sshync::args::default().
//
//     nrb.sync(file1, file2, args)?;
//     nrb.sync_dir(dir1, dir2, args)?;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn create_client_cant_req() {
        let mut snc = crate::Sshync::from_shm();
        let sync_args = crate::Args::default();

        let file1 = "/home/knara/dev/rust/sshync/tests/test_files/a.txt";
        let file2 = "/home/knara/dev/rust/sshync/tests/test_files/b.txt";

        snc.sync(file1, file2, sync_args).unwrap();
    }

    #[test]
    fn create_client_cant_req2() {
        let mut snc = crate::Sshync::from_ssh();
        let sync_args = crate::Args::default().verbose(true).backup(true);

        let file1 = "/home/knara/dev/rust/sshync/tests/test_files/a.txt";
        let file2 = "/home/knara/dev/rust/sshync/tests/test_files/b.txt";

        snc.sync(file1, file2, sync_args).unwrap();
    }
}
