use anyhow::{anyhow, Result};
use rand::rngs::ThreadRng;
use rand::{distr::Uniform, prelude::*};

use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Consumer: {:?}", msg);
        }
        42
    });
    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;
    println!("secret: {}", secret);
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    let mut rng = ThreadRng::default();
    let value_dist = Uniform::new(0, 100).unwrap();
    let sleep_dist = Uniform::new(0, 1000).unwrap();
    let exit_dist = Uniform::new(0, 5).unwrap();

    loop {
        let value = value_dist.sample(&mut rng);
        tx.send(Msg::new(idx, value))?;

        let sleep_time = sleep_dist.sample(&mut rng);
        thread::sleep(Duration::from_millis(sleep_time));

        if exit_dist.sample(&mut rng) == 0 {
            println!("Producer {} exit", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
