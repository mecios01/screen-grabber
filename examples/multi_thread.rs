use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum Signals {
    Start,
    Stop,
}

pub struct TestApp {
    signal: Option<Signals>,
    opt: Option<Box<f32>>,
    ch: Receiver<Signals>,
}

fn main() {
    let (sender, receiver) = channel::<Signals>();
    let app = Box::new(TestApp {
        signal: None,
        opt: Some(Box::new(0.0)),
        ch: receiver,
    });
    let r = Arc::new(Mutex::new(app));

    let s1 = sender.clone();
    let r1 = Arc::downgrade(&r);
    let _mtr = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(3));
        let arc = r1.upgrade().unwrap();
        let mut guard = arc.lock().unwrap();
        println!("field {:?}", guard.signal);
        if let Ok(s) = guard.ch.try_recv() {
            println!("{:?}", s);
            guard.signal = Some(s);
        }
    });
    let s2 = sender.clone();
    let r2 = r.clone();
    let _aux = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        s2.send(Signals::Start).unwrap();
        thread::sleep(Duration::from_secs(4));
        s2.send(Signals::Stop).unwrap();
    });

    drop(r);

    thread::sleep(Duration::from_secs(30));
}
