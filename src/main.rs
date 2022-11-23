use chrono::{DateTime, Local};
use fast_log::{
    appender::{FastLogRecord, LogAppender},
    Config,
};
use log::info;
use log::Level;

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

pub struct CustomLog {}
impl LogAppender for CustomLog {
    fn do_logs(&self, records: &[FastLogRecord]) {
        for record in records {
            let now: DateTime<Local> = chrono::DateTime::from(record.now);
            let data;
            match record.level {
                Level::Warn | Level::Error => {
                    data = format!(
                        "{} {} {} - {}  {}\n",
                        now, record.level, record.module_path, record.args, record.formated
                    );
                }
                _ => {
                    data = format!(
                        "{} {} {} - {}\n",
                        &now, record.level, record.module_path, record.args
                    );
                }
            }
            print!("{}", data);
        }
    }
}

fn main() {
    fast_log::init(Config::new().custom(CustomLog {})).unwrap();
    worker(Foo);
    worker(Bar);
    log::logger().flush();
}
