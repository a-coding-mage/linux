// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_uint, c_ulong};

const PAGE_SHIFT: c_uint = 12;
const ENOSPC: c_int = 28;
const EINVAL: c_int = 22;
const MSR_IDT_MCR0: c_uint = 0x2ff;
const MTRR_TYPE_WRCOMB: c_uint = 1;
const MTRR_TYPE_UNCACHABLE: c_uint = 2;
const MTRR_TYPE_WRBACK: c_uint = 6;

type MtrrType = c_uint;

#[repr(C)]
pub union Msr {
    pub q: u64,
    pub h: c_ulong,
    pub l: c_ulong,
}

#[repr(C)]
struct CentaurMcr {
    high: c_ulong,
    low: c_ulong,
}

#[repr(C)]
pub struct MtrrOps {
    pub var_regs: c_int,
    pub set: Option<unsafe extern "C" fn(c_uint, c_ulong, c_ulong, MtrrType)>,
    pub get: Option<unsafe extern "C" fn(c_uint, *mut c_ulong, *mut c_ulong, *mut MtrrType)>,
    pub get_free_region: Option<unsafe extern "C" fn(c_ulong, c_ulong, c_int) -> c_int>,
    pub validate_add_page: Option<unsafe extern "C" fn(c_ulong, c_ulong, c_uint) -> c_int>,
    pub have_wrcomb: c_int,
}

extern "C" {
    static num_var_ranges: c_int;
    static mut mtrr_if: *const MtrrOps;
    static positive_have_wrcomb: c_int;

    fn wrmsrq(msr: c_uint, value: u64);
    fn pr_warn(format: *const c_char, ...);
}

static mut centaur_mcr: [CentaurMcr; 8] = [
    CentaurMcr { high: 0, low: 0 },
    CentaurMcr { high: 0, low: 0 },
    CentaurMcr { high: 0, low: 0 },
    CentaurMcr { high: 0, low: 0 },
    CentaurMcr { high: 0, low: 0 },
    CentaurMcr { high: 0, low: 0 },
    CentaurMcr { high: 0, low: 0 },
    CentaurMcr { high: 0, low: 0 },
];

static mut centaur_mcr_reserved: u8 = 0;
static mut centaur_mcr_type: u8 = 0; // 0 for winchip, 1 for winchip2

/// centaur_get_free_region - Get a free MTRR.
///
/// `base`: The starting (base) address of the region.
/// `size`: The size (in bytes) of the region.
///
/// Returns: the index of the region on success, else -1 on error.
unsafe extern "C" fn centaur_get_free_region(
    _base: c_ulong,
    _size: c_ulong,
    replace_reg: c_int,
) -> c_int {
    let max = num_var_ranges;
    if replace_reg >= 0 && replace_reg < max {
        return replace_reg;
    }

    for i in 0..max {
        if centaur_mcr_reserved & (1u8 << i) != 0 {
            continue;
        }
        let mut lbase: c_ulong = 0;
        let mut lsize: c_ulong = 0;
        let mut ltype: MtrrType = 0;
        ((*mtrr_if).get.unwrap())(i as c_uint, &mut lbase, &mut lsize, &mut ltype);
        if lsize == 0 {
            return i;
        }
    }

    -ENOSPC
}

unsafe extern "C" fn centaur_get_mcr(
    reg: c_uint,
    base: *mut c_ulong,
    size: *mut c_ulong,
    ty: *mut MtrrType,
) {
    *base = centaur_mcr[reg as usize].high >> PAGE_SHIFT;
    *size = (-(centaur_mcr[reg as usize].low & 0xfffff000) as c_ulong) >> PAGE_SHIFT;
    *ty = MTRR_TYPE_WRCOMB;

    if centaur_mcr_type == 1 && (centaur_mcr[reg as usize].low & 31) & 2 != 0 {
        *ty = MTRR_TYPE_UNCACHABLE;
    }
    if centaur_mcr_type == 1 && (centaur_mcr[reg as usize].low & 31) == 25 {
        *ty = MTRR_TYPE_WRBACK;
    }
    if centaur_mcr_type == 0 && (centaur_mcr[reg as usize].low & 31) == 31 {
        *ty = MTRR_TYPE_WRBACK;
    }
}

unsafe extern "C" fn centaur_set_mcr(
    reg: c_uint,
    base: c_ulong,
    size: c_ulong,
    ty: MtrrType,
) {
    let mut val = Msr { q: 0 };

    if size == 0 {
        // Disable
        val = Msr { q: 0 };
    } else {
        val.h = base << PAGE_SHIFT;
        if centaur_mcr_type == 0 {
            // Only support write-combining...
            val.l = ((-(size as c_ulong)) << PAGE_SHIFT) | 0x1f;
        } else if ty == MTRR_TYPE_UNCACHABLE {
            val.l = ((-(size as c_ulong)) << PAGE_SHIFT) | 0x02; // NC
        } else {
            val.l = ((-(size as c_ulong)) << PAGE_SHIFT) | 0x09; // WWO, WC
        }
    }
    centaur_mcr[reg as usize].high = val.h;
    centaur_mcr[reg as usize].low = val.l;
    wrmsrq(MSR_IDT_MCR0 + reg, val.q);
}

unsafe extern "C" fn centaur_validate_add_page(
    _base: c_ulong,
    _size: c_ulong,
    ty: c_uint,
) -> c_int {
    // FIXME: Winchip2 supports uncached
    if ty != MTRR_TYPE_WRCOMB
        && (centaur_mcr_type == 0 || ty != MTRR_TYPE_UNCACHABLE)
    {
        pr_warn(
            b"mtrr: only write-combining%s supported\0".as_ptr() as *const c_char,
            if centaur_mcr_type != 0 {
                b" and uncacheable are\0".as_ptr() as *const c_char
            } else {
                b" is\0".as_ptr() as *const c_char
            },
        );
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub static centaur_mtrr_ops: MtrrOps = MtrrOps {
    var_regs: 8,
    set: Some(centaur_set_mcr),
    get: Some(centaur_get_mcr),
    get_free_region: Some(centaur_get_free_region),
    validate_add_page: Some(centaur_validate_add_page),
    have_wrcomb: unsafe { positive_have_wrcomb },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
