/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/fscrypt.h.  C headers and external kernel symbols
 * are intentionally left as dependencies supplied by the surrounding kernel. */

pub const FSCRYPT_CONTENTS_ALIGNMENT: usize = 16;
pub const FSCRYPT_SET_CONTEXT_MAX_SIZE: usize = 40;
pub const FSCRYPT_MAX_DEVICES: usize = 8;

#[repr(C)] pub struct fscrypt_policy { _private: [u8; 0] }
#[repr(C)] pub struct fscrypt_inode_info { _private: [u8; 0] }
#[repr(C)] pub struct fs_parameter { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }

#[repr(C)] pub struct fscrypt_str { pub name: *mut u8, pub len: u32 }
#[repr(C)] pub struct fscrypt_name {
    pub usr_fname: *const qstr, pub disk_name: fscrypt_str, pub hash: u32,
    pub minor_hash: u32, pub crypto_buf: fscrypt_str, pub is_nokey_name: bool,
}

#[repr(C)] pub struct qstr { pub name: *const u8, pub len: u32 }
#[repr(C)] pub struct inode { pub i_mode: u16, pub i_sb: *mut super_block }
#[repr(C)] pub struct super_block { pub s_cop: *const fscrypt_operations }
#[repr(C)] pub struct dentry { pub d_flags: usize, pub d_lock: spinlock_t, pub d_op: *const dentry_operations, pub d_name: qstr }
#[repr(C)] pub struct dentry_operations { pub d_revalidate: Option<unsafe extern "C" fn(*mut inode,*const qstr,*mut dentry,u32)->i32> }
#[repr(C)] pub struct page { pub mapping: *mut core::ffi::c_void }
#[repr(C)] pub struct folio { pub mapping: *mut core::ffi::c_void, pub private: *mut page }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct block_device { _private: [u8; 0] }
#[repr(C)] pub struct bio { _private: [u8; 0] }
#[repr(C)] pub struct iattr { _private: [u8; 0] }
#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct kstat { _private: [u8; 0] }
#[repr(C)] pub struct delayed_call { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
pub type gfp_t = usize; pub type loff_t = i64; pub type sector_t = u64;

pub const FSCRYPT_FLAG_PLACEHOLDER: u32 = 0; // build-time kernel constants are external

#[repr(C)] pub struct fscrypt_operations {
    pub inode_info_offs: isize,
    pub is_block_based: u32, pub needs_bounce_pages: u32, pub has_32bit_inodes: u32,
    pub supports_subblock_data_units: u32, pub legacy_key_prefix: *const i8,
    pub get_context: Option<unsafe extern "C" fn(*mut inode,*mut core::ffi::c_void,usize)->i32>,
    pub set_context: Option<unsafe extern "C" fn(*mut inode,*const core::ffi::c_void,usize,*mut core::ffi::c_void)->i32>,
    pub get_dummy_policy: Option<unsafe extern "C" fn(*mut super_block)->*const fscrypt_policy>,
    pub empty_dir: Option<unsafe extern "C" fn(*mut inode)->bool>,
    pub has_stable_inodes: Option<unsafe extern "C" fn(*mut super_block)->bool>,
    pub get_devices: Option<unsafe extern "C" fn(*mut super_block,*mut *mut block_device)->u32>,
}

#[inline] pub fn fstr_init(n:*mut u8,l:u32)->fscrypt_str { fscrypt_str{name:n,len:l} }
#[inline] pub unsafe fn fname_name(p:*const fscrypt_name)->*mut u8 { (*p).disk_name.name }
#[inline] pub unsafe fn fname_len(p:*const fscrypt_name)->u32 { (*p).disk_name.len }

