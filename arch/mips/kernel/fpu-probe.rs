// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Processor capabilities determination functions.
 *
 * Copyright (C) xxxx  the Anonymous
 * Copyright (C) 1994 - 2006 Ralf Baechle
 * Copyright (C) 2003, 2004  Maciej W. Rozycki
 * Copyright (C) 2001, 2004, 2011, 2012  MIPS Technologies, Inc.
 */

// Kernel and architecture dependencies are supplied by other translation units.

/* Get the FPU Implementation/Revision. */
unsafe fn cpu_get_fpu_id() -> ::core::primitive::c_ulong {
    let tmp: ::core::primitive::c_ulong;
    let fpu_id: ::core::primitive::c_ulong;

    tmp = read_c0_status();
    __enable_fpu(FPU_AS_IS);
    fpu_id = read_32bit_cp1_register(CP1_REVISION);
    write_c0_status(tmp);
    fpu_id
}

/* Check if the CPU has an external FPU. */
pub unsafe fn __cpu_has_fpu() -> ::core::ffi::c_int {
    ((cpu_get_fpu_id() & FPIR_IMP_MASK) != FPIR_IMP_NONE) as ::core::ffi::c_int
}

/* Determine the FCSR mask for FPU hardware. */
unsafe fn cpu_set_fpu_fcsr_mask(c: *mut cpuinfo_mips) {
    let fcsr = (*c).fpu_csr31;
    let mask = FPU_CSR_ALL_X | FPU_CSR_ALL_E | FPU_CSR_ALL_S | FPU_CSR_RM;

    let sr = read_c0_status();
    __enable_fpu(FPU_AS_IS);

    let fcsr0 = {
        write_32bit_cp1_register(CP1_STATUS, fcsr & mask);
        read_32bit_cp1_register(CP1_STATUS)
    };
    let fcsr1 = {
        write_32bit_cp1_register(CP1_STATUS, fcsr | !mask);
        read_32bit_cp1_register(CP1_STATUS)
    };

    write_32bit_cp1_register(CP1_STATUS, fcsr);
    write_c0_status(sr);
    (*c).fpu_msk31 = !(fcsr0 ^ fcsr1) & !mask;
}

/* Determine the IEEE 754 NaN encodings and ABS.fmt/NEG.fmt execution modes supported by FPU hardware. */
unsafe fn cpu_set_fpu_2008(c: *mut cpuinfo_mips) {
    if (*c).isa_level & (MIPS_CPU_ISA_M32R1 | MIPS_CPU_ISA_M64R1 |
        MIPS_CPU_ISA_M32R2 | MIPS_CPU_ISA_M64R2 | MIPS_CPU_ISA_M32R5 |
        MIPS_CPU_ISA_M64R5 | MIPS_CPU_ISA_M32R6 | MIPS_CPU_ISA_M64R6) != 0 {
        let sr = read_c0_status();
        __enable_fpu(FPU_AS_IS);
        let fir = read_32bit_cp1_register(CP1_REVISION);
        if fir & MIPS_FPIR_HAS2008 != 0 {
            let fcsr = read_32bit_cp1_register(CP1_STATUS);
            let fcsr0 = {
                write_32bit_cp1_register(CP1_STATUS, fcsr & !(FPU_CSR_ABS2008 | FPU_CSR_NAN2008 | FPU_CSR_MAC2008));
                read_32bit_cp1_register(CP1_STATUS)
            };
            let fcsr1 = {
                write_32bit_cp1_register(CP1_STATUS, fcsr | FPU_CSR_ABS2008 | FPU_CSR_NAN2008);
                read_32bit_cp1_register(CP1_STATUS)
            };
            write_32bit_cp1_register(CP1_STATUS, fcsr);
            if (*c).isa_level & (MIPS_CPU_ISA_M32R2 | MIPS_CPU_ISA_M64R2) != 0 && fcsr0 & FPU_CSR_MAC2008 != 0 {
                (*c).options |= MIPS_CPU_MAC_2008_ONLY;
            }
            if fcsr0 & FPU_CSR_NAN2008 == 0 { (*c).options |= MIPS_CPU_NAN_LEGACY; }
            if fcsr1 & FPU_CSR_NAN2008 != 0 { (*c).options |= MIPS_CPU_NAN_2008; }
            if (fcsr0 ^ fcsr1) & FPU_CSR_ABS2008 != 0 { (*c).fpu_msk31 &= !FPU_CSR_ABS2008; } else { (*c).fpu_csr31 |= fcsr & FPU_CSR_ABS2008; }
            if (fcsr0 ^ fcsr1) & FPU_CSR_NAN2008 != 0 { (*c).fpu_msk31 &= !FPU_CSR_NAN2008; } else { (*c).fpu_csr31 |= fcsr & FPU_CSR_NAN2008; }
        } else { (*c).options |= MIPS_CPU_NAN_LEGACY; }
        write_c0_status(sr);
    } else { (*c).options |= MIPS_CPU_NAN_LEGACY; }
}

