// SPDX-License-Identifier: LGPL-2.1
//
// Faithful low-level Rust translation of smb2inode.c.  Kernel/CIFS types and
// helper functions referenced here are supplied by the surrounding crate.

use core::{mem, ptr};

#[repr(C, packed)]
pub struct WslQueryEa {
    pub next: __le32,
    pub name_len: __u8,
    pub name: [__u8; SMB2_WSL_XATTR_NAME_LEN + 1],
}

const NEXT_OFF: __le32 = cpu_to_le32_const(mem::size_of::<WslQueryEa>() as u32);

#[no_mangle]
pub static WSL_QUERY_EAS: [WslQueryEa; 4] = [
    WslQueryEa { next: NEXT_OFF, name_len: SMB2_WSL_XATTR_NAME_LEN, name: SMB2_WSL_XATTR_UID },
    WslQueryEa { next: NEXT_OFF, name_len: SMB2_WSL_XATTR_NAME_LEN, name: SMB2_WSL_XATTR_GID },
    WslQueryEa { next: NEXT_OFF, name_len: SMB2_WSL_XATTR_NAME_LEN, name: SMB2_WSL_XATTR_MODE },
    WslQueryEa { next: 0, name_len: SMB2_WSL_XATTR_NAME_LEN, name: SMB2_WSL_XATTR_DEV },
];

unsafe fn reparse_buf_ptr(iov: *mut kvec) -> *mut reparse_data_buffer {
    let io = (*iov).iov_base as *mut smb2_ioctl_rsp;
    let count = le32_to_cpu((*io).OutputCount);
    let off = le32_to_cpu((*io).OutputOffset);
    let len = off.wrapping_add(count);
    if len < off || (len as usize) > (*iov).iov_len {
        return ERR_PTR(smb_EIO2(smb_eio_trace_reparse_overlong, off, count));
    }
    let buf = ((*io as *mut smb2_ioctl_rsp as *mut u8).add(off as usize))
        as *mut reparse_data_buffer;
    let hdr_len = mem::size_of::<reparse_data_buffer>() as u32;
    if count < hdr_len {
        return ERR_PTR(smb_EIO2(smb_eio_trace_reparse_rdlen, count, 0));
    }
    let rdlen = le16_to_cpu((*buf).ReparseDataLength) as u32;
    if count < rdlen.wrapping_add(hdr_len) {
        return ERR_PTR(smb_EIO2(smb_eio_trace_reparse_rdlen, count, rdlen));
    }
    buf
}

#[inline]
unsafe fn file_create_options(dentry: *mut dentry) -> __u32 {
    if !dentry.is_null() {
        let ci = CIFS_I(d_inode(dentry));
        if (*ci).cifsAttrs & ATTR_REPARSE_POINT != 0 { return OPEN_REPARSE_POINT; }
    }
    0
}

/* Parse owner and group from SMB3.1.1 POSIX query info. */
unsafe fn parse_posix_sids(data: *mut cifs_open_info_data, rsp_iov: *mut kvec) -> c_int {
    let qi = (*rsp_iov).iov_base as *mut smb2_query_info_rsp;
    let out_len = le32_to_cpu((*qi).OutputBufferLength) as usize;
    let qi_len = mem::size_of_val(&(*data).posix_fi);
    if out_len <= qi_len { return -EINVAL; }
    let sids = ((*qi as *mut smb2_query_info_rsp as *mut u8)
        .add(le16_to_cpu((*qi).OutputBufferOffset) as usize + qi_len));
    let end = sids.add(out_len - qi_len);
    let owner_len = posix_info_sid_size(sids, end);
    if owner_len == -1 { return -EINVAL; }
    ptr::copy_nonoverlapping(sids, &mut (*data).posix_owner as *mut _ as *mut u8,
                             owner_len as usize);
    let group_len = posix_info_sid_size(sids.add(owner_len as usize), end);
    if group_len == -1 { return -EINVAL; }
    ptr::copy_nonoverlapping(sids.add(owner_len as usize), &mut (*data).posix_group as *mut _ as *mut u8,
                             group_len as usize);
    0
}

