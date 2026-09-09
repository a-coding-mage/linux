// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000, 2001, 2002, 2003 Broadcom Corporation
 */

// Dependencies supplied by the surrounding kernel translation.

pub static mut sb1_pass: u32 = 0;
pub static mut soc_pass: u32 = 0;
pub static mut soc_type: u32 = 0;
pub static mut periph_rev: u32 = 0;
pub static mut zbbus_mhz: u32 = 0;

static mut soc_str: *mut core::ffi::c_char = core::ptr::null_mut();
static mut pass_str: *mut core::ffi::c_char = core::ptr::null_mut();
static mut war_pass: u32 = 0; // XXXKW don't overload PASS defines?

unsafe extern "C" {
    fn printk(fmt: *const core::ffi::c_char, ...) -> i32;
    fn machine_restart(cmd: *const core::ffi::c_char) -> !;
    fn read_c0_prid() -> u32;
    fn __raw_readq(addr: usize) -> u64;
    fn get_system_type() -> *const core::ffi::c_char;
}

unsafe fn setup_bcm1250() -> i32 {
    let mut ret = 0;

    match soc_pass {
        K_SYS_REVISION_BCM1250_PASS1 => {
            periph_rev = 1;
            pass_str = b"Pass 1\0".as_ptr() as *mut _;
        }
        K_SYS_REVISION_BCM1250_A10 => {
            periph_rev = 2;
            pass_str = b"A8/A10\0".as_ptr() as *mut _;
            // XXXKW different war_pass?
            war_pass = K_SYS_REVISION_BCM1250_PASS2;
        }
        K_SYS_REVISION_BCM1250_PASS2_2 => {
            periph_rev = 2;
            pass_str = b"B1\0".as_ptr() as *mut _;
        }
        K_SYS_REVISION_BCM1250_B2 => {
            periph_rev = 2;
            pass_str = b"B2\0".as_ptr() as *mut _;
            war_pass = K_SYS_REVISION_BCM1250_PASS2_2;
        }
        K_SYS_REVISION_BCM1250_PASS3 => {
            periph_rev = 3;
            pass_str = b"C0\0".as_ptr() as *mut _;
        }
        K_SYS_REVISION_BCM1250_C1 => {
            periph_rev = 3;
            pass_str = b"C1\0".as_ptr() as *mut _;
        }
        _ => {
            if soc_pass < K_SYS_REVISION_BCM1250_PASS2_2 {
                periph_rev = 2;
                pass_str = b"A0-A6\0".as_ptr() as *mut _;
                war_pass = K_SYS_REVISION_BCM1250_PASS2;
            } else {
                printk(b"Unknown BCM1250 rev %x\n\0".as_ptr() as *const _ , soc_pass);
                ret = 1;
            }
        }
    }
    ret
}

pub unsafe fn sb1250_m3_workaround_needed() -> i32 {
    match soc_type {
        K_SYS_SOC_TYPE_BCM1250 | K_SYS_SOC_TYPE_BCM1250_ALT | K_SYS_SOC_TYPE_BCM1250_ALT2
        | K_SYS_SOC_TYPE_BCM1125 | K_SYS_SOC_TYPE_BCM1125H => {
            (soc_pass < K_SYS_REVISION_BCM1250_C0) as i32
        }
        _ => 0,
    }
}

unsafe fn setup_bcm112x() -> i32 {
    let mut ret = 0;
    match soc_pass {
        0 => {
            // Early build didn't have revid set
            periph_rev = 3;
            pass_str = b"A1\0".as_ptr() as *mut _;
            war_pass = K_SYS_REVISION_BCM112x_A1;
        }
        K_SYS_REVISION_BCM112x_A1 => { periph_rev = 3; pass_str = b"A1\0".as_ptr() as *mut _; }
        K_SYS_REVISION_BCM112x_A2 => { periph_rev = 3; pass_str = b"A2\0".as_ptr() as *mut _; }
        K_SYS_REVISION_BCM112x_A3 => { periph_rev = 3; pass_str = b"A3\0".as_ptr() as *mut _; }
        K_SYS_REVISION_BCM112x_A4 => { periph_rev = 3; pass_str = b"A4\0".as_ptr() as *mut _; }
        K_SYS_REVISION_BCM112x_B0 => { periph_rev = 3; pass_str = b"B0\0".as_ptr() as *mut _; }
        _ => {
            printk(b"Unknown %s rev %x\n\0".as_ptr() as *const _, soc_str, soc_pass);
            ret = 1;
        }
    }
    ret
}

