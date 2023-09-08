# sshync

sshync is a file synchronization service library built on `fast_rsync`. sshync wraps around an existing bidirectional byte-level read-write protocol between a client and a server.

## Usage

1. Implement byte-level read/write functionality in sshync `Client` and `Server` for your client/server.

```rust
// my_server.rs
impl sshync::Server for MyServer {
    fn send(&mut self, response: Vec<u8>) -> anyhow::Result<()> {
        // implement server send
    }

    fn receive(&mut self) -> anyhow::Result<Vec<u8>> {
        // implement server receive
    }
}

// my_client.rs
impl sshync::Client for MyClient {
    fn request(&mut self, request: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        // implement client request-response
    }
}
```

2. Run your server, assuming it's ready to make send/recv calls.
```rust
// my_server.rs 

let mut server = RemoteServer::new(tcp_stream);
server.run().unwrap();
```

3. Sync files to and from your server through a `Sshync` object, with custom configurations supplied through `Args`.

```rust
// my_client.rs
let client = Box::new(ShmClient::init());
let default_args = Args::default().quiet().backup().recursive();

let mut snc = Sshync::init(client, default_args);
snc.sync("my_file.txt", "not_my_file.txt", None)?;
```

## TODO
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

