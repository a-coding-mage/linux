// SPDX-License-Identifier: GPL-2.0
/*
 * The base64 encode/decode code was copied from fscrypt:
 * Copyright (C) 2015, Google, Inc.
 * Copyright (C) 2015, Motorola Mobility
 * Written by Uday Savagaonkar, 2014.
 * Modified by Jaegeuk Kim, 2015.
 */

unsafe fn ceph_crypt_get_context(inode: *mut inode, ctx: *mut core::ffi::c_void, len: usize) -> i32 {
    let ci = ceph_inode(inode);
    let cfa = (*ci).fscrypt_auth as *mut ceph_fscrypt_auth;
    if cfa.is_null() || (*ci).fscrypt_auth_len < (core::mem::offset_of!(ceph_fscrypt_auth, cfa_blob) + 1) { return -ENOBUFS; }
    if le32_to_cpu((*cfa).cfa_version) != CEPH_FSCRYPT_AUTH_VERSION { return -ENOBUFS; }
    let ctxlen = le32_to_cpu((*cfa).cfa_blob_len) as usize;
    if len < ctxlen { return -ERANGE; }
    memcpy(ctx, (*cfa).cfa_blob.as_ptr() as *const _, ctxlen);
    ctxlen as i32
}

unsafe fn ceph_crypt_set_context(inode: *mut inode, ctx: *const core::ffi::c_void, len: usize, fs_data: *mut core::ffi::c_void) -> i32 {
    let mut attr: iattr = core::mem::zeroed();
    let mut cia: ceph_iattr = core::mem::zeroed();
    WARN_ON_ONCE(!fs_data.is_null());
    if len > FSCRYPT_SET_CONTEXT_MAX_SIZE { return -EINVAL; }
    let cfa = kzalloc_obj::<ceph_fscrypt_auth>();
    if cfa.is_null() { return -ENOMEM; }
    (*cfa).cfa_version = cpu_to_le32(CEPH_FSCRYPT_AUTH_VERSION);
    (*cfa).cfa_blob_len = cpu_to_le32(len as u32);
    memcpy((*cfa).cfa_blob.as_mut_ptr() as *mut _, ctx, len);
    cia.fscrypt_auth = cfa;
    let ret = __ceph_setattr(&nop_mnt_idmap, inode, &mut attr, &mut cia);
    if ret == 0 { inode_set_flags(inode, S_ENCRYPTED, S_ENCRYPTED); }
    kfree(cia.fscrypt_auth);
    ret
}

unsafe fn ceph_crypt_empty_dir(inode: *mut inode) -> bool {
    let ci = ceph_inode(inode);
    (*ci).i_rsubdirs + (*ci).i_rfiles == 1
}

unsafe fn ceph_get_dummy_policy(sb: *mut super_block) -> *const fscrypt_policy {
    ceph_sb_to_fs_client(sb).fsc_dummy_enc_policy.policy
}

static mut ceph_fscrypt_ops: fscrypt_operations = fscrypt_operations {
    inode_info_offs: 0, needs_bounce_pages: 1,
    get_context: Some(ceph_crypt_get_context), set_context: Some(ceph_crypt_set_context),
    get_dummy_policy: Some(ceph_get_dummy_policy), empty_dir: Some(ceph_crypt_empty_dir),
};

pub unsafe fn ceph_fscrypt_set_ops(sb: *mut super_block) { fscrypt_set_ops(sb, &raw const ceph_fscrypt_ops); }
pub unsafe fn ceph_fscrypt_free_dummy_policy(fsc: *mut ceph_fs_client) { fscrypt_free_dummy_policy(&mut (*fsc).fsc_dummy_enc_policy); }

