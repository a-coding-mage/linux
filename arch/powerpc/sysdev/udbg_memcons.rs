// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * A udbg backend which logs messages and reads input from in memory
 * buffers.
 *
 * The console output can be read from memcons_output which is a
 * circular buffer whose next write position is stored in memcons.output_pos.
 *
 * Input may be passed by writing into the memcons_input buffer when it is
 * empty. The input buffer is empty when both input_pos == input_start and
 * *input_start == '\0'.
 *
 * Copyright (C) 2003-2005 Anton Blanchard and Milton Miller, IBM Corp
 * Copyright (C) 2013 Alistair Popple, IBM Corp
 */

use core::ffi::c_char;

#[repr(C)]
pub struct memcons {
    pub output_start: *mut c_char,
    pub output_pos: *mut c_char,
    pub output_end: *mut c_char,
    pub input_start: *mut c_char,
    pub input_pos: *mut c_char,
    pub input_end: *mut c_char,
}

// CONFIG_PPC_MEMCONS_* are supplied by the build configuration.
static mut memcons_output: [c_char; CONFIG_PPC_MEMCONS_OUTPUT_SIZE] =
    [0; CONFIG_PPC_MEMCONS_OUTPUT_SIZE];
static mut memcons_input: [c_char; CONFIG_PPC_MEMCONS_INPUT_SIZE] =
    [0; CONFIG_PPC_MEMCONS_INPUT_SIZE];

pub static mut memcons: memcons = memcons {
    output_start: unsafe { memcons_output.as_mut_ptr() },
    output_pos: unsafe { memcons_output.as_mut_ptr() },
    output_end: unsafe { memcons_output.as_mut_ptr().add(CONFIG_PPC_MEMCONS_OUTPUT_SIZE) },
    input_start: unsafe { memcons_input.as_mut_ptr() },
    input_pos: unsafe { memcons_input.as_mut_ptr() },
    input_end: unsafe { memcons_input.as_mut_ptr().add(CONFIG_PPC_MEMCONS_INPUT_SIZE) },
};

extern "C" {
    fn wmb();
    fn cpu_relax();
    static mut udbg_putc: Option<unsafe extern "C" fn(c_char)>;
    static mut udbg_getc: Option<unsafe extern "C" fn() -> i32>;
    static mut udbg_getc_poll: Option<unsafe extern "C" fn() -> i32>;
}

unsafe fn memcons_putc(c: c_char) {
    let mut new_output_pos: *mut c_char;

    *memcons.output_pos = c;
    wmb();
    new_output_pos = memcons.output_pos.add(1);
    if new_output_pos >= memcons.output_end {
        new_output_pos = memcons.output_start;
    }

    memcons.output_pos = new_output_pos;
}

unsafe fn memcons_getc_poll() -> i32 {
    let c: c_char;
    let mut new_input_pos: *mut c_char;

    if *memcons.input_pos != 0 {
        c = *memcons.input_pos;

        new_input_pos = memcons.input_pos.add(1);
        if new_input_pos >= memcons.input_end {
            new_input_pos = memcons.input_start;
        } else if *new_input_pos == 0 {
            new_input_pos = memcons.input_start;
        }

        *memcons.input_pos = 0;
        wmb();
        memcons.input_pos = new_input_pos;
        return c as i32;
    }

    -1
}

unsafe fn memcons_getc() -> i32 {
    let mut c: i32;

    loop {
        c = memcons_getc_poll();
        if c == -1 {
            cpu_relax();
        } else {
            break;
        }
    }

    c
}

// __init
pub unsafe extern "C" fn udbg_init_memcons() {
    udbg_putc = Some(memcons_putc);
    udbg_getc = Some(memcons_getc);
    udbg_getc_poll = Some(memcons_getc_poll);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
