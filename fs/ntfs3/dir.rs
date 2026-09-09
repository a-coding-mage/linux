// SPDX-License-Identifier: GPL-2.0
/* Directory handling functions for NTFS-based filesystems. */

// Dependencies supplied by the surrounding kernel/Rust translation.

pub unsafe fn ntfs_utf16_to_nls(sbi: *mut ntfs_sb_info, name: *const __le16, mut len: u32, buf: *mut u8, mut buf_len: i32) -> i32 {
    if buf_len <= 0 { return -EINVAL; }
    buf_len -= 1;
    let nls = (*(*sbi).options).nls;
    if nls.is_null() {
        let ret = utf16s_to_utf8s(name as *const wchar_t, len, UTF16_LITTLE_ENDIAN, buf, buf_len);
        *buf.add(ret as usize) = 0;
        return ret;
    }
    let mut op = buf;
    let mut warn = 0;
    while len != 0 {
        len -= 1;
        if buf_len < NLS_MAX_CHARSET_SIZE {
            ntfs_warn((*sbi).sb, c"filename was truncated while converting.".as_ptr());
            break;
        }
        let ec = le16_to_cpu(*name);
        let charlen = ((*nls).uni2char)(ec, op, buf_len);
        if charlen > 0 { op = op.add(charlen as usize); buf_len -= charlen; } else {
            *op = b'_'; op = op.add(1); buf_len -= 1;
            if warn == 0 { warn = 1; let mut dump = [0i8; 5]; hex_byte_pack(dump.as_mut_ptr(), ec >> 8); hex_byte_pack(dump.as_mut_ptr().add(2), ec); dump[4] = 0; ntfs_err((*sbi).sb, c"failed to convert \"%s\" to %s".as_ptr(), dump.as_ptr(), (*nls).charset); }
        }
    }
    *op = 0; op.offset_from(buf) as i32
}

const PLANE_SIZE: u32 = 0x00010000;
const SURROGATE_PAIR: u32 = 0x0000d800;
const SURROGATE_LOW: u32 = 0x00000400;
const SURROGATE_BITS: u32 = 0x000003ff;

#[inline] unsafe fn put_utf16(s: *mut wchar_t, c: u32, endian: utf16_endian) {
    match endian { UTF16_LITTLE_ENDIAN => * (s as *mut __le16) = __cpu_to_le16(c), UTF16_BIG_ENDIAN => *(s as *mut __be16) = __cpu_to_be16(c), _ => *s = c as wchar_t }
}

unsafe fn _utf8s_to_utf16s(mut s: *const u8, mut inlen: i32, endian: utf16_endian, pwcs: *mut wchar_t, mut maxout: i32) -> i32 {
    let op = pwcs as *mut u16; let mut out = op; let mut u = 0u32;
    while inlen > 0 && *s != 0 {
        if *s & 0x80 != 0 {
            let size = utf8_to_utf32(s, inlen, &mut u); if size < 0 { return -EINVAL; }
            s = s.add(size as usize); inlen -= size;
            if u >= PLANE_SIZE { if maxout < 2 { return -ENAMETOOLONG; } u -= PLANE_SIZE; put_utf16(out as *mut wchar_t, SURROGATE_PAIR | ((u >> 10) & SURROGATE_BITS), endian); out = out.add(1); put_utf16(out as *mut wchar_t, SURROGATE_PAIR | SURROGATE_LOW | (u & SURROGATE_BITS), endian); out = out.add(1); maxout -= 2; }
            else { if maxout < 1 { return -ENAMETOOLONG; } put_utf16(out as *mut wchar_t, u, endian); out = out.add(1); maxout -= 1; }
        } else { if maxout < 1 { return -ENAMETOOLONG; } put_utf16(out as *mut wchar_t, *s as u32, endian); s = s.add(1); inlen -= 1; maxout -= 1; }
    }
    out.offset_from(op) as i32
}