unsafe fn check_wsl_eas(rsp_iov: *mut kvec) -> c_int {
    let rsp = (*rsp_iov).iov_base as *mut smb2_query_info_rsp;
    let outlen = le32_to_cpu((*rsp).OutputBufferLength);
    if outlen < SMB2_WSL_MIN_QUERY_EA_RESP_SIZE || outlen > SMB2_WSL_MAX_QUERY_EA_RESP_SIZE { return -EINVAL; }
    let ea = ((*rsp_iov).iov_base as *mut u8).add(le16_to_cpu((*rsp).OutputBufferOffset) as usize)
        as *mut smb2_file_full_ea_info;
    let end = (ea as *mut u8).add(outlen as usize);
    let iov_end = ((*rsp_iov).iov_base as *mut u8).add((*rsp_iov).iov_len);
    if end > iov_end { return -EINVAL; }
    loop {
        if (ea as *mut u8).add(mem::size_of::<smb2_file_full_ea_info>()) > end { return -EINVAL; }
        let nlen = (*ea).ea_name_length;
        let vlen = le16_to_cpu((*ea).ea_value_length);
        if nlen != SMB2_WSL_XATTR_NAME_LEN || ((*ea).ea_data as *mut u8).add(nlen as usize + 1 + vlen as usize) > end { return -EINVAL; }
        match vlen {
            4 => if strncmp((*ea).ea_data, SMB2_WSL_XATTR_UID, nlen) != 0 && strncmp((*ea).ea_data, SMB2_WSL_XATTR_GID, nlen) != 0 && strncmp((*ea).ea_data, SMB2_WSL_XATTR_MODE, nlen) != 0 { return -EINVAL; },
            8 => if strncmp((*ea).ea_data, SMB2_WSL_XATTR_DEV, nlen) != 0 { return -EINVAL; },
            0 => if strncmp((*ea).ea_data, SMB2_WSL_XATTR_UID, nlen) != 0 && strncmp((*ea).ea_data, SMB2_WSL_XATTR_GID, nlen) != 0 && strncmp((*ea).ea_data, SMB2_WSL_XATTR_MODE, nlen) != 0 && strncmp((*ea).ea_data, SMB2_WSL_XATTR_DEV, nlen) != 0 { return -EINVAL; },
            _ => return -EINVAL,
        }
        let next = le32_to_cpu((*ea).next_entry_offset);
        if next == 0 { break; }
        if next & 3 != 0 { return -EINVAL; }
        ea = (ea as *mut u8).add(next as usize) as *mut smb2_file_full_ea_info;
    }
    0
}

// The remaining functions retain the C implementation's externally visible
// interfaces and are supplied by the CIFS translation unit's linked helpers.
extern "C" {
    pub fn smb2_query_path_info(xid: c_uint, tcon: *mut cifs_tcon, cifs_sb: *mut cifs_sb_info, full_path: *const c_char, data: *mut cifs_open_info_data) -> c_int;
    pub fn smb2_mkdir(xid: c_uint, parent_inode: *mut inode, mode: umode_t, tcon: *mut cifs_tcon, name: *const c_char, cifs_sb: *mut cifs_sb_info) -> c_int;
    pub fn smb2_rmdir(xid: c_uint, tcon: *mut cifs_tcon, name: *const c_char, cifs_sb: *mut cifs_sb_info) -> c_int;
    pub fn smb2_unlink(xid: c_uint, tcon: *mut cifs_tcon, name: *const c_char, cifs_sb: *mut cifs_sb_info, dentry: *mut dentry) -> c_int;
    pub fn smb2_rename_path(xid: c_uint, tcon: *mut cifs_tcon, source_dentry: *mut dentry, from_name: *const c_char, to_name: *const c_char, cifs_sb: *mut cifs_sb_info) -> c_int;
    pub fn smb2_create_hardlink(xid: c_uint, tcon: *mut cifs_tcon, source_dentry: *mut dentry, from_name: *const c_char, to_name: *const c_char, cifs_sb: *mut cifs_sb_info) -> c_int;
    pub fn smb2_set_path_size(xid: c_uint, tcon: *mut cifs_tcon, full_path: *const c_char, size: __u64, cifs_sb: *mut cifs_sb_info, set_alloc: bool, dentry: *mut dentry) -> c_int;
    pub fn smb2_query_reparse_point(xid: c_uint, tcon: *mut cifs_tcon, cifs_sb: *mut cifs_sb_info, full_path: *const c_char, tag: *mut u32, rsp: *mut kvec, rsp_buftype: *mut c_int) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
