// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2012 Thomas Langer <thomas.langer@lantiq.com>
 * Copyright (C) 2012 John Crispin <john@phrozen.org>
 */

use core::ffi::{c_char, c_void};

// Dependencies supplied by the surrounding kernel code.
extern "C" {
    static mut FALCON_CHIPID: usize;
    static mut FALCON_CHIPCONF: usize;
    static mut FALCON_CHIPTYPE: usize;
    static mut KSEG1: usize;
    static mut SOC_ID_FALCON: u32;
    static mut SOC_TYPE_FALCON: u32;

    fn ltq_r32(addr: usize) -> u32;
    fn ltq_w32(value: usize, addr: *mut c_void);
    fn unreachable() -> !;

    static mut board_nmi_handler_setup: Option<unsafe extern "C" fn()>;
    static mut board_ejtag_handler_setup: Option<unsafe extern "C" fn()>;
    static mut nmi_handler: unsafe extern "C" fn();
    static mut ejtag_debug_handler: unsafe extern "C" fn();
}

const SOC_FALCON: &str = "Falcon";
const SOC_FALCON_D: &str = "Falcon-D";
const SOC_FALCON_V: &str = "Falcon-V";
const SOC_FALCON_M: &str = "Falcon-M";

const COMP_FALCON: &str = "lantiq,falcon";

const PART_SHIFT: u32 = 12;
const PART_MASK: u32 = 0x0FFFF000;
const REV_SHIFT: u32 = 28;
const REV_MASK: u32 = 0xF0000000;
const SREV_SHIFT: u32 = 22;
const SREV_MASK: u32 = 0x03C00000;
const TYPE_SHIFT: u32 = 26;
const TYPE_MASK: u32 = 0x3C000000;

/* reset, nmi and ejtag exception vectors */
const BOOT_REG_BASE: usize = KSEG1 | 0x1F200000;
const BOOT_RVEC: usize = BOOT_REG_BASE | 0x00;
const BOOT_NVEC: usize = BOOT_REG_BASE | 0x04;
const BOOT_EVEC: usize = BOOT_REG_BASE | 0x08;

#[repr(C)]
pub struct ltq_soc_info {
    pub partnum: u32,
    pub rev: u32,
    pub srev: u32,
    pub compatible: *const c_char,
    pub type_: u32,
    pub rev_type: [c_char; 8],
    pub name: *const c_char,
}

unsafe extern "C" fn ltq_soc_nmi_setup() {
    ltq_w32((&raw const nmi_handler) as usize, BOOT_NVEC as *mut c_void);
}

unsafe extern "C" fn ltq_soc_ejtag_setup() {
    ltq_w32((&raw const ejtag_debug_handler) as usize, BOOT_EVEC as *mut c_void);
}

pub unsafe extern "C" fn ltq_soc_detect(i: *mut ltq_soc_info) {
    let mut type_: u32;

    (*i).partnum = (ltq_r32(FALCON_CHIPID) & PART_MASK) >> PART_SHIFT;
    (*i).rev = (ltq_r32(FALCON_CHIPID) & REV_MASK) >> REV_SHIFT;
    (*i).srev = (ltq_r32(FALCON_CHIPCONF) & SREV_MASK) >> SREV_SHIFT;
    (*i).compatible = COMP_FALCON.as_ptr() as *const c_char;
    (*i).type_ = SOC_TYPE_FALCON;

    let rev_type = [
        if ((*i).srev & 0x4) != 0 { b'B' } else { b'A' },
        b'0' + ((*i).rev & 0x7) as u8,
        b'0' + (((*i).srev & 0x3) + 1) as u8,
        0,
        0,
        0,
        0,
        0,
    ];
    (*i).rev_type = rev_type.map(|v| v as c_char);

    match (*i).partnum {
        SOC_ID_FALCON => {
            type_ = (ltq_r32(FALCON_CHIPTYPE) & TYPE_MASK) >> TYPE_SHIFT;
            (*i).name = match type_ {
                0 => SOC_FALCON_D.as_ptr() as *const c_char,
                1 => SOC_FALCON_V.as_ptr() as *const c_char,
                2 => SOC_FALCON_M.as_ptr() as *const c_char,
                _ => SOC_FALCON.as_ptr() as *const c_char,
            };
        }
        _ => {
            unreachable();
        }
    }

    board_nmi_handler_setup = Some(ltq_soc_nmi_setup);
    board_ejtag_handler_setup = Some(ltq_soc_ejtag_setup);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
