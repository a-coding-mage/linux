/* JFFS2 xattr implementation, translated from xattr.c. */

// Kernel headers and build-time configuration are supplied by the surrounding crate.
use core::ffi::{c_char, c_void};

pub const JFFS2_XATTR_IS_CORRUPTED: i32 = 1;

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> i32;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn memset(d: *mut c_void, v: i32, n: usize) -> *mut c_void;
    fn kmalloc(n: usize, flags: u32) -> *mut c_char;
    fn kfree(p: *mut c_void);
    fn crc32(seed: u32, p: *const c_void, n: usize) -> u32;
}

// The following declarations intentionally refer to definitions supplied by the kernel port.
#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe fn xattr_datum_hashkey(xprefix: i32, xname: *const c_char, xvalue: *const c_char, xsize: i32) -> u32 {
    let n = strlen(xname);
    crc32(xprefix as u32, xname as *const c_void, n) ^ crc32(xprefix as u32, xvalue as *const c_void, xsize as usize)
}

extern "C" {
    fn ref_flags(p: *mut jffs2_raw_node_ref) -> u32;
    fn ref_offset(p: *mut jffs2_raw_node_ref) -> u32;
    fn ref_totlen(c: *mut jffs2_sb_info, b: *mut jffs2_eraseblock, p: *mut jffs2_raw_node_ref) -> u32;
    fn jffs2_flash_read(c: *mut jffs2_sb_info, o: u32, n: usize, r: *mut usize, p: *mut c_char) -> i32;
    fn jffs2_flash_write(c: *mut jffs2_sb_info, o: u32, n: usize, r: *mut usize, p: *mut c_char) -> i32;
    fn jffs2_flash_writev(c: *mut jffs2_sb_info, v: *mut kvec, n: usize, o: u32, r: *mut usize, f: i32) -> i32;
    fn jffs2_alloc_xattr_datum() -> *mut jffs2_xattr_datum;
    fn jffs2_free_xattr_datum(p: *mut jffs2_xattr_datum);
    fn jffs2_alloc_xattr_ref() -> *mut jffs2_xattr_ref;
    fn jffs2_free_xattr_ref(p: *mut jffs2_xattr_ref);
    fn jffs2_add_physical_node_ref(c: *mut jffs2_sb_info, o: u32, n: u32, p: *mut c_void);
    fn jffs2_mark_node_obsolete(c: *mut jffs2_sb_info, p: *mut jffs2_raw_node_ref);
    fn jffs2_get_ino_cache(c: *mut jffs2_sb_info, ino: u32) -> *mut jffs2_inode_cache;
    fn write_ofs(c: *mut jffs2_sb_info) -> u32;
    fn jffs2_reserve_space(c: *mut jffs2_sb_info, n: u32, l: *mut u32, a: i32, s: u32) -> i32;
    fn jffs2_reserve_space_gc(c: *mut jffs2_sb_info, n: u32, l: *mut u32, s: u32) -> i32;
    fn jffs2_complete_reservation(c: *mut jffs2_sb_info);
}

#[repr(C)] pub struct kvec { pub iov_base: *mut c_void, pub iov_len: usize }
#[repr(C)] pub struct jffs2_raw_node_ref { pub flash_offset: u32, pub next_in_ino: *mut jffs2_raw_node_ref }
#[repr(C)] pub struct jffs2_eraseblock { pub unchecked_size: u32, pub used_size: u32 }
#[repr(C)] pub struct jffs2_sb_info { pub blocks: *mut jffs2_eraseblock, pub sector_size:u32, pub unchecked_size:u32, pub used_size:u32, pub highest_xid:u32, pub highest_xseqno:u32, pub xdatum_mem_usage:u32, pub xdatum_mem_threshold:u32, pub gcblock:*mut jffs2_eraseblock, pub xref_temp:*mut jffs2_xattr_ref, pub xref_dead_list:*mut jffs2_xattr_ref, pub flags:u32 }
#[repr(C)] pub struct jffs2_raw_xattr { pub magic:u16,pub nodetype:u16,pub totlen:u32,pub hdr_crc:u32,pub xid:u32,pub version:u32,pub xprefix:u8,pub name_len:u8,pub value_len:u16,pub data_crc:u32,pub node_crc:u32 }
#[repr(C)] pub struct jffs2_raw_xref { pub magic:u16,pub nodetype:u16,pub totlen:u32,pub hdr_crc:u32,pub ino:u32,pub xid:u32,pub xseqno:u32,pub node_crc:u32 }
#[repr(C)] pub struct jffs2_xattr_datum { pub node:*mut jffs2_raw_node_ref,pub xindex:list_head,pub xid:u32,pub version:u32,pub flags:u32,pub xprefix:i32,pub name_len:usize,pub value_len:u16,pub data_crc:u32,pub hashkey:u32,pub xname:*mut c_char,pub xvalue:*mut c_char,pub refcnt:atomic_t }
#[repr(C)] pub struct jffs2_xattr_ref { pub node:*mut jffs2_raw_node_ref,pub next:*mut jffs2_xattr_ref,pub ic:*mut jffs2_inode_cache,pub xd:*mut jffs2_xattr_datum,pub ino:u32,pub xid:u32,pub xseqno:u32 }
#[repr(C)] pub struct jffs2_inode_cache { pub xref:*mut jffs2_xattr_ref,pub ino:u32,pub pino_nlink:u32,pub flags:u32 }
#[repr(C)] pub struct jffs2_inode_info { pub inocache:*mut jffs2_inode_cache }
#[repr(C)] pub struct inode { pub i_sb:*mut c_void }
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct list_head { pub next:*mut list_head,pub prev:*mut list_head }
#[repr(C)] pub struct atomic_t { pub counter:i32 }
#[repr(C)] pub struct xattr_handler;

