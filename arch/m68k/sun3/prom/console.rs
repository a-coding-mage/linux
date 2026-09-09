// SPDX-License-Identifier: GPL-2.0
/*
 * console.c: Routines that deal with sending and receiving IO
 *            to/from the current console device using the PROM.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.org)
 */

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct PromVec {
    pub pv_nbgetchar: Option<unsafe extern "C" fn() -> i32>,
    pub pv_nbputchar: Option<unsafe extern "C" fn(i8) -> i32>,
}

extern "C" {
    pub static mut romvec: *mut PromVec;
    pub fn local_irq_save(flags: *mut usize);
    pub fn local_irq_restore(flags: usize);
}

/* Non blocking get character from console input device, returns -1
 * if no input was taken.  This can be used for polling.
 */
pub unsafe fn prom_nbgetchar() -> i32 {
    let mut i: i32 = -1;
    let mut flags: usize = 0;

    local_irq_save(&mut flags as *mut usize);
    i = ((*romvec).pv_nbgetchar.unwrap())();
    local_irq_restore(flags);
    i /* Ugh, we could spin forever on unsupported proms ;( */
}

/* Non blocking put character to console device, returns -1 if
 * unsuccessful.
 */
pub unsafe fn prom_nbputchar(c: i8) -> i32 {
    let mut flags: usize = 0;
    let mut i: i32 = -1;

    local_irq_save(&mut flags as *mut usize);
    i = ((*romvec).pv_nbputchar.unwrap())(c);
    local_irq_restore(flags);
    i /* Ugh, we could spin forever on unsupported proms ;( */
}

/* Blocking version of get character routine above. */
pub unsafe fn prom_getchar() -> i8 {
    let mut character: i32;
    loop {
        character = prom_nbgetchar();
        if character != -1 {
            break;
        }
    }
    character as i8
}

/* Blocking version of put character routine above. */
pub unsafe fn prom_putchar(c: i8) {
    loop {
        if prom_nbputchar(c) != -1 {
            break;
        }
    }
    return;
}

/* Query for input device type (the original implementation is disabled
 * by #if 0 and is retained here as source documentation).
 */

/* Query for output device type (the original implementation is disabled
 * by #if 0 and is retained here as source documentation).
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
