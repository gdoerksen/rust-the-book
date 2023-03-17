fn main() {
    let fib_result = fib_iter(100);
    println!("{fib_result}");
}

// fn named_loop() {
//     let mut outer_count = 0;

//     // in Rust you can name loops 
//     'counting_up: loop {
//         println!("count = {outer_count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if outer_count == 2 {
//                 // Named loops can be broken out of
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         outer_count += 1;
//     }
//     println!("End count = {outer_count}");
// }

// fn while_rocket() {
//     let mut count = 10;

//     while count > 0 {
//         println!("{count}...");
//         count -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn iter_on_array() {
//     let a = [1, 2, 3, 4, 5];

//     let mut sum = 0;
//     for i in a {
//         sum += i;
//     }

//     println!("{sum}");
// }

fn fib_iter(n: i128)->i128 {
    // let mut sum = 0;
    
    let mut n1 = 0;
    let mut n2 = 1;
    let mut temp = 0;
    
    if n == 0 {
        return n1
    } else if n == 1 {
        return n2
    }

    let mut c = 0;
    while c < n {
        // sum += n1;
        // println!("{n1}");

        temp = n2;
        n2 += n1;
        n1 = temp;

        c += 1;
    }
    n1
}