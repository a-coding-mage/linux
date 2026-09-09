// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017-2018 HUAWEI, Inc.
 *             https://www.huawei.com/
 * Copyright (C) 2022, Alibaba Cloud
 */
// Dependencies supplied by the surrounding kernel/EROFS translation.

#[repr(C)]
pub struct erofs_qstr {
    pub name: *const u8,
    pub end: *const u8,
}

#[inline]
unsafe fn erofs_dirnamecmp(
    qn: *const erofs_qstr,
    qd: *const erofs_qstr,
    matched: *mut u32,
) -> i32 {
    let mut i = *matched;

    // on-disk error, let's only BUG_ON in the debugging mode.
    DBG_BUGON((*qd).name > (*qd).end);

    // qd could not have trailing '\0'; safe while below qd->end.
    while (*qd).name.add(i as usize) < (*qd).end && *(*qd).name.add(i as usize) != 0 {
        if *(*qn).name.add(i as usize) != *(*qd).name.add(i as usize) {
            *matched = i;
            return if *(*qn).name.add(i as usize) > *(*qd).name.add(i as usize) { 1 } else { -1 };
        }
        i += 1;
    }
    *matched = i;
    // See comments in __d_alloc on the terminating NUL character.
    if *(*qn).name.add(i as usize) == 0 { 0 } else { 1 }
}

#[inline]
unsafe fn nameoff_from_disk(off: u16, sz: u32) -> u32 {
    (le16_to_cpu(off) as u32) & (sz - 1)
}

unsafe fn find_target_dirent(
    name: *mut erofs_qstr,
    data: *mut u8,
    dirblksize: u32,
    ndirents: i32,
) -> *mut erofs_dirent {
    let mut head: i32 = 1;
    let mut back: i32 = ndirents - 1;
    let mut startprfx: u32 = 0;
    let mut endprfx: u32 = 0;
    let de = data as *mut erofs_dirent;

    while head <= back {
        let mid = head + (back - head) / 2;
        let nameoff = nameoff_from_disk((*de.add(mid as usize)).nameoff, dirblksize);
        let mut matched = core::cmp::min(startprfx, endprfx);
        let dname = erofs_qstr {
            name: data.add(nameoff as usize),
            end: if mid >= ndirents - 1 {
                data.add(dirblksize as usize)
            } else {
                data.add(nameoff_from_disk((*de.add((mid + 1) as usize)).nameoff, dirblksize) as usize)
            },
        };
        let ret = erofs_dirnamecmp(name, &dname, &mut matched);
        if ret == 0 {
            return de.add(mid as usize);
        } else if ret > 0 {
            head = mid + 1;
            startprfx = matched;
        } else {
            back = mid - 1;
            endprfx = matched;
        }
    }
    ERR_PTR(-ENOENT)
}

unsafe fn erofs_find_target_block(
    target: *mut erofs_buf,
    dir: *mut inode,
    name: *mut erofs_qstr,
    ndirents: *mut i32,
) -> *mut core::ffi::c_void {
    let bsz = i_blocksize(dir);
    let mut head: i32 = 0;
    let mut back: i32 = erofs_iblks(dir) - 1;
    let mut startprfx = 0u32;
    let mut endprfx = 0u32;
    let mut candidate: *mut core::ffi::c_void = ERR_PTR(-ENOENT);

    while head <= back {
        let mid = head + (back - head) / 2;
        let mut buf: erofs_buf = __EROFS_BUF_INITIALIZER;
        buf.mapping = (*dir).i_mapping;
        let mut de = erofs_bread(&mut buf, erofs_pos((*dir).i_sb, mid), true);
        if !IS_ERR(de) {
            let nameoff = nameoff_from_disk((*((de as *mut erofs_dirent))).nameoff, bsz);
            let count = nameoff as usize / core::mem::size_of::<erofs_dirent>();
            if count == 0 {
                erofs_put_metabuf(&mut buf);
                erofs_err((*dir).i_sb, "corrupted dir block %d @ nid %llu", mid, (*EROFS_I(dir)).nid);
                DBG_BUGON(true);
                de = ERR_PTR(-EFSCORRUPTED);
                if !IS_ERR(candidate) { erofs_put_metabuf(target); }
                return de;
            }
            let mut matched = core::cmp::min(startprfx, endprfx);
            let dname = erofs_qstr {
                name: (de as *mut u8).add(nameoff as usize),
                end: if count == 1 { (de as *mut u8).add(bsz as usize) } else {
                    (de as *mut u8).add(nameoff_from_disk((*((de as *mut erofs_dirent).add(1))).nameoff, bsz) as usize)
                },
            };
            let diff = erofs_dirnamecmp(name, &dname, &mut matched);
            if diff < 0 {
                erofs_put_metabuf(&mut buf);
                back = mid - 1;
                endprfx = matched;
                continue;
            }
            if !IS_ERR(candidate) { erofs_put_metabuf(target); }
            *target = buf;
            if diff == 0 { *ndirents = 0; return de; }
            head = mid + 1;
            startprfx = matched;
            candidate = de;
            *ndirents = count as i32;
            continue;
        }
        if !IS_ERR(candidate) { erofs_put_metabuf(target); }
        return de;
    }
    candidate
}

pub unsafe fn erofs_namei(
    dir: *mut inode,
    name: *const qstr,
    nid: *mut erofs_nid_t,
    d_type: *mut u32,
) -> i32 {
    let mut buf: erofs_buf = __EROFS_BUF_INITIALIZER;
    if (*dir).i_size == 0 { return -ENOENT; }
    let qn = erofs_qstr { name: (*name).name, end: (*name).name.add((*name).len as usize) };
    buf.mapping = (*dir).i_mapping;
    let mut ndirents = 0;
    let mut de = erofs_find_target_block(&mut buf, dir, &qn, &mut ndirents);
    if IS_ERR(de) { return PTR_ERR(de); }
    if ndirents != 0 { de = find_target_dirent(&qn, de as *mut u8, i_blocksize(dir), ndirents) as *mut core::ffi::c_void; }
    if !IS_ERR(de) {
        *nid = le64_to_cpu((*((de as *mut erofs_dirent))).nid);
        *d_type = (*((de as *mut erofs_dirent))).file_type as u32;
    }
    erofs_put_metabuf(&mut buf);
    PTR_ERR_OR_ZERO(de)
}

unsafe fn erofs_lookup(dir: *mut inode, dentry: *mut dentry, flags: u32) -> *mut dentry {
    let mut nid: erofs_nid_t = 0;
    let mut d_type = 0u32;
    trace_erofs_lookup(dir, dentry, flags);
    if (*dentry).d_name.len > EROFS_NAME_LEN { return ERR_PTR(-ENAMETOOLONG); }
    let err = erofs_namei(dir, &(*dentry).d_name, &mut nid, &mut d_type);
    let inode = if err == -ENOENT { core::ptr::null_mut() } else if err != 0 { ERR_PTR(err) } else { erofs_iget((*dir).i_sb, nid) };
    d_splice_alias(inode, dentry)
}

pub static erofs_dir_iops: inode_operations = inode_operations {
    lookup: Some(erofs_lookup),
    getattr: Some(erofs_getattr),
    listxattr: Some(erofs_listxattr),
    get_inode_acl: Some(erofs_get_acl),
    fiemap: Some(erofs_fiemap),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
