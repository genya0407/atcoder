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
    let mut cache = vec![vec![None; w as usize + 1]; n as usize];
    let v = max_value(&loads, &mut cache, n - 1, w);
    println!("{}", v)
}

fn max_value(loads: &Vec<Load>, cache: &mut Vec<Vec<Option<i64>>>, k: i64, y: i64) -> i64 {
    if k < 0 || y < 0 {
        0
    } else {
        if let Some(v) = cache[k as usize][y as usize] {
            return v;
        }
        let using = if y - loads[k as usize].weight < 0 {
            0
        } else {
            max_value(loads, cache, k - 1, y - loads[k as usize].weight) + loads[k as usize].value
        };
        let not_using = max_value(loads, cache, k - 1, y);
        let value = i64::max(using, not_using);
        cache[k as usize][y as usize] = Some(value);
        value
    }
}
