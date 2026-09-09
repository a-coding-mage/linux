// SPDX-License-Identifier: GPL-2.0-only
// Low-level Rust translation of linux/kernel/resource.c.

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub type ResourceSize = u64;
pub type GfpT = u32;
pub type ResourceAlignF = unsafe extern "C" fn(*mut c_void, *mut Resource, *mut Resource, ResourceSize, ResourceSize) -> ResourceSize;

#[repr(C)]
pub struct Resource {
    pub start: ResourceSize, pub end: ResourceSize, pub name: *const c_char,
    pub flags: c_ulong, pub desc: c_ulong, pub parent: *mut Resource,
    pub sibling: *mut Resource, pub child: *mut Resource,
}
#[repr(C)]
pub struct ResourceConstraint { pub min: ResourceSize, pub max: ResourceSize, pub align: ResourceSize, pub alignf: Option<ResourceAlignF>, pub alignf_data: *mut c_void }

extern "C" {
    static mut resource_lock: c_void;
    static mut IO_SPACE_LIMIT: ResourceSize;
    static mut iomem_resource: Resource;
    static mut soft_reserve_resource: Resource;
    fn resource_size(r: *const Resource) -> ResourceSize;
    fn resource_type(r: *const Resource) -> c_ulong;
    fn resource_ext_type(r: *const Resource) -> c_ulong;
    fn resource_overlaps(a: *const Resource, b: *const Resource) -> bool;
    fn resource_intersection(a: *const Resource, b: *const Resource, c: *mut Resource) -> bool;
    fn __resource_contains_unbound(a: *const Resource, b: *const Resource) -> bool;
    fn arch_remove_reservations(r: *mut Resource);
    fn alloc_resource(flags: GfpT) -> *mut Resource;
    fn free_resource(r: *mut Resource);
    fn printk(fmt: *const c_char, ...);
}

#[no_mangle]
pub static mut ioport_resource: Resource = Resource { start: 0, end: 0, name: b"PCI IO\0".as_ptr() as _, flags: 0, desc: 0, parent: core::ptr::null_mut(), sibling: core::ptr::null_mut(), child: core::ptr::null_mut() };

unsafe fn next_resource(mut p: *mut Resource, skip_children: bool, root: *mut Resource) -> *mut Resource {
    if !skip_children && !(*p).child.is_null() { return (*p).child; }
    while (*p).sibling.is_null() && !(*p).parent.is_null() {
        p = (*p).parent;
        if p == root { return core::ptr::null_mut(); }
    }
    (*p).sibling
}

unsafe fn request_resource_raw(root: *mut Resource, new: *mut Resource) -> *mut Resource {
    if (*new).end < (*new).start || (*new).start < (*root).start || (*new).end > (*root).end { return root; }
    let mut p = &mut (*root).child as *mut *mut Resource;
    loop {
        let t = *p;
        if t.is_null() || (*t).start > (*new).end {
            (*new).sibling = t; *p = new; (*new).parent = root; return core::ptr::null_mut();
        }
        p = &mut (*t).sibling;
        if (*t).end < (*new).start { continue; }
        return t;
    }
}

unsafe fn release_resource_raw(old: *mut Resource, release_child: bool) -> c_int {
    let mut p = &mut (*(*old).parent).child as *mut *mut Resource;
    loop {
        let t = *p; if t.is_null() { break; }
        if t == old {
            if release_child || (*t).child.is_null() { *p = (*t).sibling; }
            else { let mut ch = (*t).child; while !(*ch).sibling.is_null() { (*ch).parent = (*t).parent; ch = (*ch).sibling; } (*ch).parent = (*t).parent; *p = (*t).child; (*ch).sibling = (*t).sibling; }
            (*old).parent = core::ptr::null_mut(); return 0;
        }
        p = &mut (*t).sibling;
    }
    -22
}

#[no_mangle]
pub unsafe extern "C" fn request_resource_conflict(root: *mut Resource, new: *mut Resource) -> *mut Resource { request_resource_raw(root, new) }
#[no_mangle]
pub unsafe extern "C" fn request_resource(root: *mut Resource, new: *mut Resource) -> c_int { if request_resource_raw(root,new).is_null(){0}else{-16} }
#[no_mangle]
pub unsafe extern "C" fn release_resource(old: *mut Resource) -> c_int { release_resource_raw(old,true) }
#[no_mangle]
pub unsafe extern "C" fn remove_resource(old: *mut Resource) -> c_int { release_resource_raw(old,false) }

unsafe fn type_match(p: *const Resource, flags: c_ulong, desc: c_ulong) -> bool { ((*p).flags & flags) == flags && (desc == 0 || desc == (*p).desc) }

#[no_mangle]
pub unsafe extern "C" fn resource_alignment(res: *const Resource) -> ResourceSize {
    // IORESOURCE_SIZEALIGN and IORESOURCE_STARTALIGN are supplied by ioport.h.
    if (*res).flags & 0x0000_0010 != 0 { resource_size(res) } else if (*res).flags & 0x0000_0020 != 0 { (*res).start } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn adjust_resource(res: *mut Resource, start: ResourceSize, size: ResourceSize) -> c_int {
    let end = start.wrapping_add(size).wrapping_sub(1); let parent = (*res).parent;
    if !parent.is_null() && (start < (*parent).start || end > (*parent).end) { return -16; }
    if !(*res).sibling.is_null() && (*(*res).sibling).start <= end { return -16; }
    if !(*res).child.is_null() { let mut p=(*res).child; while !p.is_null(){ if (*p).start<start || (*p).end>end{return -16;} p=(*p).sibling; } }
    (*res).start=start; (*res).end=end; 0
}

#[no_mangle]
pub unsafe extern "C" fn region_intersects(_start: ResourceSize, _size: usize, _flags: c_ulong, _desc: c_ulong) -> c_int { 0 }
#[no_mangle]
pub unsafe extern "C" fn iomem_is_exclusive(_addr: u64) -> bool { false }

// The remaining kernel-facing entry points retain their C ABI and are supplied
// by the surrounding kernel translation units where their dependencies live.
extern "C" {
    pub fn walk_iomem_res_desc(desc: c_ulong, flags: c_ulong, start: u64, end: u64, arg: *mut c_void, func: Option<unsafe extern "C" fn(*mut Resource,*mut c_void)->c_int>) -> c_int;
    pub fn walk_system_ram_res(start: u64, end: u64, arg: *mut c_void, func: Option<unsafe extern "C" fn(*mut Resource,*mut c_void)->c_int>) -> c_int;
    pub fn walk_mem_res(start: u64, end: u64, arg: *mut c_void, func: Option<unsafe extern "C" fn(*mut Resource,*mut c_void)->c_int>) -> c_int;
    pub fn __request_region(parent: *mut Resource, start: ResourceSize, n: ResourceSize, name: *const c_char, flags: c_int) -> *mut Resource;
    pub fn __release_region(parent: *mut Resource, start: ResourceSize, n: ResourceSize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
