fn main() {
    println!("华氏{}度=摄氏{}度", 32.0, ctof(32.0));
    println!("摄氏{}度=华氏{}度", 89.6, ftoc(89.6));
    println!("{}阶斐波那契数列:", 32);
    fibonacci(32);
}

// 华氏温度to摄氏温度
fn ctof(c: f32) -> f32 {
    9.0 * c / 5.0 + 32.0
}
// 摄氏温度to华氏温暖度
fn ftoc(f: f32) -> f32 {
    5.0 * (f - 32.0) / 9.0
}

// n阶斐波那契数列
fn fibonacci(n: i32) {
    let mut p = (0, 0);
    for i in 1..n + 1 {
        let an = match i == 1 {
            true => 1,
            false => p.0 + p.1,
        };
        p.0 = p.1;
        p.1 = an;
        println!("a{}:{}", i, an);
    }
}
