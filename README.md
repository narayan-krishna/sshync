# sshync (wip)

sshync is a library for embedding remote file synchronization directly into your Rust workflow.

**over SSH**
```rust
    use sshync::{Sshync, Args};

    fn main() -> Result<()> {
        let mut sc = Sshync::from_ssh();
        let sync_args = Args::default().verbose(true).backup(true);

        let file1 = "/home/knara/dev/rust/sshync/tests/test_files/a.txt";
        let file2 = "/home/knara/dev/rust/sshync/tests/test_files/b.txt";

        sc.sync(file1, file2, sync_args).unwrap();

        Ok(())
    }
```

**over Shm**
```rust
    use sshync::{Sshync, Args};

    fn main() -> Result<()> {
        let mut snc = Sshync::from_shm();
        let sync_args = Args::default();

        let file1 = "/home/knara/dev/rust/sshync/tests/test_files/a.txt";
        let file2 = "/home/knara/dev/rust/sshync/tests/test_files/b.txt";

        snc.sync(file1, file2, sync_args)?;
        snc.sync(file1, file2, sync_args)?;

        Ok(())
    }
```

### TODO
- [ ] implement atomicity with temp files
- [ ] more robust integration and unit testing
- [ ] performance measurement (criterion)
- [ ] support for (recursive) directory synchronization
- [ ] batch processing for large groups of files
- [ ] a more strict send/receive protocol using serde serialize/deserialize
- [ ] better comments

and more.

#### Links
- [the rsync algorithm](https://www.andrew.cmu.edu/course/15-749/READINGS/required/cas/tridgell96.pdf)
- [rsync thesis](https://www.samba.org/~tridge/phd_thesis.pdf)

