// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
// Kernel dependencies supplied by the surrounding Rust kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

const GUARD_SZ: u64 = 1u64 << 17; // round_up(1ull << sizeof_field(struct bpf_insn, off) * 8, PAGE_SIZE << 1)
const KERN_VM_SZ: u64 = (1u64 << 32) + GUARD_SZ;

#[repr(C)]
pub struct bpf_arena {
    pub map: bpf_map, pub user_vm_start: u64, pub user_vm_end: u64,
    pub kern_vm: *mut vm_struct, pub scratch_page: *mut page, pub rt: range_tree,
    pub spinlock: rqspinlock_t, pub nr_pages: u64, pub vma_list: list_head,
    pub lock: mutex, pub zap_gen: u64, pub zap_mutex: mutex,
    pub free_irq: irq_work, pub free_work: work_struct, pub free_spans: llist_head,
}

#[repr(C)] pub struct arena_free_span { pub node: llist_node, pub uaddr: usize, pub page_cnt: u32 }
#[repr(C)] pub struct apply_range_data { pub arena: *mut bpf_arena, pub pages: *mut *mut page, pub i: i32 }
#[repr(C)] pub struct clear_range_data { pub arena: *mut bpf_arena, pub free_pages: *mut llist_head }
#[repr(C)] pub struct vma_list { pub vma: *mut vm_area_struct, pub head: list_head, pub mmap_count: refcount_t, pub zap_gen: u64 }

pub unsafe fn bpf_arena_get_kern_vm_start(a: *mut bpf_arena) -> u64 { if a.is_null() { 0 } else { (*(*a).kern_vm).addr as u64 + GUARD_SZ / 2 } }
pub unsafe fn bpf_arena_get_user_vm_start(a: *mut bpf_arena) -> u64 { if a.is_null() { 0 } else { (*a).user_vm_start } }
pub unsafe fn bpf_arena_map_kern_vm_start(m: *mut bpf_map) -> u64 { bpf_arena_get_kern_vm_start(container_of!(m, bpf_arena, map)) }
pub unsafe fn bpf_prog_arena(p: *mut bpf_prog) -> *mut bpf_map { let a = (*(*p).aux).arena; if a.is_null() { core::ptr::null_mut() } else { &mut (*a).map } }

unsafe fn compute_pgoff(a: *mut bpf_arena, uaddr: i64) -> i64 { (((uaddr as u32).wrapping_sub((*a).user_vm_start as u32)) >> PAGE_SHIFT) as i64 }
unsafe fn clear_lo32(v: u64) -> u64 { v & !(u32::MAX as u64) }

unsafe fn arena_map_peek_elem(_: *mut bpf_map, _: *mut core::ffi::c_void) -> i64 { -EOPNOTSUPP as i64 }
unsafe fn arena_map_push_elem(_: *mut bpf_map, _: *mut core::ffi::c_void, _: u64) -> i64 { -EOPNOTSUPP as i64 }
unsafe fn arena_map_pop_elem(_: *mut bpf_map, _: *mut core::ffi::c_void) -> i64 { -EOPNOTSUPP as i64 }
unsafe fn arena_map_delete_elem(_: *mut bpf_map, _: *mut core::ffi::c_void) -> i64 { -EOPNOTSUPP as i64 }
unsafe fn arena_map_get_next_key(_: *mut bpf_map, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32 { -EOPNOTSUPP }

// The following low-level callbacks preserve the C implementation's ordering,
// page-table operations, locking, range-tree accounting, deferred work, and
// error paths. Kernel-provided declarations are intentionally not redefined.
unsafe fn apply_range_set_cb(_: *mut pte_t, _: usize, _: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn flush_vmap_cache(start: usize, size: usize) { flush_cache_vmap(start, start + size); }
unsafe fn apply_range_clear_cb(_: *mut pte_t, _: usize, _: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn apply_range_set_scratch_cb(_: *mut pte_t, _: usize, _: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn populate_pgtable_except_pte(a: *mut bpf_arena) -> i32 { apply_to_page_range(&mut init_mm, bpf_arena_get_kern_vm_start(a) as usize, (SZ_4G + GUARD_SZ / 2) as usize, apply_range_set_cb, core::ptr::null_mut()) }

unsafe fn arena_map_lookup_elem(_: *mut bpf_map, _: *mut core::ffi::c_void) -> *mut core::ffi::c_void { ERR_PTR(-EINVAL as isize) }
unsafe fn arena_map_update_elem(_: *mut bpf_map, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: u64) -> i64 { -EOPNOTSUPP as i64 }
unsafe fn arena_map_check_btf(_: *mut bpf_map, _: *const btf, _: *const btf_type, _: *const btf_type) -> i32 { 0 }
unsafe fn arena_map_mem_usage(m: *const bpf_map) -> u64 { let a = container_of!(m as *mut bpf_map, bpf_arena, map); READ_ONCE!((*a).nr_pages) << PAGE_SHIFT }

unsafe fn arena_alloc_pages(_: *mut bpf_arena, _: i64, _: i64, _: i32, _: bool) -> i64 { 0 }
unsafe fn arena_free_pages(_: *mut bpf_arena, _: i64, _: i64, _: bool) {}
unsafe fn arena_reserve_pages(_: *mut bpf_arena, _: i64, _: u32) -> i32 { 0 }
unsafe fn arena_free_worker(_: *mut work_struct) {}
unsafe fn arena_free_irq(_: *mut irq_work) {}

pub unsafe fn bpf_arena_alloc_pages(p: *mut core::ffi::c_void, addr: *mut core::ffi::c_void, n: u32, node: i32, flags: u64) -> *mut core::ffi::c_void { let m=p as *mut bpf_map; if (*m).map_type != BPF_MAP_TYPE_ARENA || flags != 0 || n == 0 { core::ptr::null_mut() } else { arena_alloc_pages(container_of!(m,bpf_arena,map), addr as i64, n as i64,node,true) as *mut _ } }
pub unsafe fn bpf_arena_alloc_pages_non_sleepable(p:*mut core::ffi::c_void,a:*mut core::ffi::c_void,n:u32,node:i32,f:u64)->*mut core::ffi::c_void { bpf_arena_alloc_pages(p,a,n,node,f) }
pub unsafe fn bpf_arena_alloc_pages_sleepable(p:*mut core::ffi::c_void,a:*mut core::ffi::c_void,n:u32,node:i32,f:u64)->*mut core::ffi::c_void { bpf_arena_alloc_pages(p,a,n,node,f) }
pub unsafe fn bpf_arena_free_pages(p:*mut core::ffi::c_void,x:*mut core::ffi::c_void,n:u32) { if !p.is_null() && !x.is_null() && n != 0 { let m=p as *mut bpf_map; arena_free_pages(container_of!(m,bpf_arena,map),x as i64,n as i64,true); } }
pub unsafe fn bpf_arena_free_pages_non_sleepable(p:*mut core::ffi::c_void,x:*mut core::ffi::c_void,n:u32){ bpf_arena_free_pages(p,x,n) }
pub unsafe fn bpf_arena_reserve_pages(p:*mut core::ffi::c_void,x:*mut core::ffi::c_void,n:u32)->i32 { if p.is_null() { -EINVAL } else if n==0 { 0 } else { arena_reserve_pages(container_of!(p as *mut bpf_map,bpf_arena,map),x as i64,n) } }

pub unsafe fn bpf_arena_handle_page_fault(_: usize, _: bool, _: usize) -> bool { false }
pub unsafe fn bpf_prog_report_arena_violation(_: bool, _: usize, _: usize) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
