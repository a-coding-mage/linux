// SPDX-License-Identifier: GPL-2.0
/*
 * Reset a Jazz machine.
 *
 * We don't trust the firmware so we do it the classic way by poking and
 * stabbing at the keyboard controller ...
 */

// Dependencies supplied by the surrounding kernel translation.
use crate::jiffies::{jiffies, time_before_eq, HZ};
use crate::jazz::JazzKh;

const KBD_STAT_IBF: i32 = 0x02; /* Keyboard input buffer full */

extern "C" {
    static mut jazz_kh: *mut JazzKh;
}

unsafe fn jazz_write_output(val: u8) {
    let mut status: i32;

    loop {
        status = (*jazz_kh).command as i32;
        if status & KBD_STAT_IBF == 0 {
            break;
        }
    }
    (*jazz_kh).data = val;
}

unsafe fn jazz_write_command(val: u8) {
    let mut status: i32;

    loop {
        status = (*jazz_kh).command as i32;
        if status & KBD_STAT_IBF == 0 {
            break;
        }
    }
    (*jazz_kh).command = val;
}

unsafe fn jazz_read_status() -> u8 {
    (*jazz_kh).command
}

#[inline]
unsafe fn kb_wait() {
    let start: usize = jiffies;
    let timeout: usize = start + HZ / 2;

    loop {
        if jazz_read_status() & 0x02 == 0 {
            return;
        }
        if !time_before_eq(jiffies, timeout) {
            break;
        }
    }
}

pub unsafe fn jazz_machine_restart(_command: *mut u8) {
    loop {
        kb_wait();
        jazz_write_command(0xd1);
        kb_wait();
        jazz_write_output(0x00);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
