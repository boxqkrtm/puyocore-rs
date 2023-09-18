use std::time::Instant;
use puyo_core::field::Field;
use puyo_core::chain;

fn bench_pop(iter: i32) {
    let mut f = Field::new(); // Field 구조체 초기화 코드 추가

    let c: [[char; 7]; 13] = [
        ['B','.','Y','R','G','Y','\0'],
        ['B','B','B','Y','R','B','\0'],
        ['G','B','Y','R','G','G','\0'],
        ['B','G','Y','R','G','B','\0'],
        ['G','R','G','Y','R','B','\0'],
        ['R','G','Y','R','Y','B','\0'],
        ['G','R','G','Y','R','Y','\0'],
        ['G','R','G','Y','R','Y','\0'],
        ['G','B','B','G','Y','G','\0'],
        ['B','Y','R','B','G','G','\0'],
        ['G','B','Y','R','B','Y','\0'],
        ['G','B','Y','R','B','Y','\0'],
        ['G','B','Y','R','B','Y','\0'],
    ];
    f.from(&c);
    let mut mask: Vec<Field> = Vec::new();
    let time_start = Instant::now();
    let mut chain = chain::Chain { count: mask.len() as u32, score: 0 };
    for _ in 0..iter {
        let mut f_copy = f.clone();
        mask = f_copy.pop();
        chain = chain::get_score(&mut mask);
    }
    let time_end = Instant::now();
    let time = time_end.duration_since(time_start).as_millis() as i32;
    f.print();
    println!("iter = {}", iter);
    println!("chain = {} score = {} ojama = {}", chain.count, chain.score, chain.score/70);
    println!("elapsed = {}ms", time);
    println!("times/s = {}", iter as f32 / (time as f32 / 1000.0));
    println!("times/10ms = {}", iter as f32 / time as f32 * 10.0);
}

fn main() {
    let iter = 5000000; // 원하는 반복 횟수 설정
    bench_pop(iter);
}