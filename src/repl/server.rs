use std::os::unix::net::{UnixStream, UnixListener};

fn handle_client(stream: UnixStream) {
    // ...
}

fn main() -> std::io::Result<()> {
    let listener = UnixListener::bind("/tmp/gbe-v2-debugger")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = BufReader::new(stream);
                for line in stream.lines() {
                    debugger.eval(line);
                }
            }
            Err(err) => {
                break;
            }
        }
    }
    Ok(())
}
