// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux soft-fp and PowerPC math-emu headers.

use core::ffi::c_void;

pub unsafe fn fsub(frD: *mut c_void, frA: *mut c_void, frB: *mut c_void) -> i32 {
	FP_DECL_D!(A);
	FP_DECL_D!(B);
	FP_DECL_D!(R);
	FP_DECL_EX!();

	// #ifdef DEBUG
	// printk("%s: %p %p %p\\n", __func__, frD, frA, frB);
	// #endif

	FP_UNPACK_DP!(A, frA);
	FP_UNPACK_DP!(B, frB);

	// #ifdef DEBUG
	// printk("A: %ld %lu %lu %ld (%ld)\\n", A_s, A_f1, A_f0, A_e, A_c);
	// printk("B: %ld %lu %lu %ld (%ld)\\n", B_s, B_f1, B_f0, B_e, B_c);
	// #endif

	if B_c != FP_CLS_NAN {
		B_s ^= 1;
	}

	if A_s != B_s && A_c == FP_CLS_INF && B_c == FP_CLS_INF {
		FP_SET_EXCEPTION!(EFLAG_VXISI);
	}

	FP_ADD_D!(R, A, B);

	// #ifdef DEBUG
	// printk("D: %ld %lu %lu %ld (%ld)\\n", R_s, R_f1, R_f0, R_e, R_c);
	// #endif

	__FP_PACK_D!(frD, R);

	FP_CUR_EXCEPTIONS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
