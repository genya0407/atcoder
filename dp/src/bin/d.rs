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

#[derive(Debug)]
struct Load {
    pub weight: i64,
    pub value: i64,
}

fn main() {
    input! {
        n: i64,
        w: i64,
        _loads: [(i64, i64); n as usize]
    }
    let mut loads = vec![];
    for l in _loads {
        loads.push(Load {
            weight: l.0,
            value: l.1,
        })
    }
    // [k][y] -> value
    let mut max_values = vec![vec![0; (w + 1) as usize]; n as usize];
    for k in 0..=(n - 1) {
        for y in 1..=w {
            let using = access(&max_values, k - 1, y - loads[k as usize].weight)
                .map(|s| s + loads[k as usize].value)
                .unwrap_or(0);
            let not_using = access(&max_values, k - 1, y).unwrap_or(0);
            max_values[k as usize][y as usize] = i64::max(using, not_using);
        }
    }
    println!("{}", max_values[(n - 1) as usize][w as usize])
}

fn access(max_values: &Vec<Vec<i64>>, k: i64, y: i64) -> Option<i64> {
    if y < 0 {
        None
    } else if k < 0 {
        Some(0)
    } else {
        Some(max_values[k as usize][y as usize])
    }
}
