use std::{sync::atomic::AtomicI32, thread::yield_now, time::Duration};

use thread_priority::*;

static LOW_COUNT: AtomicI32 = AtomicI32::new(0);
static MEDIUM_COUNT: AtomicI32 = AtomicI32::new(0);
static HIGH_COUNT: AtomicI32 = AtomicI32::new(0);

fn low_priority() {
    set_current_thread_priority(ThreadPriority::Min).unwrap();

    loop {
        LOW_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        yield_now();
    }
}

fn medium_priority() {
    loop {
        MEDIUM_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        yield_now();
    }
}

fn high_priority() {
    set_current_thread_priority(ThreadPriority::Max).unwrap();

    loop {
        HIGH_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        yield_now();
    }
}

fn main() {
    std::thread::spawn(low_priority);
    std::thread::spawn(medium_priority);
    std::thread::spawn(high_priority);

    std::thread::sleep(Duration::from_secs(5));

    println!("LOW_COUNT: {}", LOW_COUNT.load(std::sync::atomic::Ordering::Relaxed));
    println!("MEDIUM_COUNT: {}", MEDIUM_COUNT.load(std::sync::atomic::Ordering::Relaxed));
    println!("HIGH_COUNT: {}", HIGH_COUNT.load(std::sync::atomic::Ordering::Relaxed));
}
