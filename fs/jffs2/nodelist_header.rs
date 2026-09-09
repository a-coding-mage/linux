/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

// C dependencies: linux/fs.h, linux/types.h, linux/jffs2.h,
// jffs2_fs_sb.h, jffs2_fs_i.h, xattr.h, acl.h, summary.h, os-linux.h/os-ecos.h.

pub const JFFS2_NATIVE_ENDIAN: bool = true;

// Native-endian conversions (the source selects these with build-time defines).
#[inline] pub unsafe fn cpu_to_je16(x: u16) -> jint16_t { jint16_t { v16: x } }
#[inline] pub unsafe fn cpu_to_je32(x: u32) -> jint32_t { jint32_t { v32: x } }
#[inline] pub unsafe fn cpu_to_jemode(x: u32) -> jmode_t { jmode_t { m: os_to_jffs2_mode(x) } }
#[inline] pub const fn constant_cpu_to_je16(x: u16) -> jint16_t { jint16_t { v16: x } }
#[inline] pub const fn constant_cpu_to_je32(x: u32) -> jint32_t { jint32_t { v32: x } }
#[inline] pub const unsafe fn je16_to_cpu(x: jint16_t) -> u16 { x.v16 }
#[inline] pub const unsafe fn je32_to_cpu(x: jint32_t) -> u32 { x.v32 }
#[inline] pub unsafe fn jemode_to_cpu(x: jmode_t) -> u32 { jffs2_to_os_mode(x.m) }

// External types and functions supplied by the included headers.
#[repr(C)] pub struct jint16_t { pub v16: u16 }
#[repr(C)] pub struct jint32_t { pub v32: u32 }
#[repr(C)] pub struct jmode_t { pub m: u32 }
extern "C" { fn os_to_jffs2_mode(x: u32) -> u32; fn jffs2_to_os_mode(x: u32) -> u32; }

pub const JFFS2_MIN_NODE_HEADER: usize = core::mem::size_of::<jffs2_raw_dirent>();

#[repr(C)] pub struct jffs2_raw_node_ref {
    pub next_in_ino: *mut jffs2_raw_node_ref,
    pub flash_offset: u32,
}
pub const REF_LINK_NODE: i32 = -1;
pub const REF_EMPTY_NODE: i32 = -2;
pub const REFS_PER_BLOCK: usize = (255 / core::mem::size_of::<jffs2_raw_node_ref>()) - 1;

#[inline] pub unsafe fn ref_next(mut r: *mut jffs2_raw_node_ref) -> *mut jffs2_raw_node_ref {
    r = r.add(1);
    if (*r).flash_offset as i32 == REF_LINK_NODE { r = (*r).next_in_ino; if r.is_null() { return r; } }
    if (*r).flash_offset as i32 == REF_EMPTY_NODE { return core::ptr::null_mut(); }
    r
}
#[inline] pub unsafe fn jffs2_raw_ref_to_ic(mut raw: *mut jffs2_raw_node_ref) -> *mut jffs2_inode_cache {
    while !(*raw).next_in_ino.is_null() { raw = (*raw).next_in_ino; }
    raw as *mut jffs2_inode_cache
}

pub const REF_UNCHECKED: u32 = 0; pub const REF_OBSOLETE: u32 = 1;
pub const REF_PRISTINE: u32 = 2; pub const REF_NORMAL: u32 = 3;
#[inline] pub unsafe fn ref_flags(r: *const jffs2_raw_node_ref) -> u32 { (*r).flash_offset & 3 }
#[inline] pub unsafe fn ref_offset(r: *const jffs2_raw_node_ref) -> u32 { (*r).flash_offset & !3 }
#[inline] pub unsafe fn ref_obsolete(r: *const jffs2_raw_node_ref) -> bool { (*r).flash_offset & 3 == REF_OBSOLETE }
#[inline] pub unsafe fn mark_ref_normal(r: *mut jffs2_raw_node_ref) { (*r).flash_offset = ref_offset(r) | REF_NORMAL; }
#[inline] pub unsafe fn dirent_node_state(rd: *const jffs2_raw_dirent) -> u32 { if je32_to_cpu((*rd).ino) != 0 { REF_PRISTINE } else { REF_NORMAL } }

