// SPDX-License-Identifier: GPL-2.0
/*
 * SDK7786 FPGA NMI Support.
 *
 * Copyright (C) 2010  Paul Mundt
 */

use core::ffi::{c_char, c_int, c_uint};

// Supplied by the kernel and mach/fpga dependencies.
extern "C" {
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
    fn pr_warn(fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
    fn fpga_read_reg(reg: c_uint) -> c_uint;
    fn fpga_write_reg(value: c_uint, reg: c_uint);
}

// Supplied by mach/fpga.
extern "C" {
    static NMISR: c_uint;
    static NMIMR: c_uint;
    static NMISR_MASK: c_uint;
    static NMIMR_MASK: c_uint;
    static NMISR_MAN_NMI: c_uint;
    static NMIMR_MAN_NMIM: c_uint;
    static NMISR_AUX_NMI: c_uint;
    static NMIMR_AUX_NMIM: c_uint;
}

const NMI_MODE_MANUAL: c_uint = 0;
const NMI_MODE_AUX: c_uint = 1;
const NMI_MODE_MASKED: c_uint = 2;
const NMI_MODE_ANY: c_uint = 3;
const NMI_MODE_UNKNOWN: c_uint = 4;

/*
 * Default to the manual NMI switch.
 */
static mut nmi_mode: c_uint = NMI_MODE_ANY;

unsafe extern "C" fn nmi_mode_setup(str_: *mut c_char) -> c_int {
    if str_.is_null() {
        return 0;
    }

    if strcmp(str_, b"manual\0".as_ptr() as *const c_char) == 0 {
        nmi_mode = NMI_MODE_MANUAL;
    } else if strcmp(str_, b"aux\0".as_ptr() as *const c_char) == 0 {
        nmi_mode = NMI_MODE_AUX;
    } else if strcmp(str_, b"masked\0".as_ptr() as *const c_char) == 0 {
        nmi_mode = NMI_MODE_MASKED;
    } else if strcmp(str_, b"any\0".as_ptr() as *const c_char) == 0 {
        nmi_mode = NMI_MODE_ANY;
    } else {
        nmi_mode = NMI_MODE_UNKNOWN;
        pr_warn(b"Unknown NMI mode %s\n\0".as_ptr() as *const c_char, str_);
    }

    printk(
        b"Set NMI mode to %d\n\0".as_ptr() as *const c_char,
        nmi_mode,
    );
    0
}

// early_param("nmi_mode", nmi_mode_setup);

pub unsafe extern "C" fn sdk7786_nmi_init() {
    let (source, mask): (c_uint, c_uint);

    match nmi_mode {
        NMI_MODE_MANUAL => {
            source = NMISR_MAN_NMI;
            mask = NMIMR_MAN_NMIM;
        }
        NMI_MODE_AUX => {
            source = NMISR_AUX_NMI;
            mask = NMIMR_AUX_NMIM;
        }
        NMI_MODE_ANY => {
            source = NMISR_MAN_NMI | NMISR_AUX_NMI;
            mask = NMIMR_MAN_NMIM | NMIMR_AUX_NMIM;
        }
        NMI_MODE_MASKED | NMI_MODE_UNKNOWN => {
            source = 0;
            mask = 0;
        }
        _ => {
            source = 0;
            mask = 0;
        }
    }

    /* Set the NMI source */
    let mut tmp = fpga_read_reg(NMISR);
    tmp &= !NMISR_MASK;
    tmp |= source;
    fpga_write_reg(tmp, NMISR);

    /* And the IRQ masking */
    fpga_write_reg(NMIMR_MASK ^ mask, NMIMR);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
