//problem definition https://www.hackerrank.com/challenges/compare-the-triplets/problem

fn main() {
    let alice = (5, 6, 7);
    let bob = (3, 8, 10);

    println!("{:?}", compare_triplets(alice, bob));
}

fn compare_triplets(a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32){
    let mut ares = 0;
    let mut bres = 0;

    if a.0 != b.0 {
        if a.0 > b.0 { ares += 1; } else { bres += 1; }    
    }

    if a.1 != b.1 {
        if a.1 > b.1 { ares += 1; } else { bres += 1; }    
    }

    if a.2 != b.2 {
        if a.2 > b.2 { ares += 1; } else { bres += 1; }    
    }

    return (ares, bres);
}