#[repr(C)] pub struct jffs2_inode_cache {
    pub scan_dents: *mut jffs2_full_dirent, pub nodes: *mut jffs2_raw_node_ref, pub class: u8,
    pub flags: u8, pub state: u16, pub ino: u32, pub next: *mut jffs2_inode_cache,
    #[cfg(feature = "CONFIG_JFFS2_FS_XATTR")] pub xref: *mut jffs2_xattr_ref,
    pub pino_nlink: u32,
}
pub const INO_STATE_UNCHECKED:u16=0; pub const INO_STATE_CHECKING:u16=1; pub const INO_STATE_PRESENT:u16=2;
pub const INO_STATE_CHECKEDABSENT:u16=3; pub const INO_STATE_GC:u16=4; pub const INO_STATE_READING:u16=5; pub const INO_STATE_CLEARING:u16=6;
pub const INO_FLAGS_XATTR_CHECKED:u8=0x01; pub const INO_FLAGS_IS_DIR:u8=0x02;
pub const RAWNODE_CLASS_INODE_CACHE:u8=0; pub const RAWNODE_CLASS_XATTR_DATUM:u8=1; pub const RAWNODE_CLASS_XATTR_REF:u8=2;
pub const INOCACHE_HASHSIZE_MIN: usize=128; pub const INOCACHE_HASHSIZE_MAX: usize=1024;

#[repr(C)] pub struct jffs2_full_dnode { pub raw:*mut jffs2_raw_node_ref, pub ofs:u32, pub size:u32, pub frags:u32 }
#[repr(C)] pub struct jffs2_tmp_dnode_info { pub rb: rb_node, pub fn_:*mut jffs2_full_dnode, pub version:u32, pub data_crc:u32, pub partial_crc:u32, pub csize:u32, pub overlapped:u16 }
#[repr(C)] pub struct jffs2_readinode_info { pub tn_root:rb_root, pub mdata_tn:*mut jffs2_tmp_dnode_info, pub highest_version:u32, pub latest_mctime:u32, pub mctime_ver:u32, pub fds:*mut jffs2_full_dirent, pub latest_ref:*mut jffs2_raw_node_ref }
#[repr(C)] pub union jffs2_full_dirent_raw { pub raw:*mut jffs2_raw_node_ref, pub ic:*mut jffs2_inode_cache }
#[repr(C)] pub struct jffs2_full_dirent { pub raw:jffs2_full_dirent_raw, pub next:*mut jffs2_full_dirent, pub version:u32, pub ino:u32, pub nhash:u32, pub type_:u8, pub name:[u8;0] }
#[repr(C)] pub struct jffs2_node_frag { pub rb:rb_node, pub node:*mut jffs2_full_dnode, pub size:u32, pub ofs:u32 }
#[repr(C)] pub struct jffs2_eraseblock { pub list:list_head, pub bad_count:i32, pub offset:u32, pub unchecked_size:u32, pub used_size:u32, pub dirty_size:u32, pub wasted_size:u32, pub free_size:u32, pub allocated_refs:u32, pub first_node:*mut jffs2_raw_node_ref, pub last_node:*mut jffs2_raw_node_ref, pub gc_node:*mut jffs2_raw_node_ref }

// Types supplied by kernel/JFFS2 headers.
#[repr(C)] pub struct rb_node { _private:[u8;0] } #[repr(C)] pub struct rb_root { _private:[u8;0] }
#[repr(C)] pub struct list_head { _private:[u8;0] }
#[repr(C)] pub struct jffs2_raw_dirent { pub ino:jint32_t }
#[repr(C)] pub struct jffs2_sb_info { pub flash_size:u32, pub sector_size:u32 }
#[repr(C)] pub struct jffs2_inode_info; #[repr(C)] pub struct jffs2_raw_inode; #[repr(C)] pub union jffs2_device_node { pub old_id:jint16_t, pub new_id:jint32_t }
#[repr(C)] pub struct qstr; #[repr(C)] pub struct jffs2_xattr_ref; #[repr(C)] pub struct jffs2_xattr_datum;

#[inline] pub unsafe fn jffs2_blocks_use_vmalloc(c:*const jffs2_sb_info)->i32 { if ((*c).flash_size / (*c).sector_size) * core::mem::size_of::<jffs2_eraseblock>() as u32 > 128*1024 {1} else {0} }
pub const ALLOC_NORMAL:i32=0; pub const ALLOC_DELETION:i32=1; pub const ALLOC_GC:i32=2; pub const ALLOC_NORETRY:i32=3;
#[inline] pub unsafe fn verydirty(c:*const jffs2_sb_info,size:u32)->bool { size >= (*c).sector_size/2 }
#[inline] pub const fn isdirty(size:usize)->bool { size > core::mem::size_of::<jffs2_raw_inode>() + JFFS2_MIN_DATA_LEN }
#[inline] pub const fn pad(x:usize)->usize {(x+3)&!3}
pub const JFFS2_MIN_DATA_LEN:usize=0;

