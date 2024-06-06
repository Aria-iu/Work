use std::sync::mpsc::*;
use std::thread;
use std::time::Duration;

pub struct Ticker<I: Iterator> {
    src: I,
    recv: Receiver<()>,
    kill: Sender<()>,
}

impl<I: Iterator> Ticker<I> {
    pub fn new(src: I, interval: Duration) -> Self {
        let (send, recv) = channel::<()>();
        let (kill, kill_recv) = channel::<()>();

        thread::spawn(move || loop {
            match kill_recv.recv_timeout(interval) {
                Err(RecvTimeoutError::Timeout) => {
                    if let Err(_) = send.send(()) {
                        return;
                    }
                }
                _ => {
                    return;
                }
            }
        });
        Ticker { src, recv, kill }
    }
}

impl<I: Iterator> Drop for Ticker<I> {
    fn drop(&mut self) {
        let _ = self.kill.send(());
    }
}

impl<I: Iterator> IntoIterator for Ticker<I> {
    // 枚举的是src中的元素
    type Item = I::Item;
    type IntoIter = TickIter<I>;

    fn into_iter(self) -> Self::IntoIter {
        TickIter { ticker: self }
    }
}

pub struct TickIter<I: Iterator> {
    ticker: Ticker<I>,
}

impl<I: Iterator> Iterator for TickIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let _ = self.ticker.recv.recv().expect("ticker channel to live");
        self.ticker.src.next()
    }
}