pub unsafe fn ceph_fscrypt_prepare_context(dir: *mut inode, inode: *mut inode, as_: *mut ceph_acl_sec_ctx) -> i32 {
    let mut encrypted = false;
    let ci = ceph_inode(inode);
    let ret = fscrypt_prepare_new_inode(dir, inode, &mut encrypted);
    if ret != 0 { return ret; }
    if !encrypted { return 0; }
    (*as_).fscrypt_auth = kzalloc_obj::<ceph_fscrypt_auth>();
    if (*as_).fscrypt_auth.is_null() { return -ENOMEM; }
    let ctxsize = fscrypt_context_for_new_inode((*(*as_).fscrypt_auth).cfa_blob.as_mut_ptr(), inode);
    if ctxsize < 0 { return ctxsize; }
    (*(*as_).fscrypt_auth).cfa_version = cpu_to_le32(CEPH_FSCRYPT_AUTH_VERSION);
    (*(*as_).fscrypt_auth).cfa_blob_len = cpu_to_le32(ctxsize as u32);
    WARN_ON_ONCE(!(*ci).fscrypt_auth.is_null());
    kfree((*ci).fscrypt_auth);
    (*ci).fscrypt_auth_len = ceph_fscrypt_auth_len((*as_).fscrypt_auth);
    (*ci).fscrypt_auth = kmemdup((*as_).fscrypt_auth as *const _, (*ci).fscrypt_auth_len, GFP_KERNEL);
    if (*ci).fscrypt_auth.is_null() { return -ENOMEM; }
    (*inode).i_flags |= S_ENCRYPTED;
    0
}

pub unsafe fn ceph_fscrypt_as_ctx_to_req(req: *mut ceph_mds_request, as_: *mut ceph_acl_sec_ctx) {
    core::mem::swap(&mut (*req).r_fscrypt_auth, &mut (*as_).fscrypt_auth);
}

unsafe fn parse_longname(parent: *const inode, name: *const core::ffi::c_char, name_len: *mut i32) -> *mut inode {
    let cl = ceph_inode_to_client(parent);
    if *name_len <= 0 || *name != b'_' as i8 { return ERR_PTR(-EIO); }
    let str_ = kmemdup_nul(name.add(1), (*name_len - 1) as usize, GFP_KERNEL);
    if str_.is_null() { return ERR_PTR(-ENOMEM); }
    let name_end = strrchr(str_, b'_' as i32);
    if name_end.is_null() { doutc(cl, "failed to parse long snapshot name: %s\n", str_); kfree(str_); return ERR_PTR(-EIO); }
    *name_len = name_end.offset_from(str_) as i32;
    if *name_len <= 0 { pr_err_client(cl, "failed to parse long snapshot name\n"); kfree(str_); return ERR_PTR(-EIO); }
    let mut vino: ceph_vino = core::mem::zeroed(); vino.snap = CEPH_NOSNAP;
    let ret = kstrtou64(name_end.add(1), 10, &mut vino.ino);
    if ret != 0 { doutc(cl, "failed to parse inode number: %s\n", str_); kfree(str_); return ERR_PTR(ret); }
    let mut dir = ceph_find_inode((*parent).i_sb, vino);
    if dir.is_null() { dir = ceph_get_inode((*parent).i_sb, vino, core::ptr::null_mut()); if IS_ERR(dir) { doutc(cl, "can't find inode %s (%s)\n", name_end.add(1), name); } }
    kfree(str_); dir
}

pub unsafe fn ceph_encode_encrypted_dname(parent: *mut inode, buf: *mut i8, elen: i32) -> i32 {
    let cl = ceph_inode_to_client(parent); let mut dir = parent; let mut p = buf; let mut len = 0u32; let mut name_len = elen; let mut cryptbuf: *mut u8 = core::ptr::null_mut();
    if ceph_snap(dir) == CEPH_SNAPDIR && *p == b'_' as i8 { dir = parse_longname(parent, p, &mut name_len); if IS_ERR(dir) { return PTR_ERR(dir); } p = p.add(1); }
    if !fscrypt_has_encryption_key(dir) { return elen; }
    if !fscrypt_fname_encrypted_size(dir, name_len, NAME_MAX, &mut len) { return -ENAMETOOLONG; }
    cryptbuf = kmalloc(if len > CEPH_NOHASH_NAME_MAX { NAME_MAX } else { len as usize }, GFP_KERNEL);
    if cryptbuf.is_null() { return -ENOMEM; }
    let mut qstr = qstr { name: p, len: name_len as usize }; let mut ret = fscrypt_fname_encrypt(dir, &mut qstr, cryptbuf, len as usize);
    if ret != 0 { kfree(cryptbuf); return ret; }
    if len > CEPH_NOHASH_NAME_MAX { let mut hash = [0u8; SHA256_DIGEST_SIZE]; let extra = cryptbuf.add(CEPH_NOHASH_NAME_MAX as usize); sha256(extra, (len - CEPH_NOHASH_NAME_MAX) as usize, hash.as_mut_ptr()); memcpy(extra as *mut _, hash.as_ptr() as *const _, SHA256_DIGEST_SIZE); len = CEPH_NOHASH_NAME_MAX + SHA256_DIGEST_SIZE as u32; }
    ret = base64_encode(cryptbuf, len as usize, p, false, BASE64_IMAP); doutc(cl, "base64-encoded ciphertext name = %.*s\n", ret, p);
    WARN_ON(ret > 240); let result = if dir != parent { ret + 1 + sprintf(p.add(ret as usize), "_%llu", (*dir).i_ino) } else { ret }; kfree(cryptbuf); if dir != parent { if inode_state_read_once(dir) & I_NEW != 0 { discard_new_inode(dir); } else { iput(dir); } } result
}

