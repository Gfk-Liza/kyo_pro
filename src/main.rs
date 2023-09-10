
use proconio::input;

fn main() {
    input! {N: i32};
    if N <= 0 {
        println!("The input must be a positive integer.");
        return;
    }
    if N == 1 {
        print!("2");
        return;
    }

    let mut n = 0;
    let mut i = 3;
    while N > n {
        let mut flag = false;
        for k in 3..i {
            if i % k == 0 {
                flag = true;
                break;
            }
        }

        if flag == false {
            print!("{} ", i);
            n += 1;
        }
        i += 2;
    }
}


fn faster() {
    input! {N: i32};
    if N <= 0 {
        println!("The input must be a positive integer.");
        return;
    }
    if N == 1 {
        print!("2");
        return;
    }
    let mut p = Vec::new();

    let mut n = 0;
    let mut i = 3;
    while N > n {
        let mut flag = false;
        let max_k = (i as f64).sqrt() as i32;
        for k in p.iter() {
            if *k > max_k {
                break;
            }
            if i % *k == 0 {
                flag = true;
                break;
            }
        }

        if flag == false {
            p.push(i);
            print!("{} ", i);
            n += 1;
        }
        i += 2;
    }
}
