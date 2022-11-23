use chrono::{DateTime, Local};
use fast_log::{
    appender::{FastLogRecord, LogAppender},
    Config,
};
use log::info;
use log::Level;
use std::{sync::mpsc::channel, time::Duration};
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};

pub trait Summary {
    fn instance(&self);
}

pub struct Foo;

impl Summary for Foo {
    fn instance(&self) {
        info!("foo instance");
    }
}

pub struct Bar;
impl Summary for Bar {
    fn instance(&self) {
        info!("bar instance");
    }
}

fn worker(arg: impl Summary) {
    arg.instance();
}

pub struct CustomLog {
    tx: Sender<String>,
}
impl LogAppender for CustomLog {
    fn do_logs(&self, records: &[FastLogRecord]) {
        for record in records {
            let now: DateTime<Local> = chrono::DateTime::from(record.now);
            let data;
            match record.level {
                Level::Warn | Level::Error => {
                    data = format!(
                        "{} {} {} - {}  {}",
                        now, record.level, record.module_path, record.args, record.formated
                    );
                }
                _ => {
                    data = format!(
                        "{} {} {} - {}",
                        &now, record.level, record.module_path, record.args
                    );
                }
            }
            self.tx.send(data).unwrap();
        }
    }
}

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    println!("Initializing fast_log...");
    let log_tx = tx.clone();
    let cl = CustomLog { tx: log_tx };
    fast_log::init(Config::new().custom(cl)).unwrap();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        worker(Foo);
        thread::sleep(Duration::from_secs(1));
        worker(Bar);
        println!("Thread done");
    });
    for received in rx {
        println!("RX: {}", received);
    }
    println!("Loop done");
    log::logger().flush();
    println!("Flushed");
}
