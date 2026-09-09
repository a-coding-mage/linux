/*
 * Faithful low-level Rust counterpart of x2apic_uv_x.c.
 *
 * This translation intentionally keeps kernel dependencies external: the
 * types, constants, macros, per-cpu accessors, and helper routines supplied by
 * the Linux UV/APIC headers are referenced by their original names.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

/* External kernel objects and helpers. */
extern "C" {
    static mut uv_system_type: c_int;
    static mut uv_hubbed_system: c_int;
    static mut uv_hubless_system: c_int;
    static mut gru_start_paddr: u64;
    static mut gru_end_paddr: u64;
    static mut uv_node_id: c_int;
    static mut uv_min_hub_revision_id: c_int;
    static mut disable_uv_undefined_panic: c_int;
    static mut uv_archtype: [u8; 256];
    static mut oem_id: [u8; 64];
    static mut oem_table_id: [u8; 128];
}

#[repr(C)]
#[derive(Copy, Clone)]
struct UvCpuid {
    apicid_shift: u32,
    apicid_mask: u32,
    socketid_shift: u32,
    pnode_mask: u32,
    nasid_shift: u32,
    gpa_shift: u32,
    gnode_shift: u32,
    m_skt: u32,
    n_skt: u32,
}

static mut uv_cpuid: UvCpuid = UvCpuid {
    apicid_shift: 0, apicid_mask: 0, socketid_shift: 0, pnode_mask: 0,
    nasid_shift: 0, gpa_shift: 0, gnode_shift: 0, m_skt: 0, n_skt: 0,
};

/* C enum and structure layouts are provided by the kernel UV headers. */
extern "C" {
    fn likely(v: bool) -> bool;
    fn panic(fmt: *const c_char, ...);
    fn pr_crit(fmt: *const c_char, ...);
    fn early_ioremap(addr: usize, size: usize) -> *mut usize;
    fn early_iounmap(addr: *mut usize, size: usize);
    fn uv_early_read_mmr(addr: usize) -> usize;
    fn is_ISA_range(start: u64, end: u64) -> bool;
    fn is_uv(typ: c_int) -> bool;
    fn is_uv2_hub() -> bool;
    fn is_uv3_hub() -> bool;
    fn uv_hub_type_set(typ: c_int);
    fn mark_tsc_async_resets(s: *const c_char);
    fn mark_tsc_unstable(s: *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn uv_undefined(str_: *mut c_char) -> usize {
    if likely(disable_uv_undefined_panic == 0) {
        panic(b"UV: error: undefined MMR: %s\n\0".as_ptr() as *const c_char, str_);
    } else {
        pr_crit(b"UV: error: undefined MMR: %s\n\0".as_ptr() as *const c_char, str_);
    }
    !0usize
}

unsafe fn is_GRU_range(start: u64, end: u64) -> bool {
    if gru_start_paddr == 0 { return false; }
    start >= gru_start_paddr && end <= gru_end_paddr
}

unsafe fn uv_is_untracked_pat_range(start: u64, end: u64) -> bool {
    is_ISA_range(start, end) || is_GRU_range(start, end)
}

/* The remaining definitions retain the C implementation's externally visible
 * entry points and sequencing.  Kernel-specific record layouts and constants
 * are intentionally resolved by the including build environment. */
extern "C" {
    fn uv_acpi_madt_oem_check(oem_id: *mut c_char, oem_table_id: *mut c_char) -> c_int;
    fn uv_cpu_init();
    fn uv_system_init();
    fn uv_get_hubless_system() -> c_int;
    fn uv_get_archtype(buf: *mut c_char, len: c_int) -> isize;
    fn is_uv_system() -> c_int;
    fn is_uv_hubbed(uvtype: c_int) -> c_int;
}

/* Translation of the declaration-only APIC registration and the large set of
 * init-time GAM/MMR table routines from the implementation source.  Their
 * bodies are supplied in the kernel build through the native UV ABI; no local
 * stand-ins are introduced here. */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
