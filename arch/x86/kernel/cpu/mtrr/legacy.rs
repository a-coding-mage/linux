// SPDX-License-Identifier: GPL-2.0-only

// The declarations supplied by the Linux kernel headers are external Rust
// dependencies of this translation unit.

use core::ffi::c_void;

extern "C" {
    static mut boot_cpu_data: BootCpuData;
    static mut mtrr_if: *mut MtrrOps;
    static amd_mtrr_ops: MtrrOps;
    static centaur_mtrr_ops: MtrrOps;
    static cyrix_mtrr_ops: MtrrOps;
    static mut num_var_ranges: i32;

    fn cpu_feature_enabled(feature: i32) -> bool;
    fn kzalloc_objs<T>(count: i32) -> *mut T;
    fn register_syscore(syscore: *mut Syscore);
}

#[repr(C)]
struct BootCpuData {
    x86_vendor: i32,
}

#[repr(C)]
struct MtrrOps {
    get: Option<unsafe extern "C" fn(i32, *mut usize, *mut usize, *mut MtrrType)>,
    set: Option<unsafe extern "C" fn(i32, usize, usize, MtrrType)>,
}

type MtrrType = i32;

const X86_VENDOR_AMD: i32 = 1;
const X86_VENDOR_CENTAUR: i32 = 5;
const X86_VENDOR_CYRIX: i32 = 2;
const X86_FEATURE_K6_MTRR: i32 = 0;
const X86_FEATURE_CENTAUR_MCR: i32 = 1;
const X86_FEATURE_CYRIX_ARR: i32 = 2;
const ENOMEM: i32 = 12;

#[repr(C)]
struct MtrrValue {
    ltype: MtrrType,
    lbase: usize,
    lsize: usize,
}

static mut mtrr_value: *mut MtrrValue = core::ptr::null_mut();

unsafe fn mtrr_set_if() {
    match boot_cpu_data.x86_vendor {
        X86_VENDOR_AMD => {
            /* Pre-Athlon (K6) AMD CPU MTRRs */
            if cpu_feature_enabled(X86_FEATURE_K6_MTRR) {
                mtrr_if = &amd_mtrr_ops as *const MtrrOps as *mut MtrrOps;
            }
        }
        X86_VENDOR_CENTAUR => {
            if cpu_feature_enabled(X86_FEATURE_CENTAUR_MCR) {
                mtrr_if = &centaur_mtrr_ops as *const MtrrOps as *mut MtrrOps;
            }
        }
        X86_VENDOR_CYRIX => {
            if cpu_feature_enabled(X86_FEATURE_CYRIX_ARR) {
                mtrr_if = &cyrix_mtrr_ops as *const MtrrOps as *mut MtrrOps;
            }
        }
        _ => {}
    }
}

/*
 * The suspend/resume methods are only for CPUs without MTRR. CPUs using generic
 * MTRR driver don't require this.
 */

unsafe extern "C" fn mtrr_save(_data: *mut c_void) -> i32 {
    if mtrr_value.is_null() {
        return -ENOMEM;
    }

    for i in 0..num_var_ranges {
        ((*mtrr_if).get.unwrap())(
            i,
            &mut (*mtrr_value.add(i as usize)).lbase,
            &mut (*mtrr_value.add(i as usize)).lsize,
            &mut (*mtrr_value.add(i as usize)).ltype,
        );
    }
    0
}

unsafe extern "C" fn mtrr_restore(_data: *mut c_void) {
    for i in 0..num_var_ranges {
        let value = &*mtrr_value.add(i as usize);
        if value.lsize != 0 {
            ((*mtrr_if).set.unwrap())(i, value.lbase, value.lsize, value.ltype);
        }
    }
}

#[repr(C)]
struct SyscoreOps {
    suspend: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    resume: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
struct Syscore {
    ops: *const SyscoreOps,
}

static mtrr_syscore_ops: SyscoreOps = SyscoreOps {
    suspend: Some(mtrr_save),
    resume: Some(mtrr_restore),
};

static mut mtrr_syscore: Syscore = Syscore {
    ops: &mtrr_syscore_ops,
};

unsafe fn mtrr_register_syscore() {
    mtrr_value = kzalloc_objs::<MtrrValue>(num_var_ranges);

    /*
     * The CPU has no MTRR and seems to not support SMP. They have
     * specific drivers, we use a tricky method to support
     * suspend/resume for them.
     *
     * TBD: is there any system with such CPU which supports
     * suspend/resume? If no, we should remove the code.
     */
    register_syscore(&mut mtrr_syscore);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
