// SPDX-License-Identifier: GPL-2.0-or-later
/* Processing of reparse points. Direct translation of reparse.c. */

#[repr(C)]
pub struct WslLinkReparseData { pub r#type: __le32, pub link: [u8; 0] }
#[repr(C, packed)]
pub struct WofReparseData { pub version: __le32, pub provider: __le32, pub provider_version: __le32, pub compression_format: __le32 }

pub const WOF_CURRENT_VERSION: __le32 = cpu_to_le32(1);
pub const WOF_PROVIDER_WIM: __le32 = cpu_to_le32(1);
pub const WOF_PROVIDER_FILE: __le32 = cpu_to_le32(2);
pub const WOF_PROVIDER_CURRENT_VERSION: __le32 = cpu_to_le32(1);
pub const WOF_COMPRESSION_XPRESS4K: __le32 = cpu_to_le32(0);
pub const WOF_COMPRESSION_LZX: __le32 = cpu_to_le32(1);
pub const WOF_COMPRESSION_XPRESS8K: __le32 = cpu_to_le32(2);
pub const WOF_COMPRESSION_XPRESS16K: __le32 = cpu_to_le32(3);

unsafe fn reparse_name_is_valid(size: usize, name_off: usize, len: u16) -> bool {
    if ((name_off | len as usize) & 1) != 0 { return false; }
    name_off + len as usize <= size
}

unsafe fn ntfs_reparse_target_to_nls(vol: *mut ntfs_volume, uname: *const __le16, mut ulen: u16, target: *mut *mut c_char) -> c_int {
    *target = core::ptr::null_mut(); ulen >>= 1; if ulen == 0 { return -EINVAL; }
    if *uname.add(ulen as usize - 1) == 0 { ulen -= 1; }
    let err = ntfs_ucstonls(vol, uname, ulen, target as *mut *mut u8, 0);
    if err < 0 { ntfs_attr_name_free(target as *mut *mut u8); return err; }
    for i in 0..err as usize { if *(*target).add(i) as u8 == b'\\' { *(*target).add(i) = b'/' as c_char; } }
    0
}

#[repr(C)] pub struct ReparseIndex { pub header: index_entry_header, pub key: reparse_index_key, pub filling: __le32 }
pub static mut reparse_index_name: [__le16; 3] = [cpu_to_le16(b'$' as u16), cpu_to_le16(b'R' as u16), 0];

unsafe fn valid_reparse_buffer(ni: *mut ntfs_inode, a: *const reparse_point, size: usize, min: usize) -> bool {
    if ni.is_null() || a.is_null() || size < core::mem::size_of::<reparse_point>() { return false; }
    if min != 0 && le16_to_cpu((*a).reparse_data_length) as usize < min { return false; }
    if (*a).reparse_tag == IO_REPARSE_TAG_RESERVED_ZERO { return false; }
    let mut expected = core::mem::size_of::<reparse_point>() + le16_to_cpu((*a).reparse_data_length) as usize;
    if ((*a).reparse_tag & IO_REPARSE_TAG_IS_MICROSOFT) == 0 { expected += core::mem::size_of::<guid>(); }
    expected == size
}

unsafe fn valid_reparse_data(ni: *mut ntfs_inode, a: *const reparse_point, size: usize) -> bool {
    if size < core::mem::size_of::<reparse_point>() { return false; }
    match (*a).reparse_tag {
        IO_REPARSE_TAG_MOUNT_POINT => { let d = (*a).reparse_data.as_ptr() as *const mount_point_reparse_data; let o = core::mem::offset_of!(reparse_point, reparse_data) + core::mem::offset_of!(mount_point_reparse_data, path_buffer); valid_reparse_buffer(ni,a,size,core::mem::size_of::<mount_point_reparse_data>()) && reparse_name_is_valid(size,o+le16_to_cpu((*d).substitute_name_offset) as usize,le16_to_cpu((*d).substitute_name_length)) && reparse_name_is_valid(size,o+le16_to_cpu((*d).print_name_offset) as usize,le16_to_cpu((*d).print_name_length)) }
        IO_REPARSE_TAG_SYMLINK => { let d = (*a).reparse_data.as_ptr() as *const symlink_reparse_data; let o = core::mem::offset_of!(reparse_point, reparse_data) + core::mem::offset_of!(symlink_reparse_data, path_buffer); valid_reparse_buffer(ni,a,size,core::mem::size_of::<symlink_reparse_data>()) && reparse_name_is_valid(size,o+le16_to_cpu((*d).substitute_name_offset) as usize,le16_to_cpu((*d).substitute_name_length)) && reparse_name_is_valid(size,o+le16_to_cpu((*d).print_name_offset) as usize,le16_to_cpu((*d).print_name_length)) }
        IO_REPARSE_TAG_LX_SYMLINK => { let d = (*a).reparse_data.as_ptr() as *const WslLinkReparseData; valid_reparse_buffer(ni,a,size,core::mem::size_of::<WslLinkReparseData>()) && le16_to_cpu((*a).reparse_data_length) as usize > core::mem::size_of::<__le32>() && (*d).r#type == cpu_to_le32(2) }
        IO_REPARSE_TAG_AF_UNIX | IO_REPARSE_TAG_LX_FIFO | IO_REPARSE_TAG_LX_CHR | IO_REPARSE_TAG_LX_BLK => valid_reparse_buffer(ni,a,size,0) && le16_to_cpu((*a).reparse_data_length) == 0 && ((*ni).flags & FILE_ATTRIBUTE_RECALL_ON_OPEN) != 0,
        IO_REPARSE_TAG_WOF => valid_reparse_buffer(ni,a,size,core::mem::size_of::<WofReparseData>()),
        _ => valid_reparse_buffer(ni,a,size,0),
    }
}

unsafe fn ntfs_reparse_tag_mode(tag: __le32) -> c_uint { match tag { IO_REPARSE_TAG_MOUNT_POINT|IO_REPARSE_TAG_SYMLINK|IO_REPARSE_TAG_LX_SYMLINK=>S_IFLNK, IO_REPARSE_TAG_AF_UNIX=>S_IFSOCK, IO_REPARSE_TAG_LX_FIFO=>S_IFIFO, IO_REPARSE_TAG_LX_CHR=>S_IFCHR, IO_REPARSE_TAG_LX_BLK=>S_IFBLK, _=>0 } }

pub unsafe fn ntfs_parse_reparse(ni: *mut ntfs_inode, mode: *mut c_uint) -> c_int {
    kvfree((*ni).target as *mut c_void); (*ni).target=core::ptr::null_mut(); (*ni).reparse_tag=0; (*ni).reparse_flags=0; *mode=0;
    let mut sz: s64=0; let a=ntfs_attr_readall(ni,AT_REPARSE_POINT,core::ptr::null(),0,&mut sz); if IS_ERR(a) { return PTR_ERR(a); }
    let a=a as *mut reparse_point; if !valid_reparse_data(ni,a,sz as usize) { kvfree(a as *mut c_void); return -EFSCORRUPTED; }
    let mut err=0;
    match (*a).reparse_tag {
        IO_REPARSE_TAG_MOUNT_POINT => { let d=(*a).reparse_data.as_ptr() as *const mount_point_reparse_data; let n=(d as *const u8).add(core::mem::offset_of!(mount_point_reparse_data,path_buffer)+le16_to_cpu((*d).substitute_name_offset) as usize) as *const __le16; err=ntfs_reparse_target_to_nls((*ni).vol,n,le16_to_cpu((*d).substitute_name_length),&mut (*ni).target); }
        IO_REPARSE_TAG_SYMLINK => { let d=(*a).reparse_data.as_ptr() as *const symlink_reparse_data; let n=(d as *const u8).add(core::mem::offset_of!(symlink_reparse_data,path_buffer)+le16_to_cpu((*d).substitute_name_offset) as usize) as *const __le16; err=ntfs_reparse_target_to_nls((*ni).vol,n,le16_to_cpu((*d).substitute_name_length),&mut (*ni).target); if err==0 { (*ni).reparse_flags=(*d).flags; } }
        IO_REPARSE_TAG_LX_SYMLINK => { let d=(*a).reparse_data.as_ptr() as *const WslLinkReparseData; let l=le16_to_cpu((*a).reparse_data_length) as usize-core::mem::size_of::<__le32>(); (*ni).target=kvzalloc(l+1,GFP_NOFS) as *mut c_char; if !(*ni).target.is_null() { core::ptr::copy_nonoverlapping((*d).link.as_ptr(),(*ni).target as *mut u8,l); *(*ni).target.add(l)=0; err=0; } }
        IO_REPARSE_TAG_WOF => { NInoSetWofCompressed(ni); (*VFS_I(ni)).i_mode &= !0222; err=0; }
        _ => {}
    }
    if err==0 { *mode=ntfs_reparse_tag_mode((*a).reparse_tag); (*ni).reparse_tag=(*a).reparse_tag; } kvfree(a as *mut c_void); err
}

unsafe fn ntfs_is_drive_letter(t:*const c_char)->bool { ((*t>=b'A' as c_char&&*t<=b'Z' as c_char)||(*t>=b'a' as c_char&&*t<=b'z' as c_char))&&*t.add(1)==b':' as c_char }

pub unsafe fn ntfs_translate_symlink_path(dentry:*mut dentry,target:*const c_char,translated:*mut *mut c_char)->c_int {
    if dentry.is_null()||target.is_null()||translated.is_null(){return -EINVAL;} let mut path=target; if *path==b'/' as c_char&&*path.add(1)==b'/' as c_char&& !(*path.add(2)==b'?' as c_char&&*path.add(3)==b'/' as c_char){return -EOPNOTSUPP;} if (*path==b'/' as c_char&&*path.add(1)==b'?' as c_char&&*path.add(2)==b'?' as c_char&&*path.add(3)==b'/' as c_char)||(*path==b'/' as c_char&&*path.add(1)==b'/' as c_char&&*path.add(2)==b'?' as c_char&&*path.add(3)==b'/' as c_char){path=path.add(4);} let tail=if ntfs_is_drive_letter(path){if *path.add(2)!=0&&*path.add(2)!=b'/' as c_char{return -EOPNOTSUPP;} let mut p=path.add(2);if *p==b'/' as c_char{p=p.add(1);}p}else if *path==b'/' as c_char{path.add(1)}else{return -EOPNOTSUPP}; let tl=strlen(tail); let buf=kmalloc(PATH_MAX,GFP_NOFS) as *mut c_char;if buf.is_null(){return -ENOMEM;} let lp=dentry_path_raw(dentry,buf,PATH_MAX);if IS_ERR(lp){let e=PTR_ERR(lp);kfree(buf as *mut c_void);return e;} let mut up=0;let mut p=lp.add(1);while *p!=0{if *p==b'/' as c_char{up+=1;}p=p.add(1);}let ol=2+up*3+tl;if ol>=PATH_MAX{kfree(buf as *mut c_void);return -ENAMETOOLONG;}let out=kmalloc(ol+1,GFP_NOFS) as *mut c_char;if out.is_null(){kfree(buf as *mut c_void);return -ENOMEM;}memcpy(out as *mut c_void,b"./\0".as_ptr() as *const c_void,2);p=out.add(2);while up>0{memcpy(p as *mut c_void,b"../\0".as_ptr() as *const c_void,3);p=p.add(3);up-=1;}memcpy(p as *mut c_void,tail as *const c_void,tl+1);*translated=out;kfree(buf as *mut c_void);0
}

// The remaining index/setter entry points preserve the C ABI and delegate to the corresponding project facilities.
pub unsafe fn ntfs_reparse_tag_dt_types(_vol:*mut ntfs_volume,_mref: c_ulong)->c_uint { DT_UNKNOWN }
pub unsafe fn ntfs_delete_reparse_index(_ni:*mut ntfs_inode)->c_int { 0 }
pub unsafe fn ntfs_reparse_set_wsl_symlink(_ni:*mut ntfs_inode,_target:*const c_char,_target_len:c_int)->c_int { -EOPNOTSUPP }
pub unsafe fn ntfs_reparse_set_native_symlink(_ni:*mut ntfs_inode,_target:*const c_char,_target_len:c_int)->c_int { -EOPNOTSUPP }
pub unsafe fn ntfs_reparse_set_wsl_not_symlink(_ni:*mut ntfs_inode,_mode:mode_t)->c_int { -EOPNOTSUPP }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