extern "C" { fn atomic_inc(a:*mut atomic_t); fn atomic_set(a:*mut atomic_t,v:i32); fn atomic_read(a:*mut atomic_t)->i32; fn atomic_dec_and_test(a:*mut atomic_t)->bool; fn atomic_dec_and_lock(a:*mut atomic_t,l:*mut c_void)->bool; }

// Synchronization, list, endian, diagnostics, and kernel helper macros are represented by port APIs.
extern "C" {
 fn jffs2_xattr_lock_read(c:*mut jffs2_sb_info); fn jffs2_xattr_unlock_read(c:*mut jffs2_sb_info);
 fn jffs2_xattr_lock_write(c:*mut jffs2_sb_info); fn jffs2_xattr_unlock_write(c:*mut jffs2_sb_info);
 fn list_add(n:*mut list_head,h:*mut list_head); fn list_add_tail(n:*mut list_head,h:*mut list_head); fn list_del(n:*mut list_head); fn list_del_init(n:*mut list_head); fn list_empty(h:*mut list_head)->bool;
 fn init_list_head(h:*mut list_head); fn jffs2_is_xattr_ref_dead(r:*mut jffs2_xattr_ref)->bool;
}

#[inline] unsafe fn unload_xattr_datum(c:*mut jffs2_sb_info, xd:*mut jffs2_xattr_datum) { if !(*xd).xname.is_null() { (*c).xdatum_mem_usage -= ((*xd).name_len+1+(*xd).value_len as usize) as u32; kfree((*xd).xname as *mut c_void); } list_del_init(&mut (*xd).xindex); (*xd).hashkey=0; (*xd).xname=core::ptr::null_mut(); (*xd).xvalue=core::ptr::null_mut(); }

pub unsafe fn jffs2_xattr_delete_inode(c:*mut jffs2_sb_info, ic:*mut jffs2_inode_cache) { if ic.is_null() || (*ic).pino_nlink>0{return} jffs2_xattr_lock_write(c); let mut r=(*ic).xref; while !r.is_null(){let n=(*r).next; delete_xattr_ref(c,r);r=n;} (*ic).xref=core::ptr::null_mut(); jffs2_xattr_unlock_write(c); }
pub unsafe fn jffs2_xattr_free_inode(c:*mut jffs2_sb_info, ic:*mut jffs2_inode_cache) { jffs2_xattr_lock_write(c); let mut r=(*ic).xref; while !r.is_null(){let n=(*r).next; let xd=(*r).xd; if atomic_dec_and_test(&mut (*xd).refcnt){unload_xattr_datum(c,xd);jffs2_free_xattr_datum(xd)} jffs2_free_xattr_ref(r);r=n;} (*ic).xref=core::ptr::null_mut(); jffs2_xattr_unlock_write(c); }

unsafe fn delete_xattr_ref(c:*mut jffs2_sb_info,r:*mut jffs2_xattr_ref){let xd=(*r).xd;(*r).xseqno|=0x80000000;(*r).ino=(*r).ic.as_ref().unwrap().ino;(*r).xid=(*xd).xid;(*r).next=(*c).xref_dead_list;(*c).xref_dead_list=r; unrefer_xattr_datum(c,xd)}
unsafe fn unrefer_xattr_datum(c:*mut jffs2_sb_info,xd:*mut jffs2_xattr_datum){if atomic_dec_and_lock(&mut (*xd).refcnt,c as *mut c_void){unload_xattr_datum(c,xd);(*xd).flags|=2; if (*xd).node==xd as *mut _ {jffs2_free_xattr_datum(xd)} }}

pub unsafe fn jffs2_init_xattr_subsystem(c:*mut jffs2_sb_info){(*c).xref_dead_list=core::ptr::null_mut();(*c).xref_temp=core::ptr::null_mut();(*c).highest_xid=0;(*c).highest_xseqno=0;(*c).xdatum_mem_usage=0;(*c).xdatum_mem_threshold=32*1024;}
pub unsafe fn jffs2_release_xattr_datum(_c:*mut jffs2_sb_info,xd:*mut jffs2_xattr_datum){if atomic_read(&mut (*xd).refcnt)!=0||(*xd).node!=xd as *mut _{return} jffs2_free_xattr_datum(xd)}
pub unsafe fn jffs2_release_xattr_ref(_c:*mut jffs2_sb_info,r:*mut jffs2_xattr_ref){if (*r).node==r as *mut _{jffs2_free_xattr_ref(r)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