#[repr(C)]
enum Ieee754 { STRICT, EMULATED, LEGACY, STD2008, RELAXED }
static mut ieee754: Ieee754 = Ieee754::STRICT;

unsafe fn cpu_set_nofpu_2008(c: *mut cpuinfo_mips) {
    (*c).options &= !(MIPS_CPU_NAN_2008 | MIPS_CPU_NAN_LEGACY);
    (*c).fpu_csr31 &= !(FPU_CSR_ABS2008 | FPU_CSR_NAN2008);
    (*c).fpu_msk31 &= !(FPU_CSR_ABS2008 | FPU_CSR_NAN2008);
    match ieee754 {
        Ieee754::STRICT | Ieee754::EMULATED => {
            if (*c).isa_level & (MIPS_CPU_ISA_M32R1 | MIPS_CPU_ISA_M64R1 | MIPS_CPU_ISA_M32R2 | MIPS_CPU_ISA_M64R2 | MIPS_CPU_ISA_M32R5 | MIPS_CPU_ISA_M64R5 | MIPS_CPU_ISA_M32R6 | MIPS_CPU_ISA_M64R6) != 0 { (*c).options |= MIPS_CPU_NAN_2008 | MIPS_CPU_NAN_LEGACY; } else { (*c).options |= MIPS_CPU_NAN_LEGACY; (*c).fpu_msk31 |= FPU_CSR_ABS2008 | FPU_CSR_NAN2008; }
        }
        Ieee754::LEGACY => { (*c).options |= MIPS_CPU_NAN_LEGACY; (*c).fpu_msk31 |= FPU_CSR_ABS2008 | FPU_CSR_NAN2008; }
        Ieee754::STD2008 => { (*c).options |= MIPS_CPU_NAN_2008; (*c).fpu_csr31 |= FPU_CSR_ABS2008 | FPU_CSR_NAN2008; (*c).fpu_msk31 |= FPU_CSR_ABS2008 | FPU_CSR_NAN2008; }
        Ieee754::RELAXED => { (*c).options |= MIPS_CPU_NAN_2008 | MIPS_CPU_NAN_LEGACY; }
    }
}

unsafe fn cpu_set_nan_2008(c: *mut cpuinfo_mips) {
    match ieee754 {
        Ieee754::STRICT => { mips_use_nan_legacy = cpu_has_nan_legacy != 0; mips_use_nan_2008 = cpu_has_nan_2008 != 0; }
        Ieee754::LEGACY => { mips_use_nan_legacy = cpu_has_nan_legacy != 0; mips_use_nan_2008 = cpu_has_nan_legacy == 0; }
        Ieee754::STD2008 => { mips_use_nan_legacy = cpu_has_nan_2008 == 0; mips_use_nan_2008 = cpu_has_nan_2008 != 0; }
        Ieee754::EMULATED => { (*c).fpu_msk31 &= !(FPU_CSR_NAN2008 | FPU_CSR_ABS2008); mips_use_nan_legacy = true; mips_use_nan_2008 = true; }
        Ieee754::RELAXED => { mips_use_nan_legacy = true; mips_use_nan_2008 = true; }
    }
}

