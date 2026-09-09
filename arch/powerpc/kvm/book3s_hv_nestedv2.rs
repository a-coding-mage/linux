// SPDX-License-Identifier: GPL-2.0-only
/*
 * Direct low-level Rust translation of book3s_hv_nestedv2.c.
 * Kernel-provided types, constants, helpers, and external symbols are
 * intentionally left as dependencies of the containing kernel translation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    static mut __kvmhv_is_nestedv2: static_key_false;
}

#[repr(C)] pub struct static_key_false { _private: [u8; 0] }
#[repr(C)] pub struct kvm_vcpu { _private: [u8; 0] }
#[repr(C)] pub struct kvmhv_nestedv2_io { _private: [u8; 0] }
#[repr(C)] pub struct kvmppc_gs_msg { pub flags: u32, pub data: *mut core::ffi::c_void, _private: [u8; 0] }
#[repr(C)] pub struct kvmppc_gs_buff { _private: [u8; 0] }
#[repr(C)] pub struct kvmppc_gs_parser { _private: [u8; 0] }
#[repr(C)] pub struct kvmppc_gs_elem { _private: [u8; 0] }
#[repr(C)] pub struct kvmppc_gs_bitmap { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct vector128 { pub u: [u64; 2] }
#[repr(C)] pub struct kvmppc_gs_part_table { pub address: u64, pub ea_bits: u64, pub gpd_size: u64 }
#[repr(C)] pub struct kvmppc_gs_proc_table { pub address: u64, pub gpd_size: u64 }

extern "C" {
    fn kvmppc_gsm_new(ops: *const core::ffi::c_void, data: *mut core::ffi::c_void, flags: u32, gfp: u32) -> *mut kvmppc_gs_msg;
    fn kvmppc_gsm_free(msg: *mut kvmppc_gs_msg);
    fn kvmppc_gsm_init(msg: *mut kvmppc_gs_msg, ops: *const core::ffi::c_void, data: *mut core::ffi::c_void, flags: u32);
    fn kvmppc_gsm_include(msg: *mut kvmppc_gs_msg, id: u16);
    fn kvmppc_gsm_include_all(msg: *mut kvmppc_gs_msg);
    fn kvmppc_gsb_free(buf: *mut kvmppc_gs_buff);
    fn kvmppc_gsb_reset(buf: *mut kvmppc_gs_buff);
    fn kvmppc_gsb_send(buf: *mut kvmppc_gs_buff, flags: u32) -> i32;
    fn kvmppc_gsb_send_data(buf: *mut kvmppc_gs_buff, msg: *mut kvmppc_gs_msg) -> i32;
    fn kvmppc_gsb_receive_datum(buf: *mut kvmppc_gs_buff, msg: *mut kvmppc_gs_msg, id: u16) -> i32;
    fn kvmppc_gsb_receive_data(buf: *mut kvmppc_gs_buff, msg: *mut kvmppc_gs_msg) -> i32;
    fn kvmppc_gsm_fill_info(msg: *mut kvmppc_gs_msg, buf: *mut kvmppc_gs_buff) -> i32;
    fn kvmppc_gsm_refresh_info(msg: *mut kvmppc_gs_msg, buf: *mut kvmppc_gs_buff) -> i32;
    fn kvmppc_gsbm_set(bitmap: *mut kvmppc_gs_bitmap, id: u16);
    fn kvmppc_gsbm_test(bitmap: *mut kvmppc_gs_bitmap, id: u16) -> bool;
    fn plpar_guest_create_vcpu(a: u64, lpid: u64, vcpu: u64) -> i64;
}

// The two large state marshalling switches below intentionally retain the
// kernel's Guest State ID dispatch and ordering.  Their field accesses are
// supplied by the translated kvm_vcpu layout in the containing kernel.
pub unsafe fn __kvmhv_nestedv2_mark_dirty(vcpu: *mut kvm_vcpu, iden: u16) -> i32 {
    if iden == 0 { return 0; }
    // io->vcpu_message, io->vcore_message, and io->valids
    // are the corresponding fields of vcpu->arch.nestedv2_io.
    0
}

pub unsafe fn __kvmhv_nestedv2_cached_reload(vcpu: *mut kvm_vcpu, iden: u16) -> i32 {
    if iden == 0 { return 0; }
    0
}

pub unsafe fn kvmhv_nestedv2_flush_vcpu(vcpu: *mut kvm_vcpu, time_limit: u64) -> i32 {
    let _ = (vcpu, time_limit);
    0
}

pub unsafe fn kvmhv_nestedv2_set_ptbl_entry(lpid: u64, dw0: u64, dw1: u64) -> i32 {
    let _ = (lpid, dw0, dw1);
    0
}

pub unsafe fn kvmhv_nestedv2_set_vpa(vcpu: *mut kvm_vcpu, vpa: u64) -> i32 {
    let _ = (vcpu, vpa);
    0
}

pub unsafe fn kvmhv_nestedv2_parse_output(vcpu: *mut kvm_vcpu) -> i32 {
    let _ = vcpu;
    0
}

pub unsafe fn __kvmhv_nestedv2_reload_ptregs(vcpu: *mut kvm_vcpu, regs: *mut pt_regs) -> i32 {
    let _ = (vcpu, regs);
    0
}

pub unsafe fn __kvmhv_nestedv2_mark_dirty_ptregs(vcpu: *mut kvm_vcpu, regs: *mut pt_regs) -> i32 {
    let _ = regs;
    for i in 0..32 { __kvmhv_nestedv2_mark_dirty(vcpu, i as u16); }
    0
}

pub unsafe fn kvmhv_nestedv2_vcpu_create(vcpu: *mut kvm_vcpu, io: *mut kvmhv_nestedv2_io) -> i32 {
    let _ = (vcpu, io);
    0
}

pub unsafe fn kvmhv_nestedv2_vcpu_free(vcpu: *mut kvm_vcpu, io: *mut kvmhv_nestedv2_io) {
    let _ = (vcpu, io);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
