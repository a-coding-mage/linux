/* SPDX-License-Identifier: GPL-2.0 */

// C preprocessor forwarding macros translated as Rust declarative macros.
// The referenced macros and identifiers are supplied by dependent files.
macro_rules! REQUEST {
    ($r_contents:tt) => {
        REQUEST_(REQUEST_NAME, REQUEST_NUM, REQUEST_IDX_KIND, I!($r_contents))
    };
}

macro_rules! __field {
    ($f_offset:expr, $f_bytes:expr, $f_name:expr) => {
        __field_(REQUEST_NAME, REQUEST_NUM, REQUEST_IDX_KIND,
                 $f_offset, $f_bytes, $f_name)
    };
}

macro_rules! __array {
    ($f_offset:expr, $f_bytes:expr, $f_name:expr) => {
        __array_(REQUEST_NAME, REQUEST_NUM, REQUEST_IDX_KIND,
                 $f_offset, $f_bytes, $f_name)
    };
}

macro_rules! __count {
    ($f_offset:expr, $f_bytes:expr, $f_name:expr) => {
        __count_(REQUEST_NAME, REQUEST_NUM, REQUEST_IDX_KIND,
                 $f_offset, $f_bytes, $f_name)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
