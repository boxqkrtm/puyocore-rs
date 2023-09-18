use std::time::Instant;
use puyo_core::field::Field;
use puyo_core::chain;

fn bench_pop(iter: i32) -> i64 {
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

    for _ in 0..iter {
        let mut f_copy = f.clone();
        mask = f_copy.pop();
        //chain::get_score(mask.clone());
    }
    let time_end = Instant::now();
    let time = time_end.duration_since(time_start).as_nanos() as i64;
    let chain = chain::get_score(mask);
    f.print();
    println!("count: {}", chain.count);
    println!("score: {}", chain.score);
    time / iter as i64
}

fn main() {
    let iter = 10000; // 원하는 반복 횟수 설정
    let avg_time = bench_pop(iter);
    println!("Average time: {} nanoseconds", avg_time);
}