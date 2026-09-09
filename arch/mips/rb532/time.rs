// SPDX-License-Identifier: GPL-2.0-only
/*
 * Carsten Langgaard, carstenl@mips.com
 * Copyright (C) 1999,2000 MIPS Technologies, Inc.  All rights reserved.
 *
 *  Setting up the clock on the MIPS boards.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong};

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static mut idt_cpu_freq: c_uint;
    static mut mips_hpt_frequency: c_ulong;
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn printk(fmt: *const c_char, ...) -> c_int;
}

// IDT_CLOCK_MULT and HZ are supplied by the corresponding kernel headers.

/*
 * Figure out the r4k offset, the amount to increment the compare
 * register for each time tick. There is no RTC available.
 *
 * The RC32434 counts at half the CPU *core* speed.
 */
unsafe fn cal_r4koff() -> c_ulong {
    mips_hpt_frequency = idt_cpu_freq as c_ulong * IDT_CLOCK_MULT as c_ulong / 2;

    mips_hpt_frequency / HZ as c_ulong
}

pub unsafe fn plat_time_init() {
    let mut est_freq: c_uint;
    let mut flags: c_ulong = 0;
    let mut r4k_offset: c_ulong;

    local_irq_save(&mut flags);

    printk(b"calculating r4koff... \0".as_ptr() as *const c_char);
    r4k_offset = cal_r4koff();
    printk(
        b"%08lx(%d)\n\0".as_ptr() as *const c_char,
        r4k_offset,
        r4k_offset as c_int,
    );

    est_freq = (2 * r4k_offset * HZ as c_ulong) as c_uint;
    est_freq += 5000; /* round */
    est_freq -= est_freq % 10000;
    printk(
        b"CPU frequency %d.%02d MHz\n\0".as_ptr() as *const c_char,
        (est_freq / 1000000) as c_int,
        (((est_freq % 1000000) * 100) / 1000000) as c_int,
    );
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
