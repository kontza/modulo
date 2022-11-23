use fast_log::appender::LogAppender;

pub trait Summary {
    fn instance(&self);
}

pub struct Foo;

impl Summary for Foo {
    fn instance(&self) {
        println!("foo instance");
    }
}

pub struct Bar;
impl Summary for Bar {
    fn instance(&self) {
        println!("bar instance");
    }
}

fn worker(arg: impl Summary) {
    arg.instance();
}

pub struct CustomLog {}
impl LogAppender for CustomLog {
    fn do_log(&mut self, record: &FastLogRecord) {
        print!("{}", record);
    }
}

fn main() {
    let wait = fast_log::init(Config::new().custom(CustomLog {})).unwrap();
    worker(Foo);
    worker(Bar);
    log::logger().flush();
}