/* The early_param("ieee754", ieee754_setup) registration is supplied by the kernel build. */
unsafe fn ieee754_setup(s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    if s.is_null() { return -1; }
    let name = ::core::ffi::CStr::from_ptr(s);
    ieee754 = if name.to_bytes() == b"strict" { Ieee754::STRICT } else if name.to_bytes() == b"emulated" { Ieee754::EMULATED } else if name.to_bytes() == b"legacy" { Ieee754::LEGACY } else if name.to_bytes() == b"2008" { Ieee754::STD2008 } else if name.to_bytes() == b"relaxed" { Ieee754::RELAXED } else { return -1 };
    if boot_cpu_data.options & MIPS_CPU_FPU == 0 { cpu_set_nofpu_2008(&mut boot_cpu_data); }
    cpu_set_nan_2008(&mut boot_cpu_data);
    0
}

unsafe fn cpu_set_nofpu_id(c: *mut cpuinfo_mips) {
    let mut value: u32 = 0;
    if (*c).isa_level & (MIPS_CPU_ISA_M32R1 | MIPS_CPU_ISA_M64R1 | MIPS_CPU_ISA_M32R2 | MIPS_CPU_ISA_M64R2 | MIPS_CPU_ISA_M32R5 | MIPS_CPU_ISA_M64R5 | MIPS_CPU_ISA_M32R6 | MIPS_CPU_ISA_M64R6) != 0 { value |= MIPS_FPIR_D | MIPS_FPIR_S; }
    if (*c).isa_level & (MIPS_CPU_ISA_M32R2 | MIPS_CPU_ISA_M64R2 | MIPS_CPU_ISA_M32R5 | MIPS_CPU_ISA_M64R5 | MIPS_CPU_ISA_M32R6 | MIPS_CPU_ISA_M64R6) != 0 { value |= MIPS_FPIR_F64 | MIPS_FPIR_L | MIPS_FPIR_W; }
    if (*c).options & MIPS_CPU_NAN_2008 != 0 { value |= MIPS_FPIR_HAS2008; }
    (*c).fpu_id = value;
}

static mut mips_nofpu_msk31: ::core::primitive::c_uint = 0;

pub unsafe fn cpu_set_fpu_opts(c: *mut cpuinfo_mips) {
    (*c).fpu_id = cpu_get_fpu_id();
    mips_nofpu_msk31 = (*c).fpu_msk31;
    if (*c).isa_level & (MIPS_CPU_ISA_M32R1 | MIPS_CPU_ISA_M64R1 | MIPS_CPU_ISA_M32R2 | MIPS_CPU_ISA_M64R2 | MIPS_CPU_ISA_M32R5 | MIPS_CPU_ISA_M64R5 | MIPS_CPU_ISA_M32R6 | MIPS_CPU_ISA_M64R6) != 0 {
        if (*c).fpu_id & MIPS_FPIR_3D != 0 { (*c).ases |= MIPS_ASE_MIPS3D; }
        if (*c).fpu_id & MIPS_FPIR_UFRP != 0 { (*c).options |= MIPS_CPU_UFR; }
        if (*c).fpu_id & MIPS_FPIR_FREP != 0 { (*c).options |= MIPS_CPU_FRE; }
    }
    cpu_set_fpu_fcsr_mask(c); cpu_set_fpu_2008(c); cpu_set_nan_2008(c);
}

pub unsafe fn cpu_set_nofpu_opts(c: *mut cpuinfo_mips) {
    (*c).options &= !MIPS_CPU_FPU; (*c).fpu_msk31 = mips_nofpu_msk31;
    cpu_set_nofpu_2008(c); cpu_set_nan_2008(c); cpu_set_nofpu_id(c);
}

pub static mut mips_fpu_disabled: ::core::ffi::c_int = 0;

unsafe fn fpu_disable(_s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    cpu_set_nofpu_opts(&mut boot_cpu_data); mips_fpu_disabled = 1; 1
}

/* The __setup("nofpu", fpu_disable) registration is supplied by the kernel build. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
