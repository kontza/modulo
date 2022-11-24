use std::{sync::mpsc::channel, thread, time::Duration};

use tracing::{event, Level};
use tracing_subscriber::{self, fmt};

struct MyWriter;
impl std::io::Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        print!("{}", s);
        Ok(s.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn make_my_writer() -> impl std::io::Write {
    MyWriter {}
}

fn main() {
    fmt().with_writer(make_my_writer).init();
    event!(Level::INFO, "something happened");
    let recv = {
        let (tx, rx) = channel();
        let workers = 10;
        for i in 1..workers {
            let thread_tx = tx.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(1500));
                let msg = format!("hi number {} from the spawned thread!", i);
                event!(Level::INFO, msg);
                thread_tx.send(1).unwrap();
            });
        }
        rx
    };
    for _x in recv {}
    event!(Level::INFO, "All done, TTFN!");
}
