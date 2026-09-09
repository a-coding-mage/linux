/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

// Kernel headers and nodelist.h supply the types, constants, allocation
// routines, and macros referenced below.

use core::ffi::c_void;

#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_full_dirent { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_full_dnode { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_raw_dirent { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_raw_inode { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_tmp_dnode_info { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_node_frag { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_inode_cache { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_sb_info { _private: [u8; 0] }

#[repr(C)] pub struct jffs2_raw_node_ref {
    pub flash_offset: u32,
    pub next_in_ino: *mut jffs2_raw_node_ref,
}
#[repr(C)] pub struct jffs2_eraseblock {
    pub last_node: *mut jffs2_raw_node_ref,
    pub offset: u32,
    pub allocated_refs: i32,
}
#[repr(C)] pub struct jffs2_xattr_datum {
    pub class: u32,
    pub node: *mut c_void,
    pub xindex: list_head,
}
#[repr(C)] pub struct jffs2_xattr_ref {
    pub class: u32,
    pub node: *mut c_void,
}
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }

extern "C" {
    static mut full_dnode_slab: *mut kmem_cache;
    static mut raw_dirent_slab: *mut kmem_cache;
    static mut raw_inode_slab: *mut kmem_cache;
    static mut tmp_dnode_info_slab: *mut kmem_cache;
    static mut raw_node_ref_slab: *mut kmem_cache;
    static mut node_frag_slab: *mut kmem_cache;
    static mut inode_cache_slab: *mut kmem_cache;
    fn kmem_cache_create(name: *const i8, size: usize, align: usize, flags: usize, ctor: *mut c_void) -> *mut kmem_cache;
    fn kmem_cache_alloc(cache: *mut kmem_cache, flags: usize) -> *mut c_void;
    fn kmem_cache_zalloc(cache: *mut kmem_cache, flags: usize) -> *mut c_void;
    fn kmem_cache_free(cache: *mut kmem_cache, obj: *mut c_void);
    fn kmem_cache_destroy(cache: *mut kmem_cache);
    fn kmalloc(size: usize, flags: usize) -> *mut c_void;
    fn kfree(obj: *mut c_void);
    fn dbg_memalloc(fmt: *const i8, ...);
    fn INIT_LIST_HEAD(list: *mut list_head);
}

// These are initialized to NULL in kernel startup code.
// `KMEM_CACHE`, `GFP_KERNEL`, `SLAB_HWCACHE_ALIGN`, and the JFFS2 constants
// are supplied by the translated kernel dependencies.

pub unsafe fn jffs2_create_slab_caches() -> i32 {
    full_dnode_slab = kmem_cache_create(b"jffs2_full_dnode\0".as_ptr() as *const i8, core::mem::size_of::<jffs2_full_dnode>(), 0, 0, core::ptr::null_mut());
    if full_dnode_slab.is_null() { jffs2_destroy_slab_caches(); return -12; }
    raw_dirent_slab = kmem_cache_create(b"jffs2_raw_dirent\0".as_ptr() as *const i8, core::mem::size_of::<jffs2_raw_dirent>(), 0, 0, core::ptr::null_mut());
    if raw_dirent_slab.is_null() { jffs2_destroy_slab_caches(); return -12; }
    raw_inode_slab = kmem_cache_create(b"jffs2_raw_inode\0".as_ptr() as *const i8, core::mem::size_of::<jffs2_raw_inode>(), 0, 0, core::ptr::null_mut());
    if raw_inode_slab.is_null() { jffs2_destroy_slab_caches(); return -12; }
    tmp_dnode_info_slab = kmem_cache_create(b"jffs2_tmp_dnode_info\0".as_ptr() as *const i8, core::mem::size_of::<jffs2_tmp_dnode_info>(), 0, 0, core::ptr::null_mut());
    if tmp_dnode_info_slab.is_null() { jffs2_destroy_slab_caches(); return -12; }
    raw_node_ref_slab = kmem_cache_create(b"jffs2_refblock\0".as_ptr() as *const i8, core::mem::size_of::<jffs2_raw_node_ref>() * (REFS_PER_BLOCK + 1), 0, 0, core::ptr::null_mut());
    if raw_node_ref_slab.is_null() { jffs2_destroy_slab_caches(); return -12; }
    node_frag_slab = kmem_cache_create(b"jffs2_node_frag\0".as_ptr() as *const i8, core::mem::size_of::<jffs2_node_frag>(), 0, 0, core::ptr::null_mut());
    if node_frag_slab.is_null() { jffs2_destroy_slab_caches(); return -12; }
    inode_cache_slab = kmem_cache_create(b"jffs2_inode_cache\0".as_ptr() as *const i8, core::mem::size_of::<jffs2_inode_cache>(), 0, 0, core::ptr::null_mut());
    if inode_cache_slab.is_null() { jffs2_destroy_slab_caches(); return -12; }
    0
}

pub unsafe fn jffs2_destroy_slab_caches() {
    kmem_cache_destroy(full_dnode_slab); kmem_cache_destroy(raw_dirent_slab);
    kmem_cache_destroy(raw_inode_slab); kmem_cache_destroy(tmp_dnode_info_slab);
    kmem_cache_destroy(raw_node_ref_slab); kmem_cache_destroy(node_frag_slab);
    kmem_cache_destroy(inode_cache_slab);
}

