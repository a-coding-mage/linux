/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2022-2024 Rivos, Inc
 */

/* Translated from the Linux RISC-V CPU feature header. */

#[repr(C)]
pub struct riscv_cpuinfo {
    pub mvendorid: ::core::ffi::c_ulong,
    pub marchid: ::core::ffi::c_ulong,
    pub mimpid: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct riscv_isainfo {
    /* DECLARE_BITMAP(isa, RISCV_ISA_EXT_MAX); */
    pub isa: [::core::ffi::c_ulong; 0],
}

extern "C" {
    pub static mut riscv_cpuinfo: riscv_cpuinfo;
    pub static cpuinfo_op: seq_operations;
    pub static mut hart_isa: [riscv_isainfo; NR_CPUS];
    pub static mut thead_vlenb_of: u32;

    pub fn riscv_user_isa_enable();

    pub fn check_unaligned_access_emulated_all_cpus() -> bool;
    pub fn unaligned_access_init();
    pub fn cpu_online_unaligned_access_init(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn check_vector_unaligned_access_emulated_all_cpus() -> bool;
    pub fn riscv_get_elf_hwcap() -> ::core::ffi::c_ulong;

    pub static riscv_isa_ext: riscv_isa_ext_data;
    pub static riscv_isa_ext_count: usize;
    pub static mut riscv_isa_fallback: bool;

    pub fn riscv_isa_extension_base(isa_bitmap: *const ::core::ffi::c_ulong)
        -> ::core::ffi::c_ulong;
}

/* Types and constants are supplied by the corresponding kernel dependencies. */
extern "C" {
    pub type seq_operations;
}

/* Per-cpu declarations from DECLARE_PER_CPU. */
extern "C" {
    pub static mut misaligned_access_speed: ::core::ffi::c_long;
    pub static mut vector_misaligned_access: ::core::ffi::c_long;
}

#[repr(C)]
pub struct riscv_isa_ext_data {
    pub id: ::core::ffi::c_uint,
    pub name: *const ::core::ffi::c_char,
    pub property: *const ::core::ffi::c_char,
    pub subset_ext_ids: *const ::core::ffi::c_uint,
    pub subset_ext_size: ::core::ffi::c_uint,
    pub validate: Option<unsafe extern "C" fn(
        data: *const riscv_isa_ext_data,
        isa_bitmap: *const ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int>,
}

#[macro_export]
macro_rules! _RISCV_ISA_EXT_DATA {
    ($name:ident, $id:expr, $subset_exts:expr, $subset_exts_size:expr, $validate:expr) => {
        riscv_isa_ext_data {
            id: $id,
            name: ::core::concat_idents!("", stringify!($name)).as_ptr() as *const ::core::ffi::c_char,
            property: ::core::concat_idents!("", stringify!($name)).as_ptr() as *const ::core::ffi::c_char,
            subset_ext_ids: $subset_exts,
            subset_ext_size: $subset_exts_size,
            validate: $validate,
        }
    };
}

/* The following C preprocessor convenience forms are represented directly. */
#[macro_export]
macro_rules! __RISCV_ISA_EXT_DATA {
    ($name:ident, $id:expr) => { _RISCV_ISA_EXT_DATA!($name, $id, ::core::ptr::null(), 0, None) };
}
#[macro_export]
macro_rules! __RISCV_ISA_EXT_DATA_VALIDATE {
    ($name:ident, $id:expr, $validate:expr) => {
        _RISCV_ISA_EXT_DATA!($name, $id, ::core::ptr::null(), 0, $validate)
    };
}

extern "C" {
    pub fn unaligned_ctl_available() -> bool;
    pub fn misaligned_traps_can_delegate() -> bool;
    pub fn unaligned_emulation_finish();
    pub fn check_vector_unaligned_access_emulated(work: *mut work_struct);
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

pub unsafe fn has_fast_unaligned_accesses() -> bool {
    /* CONFIG_RISCV_PROBE_UNALIGNED_ACCESS / CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS. */
    static_branch_likely(&fast_unaligned_access_speed_key)
}

extern "C" {
    pub static fast_unaligned_access_speed_key: static_key_false;
    pub fn static_branch_likely(key: *const static_key_false) -> bool;
    pub fn riscv_cpu_has_extension_likely(cpu: ::core::ffi::c_int, ext: ::core::ffi::c_ulong) -> bool;
    pub fn riscv_cpu_has_extension_unlikely(cpu: ::core::ffi::c_int, ext: ::core::ffi::c_ulong) -> bool;
    pub fn cpu_supports_shadow_stack() -> bool;
    pub fn cpu_supports_indirect_br_lp_instr() -> bool;
}

extern "C" {
    pub type static_key_false;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