pub unsafe fn ceph_fname_to_usr(fname: *const ceph_fname, tname: *mut u8, oname: *mut fscrypt_str, is_nokey: *mut bool) -> i32 {
    let mut dir = (*fname).dir; let mut _tname = fscrypt_str { name: core::ptr::null_mut(), len: 0 }; let mut _oname: fscrypt_str; let mut iname: fscrypt_str;
    let mut name = (*fname).name; let mut name_len = (*fname).name_len; let mut ret;
    if !tname.is_null() && is_vmalloc_addr(tname) { return -EIO; }
    if (*fname).name_len > NAME_MAX || (*fname).ctext_len > NAME_MAX { return -EIO; }
    if ceph_snap(dir) == CEPH_SNAPDIR && name_len > 0 && *name == b'_' as i8 { dir = parse_longname(dir, name, &mut name_len); if IS_ERR(dir) { return PTR_ERR(dir); } name = name.add(1); }
    if !IS_ENCRYPTED(dir) { (*oname).name = (*fname).name as *mut u8; (*oname).len = (*fname).name_len; ret = 0; }
    else {
        ret = ceph_fscrypt_prepare_readdir(dir); if ret != 0 { return ret; }
        if !fscrypt_has_encryption_key(dir) { if (*fname).no_copy { (*oname).name = (*fname).name as *mut u8; } else { memcpy((*oname).name as *mut _, (*fname).name as *const _, (*fname).name_len); } (*oname).len = (*fname).name_len; if !is_nokey.is_null() { *is_nokey = true; } ret = 0; }
        else {
            if tname.is_null() && ((*fname).ctext_len == 0 || is_vmalloc_addr((*fname).ctext) || is_vmalloc_addr((*oname).name)) { ret = fscrypt_fname_alloc_buffer(NAME_MAX, &mut _tname); if ret != 0 { return ret; } tname = _tname.name; }
            if (*fname).ctext_len == 0 { let declen = base64_decode(name, name_len, tname, false, BASE64_IMAP); if declen <= 0 { ret = -EIO; } else { iname = fscrypt_str { name: tname, len: declen as usize }; ret = 0; } }
            else { iname = fscrypt_str { name: if is_vmalloc_addr((*fname).ctext) { memcpy(tname as *mut _, (*fname).ctext as *const _, (*fname).ctext_len); tname } else { (*fname).ctext }, len: (*fname).ctext_len }; ret = 0; }
            if ret == 0 { _oname = fscrypt_str { name: if is_vmalloc_addr((*oname).name) { tname } else { (*oname).name }, len: (*oname).len }; ret = fscrypt_fname_disk_to_usr(dir, 0, 0, &mut iname, &mut _oname); if ret == 0 { if is_vmalloc_addr((*oname).name) { memcpy((*oname).name as *mut _, _oname.name as *const _, _oname.len); } (*oname).len = _oname.len; if dir != (*fname).dir { let mut tmp = [0i8; BASE64_CHARS(NAME_MAX)]; let n = snprintf(tmp.as_mut_ptr(), tmp.len(), "_%.*s_%llu", (*oname).len, (*oname).name, (*dir).i_ino); memcpy((*oname).name as *mut _, tmp.as_ptr() as *const _, n as usize); (*oname).len = n as usize; } } }
        }
    }
    fscrypt_fname_free_buffer(&mut _tname); if dir != (*fname).dir { if inode_state_read_once(dir) & I_NEW != 0 { discard_new_inode(dir); } else { iput(dir); } } ret
}