pub unsafe fn ntfs_nls_to_utf16(sbi: *mut ntfs_sb_info, mut name: *const u8, name_len: u32, uni: *mut cpu_str, max_ulen: u32, endian: utf16_endian) -> i32 {
    let nls = (*(*sbi).options).nls; let uname = (*uni).name; let mut ret: i32;
    if nls.is_null() { ret = _utf8s_to_utf16s(name, name_len as i32, endian, uname as *mut wchar_t, max_ulen as i32); }
    else { let end = name.add(name_len as usize); let mut slen; ret = 0; while name < end { if ret as u32 >= max_ulen { return -ENAMETOOLONG; } slen = ((*nls).char2uni)(name, end.offset_from(name) as i32, uname.add(ret as usize)); if slen == 0 { return -EINVAL; } if slen < 0 { return slen; } ret += 1; name = name.add(slen as usize); } }
    (*uni).len = ret; (*uni).ads_len = 0;
    if ret > 0 && (*(*sbi).options).ads { let mut i = 1; while i + 1 < ret { if *uname.add(i as usize) == b':' as u16 { (*uni).ads_len = ret - i - 1; (*uni).len = i; *uname.add(i as usize) = 0; break; } i += 1; } }
    ret
}

pub unsafe fn dir_search_flags(dir: *mut inode, uni: *const cpu_str, mut fnd: *mut ntfs_fnd, flags: u32) -> *mut inode {
    let sb = (*dir).i_sb; let sbi = (*sb).s_fs_info; let ni = ntfs_i(dir); let mut fnd_a = core::ptr::null_mut(); let mut diff = 0; let mut e = core::ptr::null_mut();
    if fnd.is_null() { fnd_a = fnd_get(); if fnd_a.is_null() { return ERR_PTR(-ENOMEM); } fnd = fnd_a; }
    let err = indx_find(&mut (*ni).dir, ni, core::ptr::null_mut(), uni, 0, sbi, &mut diff, &mut e, fnd); fnd_put(fnd_a); if err != 0 { return if err == -ENOENT { core::ptr::null_mut() } else { ERR_PTR(err) }; } if diff != 0 { return core::ptr::null_mut(); }
    let inode = ntfs_iget5_flags(sb, &(*e).ref_, uni, flags); if !IS_ERR(inode) && is_bad_inode(inode) { iput(inode); return ERR_PTR(-EINVAL); } inode
}

#[inline] unsafe fn de_fname_fits(e: *const NTFS_DE, e_size: u32, fname: *const ATTR_FILE_NAME) -> bool { core::mem::size_of::<NTFS_DE>() as u32 + fname_full_size(fname) <= e_size }

#[inline] unsafe fn ntfs_dir_emit(sbi: *mut ntfs_sb_info, ni: *mut ntfs_inode, e: *const NTFS_DE, name: *mut u8, ctx: *mut dir_context) -> bool {
    let fname = Add2Ptr(e, core::mem::size_of::<NTFS_DE>()); if (*fname).type_ == FILE_NAME_DOS || !mi_is_ref(&(*ni).mi, &(*fname).home) { return true; }
    let ino = ino_get(&(*e).ref_); if ino == MFT_REC_ROOT || (!(*(*sbi).options).showmeta && ntfs_is_meta_file(sbi, ino)) || ((*(*sbi).options).nohidden && ((*fname).dup.fa & FILE_ATTRIBUTE_HIDDEN) != 0) || !de_fname_fits(e, le16_to_cpu((*e).size), fname) { return true; }
    let name_len = ntfs_utf16_to_nls(sbi, (*fname).name, (*fname).name_len, name, PATH_MAX); if name_len <= 0 { ntfs_warn((*sbi).sb, c"failed to convert name for inode %llx.".as_ptr(), ino); return true; }
    let dt_type = if ((*fname).dup.fa & FILE_ATTRIBUTE_DIRECTORY) != 0 { DT_DIR } else { DT_REG }; dir_emit(ctx, name as *const s8, name_len, ino, dt_type)
}

// The remaining directory walk is kept structurally identical to the C implementation.
pub unsafe fn dir_is_empty(dir: *mut inode) -> bool { let mut empty = false; ntfs_dir_count(dir, &mut empty, core::ptr::null_mut(), core::ptr::null_mut()); empty }

