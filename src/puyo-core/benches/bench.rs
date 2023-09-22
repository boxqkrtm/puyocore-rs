use puyo_core::*;
use std::{arch::x86_64::_pext_u32, time::Instant};

fn bench_pext(iter: i32) -> u32 {
    let mut time_start: Instant;
    let mut result: u32 = 0;
    let v = 0b000101101111011u16;
    //pext bench for puyo possible mask 1pop~8pop
    let masks: Vec<u16> = vec![
        0b1111011111111111,
        0b111100111111111,
        0b1111000111111111,
        0b1111000011111111,
        0b1111000001111111,
        0b1111000000111111,
        0b1111000000011111,
        0b1111000000001111,
    ];

    time_start = Instant::now();
    for _ in 0..iter {
        for m in masks.iter() {
            unsafe {
                result = result.wrapping_add(_pext_u32(v as u32, m.clone() as u32));
            }
        }
    }
    println!(
        "_pext_u32 {}ms",
        Instant::now().duration_since(time_start).as_millis() as i32
    );

    time_start = Instant::now();
    for m in masks.iter() {
        for _ in 0..iter {
            result = result.wrapping_add(util::pext15_emu(v, m.clone()) as u32);
        }
    }
    println!(
        "pext15_emu {}ms",
        Instant::now().duration_since(time_start).as_millis() as i32
    );

    time_start = Instant::now();
    for _ in 0..iter {
        for m in masks.iter() {
            result = result.wrapping_add(util::pext32_emu(v as u32, m.clone() as u32));
        }
    }
    println!(
        "pext32_emu {}ms",
        Instant::now().duration_since(time_start).as_millis() as i32
    );

    time_start = Instant::now();
    for m in masks.iter() {
        for _ in 0..iter {
            result = result.wrapping_add(util::pext16_naive(v, m.clone()) as u32);
        }
    }
    println!(
        "pext16_naive {}ms",
        Instant::now().duration_since(time_start).as_millis() as i32
    );
    println!("sum : {}", result);
    return result;
}

fn bench_pop(iter: i32) {
    let mut f = puyo_core::field::Field::new(); // Field 구조체 초기화 코드 추가
    let c: [[char; 7]; 13] = [
        ['B', '.', 'Y', 'R', 'G', 'Y', '\0'],
        ['B', 'B', 'B', 'Y', 'R', 'B', '\0'],
        ['G', 'B', 'Y', 'R', 'G', 'G', '\0'],
        ['B', 'G', 'Y', 'R', 'G', 'B', '\0'],
        ['G', 'R', 'G', 'Y', 'R', 'B', '\0'],
        ['R', 'G', 'Y', 'R', 'Y', 'B', '\0'],
        ['G', 'R', 'G', 'Y', 'R', 'Y', '\0'],
        ['G', 'R', 'G', 'Y', 'R', 'Y', '\0'],
        ['G', 'B', 'B', 'G', 'Y', 'G', '\0'],
        ['B', 'Y', 'R', 'B', 'G', 'G', '\0'],
        ['G', 'B', 'Y', 'R', 'B', 'Y', '\0'],
        ['G', 'B', 'Y', 'R', 'B', 'Y', '\0'],
        ['G', 'B', 'Y', 'R', 'B', 'Y', '\0'],
    ];
    f.from(&c);
    let mut mask: Vec<puyo_core::field::Field> = Vec::new();
    let mut chain = chain::Chain {
        count: mask.len() as u32,
        score: 0,
    };
    let time_start = Instant::now();
    for _ in 0..iter {
        let mut f_copy = f.clone();
        mask = f_copy.pop();
        chain = chain::get_score(&mut mask);
    }
    let time_end = Instant::now();
    let time = time_end.duration_since(time_start).as_millis() as i32;
    //f.print();
    println!("iter = {}", iter);
    println!(
        "chain = {} score = {} ojama = {}",
        chain.count,
        chain.score,
        chain.score / 70
    );
    println!("elapsed = {}ms", time);
    println!("times/s = {}", iter as f32 / (time as f32 / 1000.0));
    println!("times/16ms = {}", iter as f32 / time as f32 * 16.0);
}

fn main() {
    bench_pext(10000000);
    bench_pop(5000000);
}
