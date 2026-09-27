/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2020 IBM Corporation
 */

// C header guard: _ASM_POWERPC_CPU_SETUP_H

// Forward declaration supplied by the surrounding PowerPC dependencies.
#[repr(C)]
pub struct cpu_spec;

extern "C" {
    pub fn __setup_cpu_power7(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_power8(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_power9(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_power10(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_power12(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __restore_cpu_power7();
    pub fn __restore_cpu_power8();
    pub fn __restore_cpu_power9();
    pub fn __restore_cpu_power10();
    pub fn __restore_cpu_power12();

    pub fn __setup_cpu_e500v1(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_e500v2(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_e500mc(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_440ep(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_440epx(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_440gx(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_440grx(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_440spe(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_440x5(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_460ex(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_460gt(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_460sx(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_apm821xx(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_603(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_604(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_750(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_750cx(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_750fx(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_7400(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_7410(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_745x(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);

    pub fn __setup_cpu_ppc970(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_ppc970MP(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_pa6t(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __restore_cpu_pa6t();
    pub fn __restore_cpu_ppc970();

    pub fn __setup_cpu_e5500(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_e6500(offset: ::kernel::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __restore_cpu_e5500();
    pub fn __restore_cpu_e6500();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
