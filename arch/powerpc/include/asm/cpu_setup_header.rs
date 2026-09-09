/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2020 IBM Corporation
 */

// C header guard: _ASM_POWERPC_CPU_SETUP_H

// Forward declaration supplied by the surrounding PowerPC dependencies.
#[repr(C)]
pub struct cpu_spec;

extern "C" {
    pub fn __setup_cpu_power7(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_power8(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_power9(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_power10(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_power12(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __restore_cpu_power7();
    pub fn __restore_cpu_power8();
    pub fn __restore_cpu_power9();
    pub fn __restore_cpu_power10();
    pub fn __restore_cpu_power12();

    pub fn __setup_cpu_e500v1(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_e500v2(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_e500mc(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_440ep(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_440epx(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_440gx(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_440grx(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_440spe(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_440x5(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_460ex(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_460gt(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_460sx(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_apm821xx(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_603(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_604(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_750(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_750cx(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_750fx(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_7400(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_7410(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_745x(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);

    pub fn __setup_cpu_ppc970(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_ppc970MP(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_pa6t(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __restore_cpu_pa6t();
    pub fn __restore_cpu_ppc970();

    pub fn __setup_cpu_e5500(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __setup_cpu_e6500(offset: ::core::ffi::c_ulong, spec: *mut cpu_spec);
    pub fn __restore_cpu_e5500();
    pub fn __restore_cpu_e6500();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
