/* SPDX-License-Identifier: GPL-2.0 */

// The following declarations are supplied by the surrounding kernel
// environment; their definitions are intentionally not provided here.
extern "C" {
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn get_user(value: *mut u32, address: *const u32) -> i32;
    fn put_user(value: u32, address: *mut u32) -> i32;
}

pub unsafe fn atomic_futex_op_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    oldval: u32,
    newval: u32,
) -> i32 {
    let mut flags: usize = 0;
    let mut ret: i32;
    let mut prev: u32 = 0;

    local_irq_save(&mut flags as *mut usize);

    ret = get_user(&mut prev as *mut u32, uaddr as *const u32);
    if ret == 0 && oldval == prev {
        ret = put_user(newval, uaddr);
    }

    local_irq_restore(flags);

    *uval = prev;
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
