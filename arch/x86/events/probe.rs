// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

// Types and functions supplied by the surrounding kernel translation.
pub type UmodeT = u16;

#[repr(C)]
pub struct Kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Attribute {
    _private: [u8; 0],
}

pub type IsVisibleFn = unsafe extern "C" fn(
    kobj: *mut Kobject,
    attr: *mut Attribute,
    i: i32,
) -> UmodeT;

#[repr(C)]
pub struct AttributeGroup {
    pub is_visible: Option<IsVisibleFn>,
}

pub type MsrTestFn = unsafe extern "C" fn(bit: u32, data: *mut c_void) -> bool;

#[repr(C)]
pub struct PerfMsr {
    pub msr: u32,
    pub grp: *mut AttributeGroup,
    pub no_check: bool,
    pub test: Option<MsrTestFn>,
    pub mask: u64,
}

unsafe extern "C" {
    fn rdmsrq_safe(msr: u32, val: *mut u64) -> i32;
}

unsafe extern "C" fn not_visible(
    _kobj: *mut Kobject,
    _attr: *mut Attribute,
    _i: i32,
) -> UmodeT {
    0
}

/*
 * Accepts msr[] array with non populated entries as long as either
 * msr[i].msr is 0 or msr[i].grp is NULL. Note that the default sysfs
 * visibility is visible when group->is_visible callback is set.
 */
pub unsafe extern "C" fn perf_msr_probe(
    msr: *mut PerfMsr,
    cnt: i32,
    zero: bool,
    data: *mut c_void,
) -> usize {
    let mut avail: usize = 0;
    let mut bit: u32;
    let mut val: u64;

    if cnt >= usize::BITS as i32 {
        return 0;
    }

    bit = 0;
    while bit < cnt as u32 {
        let entry = &mut *msr.add(bit as usize);
        if !entry.no_check {
            let grp = entry.grp;
            let mut mask: u64;

            /* skip entry with no group */
            if grp.is_null() {
                bit += 1;
                continue;
            }

            (*grp).is_visible = Some(not_visible);

            /* skip unpopulated entry */
            if entry.msr == 0 {
                bit += 1;
                continue;
            }

            if let Some(test) = entry.test {
                if !test(bit, data) {
                    bit += 1;
                    continue;
                }
            }
            /* Virt sucks; you cannot tell if a R/O MSR is present :/ */
            if rdmsrq_safe(entry.msr, &mut val) != 0 {
                bit += 1;
                continue;
            }

            mask = entry.mask;
            if mask == 0 {
                mask = !0u64;
            }
            /* Disable zero counters if requested. */
            if !zero && (val & mask) == 0 {
                bit += 1;
                continue;
            }

            (*grp).is_visible = None;
        }
        avail |= 1usize << bit;
        bit += 1;
    }

    avail
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
