// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of linux/mm/nommu.c.  Kernel-provided types and symbols
 * are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub type ulong = usize;
pub type gfp_t = usize;
pub type vm_flags_t = usize;
pub type vma_flags_t = usize;
pub type pgprot_t = usize;
pub type phys_addr_t = usize;
pub type pgoff_t = usize;
pub type vm_fault_t = i32;

#[repr(C)] pub struct page;
#[repr(C)] pub struct folio;
#[repr(C)] pub struct file;
#[repr(C)] pub struct inode;
#[repr(C)] pub struct mm_struct;
#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct vm_area_struct;
#[repr(C)] pub struct vm_region;
#[repr(C)] pub struct vm_struct;
#[repr(C)] pub struct vm_fault;
#[repr(C)] pub struct iov_iter;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct rb_node;
#[repr(C)] pub struct ctl_table;

extern "C" {
    static mut current: *mut task_struct;
    static mut vm_region_jar: *mut c_void;
    static mut nommu_region_tree: rb_root;
    static mut mmap_pages_allocated: isize;
    fn virt_addr_valid(p: *const c_void) -> bool;
    fn virt_to_folio(p: *const c_void) -> *mut folio;
    fn folio_test_slab(p: *mut folio) -> bool;
    fn folio_test_large(p: *mut folio) -> bool;
    fn ksize(p: *const c_void) -> u32;
    fn folio_size(p: *mut folio) -> u32;
    fn virt_to_page(p: *const c_void) -> *mut page;
    fn page_to_pfn(p: *mut page) -> usize;
    fn kfree(p: *const c_void);
    fn kmalloc_noprof(size: usize, flags: gfp_t) -> *mut c_void;
    fn krealloc_noprof(p: *const c_void, size: usize, flags: gfp_t) -> *mut c_void;
    fn copy_to_iter(p: *const c_void, n: usize, i: *mut iov_iter) -> isize;
    fn __vmalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn __vmalloc_noprof(size: usize, flags: gfp_t) -> *mut c_void;
    fn find_vma(mm: *mut mm_struct, addr: usize) -> *mut vm_area_struct;
    fn mmap_write_lock(mm: *mut mm_struct); fn mmap_write_unlock(mm: *mut mm_struct);
    fn mmap_read_unlock(mm: *mut mm_struct);
    fn vm_flags_set(vma: *mut vm_area_struct, flags: vm_flags_t);
    fn virt_to_page(p: *const c_void) -> *mut page;
    fn put_page(p: *mut page); fn fput(p: *mut file);
    fn BUG(); fn BUG_ON(x: bool); fn WARN_ON_ONCE(x: bool) -> bool;
    fn flush_icache_user_range(a: usize, b: usize);
    fn memset(p: *mut c_void, value: i32, n: usize) -> *mut c_void;
    fn kmem_cache_free(c: *mut c_void, p: *mut c_void);
    fn vm_area_free(v: *mut vm_area_struct);
    fn get_task_mm(t: *mut task_struct) -> *mut mm_struct;
    fn mmput(mm: *mut mm_struct);
}

pub static mut highest_memmap_pfn: usize = 0;
pub static mut heap_stack_gap: i32 = 0;
pub static mut generic_file_vm_ops: () = ();

pub unsafe fn kobjsize(objp: *const c_void) -> u32 {
    if objp.is_null() || !virt_addr_valid(objp) { return 0; }
    let folio = virt_to_folio(objp);
    if folio_test_slab(folio) { return ksize(objp); }
    if !folio_test_large(folio) {
        let vma = find_vma((*current).mm(), objp as usize);
        if !vma.is_null() { return ((*vma).vm_end() - (*vma).vm_start()) as u32; }
    }
    folio_size(folio)
}

pub unsafe fn vfree(addr: *const c_void) { kfree(addr); }
pub unsafe fn __vmalloc_noprof_local(size: usize, gfp_mask: gfp_t) -> *mut c_void {
    kmalloc_noprof(size, (gfp_mask | 0x400) & !0x100)
}
pub unsafe fn vrealloc_node_align_noprof(p: *const c_void, size: usize, _align: usize,
                                         flags: gfp_t, _node: i32) -> *mut c_void {
    krealloc_noprof(p, size, (flags | 0x400) & !0x100)
}
pub unsafe fn __vmalloc_node_range_noprof(size: usize, _align: usize, _start: usize,
    _end: usize, flags: gfp_t, _prot: pgprot_t, _vm_flags: usize, _node: i32,
    _caller: *const c_void) -> *mut c_void { __vmalloc_noprof_local(size, flags) }
