fn prime(x: u32) -> bool {
    let y = x / 2;
    if x < 2 {
        return false;
    }
    for i in 2..y {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}

fn coprime(x: u32, y: u32) -> bool {
    for i in 2..y / 2 {
        if y % i == 0 {
            if x % i == 0 {
                return false;
            }
        }
    }
    return true;
}

fn sing() {
    for i in (3..100).rev() {
        println!(
            "
        {i} bottles of beer on the wall,
        {i} bottles of beer.
        Take one down, pass it around,
        {i} bottles of beer on the wall."
        )
    }
    println!(
        "
        2 bottles of beer on the wall,
        2 bottles of beer!
        Take one down, and pass it around,
        1 bottle of beer on the wall!

        1 bottle of beer on the wall,
        1 bottle of beer!
        Take one down, and pass it around,
        No more bottles of beer on the wall!

        No bottles of beer on the wall,
        No bottles of beer.
        Go to the store, buy some more,
        99 bottles of beer on the wall."
    )
}

fn main() {
    for i in 1..=100 {
        if prime(i) == true {
            println!("{i} e prim");
        }
    }
    for i in 1..=100 {
        for j in 1..=100 {
            println!("{i} and {j} are coprime is {}", coprime(i, j))
        }
    }
    sing();
}