pub unsafe fn ceph_fscrypt_prepare_readdir(dir: *mut inode) -> i32 { let had_key = fscrypt_has_encryption_key(dir); if !IS_ENCRYPTED(dir) { return 0; } let err = __fscrypt_prepare_readdir(dir); if err != 0 { return err; } if !had_key && fscrypt_has_encryption_key(dir) { ceph_dir_clear_complete(dir); return 1; } 0 }

pub unsafe fn ceph_fscrypt_decrypt_block_inplace(inode: *const inode, page: *mut page, len: u32, offs: u32, lblk_num: u64) -> i32 { fscrypt_decrypt_block_inplace(inode, page, len, offs, lblk_num) }
pub unsafe fn ceph_fscrypt_encrypt_block_inplace(inode: *const inode, page: *mut page, len: u32, offs: u32, lblk_num: u64) -> i32 { fscrypt_encrypt_block_inplace(inode, page, len, offs, lblk_num) }

pub unsafe fn ceph_fscrypt_decrypt_pages(inode: *mut inode, page: *mut *mut page, off: u64, len: i32) -> i32 { let baseblk = off >> CEPH_FSCRYPT_BLOCK_SHIFT; let num_blocks = ceph_fscrypt_blocks(off, len & CEPH_FSCRYPT_BLOCK_MASK); let mut ret = 0; for i in 0..num_blocks { let blkoff = i << CEPH_FSCRYPT_BLOCK_SHIFT; let fret = ceph_fscrypt_decrypt_block_inplace(inode, *page.add((blkoff >> PAGE_SHIFT) as usize), CEPH_FSCRYPT_BLOCK_SIZE, offset_in_page(blkoff), baseblk + i as u64); if fret < 0 { if ret == 0 { ret = fret; } break; } ret += CEPH_FSCRYPT_BLOCK_SIZE as i32; } ret }

pub unsafe fn ceph_fscrypt_encrypt_pages(inode: *mut inode, page: *mut *mut page, off: u64, len: i32) -> i32 { let baseblk = off >> CEPH_FSCRYPT_BLOCK_SHIFT; let num_blocks = ceph_fscrypt_blocks(off, len & CEPH_FSCRYPT_BLOCK_MASK); let mut ret = 0; for i in 0..num_blocks { let blkoff = i << CEPH_FSCRYPT_BLOCK_SHIFT; let fret = ceph_fscrypt_encrypt_block_inplace(inode, *page.add((blkoff >> PAGE_SHIFT) as usize), CEPH_FSCRYPT_BLOCK_SIZE, offset_in_page(blkoff), baseblk + i as u64); if fret < 0 { if ret == 0 { ret = fret; } break; } ret += CEPH_FSCRYPT_BLOCK_SIZE as i32; } ret }

pub unsafe fn ceph_fscrypt_decrypt_extents(inode: *mut inode, page: *mut *mut page, off: u64, map: *mut ceph_sparse_extent, ext_cnt: u32) -> i32 {
    if ext_cnt == 0 { return 0; }
    let ci = ceph_inode(inode); let mut objno = 0u64; let mut objoff = 0u64; let mut xlen = 0u32;
    ceph_calc_file_object_mapping(&(*ci).i_layout, off, (*map).len, &mut objno, &mut objoff, &mut xlen);
    let mut ret = 0;
    for i in 0..ext_cnt { let ext = &mut *map.add(i as usize); let pgsoff = ext.off - objoff; if (ext.off | ext.len) & !CEPH_FSCRYPT_BLOCK_MASK != 0 { return -EIO; } let fret = ceph_fscrypt_decrypt_pages(inode, page.add((pgsoff >> PAGE_SHIFT) as usize), off + pgsoff, ext.len as i32); if fret < 0 { if ret == 0 { ret = fret; } break; } ret = pgsoff as i32 + fret; }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
