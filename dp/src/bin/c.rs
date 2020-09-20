macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        #[allow(unused_mut)]
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use std::iter::Iterator;

fn main() {
    input! {
        n: usize,
        choices: [[u32; 3]; n],
    }
    let mut max_happiness_on = vec![vec![0; 3]; n];
    max_happiness_on[0] = choices[0].clone();

    for i in 1..=(n - 1) {
        let ii = i as isize;
        for j in 0..=2 {
            let ij = j as isize;
            max_happiness_on[i][j] = choices[i][j]
                + u32::max(
                    max_happiness_on[(ii - 1) as usize][((ij - 1).rem_euclid(3)) as usize],
                    max_happiness_on[(ii - 1) as usize][((ij - 2).rem_euclid(3)) as usize],
                )
        }
    }
    let max_happiness_on_last_day = max_happiness_on[n - 1].iter().max().unwrap();
    println!("{}", max_happiness_on_last_day)
}