extern "C" {
    pub fn fscrypt_d_revalidate(dir:*mut inode,name:*const qstr,dentry:*mut dentry,flags:u32)->i32;
    pub fn fscrypt_encrypt_pagecache_blocks(folio:*mut folio,len:usize,offs:usize,gfp:gfp_t)->*mut page;
    pub fn fscrypt_encrypt_block_inplace(inode:*const inode,page:*mut page,len:u32,offs:u32,lblk:u64)->i32;
    pub fn fscrypt_decrypt_block_inplace(inode:*const inode,page:*mut page,len:u32,offs:u32,lblk:u64)->i32;
    pub fn fscrypt_free_bounce_page(page:*mut page);
    pub fn fscrypt_ioctl_set_policy(filp:*mut file,arg:*const core::ffi::c_void)->i32;
    pub fn fscrypt_ioctl_get_policy(filp:*mut file,arg:*mut core::ffi::c_void)->i32;
    pub fn fscrypt_ioctl_get_policy_ex(filp:*mut file,arg:*mut core::ffi::c_void)->i32;
    pub fn fscrypt_ioctl_get_nonce(filp:*mut file,arg:*mut core::ffi::c_void)->i32;
    pub fn fscrypt_has_permitted_context(parent:*mut inode,child:*mut inode)->i32;
    pub fn fscrypt_context_for_new_inode(ctx:*mut core::ffi::c_void,inode:*mut inode)->i32;
    pub fn fscrypt_set_context(inode:*mut inode,fs_data:*mut core::ffi::c_void)->i32;
    pub fn fscrypt_destroy_keyring(sb:*mut super_block);
    pub fn fscrypt_prepare_new_inode(dir:*mut inode,inode:*mut inode,encrypt_ret:*mut bool)->i32;
    pub fn fscrypt_put_encryption_info(inode:*mut inode); pub fn fscrypt_free_inode(inode:*mut inode);
    pub fn fscrypt_drop_inode(inode:*mut inode)->i32;
    pub fn fscrypt_fname_encrypt(inode:*const inode,iname:*const qstr,out:*mut u8,olen:u32)->i32;
    pub fn fscrypt_fname_encrypted_size(inode:*const inode,orig:u32,max:u32,out:*mut u32)->bool;
    pub fn fscrypt_setup_filename(inode:*mut inode,iname:*const qstr,lookup:i32,fname:*mut fscrypt_name)->i32;
    pub fn fscrypt_fname_alloc_buffer(max:u32,s:*mut fscrypt_str)->i32; pub fn fscrypt_fname_free_buffer(s:*mut fscrypt_str);
    pub fn fscrypt_fname_disk_to_usr(inode:*const inode,hash:u32,minor:u32,i:*const fscrypt_str,o:*mut fscrypt_str)->i32;
    pub fn fscrypt_match_name(fname:*const fscrypt_name,de:*const u8,len:u32)->bool;
    pub fn fscrypt_fname_siphash(dir:*const inode,name:*const qstr)->u64;
    pub fn fscrypt_file_open(inode:*mut inode,filp:*mut file)->i32;
    pub fn fscrypt_prepare_setflags(inode:*mut inode,old:u32,flags:u32)->i32;
    pub fn fscrypt_prepare_symlink(dir:*mut inode,target:*const i8,len:u32,max:u32,link:*mut fscrypt_str)->i32;
}

#[repr(C)] pub struct fscrypt_dummy_policy { pub policy:*const fscrypt_policy }
#[inline] pub unsafe fn fscrypt_is_dummy_policy_set(p:*const fscrypt_dummy_policy)->bool { !(*p).policy.is_null() }

/* The CONFIG_FS_ENCRYPTION-disabled implementations retain the kernel's
 * return values and are represented here by the same external ABI. */
#[inline] pub unsafe fn fscrypt_is_bounce_page(p:*const page)->bool { (*p).mapping.is_null() }
#[inline] pub unsafe fn fscrypt_is_bounce_folio(p:*const folio)->bool { (*p).mapping.is_null() }
#[inline] pub unsafe fn fscrypt_has_encryption_key(i:*const inode)->bool { !fscrypt_get_inode_info(i).is_null() }
extern "C" { pub fn fscrypt_get_inode_info(inode:*const inode)->*mut fscrypt_inode_info; }

