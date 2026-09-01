// SPDX-License-Identifier: GPL-2.0
// C dependencies: "spark.h", <limits.h>, <linux/kernel.h>

use core::ffi::{c_char, c_int, c_ulong};

const SPARK_SHIFT: c_int = 8;
const NUM_SPARKS: usize = 8;

unsafe extern "C" {
    fn scnprintf(s: *mut c_char, size: c_ulong, format: *const c_char, ...) -> c_int;
}

/* Print spark lines on outf for numval values in val. */
#[no_mangle]
pub unsafe extern "C" fn print_spark(
    bf: *mut c_char,
    size: c_int,
    val: *mut c_ulong,
    numval: c_int,
) -> c_int {
    static TICKS: [*const c_char; NUM_SPARKS] = [
        b"\xe2\x96\x81\0".as_ptr() as *const c_char,
        b"\xe2\x96\x82\0".as_ptr() as *const c_char,
        b"\xe2\x96\x83\0".as_ptr() as *const c_char,
        b"\xe2\x96\x84\0".as_ptr() as *const c_char,
        b"\xe2\x96\x85\0".as_ptr() as *const c_char,
        b"\xe2\x96\x86\0".as_ptr() as *const c_char,
        b"\xe2\x96\x87\0".as_ptr() as *const c_char,
        b"\xe2\x96\x88\0".as_ptr() as *const c_char,
    ];
    let mut i: c_int;
    let mut printed: c_int = 0;
    let mut min: c_ulong = c_ulong::MAX;
    let mut max: c_ulong = 0;
    let mut f: c_ulong;

    i = 0;
    while i < numval {
        let current = unsafe { *val.offset(i as isize) };
        if current < min {
            min = current;
        }
        if current > max {
            max = current;
        }
        i += 1;
    }

    f = (max.wrapping_sub(min) << SPARK_SHIFT) / ((NUM_SPARKS - 1) as c_ulong);
    if f < 1 {
        f = 1;
    }

    i = 0;
    while i < numval {
        let tick = (((unsafe { *val.offset(i as isize) }).wrapping_sub(min)) << SPARK_SHIFT) / f;
        printed += unsafe {
            scnprintf(
                bf.offset(printed as isize),
                size.wrapping_sub(printed) as c_ulong,
                b"%s\0".as_ptr() as *const c_char,
                TICKS[tick as usize],
            )
        };
        i += 1;
    }

    printed
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
