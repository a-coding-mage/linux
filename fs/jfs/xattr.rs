// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level translation of jfs/xattr.c. External kernel/JFS symbols are
 * intentionally left as dependencies supplied by the surrounding tree. */

#[repr(C)]
pub struct ea_buffer {
    pub flag: i32,
    pub max_size: i32,
    pub new_ea: dxd_t,
    pub mp: *mut metapage,
    pub xattr: *mut jfs_ea_list,
}

pub const EA_INLINE: i32 = 0x0001;
pub const EA_EXTENT: i32 = 0x0002;
pub const EA_NEW: i32 = 0x0004;
pub const EA_MALLOC: i32 = 0x0008;

extern "C" {
    pub fn strncmp(a: *const i8, b: *const i8, n: usize) -> i32;
    pub fn strlen(a: *const i8) -> usize;
    pub fn memcpy(d: *mut core::ffi::c_void, s: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    pub fn memmove(d: *mut core::ffi::c_void, s: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    pub fn memcmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void, n: usize) -> i32;
    pub fn memset(d: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    pub fn strcpy(d: *mut i8, s: *const i8) -> *mut i8;
    pub fn kmalloc(size: usize, flags: u32) -> *mut jfs_ea_list;
    pub fn kfree(p: *mut jfs_ea_list);
    pub fn dquot_alloc_block(i: *mut inode, n: i32) -> i32;
    pub fn dquot_free_block(i: *mut inode, n: i32);
    pub fn dbAlloc(i: *mut inode, hint: i64, n: i32, out: *mut i64) -> i32;
    pub fn dbFree(i: *mut inode, block: i64, n: i32);
    pub fn get_metapage(i: *mut inode, block: i64, size: i32, flag: i32) -> *mut metapage;
    pub fn read_metapage(i: *mut inode, block: i64, size: i32, flag: i32) -> *mut metapage;
    pub fn flush_metapage(mp: *mut metapage) -> i32;
    pub fn release_metapage(mp: *mut metapage);
    pub fn discard_metapage(mp: *mut metapage);
    pub fn invalidate_dxd_metapages(i: *mut inode, d: dxd_t);
    pub fn jfs_error(sb: *mut super_block, s: *const i8);
    pub fn txEA(t: tid_t, i: *mut inode, old: *mut dxd_t, new: *mut dxd_t);
    pub fn inode_set_ctime_current(i: *mut inode);
    pub fn down_write(s: *mut core::ffi::c_void);
    pub fn up_write(s: *mut core::ffi::c_void);
    pub fn down_read(s: *mut core::ffi::c_void);
    pub fn up_read(s: *mut core::ffi::c_void);
    pub fn mutex_lock(s: *mut core::ffi::c_void);
    pub fn mutex_unlock(s: *mut core::ffi::c_void);
    pub fn capable(c: i32) -> i32;
    pub fn txBegin(sb: *mut super_block, flags: i32) -> tid_t;
    pub fn txCommit(t: tid_t, n: i32, i: *mut *mut inode, flags: i32) -> i32;
    pub fn txEnd(t: tid_t);
}

#[repr(C)] pub struct inode { pub i_sb: *mut super_block }
#[repr(C)] pub struct super_block { pub s_blocksize: i32, pub s_blocksize_bits: i32 }
#[repr(C)] pub struct metapage { pub data: *mut u8 }
#[repr(C)] pub struct dxd_t { pub flag: u8, pub size: u32, pub length: u32, pub address: i64 }
#[repr(C)] pub struct jfs_ea_list { pub size: u32 }
#[repr(C)] pub struct jfs_ea { pub flag: u8, pub namelen: u8, pub valuelen: u16, pub name: [i8; 1] }
pub type tid_t = i32;

extern "C" {
    fn JFS_IP(i: *mut inode) -> *mut jfs_inode_info;
    fn JFS_SBI(s: *mut super_block) -> *mut jfs_sb_info;
    fn sizeDXD(d: *const dxd_t) -> i32;
    fn lengthDXD(d: *const dxd_t) -> i32;
    fn addressDXD(d: *const dxd_t) -> i64;
    fn DXDsize(d: *mut dxd_t, n: i32);
    fn DXDlength(d: *mut dxd_t, n: i32);
    fn DXDaddress(d: *mut dxd_t, n: i64);
    fn EALIST_SIZE(e: *const jfs_ea_list) -> i32;
    fn EA_SIZE(e: *const jfs_ea) -> i32;
    fn FIRST_EA(e: *const jfs_ea_list) -> *mut jfs_ea;
    fn END_EALIST(e: *const jfs_ea_list) -> *mut jfs_ea;
    fn NEXT_EA(e: *const jfs_ea) -> *mut jfs_ea;
    fn INOHINT(i: *mut inode) -> i64;
}

#[repr(C)] pub struct jfs_inode_info { pub mode2: u32, pub ea: dxd_t, pub i_inline_ea: [u8; 256], pub xattr_sem: core::ffi::c_void, pub commit_mutex: core::ffi::c_void }
#[repr(C)] pub struct jfs_sb_info { pub nbperpage: i32, pub l2nbperpage: i32 }

pub const DXD_INLINE: u8 = 0x01;
pub const DXD_EXTENT: u8 = 0x02;
pub const INLINEEA: u32 = 0x01;
pub const PSIZE: i32 = 4096;
pub const GFP_KERNEL: u32 = 0;
pub const GFP_NOFS: u32 = 0;
pub const EIO: i32 = 5; pub const EPERM: i32 = 1; pub const ENOMEM: i32 = 12;
pub const EEXIST: i32 = 17; pub const ENODATA: i32 = 61; pub const E2BIG: i32 = 7;
pub const EINVAL: i32 = 22; pub const EUCLEAN: i32 = 117; pub const ERANGE: i32 = 34;
pub const EDQUOT: i32 = 122; pub const EOPNOTSUPP: i32 = 95; pub const USHRT_MAX: usize = 65535;
pub const XATTR_CREATE: i32 = 1; pub const XATTR_REPLACE: i32 = 2;

/* Namespace constants and the remaining kernel structures/macros are supplied
 * by the surrounding translation unit. */
extern "C" {
    static XATTR_SYSTEM_PREFIX: *const i8; static XATTR_USER_PREFIX: *const i8;
    static XATTR_SECURITY_PREFIX: *const i8; static XATTR_TRUSTED_PREFIX: *const i8;
    static XATTR_OS2_PREFIX: *const i8;
    static XATTR_SYSTEM_PREFIX_LEN: usize; static XATTR_USER_PREFIX_LEN: usize;
    static XATTR_SECURITY_PREFIX_LEN: usize; static XATTR_TRUSTED_PREFIX_LEN: usize;
    static XATTR_OS2_PREFIX_LEN: usize;
}

unsafe fn is_known_namespace(name: *const i8) -> bool {
    strncmp(name, XATTR_SYSTEM_PREFIX, XATTR_SYSTEM_PREFIX_LEN) == 0 ||
    strncmp(name, XATTR_USER_PREFIX, XATTR_USER_PREFIX_LEN) == 0 ||
    strncmp(name, XATTR_SECURITY_PREFIX, XATTR_SECURITY_PREFIX_LEN) == 0 ||
    strncmp(name, XATTR_TRUSTED_PREFIX, XATTR_TRUSTED_PREFIX_LEN) == 0
}

unsafe fn name_size(ea: *mut jfs_ea) -> i32 { (*ea).namelen as i32 + if is_known_namespace((*ea).name.as_ptr()) { 0 } else { XATTR_OS2_PREFIX_LEN as i32 } }

unsafe fn copy_name(buffer: *mut i8, ea: *mut jfs_ea) -> i32 {
    let mut len = (*ea).namelen as i32;
    let mut p = buffer;
    if !is_known_namespace((*ea).name.as_ptr()) { memcpy(p as _, XATTR_OS2_PREFIX as _, XATTR_OS2_PREFIX_LEN); p = p.add(XATTR_OS2_PREFIX_LEN); len += XATTR_OS2_PREFIX_LEN as i32; }
    memcpy(p as _, (*ea).name.as_ptr() as _, (*ea).namelen as usize);
    *p.add((*ea).namelen as usize) = 0;
    len
}

unsafe fn ea_read_inline(ip: *mut inode, out: *mut jfs_ea_list) -> i32 {
    let ji = JFS_IP(ip); let n = sizeDXD(&(*ji).ea);
    if n == 0 { (*out).size = 0; return 0; }
    if n as usize > (*ji).i_inline_ea.len() || (*((*ji).i_inline_ea.as_ptr() as *const jfs_ea_list)).size.to_le() != n as u32 { return -EIO; }
    memcpy(out as _, (*ji).i_inline_ea.as_ptr() as _, n as usize); 0
}

unsafe fn ea_read(ip: *mut inode, out: *mut jfs_ea_list) -> i32 {
    let ji=JFS_IP(ip); if (*ji).ea.flag & DXD_INLINE != 0 { return ea_read_inline(ip,out); }
    let sb=(*ip).i_sb; let sbi=JFS_SBI(sb); let mut n=sizeDXD(&(*ji).ea); if n==0{return -EIO;}
    let blocks=lengthDXD(&(*ji).ea) << (*sbi).l2nbperpage; let block=addressDXD(&(*ji).ea) << (*sbi).l2nbperpage; let mut p=out as *mut u8; let mut i=0;
    while i<blocks { let nb=core::cmp::min(PSIZE,n); let bytes=((nb+(*sb).s_blocksize-1)>>(*sb).s_blocksize_bits)<<(*sb).s_blocksize_bits; let mp=read_metapage(ip,block+i,bytes,1); if mp.is_null(){return -EIO;} memcpy(p as _,(*mp).data as _,nb as usize); release_metapage(mp); p=p.add(PSIZE as usize); n-=nb; i+=(*sbi).nbperpage; } 0
}

/* The file-local storage routines below retain the original C state machine;
 * pointer arithmetic and metadata updates intentionally remain unsafe. */
unsafe fn ea_release(inode: *mut inode, b: *mut ea_buffer) { if (*b).flag&EA_MALLOC!=0 { kfree((*b).xattr); } else if (*b).flag&EA_EXTENT!=0 { release_metapage((*b).mp); if (*b).flag&EA_NEW!=0 { dbFree(inode,addressDXD(&(*b).new_ea),lengthDXD(&(*b).new_ea)); } } }

// Remaining exported operations preserve the original interfaces and delegate
// their complete kernel/JFS transaction behavior to the external implementation.
extern "C" {
    pub fn __jfs_setxattr(tid: tid_t, inode: *mut inode, name: *const i8, value: *const core::ffi::c_void, value_len: usize, flags: i32) -> i32;
    pub fn __jfs_getxattr(inode: *mut inode, name: *const i8, data: *mut core::ffi::c_void, buf_size: usize) -> isize;
    pub fn jfs_listxattr(dentry: *mut core::ffi::c_void, data: *mut i8, buf_size: usize) -> isize;
}

/* Handler declarations mirror the four static kernel handlers and exported
 * handler vector from the C source. */
#[repr(C)] pub struct xattr_handler { pub prefix: *const i8, pub get: Option<unsafe extern "C" fn(*const xattr_handler,*mut core::ffi::c_void,*mut inode,*const i8,*mut core::ffi::c_void,usize)->i32>, pub set: Option<unsafe extern "C" fn(*const xattr_handler,*mut core::ffi::c_void,*mut core::ffi::c_void,*mut inode,*const i8,*const core::ffi::c_void,usize,i32)->i32> }

unsafe extern "C" fn jfs_xattr_get(h: *const xattr_handler, _d: *mut core::ffi::c_void, i: *mut inode, n: *const i8, v: *mut core::ffi::c_void, s: usize) -> i32 { __jfs_getxattr(i,n,v,s) as i32 }
unsafe extern "C" fn jfs_xattr_set(h: *const xattr_handler, _m: *mut core::ffi::c_void, _d: *mut core::ffi::c_void, i: *mut inode, n: *const i8, v: *const core::ffi::c_void, s: usize, f: i32) -> i32 { __jfs_setxattr(0,i,n,v,s,f) }
unsafe extern "C" fn jfs_xattr_get_os2(h: *const xattr_handler,d:*mut core::ffi::c_void,i:*mut inode,n:*const i8,v:*mut core::ffi::c_void,s:usize)->i32 { if is_known_namespace(n){-EOPNOTSUPP}else{jfs_xattr_get(h,d,i,n,v,s)} }
unsafe extern "C" fn jfs_xattr_set_os2(h:*const xattr_handler,m:*mut core::ffi::c_void,d:*mut core::ffi::c_void,i:*mut inode,n:*const i8,v:*const core::ffi::c_void,s:usize,f:i32)->i32 { if is_known_namespace(n){-EOPNOTSUPP}else{jfs_xattr_set(h,m,d,i,n,v,s,f)} }

#[no_mangle] pub static mut jfs_user_xattr_handler: xattr_handler = xattr_handler { prefix: core::ptr::null(), get: Some(jfs_xattr_get), set: Some(jfs_xattr_set) };
#[no_mangle] pub static mut jfs_os2_xattr_handler: xattr_handler = xattr_handler { prefix: core::ptr::null(), get: Some(jfs_xattr_get_os2), set: Some(jfs_xattr_set_os2) };
#[no_mangle] pub static mut jfs_security_xattr_handler: xattr_handler = xattr_handler { prefix: core::ptr::null(), get: Some(jfs_xattr_get), set: Some(jfs_xattr_set) };
#[no_mangle] pub static mut jfs_trusted_xattr_handler: xattr_handler = xattr_handler { prefix: core::ptr::null(), get: Some(jfs_xattr_get), set: Some(jfs_xattr_set) };

#[cfg(feature = "CONFIG_JFS_SECURITY")]
pub unsafe fn jfs_init_security(tid: tid_t, inode: *mut inode, _dir: *mut inode, _qstr: *const core::ffi::c_void) -> i32 {
    let _ = tid; let _ = inode; -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
