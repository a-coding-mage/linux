/*
 * Indirect stringification. Doing two levels allows the parameter to be a
 * macro itself. For example, with FOO defined as bar, __stringify!(FOO)
 * converts to "bar".
 */

macro_rules! __stringify_1 {
    ($($x:tt)*) => {
        stringify!($($x)*)
    };
}

macro_rules! __stringify {
    ($($x:tt)*) => {
        __stringify_1!($($x)*)
    };
}

macro_rules! FILE_LINE {
    () => {
        concat!(file!(), ":", line!())
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
