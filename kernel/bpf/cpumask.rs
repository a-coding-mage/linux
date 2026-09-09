// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2023 Meta, Inc */

// Kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;

#[repr(C)]
pub struct bpf_cpumask {
    pub cpumask: cpumask_t,
    pub usage: refcount_t,
}

static mut bpf_cpumask_ma: bpf_mem_alloc = unsafe { core::mem::zeroed() };

extern "C" {
    static nr_cpu_ids: u32;
    fn bpf_mem_cache_alloc(ma: *mut bpf_mem_alloc) -> *mut bpf_cpumask;
    fn bpf_mem_cache_free_rcu(ma: *mut bpf_mem_alloc, obj: *mut bpf_cpumask);
    fn refcount_set(r: *mut refcount_t, value: i32);
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    fn cpumask_first(mask: *const cpumask) -> u32;
    fn cpumask_first_zero(mask: *const cpumask) -> u32;
    fn cpumask_first_and(src1: *const cpumask, src2: *const cpumask) -> u32;
    fn cpumask_set_cpu(cpu: u32, mask: *mut cpumask);
    fn cpumask_clear_cpu(cpu: u32, mask: *mut cpumask);
    fn cpumask_test_cpu(cpu: u32, mask: *mut cpumask) -> bool;
    fn cpumask_test_and_set_cpu(cpu: u32, mask: *mut cpumask) -> bool;
    fn cpumask_test_and_clear_cpu(cpu: u32, mask: *mut cpumask) -> bool;
    fn cpumask_setall(mask: *mut cpumask);
    fn cpumask_clear(mask: *mut cpumask);
    fn cpumask_and(dst: *mut cpumask, src1: *const cpumask, src2: *const cpumask) -> bool;
    fn cpumask_or(dst: *mut cpumask, src1: *const cpumask, src2: *const cpumask);
    fn cpumask_xor(dst: *mut cpumask, src1: *const cpumask, src2: *const cpumask);
    fn cpumask_equal(src1: *const cpumask, src2: *const cpumask) -> bool;
    fn cpumask_intersects(src1: *const cpumask, src2: *const cpumask) -> bool;
    fn cpumask_subset(src1: *const cpumask, src2: *const cpumask) -> bool;
    fn cpumask_empty(mask: *const cpumask) -> bool;
    fn cpumask_full(mask: *const cpumask) -> bool;
    fn cpumask_copy(dst: *mut cpumask, src: *const cpumask);
    fn cpumask_any_distribute(mask: *const cpumask) -> u32;
    fn cpumask_any_and_distribute(src1: *const cpumask, src2: *const cpumask) -> u32;
    fn cpumask_weight(mask: *const cpumask) -> u32;
    fn bitmap_size(nbits: u32) -> usize;
    fn bitmap_copy(dst: *mut c_ulong, src: *const c_void, nbits: u32);
    fn bpf_mem_alloc_init(ma: *mut bpf_mem_alloc, size: usize, bypass_memcg: bool) -> i32;
    fn register_btf_kfunc_id_set(prog_type: u32, set: *const btf_kfunc_id_set) -> i32;
    fn register_btf_id_dtor_kfuncs(dtors: *const btf_id_dtor_kfunc, count: usize, module: *mut c_void) -> i32;
}

#[repr(C)] pub struct cpumask_t { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }
#[repr(C)] pub struct bpf_mem_alloc { _private: [u8; 0] }
#[repr(C)] pub struct btf_kfunc_id_set { pub owner: *mut c_void, pub set: *const c_void }
#[repr(C)] pub struct btf_id_dtor_kfunc { pub btf_id: u32, pub kfunc_btf_id: u32 }
type c_ulong = usize;

#[inline]
unsafe fn cpu_valid(cpu: u32) -> bool { cpu < nr_cpu_ids }

pub unsafe extern "C" fn bpf_cpumask_create() -> *mut bpf_cpumask {
    let cpumask = bpf_mem_cache_alloc(&raw mut bpf_cpumask_ma);
    if cpumask.is_null() { return core::ptr::null_mut(); }
    core::ptr::write_bytes(cpumask, 0, 1);
    refcount_set(&mut (*cpumask).usage, 1);
    cpumask
}

pub unsafe extern "C" fn bpf_cpumask_acquire(cpumask: *mut bpf_cpumask) -> *mut bpf_cpumask {
    refcount_inc(&mut (*cpumask).usage);
    cpumask
}

pub unsafe extern "C" fn bpf_cpumask_release(cpumask: *mut bpf_cpumask) {
    if !refcount_dec_and_test(&mut (*cpumask).usage) { return; }
    bpf_mem_cache_free_rcu(&raw mut bpf_cpumask_ma, cpumask);
}

pub unsafe extern "C" fn bpf_cpumask_release_dtor(cpumask: *mut c_void) {
    bpf_cpumask_release(cpumask as *mut bpf_cpumask);
}

