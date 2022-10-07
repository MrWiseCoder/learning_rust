pub fn main2() {
    let mut count = 0;

    'outer_loop: loop {
        println!("Count: {}", count);
        let mut remaining = 10;

        loop {
            println!("rem: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer_loop;
            }
            remaining -= 1;
        }

        count += 1
    }
    println!("Count: {}", count);
}

pub fn main3() {
    let mut number = 3;
    while number != 0 {
        println!("no: {}", number);
        number -= 1;
    }
    print!("finished");
}

pub fn main4() {
    let arr = [1, 2, 55, 42, 3];
    let mut index = 0;
    while index < 5 {
        println!("arr[{}]={}", index, arr[index]);
        index += 1;
    }
    println!("arr show!");
}

pub fn main5() {
    let arr = [1, 2, 55, 42, 3];
    for elem in arr {
        println!("\t arr[?] = {}", elem);
    }
    println!("arr show 2!");
}

pub fn main6() {
    for elem in (4..10).rev() {
        println!("elem: {}", elem);
    }
}