unsafe fn sys_rev_decode() -> i32 {
    war_pass = soc_pass;
    match soc_type {
        K_SYS_SOC_TYPE_BCM1250 | K_SYS_SOC_TYPE_BCM1250_ALT | K_SYS_SOC_TYPE_BCM1250_ALT2 => {
            soc_str = b"BCM1250\0".as_ptr() as *mut _;
            setup_bcm1250()
        }
        K_SYS_SOC_TYPE_BCM1120 => { soc_str = b"BCM1120\0".as_ptr() as *mut _; setup_bcm112x() }
        K_SYS_SOC_TYPE_BCM1125 => { soc_str = b"BCM1125\0".as_ptr() as *mut _; setup_bcm112x() }
        K_SYS_SOC_TYPE_BCM1125H => { soc_str = b"BCM1125H\0".as_ptr() as *mut _; setup_bcm112x() }
        _ => { printk(b"Unknown SOC type %x\n\0".as_ptr() as *const _, soc_type); 1 }
    }
}

pub unsafe fn sb1250_setup() {
    let sys_rev: u64;
    let plldiv: i32;
    let mut bad_config = 0;

    sb1_pass = read_c0_prid() & PRID_REV_MASK;
    sys_rev = __raw_readq(IOADDR(A_SCD_SYSTEM_REVISION));
    soc_type = SYS_SOC_TYPE(sys_rev);
    soc_pass = G_SYS_REVISION(sys_rev);

    if sys_rev_decode() != 0 {
        printk(b"Restart after failure to identify SiByte chip\n\0".as_ptr() as *const _);
        machine_restart(core::ptr::null());
    }

    plldiv = G_SYS_PLL_DIV(__raw_readq(IOADDR(A_SCD_SYSTEM_CFG))) as i32;
    zbbus_mhz = ((plldiv >> 1) * 50 + (plldiv & 1) * 25) as u32;
    printk(b"Broadcom SiByte %s %s @ %d MHz (SB1 rev %d)\n\0".as_ptr() as *const _, soc_str, pass_str, zbbus_mhz * 2, sb1_pass);
    printk(b"Board type: %s\n\0".as_ptr() as *const _, get_system_type());

    match war_pass {
        K_SYS_REVISION_BCM1250_PASS1 => { printk(b"@@@@ This is a BCM1250 A0-A2 (Pass 1) board, and the kernel doesn't have the proper workarounds compiled in. @@@@\n\0".as_ptr() as *const _); bad_config = 1; }
        K_SYS_REVISION_BCM1250_PASS2 => {
            // Pass 2 - easiest as default for now - so many numbers
            // Build-time configuration conditions are preserved by these comments.
            #[cfg(any())]
            { printk(b"@@@@ This is a BCM1250 A3-A10 board, and the kernel doesn't have the proper workarounds compiled in. @@@@\n\0".as_ptr() as *const _); bad_config = 1; }
            #[cfg(any())]
            { printk(b"@@@@ Prefetches may be enabled in this kernel, but are buggy on this board.  @@@@\n\0".as_ptr() as *const _); bad_config = 1; }
        }
        K_SYS_REVISION_BCM1250_PASS2_2 => {
            #[cfg(any())]
            { printk(b"@@@@ This is a BCM1250 B1/B2. board, and the kernel doesn't have the proper workarounds compiled in. @@@@\n\0".as_ptr() as *const _); bad_config = 1; }
            #[cfg(any())]
            { printk(b"@@@@ This is a BCM1250 B1/B2, but the kernel is conservatively configured for an 'A' stepping. @@@@\n\0".as_ptr() as *const _); }
        }
        _ => {}
    }
    if bad_config != 0 {
        printk(b"Invalid configuration for this chip.\n\0".as_ptr() as *const _);
        machine_restart(core::ptr::null());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
