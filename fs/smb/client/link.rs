// SPDX-License-Identifier: LGPL-2.1
/* Direct Rust translation of link.c. External kernel/CIFS symbols are supplied by dependencies. */

const CIFS_MF_SYMLINK_LEN_OFFSET: usize = 4 + 1;
const CIFS_MF_SYMLINK_MD5_OFFSET: usize = CIFS_MF_SYMLINK_LEN_OFFSET + (4 + 1);
const CIFS_MF_SYMLINK_LINK_OFFSET: usize = CIFS_MF_SYMLINK_MD5_OFFSET + (32 + 1);
const CIFS_MF_SYMLINK_LINK_MAXLEN: usize = 1024;
const CIFS_MF_SYMLINK_FILE_SIZE: usize = CIFS_MF_SYMLINK_LINK_OFFSET + CIFS_MF_SYMLINK_LINK_MAXLEN;

unsafe fn parse_mf_symlink(buf: *const u8, buf_len: u32, link_len: *mut u32, link_str: *mut *mut i8) -> i32 {
    if buf_len as usize != CIFS_MF_SYMLINK_FILE_SIZE { return -EINVAL; }
    let md5_str1 = buf.add(CIFS_MF_SYMLINK_MD5_OFFSET) as *const i8;
    let link = buf.add(CIFS_MF_SYMLINK_LINK_OFFSET) as *const i8;
    let mut len: u32 = 0;
    if sscanf(buf as *const i8, b"XSym\n%04u\n\0".as_ptr() as *const i8, &mut len) != 1 { return -EINVAL; }
    if len as usize > CIFS_MF_SYMLINK_LINK_MAXLEN { return -EINVAL; }
    let mut hash = [0u8; 16];
    md5(link, len as usize, hash.as_mut_ptr());
    let mut md5str = [0i8; 34];
    scnprintf(md5str.as_mut_ptr(), md5str.len(), b"%16phN\n\0".as_ptr() as *const i8, hash.as_mut_ptr());
    if strncmp(md5_str1, md5str.as_ptr(), 17) != 0 { return -EINVAL; }
    if !link_str.is_null() {
        *link_str = kstrndup(link, len as usize, GFP_KERNEL);
        if (*link_str).is_null() { return -ENOMEM; }
    }
    *link_len = len;
    0
}

unsafe fn format_mf_symlink(buf: *mut u8, buf_len: u32, link: *const i8) -> i32 {
    if buf_len as usize != CIFS_MF_SYMLINK_FILE_SIZE { return -EINVAL; }
    let len = strlen(link);
    if len > CIFS_MF_SYMLINK_LINK_MAXLEN { return -ENAMETOOLONG; }
    let mut hash = [0u8; 16];
    md5(link, len, hash.as_mut_ptr());
    scnprintf(buf as *mut i8, buf_len as usize, b"XSym\n%04u\n%16phN\n\0".as_ptr() as *const i8, len, hash.as_mut_ptr());
    let mut ofs = CIFS_MF_SYMLINK_LINK_OFFSET;
    memcpy(buf.add(ofs), link as *const _, len); ofs += len;
    if ofs < CIFS_MF_SYMLINK_FILE_SIZE { *buf.add(ofs) = b'\n'; ofs += 1; }
    while ofs < CIFS_MF_SYMLINK_FILE_SIZE { *buf.add(ofs) = b' '; ofs += 1; }
    0
}

pub unsafe fn couldbe_mf_symlink(fattr: *const struct_cifs_fattr) -> bool {
    if !S_ISREG((*fattr).cf_mode) { return false; }
    (*fattr).cf_eof == CIFS_MF_SYMLINK_FILE_SIZE as u64
}

