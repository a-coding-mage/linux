/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2024 Ventana Micro Systems Inc.
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation.
pub struct kvm_vcpu_arch;
pub type phys_addr_t = usize;

unsafe extern "C" {
    pub static kvm_riscv_nacl_available: bool;
    pub static kvm_riscv_nacl_sync_csr_available: bool;
    pub static kvm_riscv_nacl_sync_hfence_available: bool;
    pub static kvm_riscv_nacl_sync_sret_available: bool;
    pub static kvm_riscv_nacl_autoswap_csr_available: bool;

    pub fn __kvm_riscv_nacl_hfence(
        shmem: *mut c_void,
        control: c_ulong,
        page_num: c_ulong,
        page_count: c_ulong,
    );
    pub fn __kvm_riscv_nacl_switch_to(
        vcpu_arch: *mut kvm_vcpu_arch,
        sbi_ext_id: c_ulong,
        sbi_func_id: c_ulong,
    );
    pub fn kvm_riscv_nacl_enable() -> c_int;
    pub fn kvm_riscv_nacl_disable();
    pub fn kvm_riscv_nacl_exit();
    pub fn kvm_riscv_nacl_init() -> c_int;
}

pub type c_ulong = usize;
pub type c_int = i32;

#[repr(C)]
pub struct kvm_riscv_nacl {
    pub shmem: *mut c_void,
    pub shmem_phys: phys_addr_t,
}

// DECLARE_PER_CPU(struct kvm_riscv_nacl, kvm_riscv_nacl);
unsafe extern "C" {
    pub static mut kvm_riscv_nacl: kvm_riscv_nacl;
}

// CONFIG_32BIT selects 32-bit little-endian long conversion; otherwise 64-bit.
#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub unsafe fn lelong_to_cpu(x: u32) -> c_ulong { u32::from_le(x) as c_ulong }
#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub unsafe fn cpu_to_lelong(x: c_ulong) -> u32 { (x as u32).to_le() }
#[cfg(target_pointer_width = "32")]
pub type __lelong = u32;

#[cfg(not(target_pointer_width = "32"))]
#[inline(always)]
pub unsafe fn lelong_to_cpu(x: u64) -> c_ulong { u64::from_le(x) as c_ulong }
#[cfg(not(target_pointer_width = "32"))]
#[inline(always)]
pub unsafe fn cpu_to_lelong(x: c_ulong) -> u64 { (x as u64).to_le() }
#[cfg(not(target_pointer_width = "32"))]
pub type __lelong = u64;

#[macro_export]
macro_rules! nacl_shmem { () => { unsafe { $crate::kvm_riscv_nacl.shmem } }; }

// SBI_NACL_* constants and sbi_ecall/csr_* functions are supplied externally.
#[macro_export]
macro_rules! nacl_scratch_read_long { ($shmem:expr, $offset:expr) => {{ unsafe {
    let p = ($shmem as *mut u8).add(SBI_NACL_SHMEM_SCRATCH_OFFSET + ($offset) as usize)
        as *const __lelong;
    lelong_to_cpu(*p)
}}}; }

#[macro_export]
macro_rules! nacl_scratch_write_long { ($shmem:expr, $offset:expr, $val:expr) => {{ unsafe {
    let p = ($shmem as *mut u8).add(SBI_NACL_SHMEM_SCRATCH_OFFSET + ($offset) as usize)
        as *mut __lelong;
    *p = cpu_to_lelong($val);
}}}; }

#[macro_export]
macro_rules! nacl_scratch_write_longs { ($shmem:expr, $offset:expr, $array:expr, $count:expr) => {{ unsafe {
    let p = ($shmem as *mut u8).add(SBI_NACL_SHMEM_SCRATCH_OFFSET + ($offset) as usize)
        as *mut __lelong;
    for i in 0..($count) { *p.add(i as usize) = cpu_to_lelong(($array)[i as usize]); }
}}}; }

#[macro_export]
macro_rules! nacl_sync_hfence { ($e:expr) => { unsafe { sbi_ecall(SBI_EXT_NACL, SBI_EXT_NACL_SYNC_HFENCE, $e, 0, 0, 0, 0, 0) } }; }