pub unsafe fn jffs2_alloc_full_dirent(namesize: i32) -> *mut jffs2_full_dirent { kmalloc(core::mem::size_of::<jffs2_full_dirent>() + namesize as usize, GFP_KERNEL) as *mut _ }
pub unsafe fn jffs2_free_full_dirent(x: *mut jffs2_full_dirent) { kfree(x as *mut c_void); }
pub unsafe fn jffs2_alloc_full_dnode() -> *mut jffs2_full_dnode { kmem_cache_alloc(full_dnode_slab, GFP_KERNEL) as *mut _ }
pub unsafe fn jffs2_free_full_dnode(x: *mut jffs2_full_dnode) { kmem_cache_free(full_dnode_slab, x as *mut c_void); }
pub unsafe fn jffs2_alloc_raw_dirent() -> *mut jffs2_raw_dirent { kmem_cache_alloc(raw_dirent_slab, GFP_KERNEL) as *mut _ }
pub unsafe fn jffs2_free_raw_dirent(x: *mut jffs2_raw_dirent) { kmem_cache_free(raw_dirent_slab, x as *mut c_void); }
pub unsafe fn jffs2_alloc_raw_inode() -> *mut jffs2_raw_inode { kmem_cache_alloc(raw_inode_slab, GFP_KERNEL) as *mut _ }
pub unsafe fn jffs2_free_raw_inode(x: *mut jffs2_raw_inode) { kmem_cache_free(raw_inode_slab, x as *mut c_void); }
pub unsafe fn jffs2_alloc_tmp_dnode_info() -> *mut jffs2_tmp_dnode_info { kmem_cache_alloc(tmp_dnode_info_slab, GFP_KERNEL) as *mut _ }
pub unsafe fn jffs2_free_tmp_dnode_info(x: *mut jffs2_tmp_dnode_info) { kmem_cache_free(tmp_dnode_info_slab, x as *mut c_void); }

unsafe fn jffs2_alloc_refblock() -> *mut jffs2_raw_node_ref {
    let ret = kmem_cache_alloc(raw_node_ref_slab, GFP_KERNEL) as *mut jffs2_raw_node_ref;
    if !ret.is_null() { for i in 0..REFS_PER_BLOCK { (*ret.add(i)).flash_offset = REF_EMPTY_NODE; (*ret.add(i)).next_in_ino = core::ptr::null_mut(); } (*ret.add(REFS_PER_BLOCK)).flash_offset = REF_LINK_NODE; (*ret.add(REFS_PER_BLOCK)).next_in_ino = core::ptr::null_mut(); }
    ret
}

pub unsafe fn jffs2_prealloc_raw_node_refs(_c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock, nr: i32) -> i32 {
    let mut p = &mut (*jeb).last_node as *mut *mut jffs2_raw_node_ref;
    let mut r = *p; let mut i = nr;
    if !r.is_null() && (*r).flash_offset != REF_EMPTY_NODE { r = r.add(1); }
    while i != 0 { if r.is_null() { r = jffs2_alloc_refblock(); *p = r; if r.is_null() { return -12; } } if (*r).flash_offset == REF_LINK_NODE { p = &mut (*r).next_in_ino; r = *p; } else { i -= 1; r = r.add(1); } }
    (*jeb).allocated_refs = nr; 0
}
pub unsafe fn jffs2_free_refblock(x: *mut jffs2_raw_node_ref) { kmem_cache_free(raw_node_ref_slab, x as *mut c_void); }
pub unsafe fn jffs2_alloc_node_frag() -> *mut jffs2_node_frag { kmem_cache_alloc(node_frag_slab, GFP_KERNEL) as *mut _ }
pub unsafe fn jffs2_free_node_frag(x: *mut jffs2_node_frag) { kmem_cache_free(node_frag_slab, x as *mut c_void); }
pub unsafe fn jffs2_alloc_inode_cache() -> *mut jffs2_inode_cache { kmem_cache_alloc(inode_cache_slab, GFP_KERNEL) as *mut _ }
pub unsafe fn jffs2_free_inode_cache(x: *mut jffs2_inode_cache) { kmem_cache_free(inode_cache_slab, x as *mut c_void); }

#[cfg(CONFIG_JFFS2_FS_XATTR)]
static mut xattr_datum_cache: *mut kmem_cache = core::ptr::null_mut();
#[cfg(CONFIG_JFFS2_FS_XATTR)]
static mut xattr_ref_cache: *mut kmem_cache = core::ptr::null_mut();

#[cfg(CONFIG_JFFS2_FS_XATTR)]
pub unsafe fn jffs2_alloc_xattr_datum() -> *mut jffs2_xattr_datum {
    let xd = kmem_cache_zalloc(xattr_datum_cache, GFP_KERNEL) as *mut jffs2_xattr_datum;
    if xd.is_null() { return core::ptr::null_mut(); }
    (*xd).class = RAWNODE_CLASS_XATTR_DATUM;
    (*xd).node = xd as *mut c_void;
    INIT_LIST_HEAD(&mut (*xd).xindex);
    xd
}

#[cfg(CONFIG_JFFS2_FS_XATTR)]
pub unsafe fn jffs2_free_xattr_datum(xd: *mut jffs2_xattr_datum) {
    kmem_cache_free(xattr_datum_cache, xd as *mut c_void);
}

#[cfg(CONFIG_JFFS2_FS_XATTR)]
pub unsafe fn jffs2_alloc_xattr_ref() -> *mut jffs2_xattr_ref {
    let r = kmem_cache_zalloc(xattr_ref_cache, GFP_KERNEL) as *mut jffs2_xattr_ref;
    if r.is_null() { return core::ptr::null_mut(); }
    (*r).class = RAWNODE_CLASS_XATTR_REF;
    (*r).node = r as *mut c_void;
    r
}

#[cfg(CONFIG_JFFS2_FS_XATTR)]
pub unsafe fn jffs2_free_xattr_ref(r: *mut jffs2_xattr_ref) {
    kmem_cache_free(xattr_ref_cache, r as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