#[inline] pub unsafe fn fscrypt_prepare_link(old:*mut dentry,dir:*mut inode,d:*mut dentry)->i32 { if IS_ENCRYPTED(dir) { 0 } else { 0 } }
extern "C" { pub fn IS_ENCRYPTED(i:*const inode)->bool; }

extern "C" {
    pub fn fscrypt_parse_test_dummy_encryption(p:*const fs_parameter,d:*mut fscrypt_dummy_policy)->i32;
    pub fn fscrypt_dummy_policies_equal(a:*const fscrypt_dummy_policy,b:*const fscrypt_dummy_policy)->bool;
    pub fn fscrypt_show_test_dummy_encryption(s:*mut seq_file,sep:i8,sb:*mut super_block);
    pub fn fscrypt_free_dummy_policy(p:*mut fscrypt_dummy_policy);
    pub fn fscrypt_ioctl_add_key(f:*mut file,a:*mut core::ffi::c_void)->i32;
    pub fn fscrypt_ioctl_remove_key(f:*mut file,a:*mut core::ffi::c_void)->i32;
    pub fn fscrypt_ioctl_remove_key_all_users(f:*mut file,a:*mut core::ffi::c_void)->i32;
    pub fn fscrypt_ioctl_get_key_status(f:*mut file,a:*mut core::ffi::c_void)->i32;
    pub fn __fscrypt_prepare_link(i:*mut inode,d:*mut inode,e:*mut dentry)->i32;
    pub fn __fscrypt_prepare_rename(od:*mut inode,oe:*mut dentry,nd:*mut inode,ne:*mut dentry,flags:u32)->i32;
    pub fn __fscrypt_prepare_lookup(d:*mut inode,e:*mut dentry,n:*mut fscrypt_name)->i32;
    pub fn fscrypt_prepare_lookup_partial(d:*mut inode,e:*mut dentry)->i32;
    pub fn __fscrypt_prepare_readdir(d:*mut inode)->i32;
    pub fn __fscrypt_prepare_setattr(d:*mut dentry,a:*mut iattr)->i32;
    pub fn __fscrypt_encrypt_symlink(i:*mut inode,t:*const i8,l:u32,d:*mut fscrypt_str)->i32;
    pub fn fscrypt_get_symlink(i:*mut inode,c:*const core::ffi::c_void,m:u32,d:*mut delayed_call)->*const i8;
    pub fn fscrypt_symlink_getattr(p:*const path,s:*mut kstat)->i32;
    pub fn fscrypt_set_bio_crypt_ctx(b:*mut bio,i:*const inode,p:loff_t,g:gfp_t);
    pub fn fscrypt_mergeable_bio(b:*mut bio,i:*const inode,p:loff_t)->bool;
    pub fn fscrypt_limit_io_blocks(i:*const inode,l:u64,n:u64)->u64;
    pub fn fscrypt_zeroout_range(i:*const inode,p:loff_t,s:sector_t,l:u64)->i32;
}

#[inline] pub unsafe fn fscrypt_needs_contents_encryption(i:*const inode)->bool { IS_ENCRYPTED(i) && ((*i).i_mode & 0o170000)==0o100000 }
#[inline] pub unsafe fn fscrypt_prepare_readdir(i:*mut inode)->i32 { if IS_ENCRYPTED(i) { __fscrypt_prepare_readdir(i) } else { 0 } }
#[inline] pub unsafe fn fscrypt_prepare_setattr(d:*mut dentry,a:*mut iattr)->i32 { if IS_ENCRYPTED(core::ptr::null()) { __fscrypt_prepare_setattr(d,a) } else { 0 } }
#[inline] pub unsafe fn fscrypt_encrypt_symlink(i:*mut inode,t:*const i8,l:u32,d:*mut fscrypt_str)->i32 { if IS_ENCRYPTED(i) { __fscrypt_encrypt_symlink(i,t,l,d) } else { 0 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