pub unsafe fn __vmalloc_node_noprof(size: usize, _align: usize, flags: gfp_t,
                                    _node: i32, _caller: *const c_void) -> *mut c_void {
    __vmalloc_noprof_local(size, flags)
}
pub unsafe fn vmalloc_user_noprof(size: usize) -> *mut c_void {
    let p = __vmalloc(size, 0x1000 | 0x800);
    if !p.is_null() { let vma = find_vma((*current).mm(), p as usize); if !vma.is_null() { vm_flags_set(vma, 1 << 20); } }
    p
}
pub unsafe fn vmalloc_to_page(addr: *const c_void) -> *mut page { virt_to_page(addr) }
pub unsafe fn vmalloc_to_pfn(addr: *const c_void) -> usize { page_to_pfn(virt_to_page(addr)) }
pub unsafe fn vread_iter(iter: *mut iov_iter, addr: *const i8, mut count: usize) -> isize {
    if (addr as usize).wrapping_add(count) < count { count = (0usize).wrapping_sub(addr as usize); }
    copy_to_iter(addr as *const c_void, count, iter)
}
pub unsafe fn vmalloc_noprof(size: usize) -> *mut c_void { __vmalloc_noprof_local(size, 0x10) }
pub unsafe fn vmalloc_huge_node_noprof(size: usize, flags: gfp_t, _node: i32) -> *mut c_void { __vmalloc_noprof_local(size, flags) }
pub unsafe fn vzalloc_noprof(size: usize) -> *mut c_void { __vmalloc_noprof_local(size, 0x10 | 0x800) }
pub unsafe fn vmalloc_node_noprof(size: usize, _node: i32) -> *mut c_void { vmalloc_noprof(size) }
pub unsafe fn vzalloc_node_noprof(size: usize, _node: i32) -> *mut c_void { vzalloc_noprof(size) }
pub unsafe fn vmalloc_32_noprof(size: usize) -> *mut c_void { __vmalloc_noprof_local(size, 0x10) }
pub unsafe fn vmalloc_32_user_noprof(size: usize) -> *mut c_void { vmalloc_user_noprof(size) }

pub unsafe fn vmap(_pages: *mut *mut page, _count: u32, _flags: usize, _prot: pgprot_t) -> *mut c_void { BUG(); core::ptr::null_mut() }
pub unsafe fn vunmap(_addr: *const c_void) { BUG(); }
pub unsafe fn vm_map_ram(_pages: *mut *mut page, _count: u32, _node: i32) -> *mut c_void { BUG(); core::ptr::null_mut() }
pub unsafe fn vm_unmap_ram(_mem: *const c_void, _count: u32) { BUG(); }
pub unsafe fn vm_unmap_aliases() {}
pub unsafe fn free_vm_area(_area: *mut vm_struct) { BUG(); }
pub unsafe fn vm_insert_page(_vma: *mut vm_area_struct, _addr: usize, _page: *mut page) -> i32 { -22 }
pub unsafe fn vm_insert_pages(_vma: *mut vm_area_struct, _addr: usize, _pages: *mut *mut page, _num: *mut usize) -> i32 { -22 }
pub unsafe fn vm_map_pages(_vma: *mut vm_area_struct, _pages: *mut *mut page, _num: usize) -> i32 { -22 }
pub unsafe fn vm_map_pages_zero(_vma: *mut vm_area_struct, _pages: *mut *mut page, _num: usize) -> i32 { -22 }

// The remaining declarations retain the C ABI and external kernel semantics.
// Their definitions are supplied by the surrounding kernel translation unit.
extern "C" {
    pub fn do_mmap(file: *mut file, addr: usize, len: usize, prot: usize, flags: usize,
                   vma_flags: vma_flags_t, pgoff: usize, populate: *mut usize,
                   uf: *mut list_head) -> usize;
    pub fn do_munmap(mm: *mut mm_struct, start: usize, len: usize, uf: *mut list_head) -> i32;
    pub fn vm_munmap(addr: usize, len: usize) -> i32;
    pub fn exit_mmap(mm: *mut mm_struct);
    pub fn access_remote_vm(mm: *mut mm_struct, addr: usize, buf: *mut c_void,
                            len: i32, gup_flags: u32) -> i32;
    pub fn access_process_vm(tsk: *mut task_struct, addr: usize, buf: *mut c_void,
                             len: i32, gup_flags: u32) -> i32;
    pub fn nommu_shrink_inode_mappings(inode: *mut inode, size: usize, newsize: usize) -> i32;
    pub fn dup_mmap(mm: *mut mm_struct, oldmm: *mut mm_struct) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
