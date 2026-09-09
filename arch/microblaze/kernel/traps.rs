/*
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn __enable_hw_exceptions();
    fn kstrtoul(s: *mut core::ffi::c_char, base: u32, result: *mut usize) -> i32;
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn print_hex_dump(
        level: *const core::ffi::c_char,
        prefix_str: *const core::ffi::c_char,
        prefix_type: i32,
        rowsize: i32,
        groupsize: i32,
        buf: *const core::ffi::c_void,
        len: usize,
        ascii: i32,
        ...
    );
    fn microblaze_unwind(
        task: *mut task_struct,
        stack: *mut core::ffi::c_void,
        loglvl: *const core::ffi::c_char,
    );
    fn debug_show_held_locks(task: *mut task_struct);
    static mut current: *mut task_struct;
}

#[repr(C)]
pub struct task_struct {
    _opaque: [u8; 0],
}

#[repr(C)]
struct thread_info {
    _opaque: [u8; 0],
}

const THREAD_SIZE: usize = 0; // Supplied by the target architecture.
const KERN_INFO: *const core::ffi::c_char = c"".as_ptr();
const DUMP_PREFIX_ADDRESS: i32 = 0;

pub unsafe fn trap_init() {
    __enable_hw_exceptions();
}

static mut kstack_depth_to_print: usize = 0; /* 0 == entire stack */

unsafe extern "C" fn kstack_setup(s: *mut core::ffi::c_char) -> i32 {
    (!kstrtoul(s, 0, &mut kstack_depth_to_print)).into()
}

// __setup("kstack=", kstack_setup);

pub unsafe fn show_stack(
    task: *mut task_struct,
    sp: *mut usize,
    loglvl: *const core::ffi::c_char,
) {
    let mut words_to_show: usize;
    let mut fp = sp as usize as u32;

    if fp == 0 {
        if !task.is_null() {
            // ((struct thread_info *)(task->stack))->cpu_context.r1
            fp = (*(task as *mut *mut thread_info) as *mut u32).read();
        } else {
            /* Pick up caller of dump_stack() */
            fp = (&sp as *const *mut usize as usize - 8) as u32;
        }
    }

    words_to_show = (THREAD_SIZE - (fp as usize & (THREAD_SIZE - 1))) >> 2;
    if kstack_depth_to_print != 0 && words_to_show > kstack_depth_to_print {
        words_to_show = kstack_depth_to_print;
    }

    printk(c"%sKernel Stack:\n".as_ptr(), loglvl);

    /*
     * Make the first line an 'odd' size if necessary to get
     * remaining lines to start at an address multiple of 0x10
     */
    if fp & 0xF != 0 {
        let line1_words = (0x10 - (fp & 0xF)) >> 2;
        if line1_words < words_to_show {
            print_hex_dump(
                KERN_INFO,
                c"".as_ptr(),
                DUMP_PREFIX_ADDRESS,
                32,
                4,
                fp as usize as *const core::ffi::c_void,
                (line1_words << 2) as usize,
                0,
            );
            fp = fp.wrapping_add(line1_words << 2);
            words_to_show -= line1_words;
        }
    }
    print_hex_dump(
        loglvl,
        c"".as_ptr(),
        DUMP_PREFIX_ADDRESS,
        32,
        4,
        fp as usize as *const core::ffi::c_void,
        words_to_show << 2,
        0,
    );
    printk(c"%s\n\nCall Trace:\n".as_ptr(), loglvl);
    microblaze_unwind(task, core::ptr::null_mut(), loglvl);
    printk(c"%s\n".as_ptr(), loglvl);

    if task.is_null() {
        task = current;
    }

    debug_show_held_locks(task);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
