macro_rules! DEFINE_RATELIMIT_STATE {
    ($name:ident, $interval_init:expr, $burst_init:expr) => {
        static mut $name: i32 = 0;
    };
}

macro_rules! __ratelimit {
    ($x:expr) => {
        unsafe { *($x) }
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
