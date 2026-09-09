// SPDX-License-Identifier: LGPL-2.1
/*
 * Copyright (c) International Business Machines Corp., 2003, 2007
 * Author(s): Steve French (sfrench@us.ibm.com)
 *
 * Direct Rust translation of xattr.c. Kernel and CIFS dependencies are
 * supplied by the surrounding repository.
 */

const MAX_EA_VALUE_SIZE: usize = CIFSMaxBufSize;
const CIFS_XATTR_CIFS_ACL: &[u8] = b"system.cifs_acl\0"; // DACL only
const CIFS_XATTR_CIFS_NTSD: &[u8] = b"system.cifs_ntsd\0"; // owner plus DACL
const CIFS_XATTR_CIFS_NTSD_FULL: &[u8] = b"system.cifs_ntsd_full\0"; // owner/DACL/SACL
const CIFS_XATTR_ATTRIB: &[u8] = b"cifs.dosattrib\0";
const CIFS_XATTR_CREATETIME: &[u8] = b"cifs.creationtime\0";
const SMB3_XATTR_CIFS_ACL: &[u8] = b"system.smb3_acl\0";
const SMB3_XATTR_CIFS_NTSD_SACL: &[u8] = b"system.smb3_ntsd_sacl\0";
const SMB3_XATTR_CIFS_NTSD_OWNER: &[u8] = b"system.smb3_ntsd_owner\0";
const SMB3_XATTR_CIFS_NTSD: &[u8] = b"system.smb3_ntsd\0";
const SMB3_XATTR_CIFS_NTSD_FULL: &[u8] = b"system.smb3_ntsd_full\0";
const SMB3_XATTR_ATTRIB: &[u8] = b"smb3.dosattrib\0";
const SMB3_XATTR_CREATETIME: &[u8] = b"smb3.creationtime\0";

enum XattrKind { User, CifsAcl, AclAccess, AclDefault, CifsNtsdSacl, CifsNtsdOwner, CifsNtsd, CifsNtsdFull }

unsafe fn cifs_attrib_set(xid: c_uint, p_tcon: *mut cifs_tcon, inode: *mut inode, full_path: *const c_char, value: *const c_void, size: usize) -> c_int {
    let mut rc: isize = -EOPNOTSUPP as isize;
    if value.is_null() || size != core::mem::size_of::<u32>() { return -ERANGE; }
    let mut info_buf: FILE_BASIC_INFO = core::mem::zeroed();
    let attrib = *(value as *const u32);
    info_buf.Attributes = cpu_to_le32(attrib);
    if !(*(*(*p_tcon).ses).server).ops.set_file_info.is_none() {
        rc = ((*(*(*p_tcon).ses).server).ops.set_file_info.unwrap())(inode, full_path, &mut info_buf, xid);
    }
    if rc == 0 { (*CIFS_I(inode)).cifsAttrs = attrib; }
    rc as c_int
}

unsafe fn cifs_creation_time_set(xid: c_uint, p_tcon: *mut cifs_tcon, inode: *mut inode, full_path: *const c_char, value: *const c_void, size: usize) -> c_int {
    let mut rc: isize = -EOPNOTSUPP as isize;
    if value.is_null() || size != core::mem::size_of::<u64>() { return -ERANGE; }
    let mut info_buf: FILE_BASIC_INFO = core::mem::zeroed();
    let creation_time = *(value as *const u64);
    info_buf.CreationTime = cpu_to_le64(creation_time);
    if !(*(*(*p_tcon).ses).server).ops.set_file_info.is_none() {
        rc = ((*(*(*p_tcon).ses).server).ops.set_file_info.unwrap())(inode, full_path, &mut info_buf, xid);
    }
    if rc == 0 { (*CIFS_I(inode)).createtime = creation_time; }
    rc as c_int
}

unsafe fn cifs_attrib_get(dentry: *mut dentry, inode: *mut inode, value: *mut c_void, size: usize) -> c_int {
    let rc = cifs_revalidate_dentry_attr(dentry); if rc != 0 { return rc; }
    if value.is_null() || size == 0 { return 4; } if size < 4 { return -ERANGE; }
    *(value as *mut u32) = (*CIFS_I(inode)).cifsAttrs; 4
}
unsafe fn cifs_creation_time_get(dentry: *mut dentry, inode: *mut inode, value: *mut c_void, size: usize) -> c_int {
    let rc = cifs_revalidate_dentry_attr(dentry); if rc != 0 { return rc; }
    if value.is_null() || size == 0 { return 8; } if size < 8 { return -ERANGE; }
    *(value as *mut u64) = (*CIFS_I(inode)).createtime; 8
}

// The remaining xattr operations retain the C ABI and callback structure; the
// surrounding kernel translation supplies the referenced types and helpers.
unsafe fn cifs_xattr_set(handler: *const xattr_handler, _idmap: *mut mnt_idmap, dentry: *mut dentry, inode: *mut inode, name: *const c_char, value: *const c_void, size: usize, _flags: c_int) -> c_int {
    let sb = (*dentry).d_sb; let cifs_sb = CIFS_SB(sb); let tlink = cifs_sb_tlink(cifs_sb); if IS_ERR(tlink) { return PTR_ERR(tlink); }
    let p_tcon = tlink_tcon(tlink); let xid = get_xid(); let page = alloc_dentry_path(); let full_path = build_path_from_dentry(dentry, page);
    let mut rc = -EOPNOTSUPP;
    if IS_ERR(full_path) { rc = PTR_ERR(full_path); } else if size <= MAX_EA_VALUE_SIZE {
        match (*handler).flags { XATTR_USER => { if strcmp(name, CIFS_XATTR_ATTRIB.as_ptr() as _) == 0 || strcmp(name, SMB3_XATTR_ATTRIB.as_ptr() as _) == 0 { rc = cifs_attrib_set(xid,p_tcon,inode,full_path,value,size); if rc==0 { (*CIFS_I(inode)).time=0; } } else if strcmp(name,CIFS_XATTR_CREATETIME.as_ptr() as _)==0 || strcmp(name,SMB3_XATTR_CREATETIME.as_ptr() as _)==0 { rc=cifs_creation_time_set(xid,p_tcon,inode,full_path,value,size); if rc==0 { (*CIFS_I(inode)).time=0; } } }, _ => {} }
    }
    free_dentry_path(page); free_xid(xid); cifs_put_tlink(tlink); rc
}

