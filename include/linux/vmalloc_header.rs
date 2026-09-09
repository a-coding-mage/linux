/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/vmalloc.h. External types and functions are supplied by dependencies.

use core::ffi::c_void;

pub const VM_IOREMAP: usize = 0x00000001;
pub const VM_ALLOC: usize = 0x00000002;
pub const VM_MAP: usize = 0x00000004;
pub const VM_USERMAP: usize = 0x00000008;
pub const VM_DMA_COHERENT: usize = 0x00000010;
pub const VM_UNINITIALIZED: usize = 0x00000020;
pub const VM_NO_GUARD: usize = 0x00000040;
pub const VM_KASAN: usize = 0x00000080;
pub const VM_FLUSH_RESET_PERMS: usize = 0x00000100;
pub const VM_MAP_PUT_PAGES: usize = 0x00000200;
pub const VM_ALLOW_HUGE_VMAP: usize = 0x00000400;
// CONFIG_KASAN_GENERIC/CONFIG_KASAN_SW_TAGS and CONFIG_KASAN_VMALLOC are build-time conditions.
pub const VM_DEFER_KMEMLEAK: usize = 0;
pub const VM_SPARSE: usize = 0x00001000;

pub const IOREMAP_MAX_ORDER: usize = 7 + PAGE_SHIFT;

#[repr(C)]
pub union VmStructLink {
    pub next: *mut VmStruct,
    pub llnode: LlistNode,
}

#[repr(C)]
pub struct VmStruct {
    pub link: VmStructLink,
    pub addr: *mut c_void,
    pub size: c_ulong,
    pub flags: c_ulong,
    pub pages: *mut *mut Page,
    // CONFIG_HAVE_ARCH_HUGE_VMALLOC
    pub page_order: c_uint,
    pub nr_pages: c_ulong,
    pub phys_addr: PhysAddr,
    pub caller: *const c_void,
    pub requested_size: c_ulong,
}

#[repr(C)]
pub union VmapAreaTree {
    pub subtree_max_size: c_ulong,
    pub vm: *mut VmStruct,
}

#[repr(C)]
pub struct VmapArea {
    pub va_start: c_ulong,
    pub va_end: c_ulong,
    pub rb_node: RbNode,
    pub list: ListHead,
    pub tree: VmapAreaTree,
    pub flags: c_ulong,
}

// Architecture overrides may provide these functions.
#[inline]
pub fn arch_vmap_p4d_supported(_prot: PgprotT) -> bool { false }
#[inline]
pub fn arch_vmap_pud_supported(_prot: PgprotT) -> bool { false }
#[inline]
pub fn arch_vmap_pmd_supported(_prot: PgprotT) -> bool { false }
#[inline]
pub fn arch_vmap_pte_range_map_size(_addr: c_ulong, _end: c_ulong, _pfn: u64, _max_page_shift: c_uint) -> c_ulong { PAGE_SIZE }
#[inline]
pub fn arch_vmap_pte_range_unmap_size(_addr: c_ulong, _ptep: *mut PteT) -> c_ulong { PAGE_SIZE }
#[inline]
pub fn arch_vmap_pte_supported_shift(_size: c_ulong) -> c_int { PAGE_SHIFT as c_int }
#[inline]
pub fn arch_vmap_pgprot_tagged(prot: PgprotT) -> PgprotT { prot }

