#[allow(unused_variables)] // To allow some variables that mimic the C structure
const ALL: usize = 100; // all people in the ring
const OUT: i32 = 3; // if count to 3, then out

fn init_ring() -> Vec<usize> {
    let mut next: Vec<usize> = vec![0; ALL];
    for i in 0..ALL {
        next[i] = (i + 1) % ALL;
    }
    next
}

fn print_ring(next: &[usize]) {
    for &person in next.iter() {
        print!("{} ", person);
    }
    println!("   (next)");
}

fn main() {
    println!("demo josephus ring problem");

    let mut next = init_ring();

    print_ring(&next);

    let mut left = ALL; // left = all people
    let mut counter = 0; // counter = 1, 2, 3
    let mut i = 0; // begin from [0]
    let mut prev = ALL - 1; // 0's prev = ALL - 1
    while left > 0 {
        counter += 1;

        if counter == OUT {
            left -= 1; // someone is out
            println!("{} is out", i + 1);

            next[prev] = next[i];
            counter = 0; // reset counter
        }

        prev = i;
        i = next[i]; // get next i
    }

    println!("problem is finished");
}