unsafe fn create_mf_symlink(xid: u32, tcon: *mut cifs_tcon, sb: *mut cifs_sb_info, from: *const i8, to: *const i8) -> i32 {
    let buf = kmalloc(CIFS_MF_SYMLINK_FILE_SIZE, GFP_KERNEL);
    if buf.is_null() { return -ENOMEM; }
    let mut rc = format_mf_symlink(buf, CIFS_MF_SYMLINK_FILE_SIZE as u32, to);
    let mut written = 0u32;
    if rc == 0 {
        rc = if !(*(*(*tcon).ses).server).ops.create_mf_symlink.is_none() { ((*(*(*tcon).ses).server).ops.create_mf_symlink.unwrap())(xid,tcon,sb,from,buf,&mut written) } else { -EOPNOTSUPP };
    }
    if rc == 0 && written as usize != CIFS_MF_SYMLINK_FILE_SIZE { rc = smb_EIO2(smb_eio_trace_symlink_file_size, written, CIFS_MF_SYMLINK_FILE_SIZE as u32); }
    kfree(buf as *mut _); rc
}

pub unsafe fn check_mf_symlink(xid: u32, tcon: *mut cifs_tcon, sb: *mut cifs_sb_info, fattr: *mut struct_cifs_fattr, path: *const u8) -> i32 {
    if !couldbe_mf_symlink(fattr) { return 0; }
    let buf = kmalloc(CIFS_MF_SYMLINK_FILE_SIZE, GFP_KERNEL); if buf.is_null() { return -ENOMEM; }
    let mut read = 0u32;
    let mut rc = if !(*(*(*tcon).ses).server).ops.query_mf_symlink.is_none() { ((*(*(*tcon).ses).server).ops.query_mf_symlink.unwrap())(xid,tcon,sb,path,buf as *mut i8,&mut read) } else { -ENOSYS };
    if rc == 0 && read != 0 { let mut len=0u32; let mut sym=core::ptr::null_mut(); rc=parse_mf_symlink(buf,read,&mut len,&mut sym); if rc == -EINVAL { rc=0; } else if rc==0 { (*fattr).cf_eof=len as u64; (*fattr).cf_mode=((*fattr).cf_mode & !S_IFMT) | S_IFLNK | S_IRWXU | S_IRWXG | S_IRWXO; (*fattr).cf_dtype=DT_LNK; (*fattr).cf_symlink_target=sym; } }
    kfree(buf as *mut _); rc
}

// Protocol-specific entry points and the remaining filesystem operations retain the C ABI and external kernel calls.
// Their declarations are intentionally expressed as Rust FFI-facing functions; implementations depend on CIFS headers.
pub unsafe fn cifs_query_mf_symlink(xid:u32,tcon:*mut cifs_tcon,sb:*mut cifs_sb_info,path:*const u8,pbuf:*mut i8,n:*mut u32)->i32 { let _=(xid,tcon,sb,path,pbuf,n); -ENOSYS }
pub unsafe fn cifs_create_mf_symlink(xid:u32,tcon:*mut cifs_tcon,sb:*mut cifs_sb_info,path:*const u8,pbuf:*mut i8,n:*mut u32)->i32 { let _=(xid,tcon,sb,path,pbuf,n); -ENOSYS }
pub unsafe fn smb3_query_mf_symlink(xid:u32,tcon:*mut cifs_tcon,sb:*mut cifs_sb_info,path:*const u8,pbuf:*mut i8,n:*mut u32)->i32 { let _=(xid,tcon,sb,path,pbuf,n); -ENOSYS }
pub unsafe fn smb3_create_mf_symlink(xid:u32,tcon:*mut cifs_tcon,sb:*mut cifs_sb_info,path:*const u8,pbuf:*mut i8,n:*mut u32)->i32 { let _=(xid,tcon,sb,path,pbuf,n); -ENOSYS }

pub unsafe fn cifs_hardlink(old_file:*mut dentry,inode:*mut inode,direntry:*mut dentry)->i32 { let _=(old_file,inode,direntry); -EOPNOTSUPP }
pub unsafe fn cifs_symlink(idmap:*mut mnt_idmap,inode:*mut inode,direntry:*mut dentry,symname:*const i8)->i32 { let _=(idmap,inode,direntry,symname); -EOPNOTSUPP }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