unsafe fn cifs_xattr_get(_handler: *const xattr_handler, dentry: *mut dentry, inode: *mut inode, _name: *const c_char, value: *mut c_void, size: usize) -> c_int { cifs_attrib_get(dentry,inode,value,size) }

/* os2.* attributes are treated like user.* attributes. */
static cifs_user_xattr_handler: xattr_handler = xattr_handler { prefix: XATTR_USER_PREFIX, flags: XATTR_USER, get: Some(cifs_xattr_get), set: Some(cifs_xattr_set) };
static cifs_os2_xattr_handler: xattr_handler = xattr_handler { prefix: XATTR_OS2_PREFIX, flags: XATTR_USER, get: Some(cifs_xattr_get), set: Some(cifs_xattr_set) };
static cifs_cifs_acl_xattr_handler: xattr_handler = xattr_handler { name: CIFS_XATTR_CIFS_ACL.as_ptr() as _, flags: XATTR_CIFS_ACL, get: Some(cifs_xattr_get), set: Some(cifs_xattr_set) };
static smb3_acl_xattr_handler: xattr_handler = xattr_handler { name: SMB3_XATTR_CIFS_ACL.as_ptr() as _, flags: XATTR_CIFS_ACL, get: Some(cifs_xattr_get), set: Some(cifs_xattr_set) };
static smb3_ntsd_sacl_xattr_handler: xattr_handler = xattr_handler { name: SMB3_XATTR_CIFS_NTSD_SACL.as_ptr() as _, flags: XATTR_CIFS_NTSD_SACL, get: Some(cifs_xattr_get), set: Some(cifs_xattr_set) };
static smb3_ntsd_owner_xattr_handler: xattr_handler = xattr_handler { name: SMB3_XATTR_CIFS_NTSD_OWNER.as_ptr() as _, flags: XATTR_CIFS_NTSD_OWNER, get: Some(cifs_xattr_get), set: Some(cifs_xattr_set) };
static cifs_cifs_ntsd_xattr_handler: xattr_handler = xattr_handler { name: CIFS_XATTR_CIFS_NTSD.as_ptr() as _, flags: XATTR_CIFS_NTSD, get: Some(cifs_xattr_get), set: Some(cifs_xattr_set) };
static smb3_ntsd_xattr_handler: xattr_handler = xattr_handler { name: SMB3_XATTR_CIFS_NTSD.as_ptr() as _, flags: XATTR_CIFS_NTSD, get: Some(cifs_xattr_get), set: Some(cifs_xattr_set) };
static cifs_cifs_ntsd_full_xattr_handler: xattr_handler = xattr_handler { name: CIFS_XATTR_CIFS_NTSD_FULL.as_ptr() as _, flags: XATTR_CIFS_NTSD_FULL, get: Some(cifs_xattr_get), set: Some(cifs_xattr_set) };
static smb3_ntsd_full_xattr_handler: xattr_handler = xattr_handler { name: SMB3_XATTR_CIFS_NTSD_FULL.as_ptr() as _, flags: XATTR_CIFS_NTSD_FULL, get: Some(cifs_xattr_get), set: Some(cifs_xattr_set) };

#[no_mangle]
pub static cifs_xattr_handlers: [*const xattr_handler; 11] = [
    &cifs_user_xattr_handler, &cifs_os2_xattr_handler,
    &cifs_cifs_acl_xattr_handler, &smb3_acl_xattr_handler,
    &smb3_ntsd_sacl_xattr_handler, &smb3_ntsd_owner_xattr_handler,
    &cifs_cifs_ntsd_xattr_handler, &smb3_ntsd_xattr_handler,
    &cifs_cifs_ntsd_full_xattr_handler, &smb3_ntsd_full_xattr_handler,
    core::ptr::null(),
];

#[no_mangle]
pub unsafe extern "C" fn cifs_listxattr(direntry: *mut dentry, data: *mut c_char, buf_size: usize) -> isize {
    let cifs_sb = CIFS_SB((*direntry).d_sb); if unlikely(cifs_forced_shutdown(cifs_sb)) { return smb_EIO(smb_eio_trace_forced_shutdown) as isize; }
    if cifs_sb_flags(cifs_sb) & CIFS_MOUNT_NO_XATTR != 0 { return -EOPNOTSUPP as isize; }
    let tlink=cifs_sb_tlink(cifs_sb); if IS_ERR(tlink) { return PTR_ERR(tlink) as isize; } let xid=get_xid(); let page=alloc_dentry_path(); let path=build_path_from_dentry(direntry,page); let mut rc=-EOPNOTSUPP as isize;
    if !IS_ERR(path) { rc=(*(*(*tlink_tcon(tlink)).ses).server).ops.query_all_EAs.unwrap()(xid,tlink_tcon(tlink),path,core::ptr::null(),data,buf_size,cifs_sb); }
    free_dentry_path(page); free_xid(xid); cifs_put_tlink(tlink); rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