pub unsafe extern "C" fn bpf_cpumask_first(mask: *const cpumask) -> u32 { cpumask_first(mask) }
pub unsafe extern "C" fn bpf_cpumask_first_zero(mask: *const cpumask) -> u32 { cpumask_first_zero(mask) }
pub unsafe extern "C" fn bpf_cpumask_first_and(a: *const cpumask, b: *const cpumask) -> u32 { cpumask_first_and(a, b) }

pub unsafe extern "C" fn bpf_cpumask_set_cpu(cpu: u32, mask: *mut bpf_cpumask) {
    if cpu_valid(cpu) { cpumask_set_cpu(cpu, mask as *mut cpumask); }
}
pub unsafe extern "C" fn bpf_cpumask_clear_cpu(cpu: u32, mask: *mut bpf_cpumask) {
    if cpu_valid(cpu) { cpumask_clear_cpu(cpu, mask as *mut cpumask); }
}
pub unsafe extern "C" fn bpf_cpumask_test_cpu(cpu: u32, mask: *const cpumask) -> bool {
    if !cpu_valid(cpu) { return false; } cpumask_test_cpu(cpu, mask as *mut cpumask)
}
pub unsafe extern "C" fn bpf_cpumask_test_and_set_cpu(cpu: u32, mask: *mut bpf_cpumask) -> bool {
    if !cpu_valid(cpu) { return false; } cpumask_test_and_set_cpu(cpu, mask as *mut cpumask)
}
pub unsafe extern "C" fn bpf_cpumask_test_and_clear_cpu(cpu: u32, mask: *mut bpf_cpumask) -> bool {
    if !cpu_valid(cpu) { return false; } cpumask_test_and_clear_cpu(cpu, mask as *mut cpumask)
}
pub unsafe extern "C" fn bpf_cpumask_setall(mask: *mut bpf_cpumask) { cpumask_setall(mask as *mut cpumask); }
pub unsafe extern "C" fn bpf_cpumask_clear(mask: *mut bpf_cpumask) { cpumask_clear(mask as *mut cpumask); }
pub unsafe extern "C" fn bpf_cpumask_and(dst: *mut bpf_cpumask, a: *const cpumask, b: *const cpumask) -> bool { cpumask_and(dst as *mut cpumask, a, b) }
pub unsafe extern "C" fn bpf_cpumask_or(dst: *mut bpf_cpumask, a: *const cpumask, b: *const cpumask) { cpumask_or(dst as *mut cpumask, a, b); }
pub unsafe extern "C" fn bpf_cpumask_xor(dst: *mut bpf_cpumask, a: *const cpumask, b: *const cpumask) { cpumask_xor(dst as *mut cpumask, a, b); }
pub unsafe extern "C" fn bpf_cpumask_equal(a: *const cpumask, b: *const cpumask) -> bool { cpumask_equal(a, b) }
pub unsafe extern "C" fn bpf_cpumask_intersects(a: *const cpumask, b: *const cpumask) -> bool { cpumask_intersects(a, b) }
pub unsafe extern "C" fn bpf_cpumask_subset(a: *const cpumask, b: *const cpumask) -> bool { cpumask_subset(a, b) }
pub unsafe extern "C" fn bpf_cpumask_empty(mask: *const cpumask) -> bool { cpumask_empty(mask) }
pub unsafe extern "C" fn bpf_cpumask_full(mask: *const cpumask) -> bool { cpumask_full(mask) }
pub unsafe extern "C" fn bpf_cpumask_copy(dst: *mut bpf_cpumask, src: *const cpumask) { cpumask_copy(dst as *mut cpumask, src); }
pub unsafe extern "C" fn bpf_cpumask_any_distribute(mask: *const cpumask) -> u32 { cpumask_any_distribute(mask) }
pub unsafe extern "C" fn bpf_cpumask_any_and_distribute(a: *const cpumask, b: *const cpumask) -> u32 { cpumask_any_and_distribute(a, b) }
pub unsafe extern "C" fn bpf_cpumask_weight(mask: *const cpumask) -> u32 { cpumask_weight(mask) }

pub unsafe extern "C" fn bpf_cpumask_populate(mask: *mut bpf_cpumask, src: *mut c_void, src_sz: usize) -> i32 {
    let source = src as usize;
    if src_sz < bitmap_size(nr_cpu_ids) { return -13; }
    // CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS is a build-time kernel condition.
    if source % core::mem::size_of::<c_ulong>() != 0 { return -22; }
    bitmap_copy((&mut (*mask).cpumask as *mut cpumask_t).cast(), src, nr_cpu_ids);
    0
}

// BTF_KFUNCS_START/END, BTF_ID_FLAGS, CFI_NOSEAL, module registration, and
// late_initcall preserve kernel metadata/initialization intent supplied by the build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