#[macro_export]
macro_rules! nacl_hfence_mkconfig { ($ty:expr, $order:expr, $vmid:expr, $asid:expr) => {{
    let mut c: c_ulong = SBI_NACL_SHMEM_HFENCE_CONFIG_PEND;
    c |= (($ty) & SBI_NACL_SHMEM_HFENCE_CONFIG_TYPE_MASK) << SBI_NACL_SHMEM_HFENCE_CONFIG_TYPE_SHIFT;
    c |= ((($order) - SBI_NACL_SHMEM_HFENCE_ORDER_BASE) & SBI_NACL_SHMEM_HFENCE_CONFIG_ORDER_MASK) << SBI_NACL_SHMEM_HFENCE_CONFIG_ORDER_SHIFT;
    c |= (($vmid) & SBI_NACL_SHMEM_HFENCE_CONFIG_VMID_MASK) << SBI_NACL_SHMEM_HFENCE_CONFIG_VMID_SHIFT;
    c |= ($asid) & SBI_NACL_SHMEM_HFENCE_CONFIG_ASID_MASK;
    c
}}; }
#[macro_export]
macro_rules! nacl_hfence_mkpnum { ($order:expr, $addr:expr) => { ($addr) >> ($order) }; }
#[macro_export]
macro_rules! nacl_hfence_mkpcount { ($order:expr, $size:expr) => { ($size) >> ($order) }; }

#[macro_export]
macro_rules! nacl_hfence_gvma { ($s:expr,$gpa:expr,$sz:expr,$o:expr) => { unsafe { __kvm_riscv_nacl_hfence($s,nacl_hfence_mkconfig!(SBI_NACL_SHMEM_HFENCE_TYPE_GVMA,$o,0,0),nacl_hfence_mkpnum!($o,$gpa),nacl_hfence_mkpcount!($o,$sz)) } }; }
#[macro_export]
macro_rules! nacl_hfence_gvma_all { ($s:expr) => { unsafe { __kvm_riscv_nacl_hfence($s,nacl_hfence_mkconfig!(SBI_NACL_SHMEM_HFENCE_TYPE_GVMA_ALL,0,0,0),0,0) } }; }
#[macro_export]
macro_rules! nacl_hfence_gvma_vmid { ($s:expr,$v:expr,$gpa:expr,$sz:expr,$o:expr) => { unsafe { __kvm_riscv_nacl_hfence($s,nacl_hfence_mkconfig!(SBI_NACL_SHMEM_HFENCE_TYPE_GVMA_VMID,$o,$v,0),nacl_hfence_mkpnum!($o,$gpa),nacl_hfence_mkpcount!($o,$sz)) } }; }
#[macro_export]
macro_rules! nacl_hfence_gvma_vmid_all { ($s:expr,$v:expr) => { unsafe { __kvm_riscv_nacl_hfence($s,nacl_hfence_mkconfig!(SBI_NACL_SHMEM_HFENCE_TYPE_GVMA_VMID_ALL,0,$v,0),0,0) } }; }
#[macro_export]
macro_rules! nacl_hfence_vvma { ($s:expr,$v:expr,$g:expr,$sz:expr,$o:expr) => { unsafe { __kvm_riscv_nacl_hfence($s,nacl_hfence_mkconfig!(SBI_NACL_SHMEM_HFENCE_TYPE_VVMA,$o,$v,0),nacl_hfence_mkpnum!($o,$g),nacl_hfence_mkpcount!($o,$sz)) } }; }
#[macro_export]
macro_rules! nacl_hfence_vvma_all { ($s:expr,$v:expr) => { unsafe { __kvm_riscv_nacl_hfence($s,nacl_hfence_mkconfig!(SBI_NACL_SHMEM_HFENCE_TYPE_VVMA_ALL,0,$v,0),0,0) } }; }
#[macro_export]
macro_rules! nacl_hfence_vvma_asid { ($s:expr,$v:expr,$a:expr,$g:expr,$sz:expr,$o:expr) => { unsafe { __kvm_riscv_nacl_hfence($s,nacl_hfence_mkconfig!(SBI_NACL_SHMEM_HFENCE_TYPE_VVMA_ASID,$o,$v,$a),nacl_hfence_mkpnum!($o,$g),nacl_hfence_mkpcount!($o,$sz)) } }; }
#[macro_export]
macro_rules! nacl_hfence_vvma_asid_all { ($s:expr,$v:expr,$a:expr) => { unsafe { __kvm_riscv_nacl_hfence($s,nacl_hfence_mkconfig!(SBI_NACL_SHMEM_HFENCE_TYPE_VVMA_ASID_ALL,0,$v,$a),0,0) } }; }

