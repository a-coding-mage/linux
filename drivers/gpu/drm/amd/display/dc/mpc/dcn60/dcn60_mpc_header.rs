// SPDX-License-Identifier: MIT
// Copyright 2025 Advanced Micro Devices, Inc.
//
// Rust translation of dcn60_mpc.h.  The register and field-list macros are
// supplied by the corresponding earlier-generation MPC headers.

// Dependencies: dcn401/dcn401_mpc.h and dcn42/dcn42_mpc.h

macro_rules! TO_DCN60_MPC {
    ($mpc_base:expr) => {
        container_of!($mpc_base, dcn60_mpc, base)
    };
}

macro_rules! MPC_REG_VARIABLE_LIST_DCN6_0 {
    () => { MPC_REG_VARIABLE_LIST_DCN42!() };
}

// The complete field selection is the DCN6.0 extension of the DCN42 list.
// SF entries are intentionally retained as a macro expansion so register
// definitions remain provided by the generated register headers.
macro_rules! MPC_COMMON_MASK_SH_LIST_DCN6_0 {
    ($mask_sh:expr) => {
        MPC_COMMON_MASK_SH_LIST_DCN42!($mask_sh)
    };
}

macro_rules! MPC_REG_LIST_DCN6_0_RI {
    ($inst:ident) => { MPC_REG_LIST_DCN42!($inst) };
}

macro_rules! MPC_REG_FIELD_LIST_DCN6_0 {
    ($type:ty) => { MPC_REG_FIELD_LIST_DCN42!($type) };
}

#[repr(C)]
pub struct dcn60_mpc_shift {
    pub fields: MPC_REG_FIELD_LIST_DCN6_0!(u8),
}

#[repr(C)]
pub struct dcn60_mpc_mask {
    pub fields: MPC_REG_FIELD_LIST_DCN6_0!(u32),
}

#[repr(C)]
pub struct dcn60_mpc_registers {
    pub registers: MPC_REG_VARIABLE_LIST_DCN6_0!(),
}

#[repr(C)]
pub struct dcn60_mpc {
    pub base: mpc,
    pub mpcc_in_use_mask: ::core::ffi::c_int,
    pub num_mpcc: ::core::ffi::c_int,
    pub mpc_regs: *const dcn60_mpc_registers,
    pub mpc_shift: *const dcn60_mpc_shift,
    pub mpc_mask: *const dcn60_mpc_mask,
    pub num_rmu: ::core::ffi::c_int,
}

extern "C" {
    pub fn dcn60_mpc_construct(
        mpc401: *mut dcn60_mpc,
        ctx: *mut dc_context,
        mpc_regs: *const dcn60_mpc_registers,
        mpc_shift: *const dcn60_mpc_shift,
        mpc_mask: *const dcn60_mpc_mask,
        num_mpcc: ::core::ffi::c_int,
        num_rmu: ::core::ffi::c_int,
    );

    pub fn mpc60_program_rmcm_lut_read_write_control(
        mpc: *mut mpc,
        id: MCM_LUT_ID,
        lut_bank_a: bool,
        enabled: bool,
        mpcc_id: ::core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