extern "C" {
    fn old_valid_dev(rdev: u64)->bool; fn old_encode_dev(rdev:u64)->u16; fn new_encode_dev(rdev:u64)->u32;
    fn rb_first(root:*mut rb_root)->*mut rb_node; fn rb_last(root:*mut rb_root)->*mut rb_node;
    fn __jffs2_ref_totlen(c:*mut jffs2_sb_info,jeb:*mut jffs2_eraseblock,ref_:*mut jffs2_raw_node_ref)->u32;
}
#[inline] pub unsafe fn jffs2_encode_dev(jdev:*mut jffs2_device_node,rdev:u64)->i32 { if old_valid_dev(rdev) { (*jdev).old_id=cpu_to_je16(old_encode_dev(rdev)); core::mem::size_of::<jint16_t>() as i32 } else { (*jdev).new_id=cpu_to_je32(new_encode_dev(rdev)); core::mem::size_of::<jint32_t>() as i32 } }

// Remaining declarations from nodelist.c, nodemgmt.c, write.c, readinode.c,
// malloc.c, gc.c, read.c, scan.c, build.c, erase.c, and wbuf.c.
extern "C" {
    pub fn jffs2_add_fd_to_list(c:*mut jffs2_sb_info,new_:*mut jffs2_full_dirent,list:*mut *mut jffs2_full_dirent);
    pub fn jffs2_set_inocache_state(c:*mut jffs2_sb_info,ic:*mut jffs2_inode_cache,state:i32);
    pub fn jffs2_get_ino_cache(c:*mut jffs2_sb_info,ino:u32)->*mut jffs2_inode_cache;
    pub fn jffs2_free_ino_caches(c:*mut jffs2_sb_info); pub fn jffs2_free_raw_node_refs(c:*mut jffs2_sb_info);
    pub fn jffs2_create_slab_caches()->i32; pub fn jffs2_destroy_slab_caches();
    pub fn jffs2_garbage_collect_pass(c:*mut jffs2_sb_info)->i32; pub fn jffs2_scan_medium(c:*mut jffs2_sb_info)->i32;
    pub fn jffs2_do_mount_fs(c:*mut jffs2_sb_info)->i32;
    pub fn jffs2_add_ino_cache(c:*mut jffs2_sb_info,new_:*mut jffs2_inode_cache);
    pub fn jffs2_del_ino_cache(c:*mut jffs2_sb_info,old:*mut jffs2_inode_cache);
    pub fn jffs2_lookup_node_frag(t:*mut rb_root,offset:u32)->*mut jffs2_node_frag;
    pub fn jffs2_kill_fragtree(root:*mut rb_root,c_delete:*mut jffs2_sb_info);
    pub fn jffs2_add_full_dnode_to_inode(c:*mut jffs2_sb_info,f:*mut jffs2_inode_info,fn_:*mut jffs2_full_dnode)->i32;
    pub fn jffs2_truncate_fragtree(c:*mut jffs2_sb_info,list:*mut rb_root,size:u32)->u32;
    pub fn jffs2_link_node_ref(c:*mut jffs2_sb_info,jeb:*mut jffs2_eraseblock,ofs:u32,len:u32,ic:*mut jffs2_inode_cache)->*mut jffs2_raw_node_ref;
    pub fn jffs2_thread_should_wake(c:*mut jffs2_sb_info)->i32;
    pub fn jffs2_reserve_space(c:*mut jffs2_sb_info,minsize:u32,len:*mut u32,prio:i32,sumsize:u32)->i32;
    pub fn jffs2_reserve_space_gc(c:*mut jffs2_sb_info,minsize:u32,len:*mut u32,sumsize:u32)->i32;
    pub fn jffs2_add_physical_node_ref(c:*mut jffs2_sb_info,ofs:u32,len:u32,ic:*mut jffs2_inode_cache)->*mut jffs2_raw_node_ref;
    pub fn jffs2_complete_reservation(c:*mut jffs2_sb_info);
    pub fn jffs2_mark_node_obsolete(c:*mut jffs2_sb_info,raw:*mut jffs2_raw_node_ref);
    pub fn jffs2_do_new_inode(c:*mut jffs2_sb_info,f:*mut jffs2_inode_info,mode:u32,ri:*mut jffs2_raw_inode)->i32;
    pub fn jffs2_write_dnode(c:*mut jffs2_sb_info,f:*mut jffs2_inode_info,ri:*mut jffs2_raw_inode,data:*const u8,datalen:u32,alloc_mode:i32)->*mut jffs2_full_dnode;
    pub fn jffs2_write_dirent(c:*mut jffs2_sb_info,f:*mut jffs2_inode_info,rd:*mut jffs2_raw_dirent,name:*const u8,namelen:u32,alloc_mode:i32)->*mut jffs2_full_dirent;
    pub fn jffs2_write_inode_range(c:*mut jffs2_sb_info,f:*mut jffs2_inode_info,ri:*mut jffs2_raw_inode,buf:*mut u8,offset:u32,writelen:u32,retlen:*mut u32)->i32;
    pub fn jffs2_do_create(c:*mut jffs2_sb_info,dir_f:*mut jffs2_inode_info,f:*mut jffs2_inode_info,ri:*mut jffs2_raw_inode,qstr:*const qstr)->i32;
    pub fn jffs2_do_unlink(c:*mut jffs2_sb_info,dir_f:*mut jffs2_inode_info,name:*const i8,namelen:i32,dead_f:*mut jffs2_inode_info,time:u32)->i32;
    pub fn jffs2_do_link(c:*mut jffs2_sb_info,dir_f:*mut jffs2_inode_info,ino:u32,type_:u8,name:*const i8,namelen:i32,time:u32)->i32;
    pub fn jffs2_do_read_inode(c:*mut jffs2_sb_info,f:*mut jffs2_inode_info,ino:u32,latest_node:*mut jffs2_raw_inode)->i32;
    pub fn jffs2_do_crccheck_inode(c:*mut jffs2_sb_info,ic:*mut jffs2_inode_cache)->i32;
    pub fn jffs2_do_clear_inode(c:*mut jffs2_sb_info,f:*mut jffs2_inode_info);
    pub fn jffs2_alloc_full_dirent(namesize:i32)->*mut jffs2_full_dirent; pub fn jffs2_free_full_dirent(p:*mut jffs2_full_dirent);
    pub fn jffs2_alloc_full_dnode()->*mut jffs2_full_dnode; pub fn jffs2_free_full_dnode(p:*mut jffs2_full_dnode);
    pub fn jffs2_alloc_raw_dirent()->*mut jffs2_raw_dirent; pub fn jffs2_free_raw_dirent(p:*mut jffs2_raw_dirent);
    pub fn jffs2_alloc_raw_inode()->*mut jffs2_raw_inode; pub fn jffs2_free_raw_inode(p:*mut jffs2_raw_inode);
    pub fn jffs2_alloc_tmp_dnode_info()->*mut jffs2_tmp_dnode_info; pub fn jffs2_free_tmp_dnode_info(p:*mut jffs2_tmp_dnode_info);
    pub fn jffs2_prealloc_raw_node_refs(c:*mut jffs2_sb_info,jeb:*mut jffs2_eraseblock,nr:i32)->i32;
    pub fn jffs2_free_refblock(p:*mut jffs2_raw_node_ref); pub fn jffs2_alloc_node_frag()->*mut jffs2_node_frag; pub fn jffs2_free_node_frag(p:*mut jffs2_node_frag);
    pub fn jffs2_alloc_inode_cache()->*mut jffs2_inode_cache; pub fn jffs2_free_inode_cache(p:*mut jffs2_inode_cache);
    pub fn jffs2_read_dnode(c:*mut jffs2_sb_info,f:*mut jffs2_inode_info,fd:*mut jffs2_full_dnode,buf:*mut u8,ofs:i32,len:i32)->i32;
    pub fn jffs2_read_inode_range(c:*mut jffs2_sb_info,f:*mut jffs2_inode_info,buf:*mut u8,offset:u32,len:u32)->i32;
    pub fn jffs2_getlink(c:*mut jffs2_sb_info,f:*mut jffs2_inode_info)->*mut i8;
    pub fn jffs2_rotate_lists(c:*mut jffs2_sb_info); pub fn jffs2_scan_make_ino_cache(c:*mut jffs2_sb_info,ino:u32)->*mut jffs2_inode_cache;
    pub fn jffs2_scan_classify_jeb(c:*mut jffs2_sb_info,jeb:*mut jffs2_eraseblock)->i32; pub fn jffs2_scan_dirty_space(c:*mut jffs2_sb_info,jeb:*mut jffs2_eraseblock,size:u32)->i32;
    pub fn jffs2_erase_pending_blocks(c:*mut jffs2_sb_info,count:i32)->i32; pub fn jffs2_free_jeb_node_refs(c:*mut jffs2_sb_info,jeb:*mut jffs2_eraseblock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
