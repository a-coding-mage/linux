/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright Novell Inc. 2010
 *
 * Authors: Alexander Graf <agraf@suse.de>
 */

// Dependency equivalent of: #include <linux/types.h>

extern "C" {
    pub fn fps_fres(fpscr: *mut u64, dst: *mut u32, src1: *mut u32);
    pub fn fps_frsqrte(fpscr: *mut u64, dst: *mut u32, src1: *mut u32);
    pub fn fps_fsqrts(fpscr: *mut u64, dst: *mut u32, src1: *mut u32);

    pub fn fps_fadds(fpscr: *mut u64, dst: *mut u32, src1: *mut u32, src2: *mut u32);
    pub fn fps_fdivs(fpscr: *mut u64, dst: *mut u32, src1: *mut u32, src2: *mut u32);
    pub fn fps_fmuls(fpscr: *mut u64, dst: *mut u32, src1: *mut u32, src2: *mut u32);
    pub fn fps_fsubs(fpscr: *mut u64, dst: *mut u32, src1: *mut u32, src2: *mut u32);

    pub fn fps_fmadds(
        fpscr: *mut u64,
        dst: *mut u32,
        src1: *mut u32,
        src2: *mut u32,
        src3: *mut u32,
    );
    pub fn fps_fmsubs(
        fpscr: *mut u64,
        dst: *mut u32,
        src1: *mut u32,
        src2: *mut u32,
        src3: *mut u32,
    );
    pub fn fps_fnmadds(
        fpscr: *mut u64,
        dst: *mut u32,
        src1: *mut u32,
        src2: *mut u32,
        src3: *mut u32,
    );
    pub fn fps_fnmsubs(
        fpscr: *mut u64,
        dst: *mut u32,
        src1: *mut u32,
        src2: *mut u32,
        src3: *mut u32,
    );
    pub fn fps_fsel(
        fpscr: *mut u64,
        dst: *mut u32,
        src1: *mut u32,
        src2: *mut u32,
        src3: *mut u32,
    );

    pub fn fpd_fcmpu(fpscr: *mut u64, cr: *mut u32, src1: *mut u64, src2: *mut u64);
    pub fn fpd_fcmpo(fpscr: *mut u64, cr: *mut u32, src1: *mut u64, src2: *mut u64);

    pub fn fpd_fsqrts(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64);
    pub fn fpd_frsqrtes(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64);
    pub fn fpd_fres(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64);
    pub fn fpd_frsp(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64);
    pub fn fpd_fctiw(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64);
    pub fn fpd_fctiwz(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64);
    pub fn fpd_fsqrt(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64);
    pub fn fpd_fre(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64);
    pub fn fpd_frsqrte(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64);
    pub fn fpd_fneg(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64);
    pub fn fpd_fabs(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64);

    pub fn fpd_fadds(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64);
    pub fn fpd_fsubs(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64);
    pub fn fpd_fdivs(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64);
    pub fn fpd_fmuls(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64);
    pub fn fpd_fcpsgn(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64);
    pub fn fpd_fdiv(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64);
    pub fn fpd_fadd(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64);
    pub fn fpd_fmul(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64);
    pub fn fpd_fsub(fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64);

    pub fn fpd_fmsubs(
        fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64, src3: *mut u64,
    );
    pub fn fpd_fmadds(
        fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64, src3: *mut u64,
    );
    pub fn fpd_fnmsubs(
        fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64, src3: *mut u64,
    );
    pub fn fpd_fnmadds(
        fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64, src3: *mut u64,
    );
    pub fn fpd_fsel(
        fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64, src3: *mut u64,
    );
    pub fn fpd_fmsub(
        fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64, src3: *mut u64,
    );
    pub fn fpd_fmadd(
        fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64, src3: *mut u64,
    );
    pub fn fpd_fnmsub(
        fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64, src3: *mut u64,
    );
    pub fn fpd_fnmadd(
        fpscr: *mut u64, cr: *mut u32, dst: *mut u64, src1: *mut u64, src2: *mut u64, src3: *mut u64,
    );

    pub fn kvm_cvt_fd(from: *mut u32, to: *mut u64);
    pub fn kvm_cvt_df(from: *mut u64, to: *mut u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
