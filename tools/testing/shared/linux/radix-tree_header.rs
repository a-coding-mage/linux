/* SPDX-License-Identifier: GPL-2.0 */

/* Rust translation of testing/shared/linux/radix-tree.h. */
/* C include dependency preserved for the surrounding translation unit:
 * ../../../../include/linux/radix-tree.h
 */

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    pub static mut kmalloc_verbose: c_int;
    pub static mut test_verbose: c_int;

    pub fn printf(fmt: *const c_char, ...) -> c_int;
    pub fn call_rcu(head: *mut rcu_head, func: Option<unsafe extern "C" fn(head: *mut rcu_head)>);
}

#[inline]
pub unsafe fn trace_call_rcu(
    head: *mut rcu_head,
    func: Option<unsafe extern "C" fn(head: *mut rcu_head)>,
) {
    if unsafe { kmalloc_verbose } != 0 {
        unsafe {
            printf(
                c"Delaying free of %p to slab\n".as_ptr(),
                (head as *mut c_char).offset(-(core::mem::offset_of!(radix_tree_node, rcu_head) as isize)),
            );
        }
    }
    unsafe {
        call_rcu(head, func);
    }
}

#[macro_export]
macro_rules! printv {
    ($verbosity_level:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        if unsafe { test_verbose } >= $verbosity_level {
            unsafe {
                printf($fmt $(, $arg)*);
            }
        }
    };
}

/* C had:
 *   #undef call_rcu
 *   #define call_rcu(x, y) trace_call_rcu(x, y)
 */
#[macro_export]
macro_rules! call_rcu {
    ($x:expr, $y:expr $(,)?) => {
        trace_call_rcu($x, $y)
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
