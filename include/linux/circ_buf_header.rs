/* SPDX-License-Identifier: GPL-2.0 */
/*
 * See Documentation/core-api/circular-buffers.rst for more information.
 */

#[repr(C)]
pub struct circ_buf {
    pub buf: *mut core::ffi::c_char,
    pub head: i32,
    pub tail: i32,
}

/* Return count in buffer.  */
#[macro_export]
macro_rules! CIRC_CNT {
    ($head:expr, $tail:expr, $size:expr) => {
        (($head) - ($tail)) & (($size) - 1)
    };
}

/* Return space available, 0..size-1.  We always leave one free char
   as a completely full buffer has head == tail, which is the same as
   empty.  */
#[macro_export]
macro_rules! CIRC_SPACE {
    ($head:expr, $tail:expr, $size:expr) => {
        $crate::CIRC_CNT!(($tail), (($head) + 1), ($size))
    };
}

/* Return count up to the end of the buffer.  Carefully avoid
   accessing head and tail more than once, so they can change
   underneath us without returning inconsistent results.  */
#[macro_export]
macro_rules! CIRC_CNT_TO_END {
    ($head:expr, $tail:expr, $size:expr) => {{
        let end = ($size) - ($tail);
        let n = (($head) + end) & (($size) - 1);
        if n < end { n } else { end }
    }};
}

/* Return space available up to the end of the buffer.  */
#[macro_export]
macro_rules! CIRC_SPACE_TO_END {
    ($head:expr, $tail:expr, $size:expr) => {{
        let end = ($size) - 1 - ($head);
        let n = (end + ($tail)) & (($size) - 1);
        if n <= end { n } else { end + 1 }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