// ncsr_xyz macros retain the source's static-branch selection semantics.
#[macro_export]
macro_rules! nacl_sync_csr { ($csr:expr) => { unsafe { sbi_ecall(SBI_EXT_NACL,SBI_EXT_NACL_SYNC_CSR,$csr,0,0,0,0,0) } }; }

#[macro_export]
macro_rules! kvm_riscv_nacl_available { () => { unsafe { $crate::kvm_riscv_nacl_available } }; }
#[macro_export]
macro_rules! kvm_riscv_nacl_sync_csr_available { () => { unsafe { $crate::kvm_riscv_nacl_sync_csr_available } }; }
#[macro_export]
macro_rules! kvm_riscv_nacl_sync_hfence_available { () => { unsafe { $crate::kvm_riscv_nacl_sync_hfence_available } }; }
#[macro_export]
macro_rules! kvm_riscv_nacl_sync_sret_available { () => { unsafe { $crate::kvm_riscv_nacl_sync_sret_available } }; }
#[macro_export]
macro_rules! kvm_riscv_nacl_autoswap_csr_available { () => { unsafe { $crate::kvm_riscv_nacl_autoswap_csr_available } }; }

#[macro_export]
macro_rules! nacl_csr_read { ($s:expr,$csr:expr) => {{ unsafe {
    let a = ($s as *mut u8).add(SBI_NACL_SHMEM_CSR_OFFSET) as *const __lelong;
    lelong_to_cpu(*a.add(SBI_NACL_SHMEM_CSR_INDEX($csr) as usize))
}}}; }
#[macro_export]
macro_rules! nacl_csr_write { ($s:expr,$csr:expr,$val:expr) => {{ unsafe {
    let s = $s as *mut u8;
    let i = SBI_NACL_SHMEM_CSR_INDEX($csr) as usize;
    let a = s.add(SBI_NACL_SHMEM_CSR_OFFSET) as *mut __lelong;
    let b = s.add(SBI_NACL_SHMEM_DBITMAP_OFFSET);
    *a.add(i) = cpu_to_lelong($val);
    *b.add(i >> 3) |= 1u8 << (i & 0x7);
}}}; }
#[macro_export]
macro_rules! nacl_csr_swap { ($s:expr,$csr:expr,$val:expr) => {{ unsafe {
    let s = $s as *mut u8;
    let i = SBI_NACL_SHMEM_CSR_INDEX($csr) as usize;
    let a = s.add(SBI_NACL_SHMEM_CSR_OFFSET) as *mut __lelong;
    let b = s.add(SBI_NACL_SHMEM_DBITMAP_OFFSET);
    let r = lelong_to_cpu(*a.add(i));
    *a.add(i) = cpu_to_lelong($val);
    *b.add(i >> 3) |= 1u8 << (i & 0x7);
    r
}}}; }

#[macro_export]
macro_rules! ncsr_read { ($csr:expr) => {{ if kvm_riscv_nacl_available!() { nacl_csr_read!(nacl_shmem!(),$csr) } else { unsafe { csr_read($csr) } } }}; }
#[macro_export]
macro_rules! ncsr_write { ($csr:expr,$val:expr) => {{ if kvm_riscv_nacl_sync_csr_available!() { nacl_csr_write!(nacl_shmem!(),$csr,$val) } else { unsafe { csr_write($csr,$val) } } }}; }
#[macro_export]
macro_rules! ncsr_swap { ($csr:expr,$val:expr) => {{ if kvm_riscv_nacl_sync_csr_available!() { nacl_csr_swap!(nacl_shmem!(),$csr,$val) } else { unsafe { csr_swap($csr,$val) } } }}; }
#[macro_export]
macro_rules! nsync_csr { ($csr:expr) => {{ if kvm_riscv_nacl_sync_csr_available!() { nacl_sync_csr!($csr); } }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