extern "C" {
    pub fn vm_unmap_ram(mem: *const c_void, count: c_uint);
    pub fn vm_map_ram(pages: *mut *mut Page, count: c_uint, node: c_int) -> *mut c_void;
    pub fn vm_unmap_aliases();
    pub fn vmalloc_noprof(size: c_ulong) -> *mut c_void;
    pub fn vzalloc_noprof(size: c_ulong) -> *mut c_void;
    pub fn vmalloc_user_noprof(size: c_ulong) -> *mut c_void;
    pub fn vmalloc_node_noprof(size: c_ulong, node: c_int) -> *mut c_void;
    pub fn vzalloc_node_noprof(size: c_ulong, node: c_int) -> *mut c_void;
    pub fn vmalloc_32_noprof(size: c_ulong) -> *mut c_void;
    pub fn vmalloc_32_user_noprof(size: c_ulong) -> *mut c_void;
    pub fn __vmalloc_noprof(size: c_ulong, gfp_mask: GfpT) -> *mut c_void;
    pub fn __vmalloc_node_range_noprof(size: c_ulong, align: c_ulong, start: c_ulong, end: c_ulong, gfp_mask: GfpT, prot: PgprotT, vm_flags: c_ulong, node: c_int, caller: *const c_void) -> *mut c_void;
    pub fn __vmalloc_node_noprof(size: c_ulong, align: c_ulong, gfp_mask: GfpT, node: c_int, caller: *const c_void) -> *mut c_void;
    pub fn vmalloc_huge_node_noprof(size: c_ulong, gfp_mask: GfpT, node: c_int) -> *mut c_void;
    pub fn __vmalloc_array_noprof(n: usize, size: usize, flags: GfpT) -> *mut c_void;
    pub fn vmalloc_array_noprof(n: usize, size: usize) -> *mut c_void;
    pub fn __vcalloc_noprof(n: usize, size: usize, flags: GfpT) -> *mut c_void;
    pub fn vcalloc_noprof(n: usize, size: usize) -> *mut c_void;
    pub fn vrealloc_node_align_noprof(p: *const c_void, size: usize, align: c_ulong, flags: GfpT, nid: c_int) -> *mut c_void;
    pub fn vfree(addr: *const c_void);
    pub fn vfree_atomic(addr: *const c_void);
    pub fn vmap(pages: *mut *mut Page, count: c_uint, flags: c_ulong, prot: PgprotT) -> *mut c_void;
    pub fn vmap_pfn(pfns: *mut c_ulong, count: c_uint, prot: PgprotT) -> *mut c_void;
    pub fn vunmap(addr: *const c_void);
    pub fn remap_vmalloc_range_partial(vma: *mut VmAreaStruct, uaddr: c_ulong, kaddr: *mut c_void, pgoff: c_ulong, size: c_ulong) -> c_int;
    pub fn remap_vmalloc_range(vma: *mut VmAreaStruct, addr: *mut c_void, pgoff: c_ulong) -> c_int;
    pub fn vmap_pages_range(addr: c_ulong, end: c_ulong, prot: PgprotT, pages: *mut *mut Page, page_shift: c_uint) -> c_int;
    pub fn find_vm_area(addr: *const c_void) -> *mut VmStruct;
}

#[inline]
pub unsafe fn get_vm_area_size(area: *const VmStruct) -> usize {
    if (*area).flags & VM_NO_GUARD as c_ulong != 0 { (*area).size as usize } else { ((*area).size - PAGE_SIZE) as usize }
}

// The following declarations retain the header's external interfaces.
extern "C" {
    pub fn get_vm_area(size: c_ulong, flags: c_ulong) -> *mut VmStruct;
    pub fn get_vm_area_caller(size: c_ulong, flags: c_ulong, caller: *const c_void) -> *mut VmStruct;
    pub fn __get_vm_area_caller(size: c_ulong, flags: c_ulong, start: c_ulong, end: c_ulong, caller: *const c_void) -> *mut VmStruct;
    pub fn free_vm_area(area: *mut VmStruct);
    pub fn remove_vm_area(addr: *const c_void) -> *mut VmStruct;
    pub fn find_vmap_area(addr: c_ulong) -> *mut VmapArea;
    pub fn vread_iter(iter: *mut IovIter, addr: *const i8, count: usize) -> c_long;
    pub fn vm_area_add_early(vm: *mut VmStruct);
    pub fn vm_area_register_early(vm: *mut VmStruct, align: usize);
    pub fn register_vmap_purge_notifier(nb: *mut NotifierBlock) -> c_int;
    pub fn unregister_vmap_purge_notifier(nb: *mut NotifierBlock) -> c_int;
    pub fn memalloc_apply_gfp_scope(gfp_mask: GfpT) -> c_uint;
    pub fn memalloc_restore_scope(flags: c_uint);
}

#[inline]
pub unsafe fn vmalloc_huge(size: c_ulong, gfp_mask: GfpT) -> *mut c_void {
    vmalloc_huge_node_noprof(size, gfp_mask, NUMA_NO_NODE)
}

#[inline]
pub unsafe fn is_vm_area_hugepages(addr: *const c_void) -> bool {
    // CONFIG_HAVE_ARCH_HUGE_VMALLOC
    let area = find_vm_area(addr);
    !area.is_null() && (*area).page_order > 0
}

// CONFIG_MMU: VMALLOC_TOTAL is VMALLOC_END - VMALLOC_START; otherwise it is 0.
pub const VMALLOC_TOTAL: usize = 0;

extern "C" {
    pub fn set_vm_flush_reset_perms(addr: *mut c_void);
    pub fn vm_area_map_pages(area: *mut VmStruct, start: c_ulong, end: c_ulong, pages: *mut *mut Page) -> c_int;
    pub fn vm_area_unmap_pages(area: *mut VmStruct, start: c_ulong, end: c_ulong);
    pub fn vunmap_range(addr: c_ulong, end: c_ulong);
    pub fn pcpu_get_vm_areas(offsets: *const c_ulong, sizes: *const usize, nr_vms: c_int, align: usize, gfp: GfpT) -> *mut *mut VmStruct;
    pub fn pcpu_free_vm_areas(vms: *mut *mut VmStruct, nr_vms: c_int);
    pub fn vmalloc_dump_obj(object: *mut c_void) -> bool;
}

// External dependency types/constants are supplied by the translated kernel headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
