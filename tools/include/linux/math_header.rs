/*
 * This looks more complex than it should be. But we need to
 * get the type for the ~ right in round_down (it needs to be
 * as wide as the result!), and we want to evaluate the macro
 * arguments just once each.
 */
macro_rules! __round_mask {
    ($x:expr, $y:expr) => {
        (($y) - 1) as _
    };
}

macro_rules! round_up {
    ($x:expr, $y:expr) => {
        ((($x) - 1) | __round_mask!($x, $y)) + 1
    };
}

macro_rules! round_down {
    ($x:expr, $y:expr) => {
        ($x) & !__round_mask!($x, $y)
    };
}

macro_rules! DIV_ROUND_UP {
    ($n:expr, $d:expr) => {
        (($n) + ($d) - 1) / ($d)
    };
}

/* Original C condition: #ifndef roundup */
macro_rules! roundup {
    ($x:expr, $y:expr) => {{
        let __y = $y;
        ((($x) + (__y - 1)) / __y) * __y
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
