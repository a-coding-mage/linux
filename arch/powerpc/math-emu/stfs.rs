// SPDX-License-Identifier: GPL-2.0

// The declarations and operations below correspond to the soft-fp and
// uaccess facilities supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct FpDouble {
    pub s: libc::c_long,
    pub f1: libc::c_ulong,
    pub f0: libc::c_ulong,
    pub e: libc::c_long,
    pub c: libc::c_long,
}

#[repr(C)]
pub struct FpSingle {
    pub s: libc::c_long,
    pub f: libc::c_ulong,
    pub e: libc::c_long,
    pub c: libc::c_long,
}

extern "C" {
    fn fp_unpack_dp(a: *mut FpDouble, p: *const core::ffi::c_void);
    fn fp_conv_s_d_1_2(r: *mut FpSingle, a: *const FpDouble);
    fn fp_pack_canonical_s_1(r: *mut FpSingle);
    fn fp_pack_raw_1_p_s(p: *mut f32, r: *const FpSingle);
    fn fp_cur_exceptions() -> libc::c_int;
    fn fpu_trap_p(exceptions: libc::c_int) -> libc::c_int;
    fn copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> libc::c_ulong;
}

const EFAULT: libc::c_int = 14;

pub unsafe fn stfs(fr_s: *mut core::ffi::c_void, ea: *mut core::ffi::c_void) -> libc::c_int {
    // FP_DECL_D(A);
    // FP_DECL_S(R);
    // FP_DECL_EX;
    let mut a = FpDouble {
        s: 0,
        f1: 0,
        f0: 0,
        e: 0,
        c: 0,
    };
    let mut r = FpSingle {
        s: 0,
        f: 0,
        e: 0,
        c: 0,
    };
    let mut f: f32 = 0.0;

    // #ifdef DEBUG
    // printk("%s: S %p, ea %p\n", __func__, frS, ea);
    // #endif
    fp_unpack_dp(&mut a, fr_s as *const core::ffi::c_void);

    // #ifdef DEBUG
    // printk("A: %ld %lu %lu %ld (%ld)\n", A_s, A_f1, A_f0, A_e, A_c);
    // #endif
    fp_conv_s_d_1_2(&mut r, &a);

    // #ifdef DEBUG
    // printk("R: %ld %lu %ld (%ld)\n", R_s, R_f, R_e, R_c);
    // #endif
    fp_pack_canonical_s_1(&mut r);
    let exceptions = fp_cur_exceptions();
    if exceptions == 0 || fpu_trap_p(exceptions) == 0 {
        fp_pack_raw_1_p_s(&mut f, &r);
        if copy_to_user(
            ea,
            &f as *const f32 as *const core::ffi::c_void,
            core::mem::size_of::<f32>(),
        ) != 0
        {
            return -EFAULT;
        }
    }

    exceptions
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