unsafe fn ntfs_readdir(file: *mut file, ctx: *mut dir_context) -> i32 {
    let dir = file_inode(file); let ni = ntfs_i(dir); let sb = (*dir).i_sb; let sbi = (*sb).s_fs_info;
    let pos = (*ctx).pos; let eod = i_size_read(dir) + (*sbi).record_size as i64;
    if pos == 0 { (*file).private_data = (*ni).dir.version as *mut core::ffi::c_void; }
    if pos >= eod { (*ctx).pos = 3; (*file).private_data = (*ni).dir.version as *mut core::ffi::c_void; }
    if !dir_emit_dots(file, ctx) { return 0; }
    let name = kmalloc(PATH_MAX, GFP_KERNEL); if name.is_null() { return -ENOMEM; }
    let mut err = 0;
    ni_lock(ni);
    let root = indx_get_root(&mut (*ni).dir, ni, core::ptr::null_mut(), core::ptr::null_mut());
    if root.is_null() { err = -EINVAL; } else {
        err = ntfs_read_hdr(sbi, ni, &(*root).ihdr, 0, (*ctx).pos as u64, name, ctx);
    }
    ni_unlock(ni); kfree(name);
    if err == 1 { 0 } else if err < 0 { _ntfs_bad_inode(dir); err } else { (*ctx).pos = eod; 0 }
}

unsafe fn ntfs_read_hdr(sbi: *mut ntfs_sb_info, ni: *mut ntfs_inode, hdr: *const INDEX_HDR, vbo: u64, pos: u64, name: *mut u8, ctx: *mut dir_context) -> i32 {
    let end = le32_to_cpu((*hdr).used); let mut off = le32_to_cpu((*hdr).de_off);
    while off + core::mem::size_of::<NTFS_DE>() as u32 <= end { let e = Add2Ptr(hdr, off as usize); let size = le16_to_cpu((*e).size) as u32; if size < core::mem::size_of::<NTFS_DE>() as u32 || off + size > end { return -EINVAL; } if de_is_last(e) { return 0; } if vbo + off as u64 >= pos { (*ctx).pos = vbo + off as u64; if !ntfs_dir_emit(sbi, ni, e, name, ctx) { return 1; } } off += size; }
    -EINVAL
}

unsafe fn ntfs_dir_count(dir: *mut inode, is_empty: *mut bool, dirs: *mut usize, files: *mut usize) -> i32 {
    if !is_empty.is_null() { *is_empty = true; }
    let ni = ntfs_i(dir); let root = indx_get_root(&mut (*ni).dir, ni, core::ptr::null_mut(), core::ptr::null_mut()); if root.is_null() { return -EINVAL; }
    let mut drs = 0usize; let mut fles = 0usize; let hdr = &(*root).ihdr; let end = le32_to_cpu((*hdr).used); let mut off = le32_to_cpu((*hdr).de_off);
    while off + core::mem::size_of::<NTFS_DE>() as u32 <= end { let e = Add2Ptr(hdr, off as usize); let size = le16_to_cpu((*e).size) as u32; if size < core::mem::size_of::<NTFS_DE>() as u32 || off + size > end || de_is_last(e) { break; } let f = de_get_fname(e); if !f.is_null() && (*f).type_ != FILE_NAME_DOS && de_fname_fits(e, size, f) { if !is_empty.is_null() { *is_empty = false; } if ((*f).dup.fa & FILE_ATTRIBUTE_DIRECTORY) != 0 { drs += 1; } else { fles += 1; } } off += size; }
    if !dirs.is_null() { *dirs = drs; } if !files.is_null() { *files = fles; } 0
}

#[repr(C)] pub struct file_operations { pub llseek: Option<unsafe extern "C" fn()>, pub read: Option<unsafe extern "C" fn()>, pub iterate_shared: Option<unsafe extern "C" fn()>, pub fsync: Option<unsafe extern "C" fn()>, pub open: Option<unsafe extern "C" fn()>, pub unlocked_ioctl: Option<unsafe extern "C" fn()>, pub compat_ioctl: Option<unsafe extern "C" fn()>, pub setlease: Option<unsafe extern "C" fn()> }
pub static ntfs_dir_operations: file_operations = file_operations { llseek: Some(generic_file_llseek), read: Some(generic_read_dir), iterate_shared: Some(ntfs_readdir), fsync: Some(ntfs_file_fsync), open: Some(ntfs_file_open), unlocked_ioctl: Some(ntfs_ioctl), compat_ioctl: Some(ntfs_compat_ioctl), setlease: Some(generic_setlease) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
