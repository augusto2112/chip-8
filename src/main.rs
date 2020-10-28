extern crate minifb;

mod engine;

use engine::key::Key;
use engine::vm::{HEIGHT, VM, WIDTH};
use minifb::{Scale, Window, WindowOptions};
use std::collections::HashSet;
use std::env;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!(String::from("Usage: \"chip-8 <ROM>\""));
    }
    let rom = args[1].clone();

    let mut options = WindowOptions::default();
    options.scale = Scale::X16;
    options.resize = true;
    let mut window = Window::new("CHIP-8", WIDTH, HEIGHT, options).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let (tx_buf, rx_buf) = std::sync::mpsc::channel::<Vec<u32>>();
    // We have to send keys pressed and keys released because the keys aren't sent every iteration
    let (tx_key_pressed, rx_key_pressed) = std::sync::mpsc::channel::<Vec<Key>>();
    let (tx_key_released, rx_key_released) = std::sync::mpsc::channel::<Vec<Key>>();

    std::thread::spawn(move || {
        let rom = std::fs::read(rom).unwrap();
        let mut vm = VM::new(&rom);

        let mut last_instant = Instant::now();
        let mut keys: HashSet<Key> = HashSet::new();
        loop {
            std::thread::sleep(Duration::from_micros(1660) - last_instant.elapsed());
            keys.extend(rx_key_pressed.try_iter().flatten());
            for element in rx_key_released.try_iter().flatten() {
                keys.remove(&element);
            }
            let keys: Vec<Key> = keys.clone().into_iter().collect();
            vm.tick(&keys);
            tx_buf
                .send(vm.get_current_frame())
                .expect("unable to send buffer");

            last_instant = Instant::now();
        }
    });
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window
            .get_keys_pressed(minifb::KeyRepeat::No)
            .iter()
            .for_each(|keys| {
                tx_key_pressed
                    .send(keys.iter().filter_map(|k| Key::from(*k)).collect())
                    .expect("key send failed")
            });
        window.get_keys_released().iter().for_each(|keys| {
            tx_key_released
                .send(keys.iter().filter_map(|k| Key::from(*k)).collect())
                .expect("key send failed")
        });

        match rx_buf.try_iter().last() {
            Some(buffer) => window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap(),
            None => window.update(),
        }
    }
}
