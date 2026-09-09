// SPDX-License-Identifier: GPL-2.0
/*
 * Smp timebase synchronization for ppc.
 *
 * Copyright (C) 2003 Samuel Rydh (samuel@ibrium.se)
 */

// Linux and architecture-specific headers provide the declarations used below.

const NUM_ITER: i32 = 300;

const K_EXIT: i32 = 0;
const K_SET_AND_TEST: i32 = 1;
const K_TEST: i32 = 2;

#[repr(C)]
struct TbSync {
    tb: u64,
    mark: u64,
    cmd: i32,
    handshake: i32,
    filler: [i32; 2],
    ack: i32,
    filler2: [i32; 7],
    race_result: i32,
}

static mut TBSYNC: *mut TbSync = core::ptr::null_mut();
static mut RUNNING: i32 = 0;

// Supplied by the kernel/architecture support.
extern "C" {
    fn get_tb() -> u64;
    fn set_tb(upper: u32, lower: u32);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn local_irq_disable();
    fn local_irq_enable();
    fn barrier();
    fn rmb();
    fn mb();
    fn wmb();
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
    fn abs(value: i32) -> i32;
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
}

unsafe fn enter_contest(mark: u64, add: i64) {
    while get_tb() < mark {
        (*TBSYNC).race_result = add as i32;
    }
}

#[no_mangle]
pub unsafe extern "C" fn smp_generic_take_timebase() {
    let mut cmd: i32;
    let mut tb: u64;
    let mut flags: usize = 0;

    local_irq_save(&mut flags);
    while RUNNING == 0 {
        barrier();
    }
    rmb();

    loop {
        (*TBSYNC).ack = 1;
        while (*TBSYNC).handshake == 0 {
            barrier();
        }
        rmb();

        cmd = (*TBSYNC).cmd;
        tb = (*TBSYNC).tb;
        mb();
        (*TBSYNC).ack = 0;
        if cmd == K_EXIT {
            break;
        }

        while (*TBSYNC).handshake != 0 {
            barrier();
        }
        if cmd == K_SET_AND_TEST {
            set_tb((tb >> 32) as u32, (tb & 0xffff_ffff) as u32);
        }
        enter_contest((*TBSYNC).mark, -1);
    }
    local_irq_restore(flags);
}

unsafe fn start_contest(cmd: i32, offset: i64, num: i32) -> i32 {
    let mut i: i32;
    let mut score: i32 = 0;
    let mut tb: u64;
    let mut mark: u64;

    (*TBSYNC).cmd = cmd;

    local_irq_disable();
    i = -3;
    while i < num {
        tb = get_tb().wrapping_add(400);
        (*TBSYNC).tb = tb.wrapping_add(offset as u64);
        mark = tb.wrapping_add(400);
        (*TBSYNC).mark = mark;

        wmb();

        (*TBSYNC).handshake = 1;
        while (*TBSYNC).ack != 0 {
            barrier();
        }

        while get_tb() <= tb {
            barrier();
        }
        (*TBSYNC).handshake = 0;
        enter_contest(mark, 1);

        while (*TBSYNC).ack == 0 {
            barrier();
        }

        if i > 0 {
            score += (*TBSYNC).race_result;
        }
        i += 1;
    }
    local_irq_enable();
    score
}

#[no_mangle]
pub unsafe extern "C" fn smp_generic_give_timebase() {
    let mut i: i32;
    let mut score: i32;
    let mut score2: i32;
    let mut old: i32;
    let mut min: i32 = 0;
    let mut max: i32 = 5000;
    let mut offset: i32 = 1000;

    pr_debug(b"Software timebase sync\0".as_ptr() as *const core::ffi::c_char);

    // If this fails then this kernel won't work anyway...
    TBSYNC = kzalloc_obj::<TbSync>();
    mb();
    RUNNING = 1;

    while (*TBSYNC).ack == 0 {
        barrier();
    }

    pr_debug(b"Got ack\0".as_ptr() as *const core::ffi::c_char);

    // Binary search.
    old = -1;
    while old != offset {
        score = start_contest(K_SET_AND_TEST, offset as i64, NUM_ITER);

        if score > 0 {
            max = offset;
        } else {
            min = offset;
        }
        old = offset;
        offset = (min + max) / 2;
    }
    score = start_contest(K_SET_AND_TEST, min as i64, NUM_ITER);
    score2 = start_contest(K_SET_AND_TEST, max as i64, NUM_ITER);

    score = abs(score);
    score2 = abs(score2);
    offset = if score < score2 { min } else { max };

    // Guard against inaccurate mttb.
    i = 0;
    while i < 10 {
        start_contest(K_SET_AND_TEST, offset as i64, NUM_ITER / 10);

        score2 = start_contest(K_TEST, offset as i64, NUM_ITER);
        if score2 < 0 {
            score2 = -score2;
        }
        if score2 <= score || score2 < 20 {
            break;
        }
        i += 1;
    }

    // Exiting.
    (*TBSYNC).cmd = K_EXIT;
    wmb();
    (*TBSYNC).handshake = 1;
    while (*TBSYNC).ack != 0 {
        barrier();
    }
    (*TBSYNC).handshake = 0;
    kfree(TBSYNC as *mut core::ffi::c_void);
    TBSYNC = core::ptr::null_mut();
    RUNNING = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
