/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Time operations for IP22 machines. Original code may come from
 * Ralf Baechle or David S. Miller (sorry guys, i'm really not sure)
 *
 * Copyright (C) 2001 by Ladislav Michl
 * Copyright (C) 2003, 06 Ralf Baechle (ralf@linux-mips.org)
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Declarations supplied by the kernel and platform headers.
extern "C" {
    static mut sgint: *mut c_void;
    static mut mips_hpt_frequency: c_ulong;

    fn read_c0_count() -> u32;
    fn readb(addr: *const u8) -> u8;
    fn writeb(value: u8, addr: *mut u8);
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn irq_enter();
    fn irq_exit();
    fn kstat_incr_irq_this_cpu(irq: c_int);
    fn ArcRead(channel: c_int, buffer: *mut c_char, length: c_int, count: *mut c_ulong);
    fn ArcEnterInteractiveMode();
    fn ip22_is_fullhouse() -> bool;
    fn setup_pit_timer();
}

// These constants are provided by the included kernel/platform headers.
extern "C" {
    static HZ: c_ulong;
    static SGINT_TCWORD_CNT2: u8;
    static SGINT_TCWORD_CALL: u8;
    static SGINT_TCWORD_MRGEN: u8;
    static SGINT_TCWORD_CLAT: u8;
    static SGINT_TCWORD_MSWST: u8;
    static SGINT_TCSAMP_COUNTER: u16;
    static SGI_8254_0_IRQ: c_int;
}

#[inline]
unsafe fn sgint_byte(offset: usize) -> *mut u8 {
    (sgint as *mut u8).add(offset)
}

unsafe fn dosample() -> c_ulong {
    let mut ct0: u32;
    let mut ct1: u32 = 0;
    let mut msb: u8;

    // Start the counter.
    core::ptr::write_volatile(sgint_byte(0), SGINT_TCWORD_CNT2 | SGINT_TCWORD_CALL | SGINT_TCWORD_MRGEN);
    core::ptr::write_volatile(sgint_byte(1), (SGINT_TCSAMP_COUNTER & 0xff) as u8);
    core::ptr::write_volatile(sgint_byte(1), (SGINT_TCSAMP_COUNTER >> 8) as u8);

    // Get initial counter invariant
    ct0 = read_c0_count();

    // Latch and spin until top byte of counter2 is zero
    loop {
        writeb(SGINT_TCWORD_CNT2 | SGINT_TCWORD_CLAT, sgint_byte(0));
        let _ = readb(sgint_byte(1));
        msb = readb(sgint_byte(1));
        ct1 = read_c0_count();
        if msb == 0 { break; }
    }

    // Stop the counter.
    writeb(SGINT_TCWORD_CNT2 | SGINT_TCWORD_CALL | SGINT_TCWORD_MSWST, sgint_byte(0));

    // Return the difference, rounded to the nearest 1 MHz of master clock.
    let divisor = 500000 / HZ;
    ((ct1.wrapping_sub(ct0) as c_ulong) / divisor) * divisor
}

#[no_mangle]
pub unsafe extern "C" fn plat_time_init() {
    let mut r4k_ticks = [0 as c_ulong; 3];
    let r4k_tick: c_ulong;

    printk(b"Calibrating system timer... \0".as_ptr() as *const c_char);
    dosample();
    dosample();
    loop {
        r4k_ticks[0] = dosample();
        if r4k_ticks[0] != 0 { break; }
    }
    loop {
        r4k_ticks[1] = dosample();
        if r4k_ticks[1] != 0 { break; }
    }

    if r4k_ticks[0] != r4k_ticks[1] {
        printk(b"warning: timer counts differ, retrying... \0".as_ptr() as *const c_char);
        r4k_ticks[2] = dosample();
        if r4k_ticks[2] == r4k_ticks[0] || r4k_ticks[2] == r4k_ticks[1] {
            r4k_tick = r4k_ticks[2];
        } else {
            printk(b"disagreement, using average... \0".as_ptr() as *const c_char);
            r4k_tick = (r4k_ticks[0] + r4k_ticks[1] + r4k_ticks[2]) / 3;
        }
    } else {
        r4k_tick = r4k_ticks[0];
    }

    let divisor = 500000 / HZ;
    printk(
        b"%d [%d.%04d MHz CPU]\n\0".as_ptr() as *const c_char,
        r4k_tick as c_int,
        (r4k_tick / divisor) as c_int,
        (r4k_tick % divisor) as c_int,
    );
    mips_hpt_frequency = r4k_tick * HZ;
    if ip22_is_fullhouse() {
        setup_pit_timer();
    }
}

// Generic SGI handler for (spurious) 8254 interrupts
#[no_mangle]
pub unsafe extern "C" fn indy_8254timer_irq() {
    let irq = SGI_8254_0_IRQ;
    let mut cnt: c_ulong = 0;
    let mut c: c_char = 0;

    irq_enter();
    kstat_incr_irq_this_cpu(irq);
    printk(b"Oops, got 8254 interrupt.\n\0".as_ptr() as *const c_char);
    ArcRead(0, &mut c, 1, &mut cnt);
    ArcEnterInteractiveMode();
    irq_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
