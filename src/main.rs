use std::io;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line.");

    println!("You guessed: {}", guess);

    let p = Point { x: 100, y: 200};
    println!("{} {}", p.x, p.y);
    
    //配列は固定長
    let arr = [10, 20, 30];
    println!("{} {} {}", arr[0], arr[1], arr[2]);
    //ベクターは可変長として扱える
    let mut vector = vec![10, 20, 30];
    vector.push(40); 
    println!("{}", vector[3]);
    //ヒープ領域
    let p: Box<Point> = Box::new(Point {x: 100, y: 200});
    println!("{} {}", p.x, p.y);
    //関数の呼び出し
    println!("{}", calc_try_angle(3,5));  
}

fn calc_try_angle(x: i32, y: i32) -> i32 {
    return x * y / 2;
}
