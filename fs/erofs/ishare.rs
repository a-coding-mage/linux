// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2024, Alibaba Cloud
 */

static mut EROFS_ISHARE_MNT: *mut vfsmount = core::ptr::null_mut();

unsafe fn erofs_ishare_iget5_eq(inode: *mut inode, data: *mut core::ffi::c_void) -> i32 {
    let fp1: *mut erofs_inode_fingerprint = &mut (*EROFS_I(inode)).fingerprint;
    let fp2: *mut erofs_inode_fingerprint = data as *mut erofs_inode_fingerprint;

    ((*fp1).size == (*fp2).size
        && libc::memcmp((*fp1).opaque as *const core::ffi::c_void,
                        (*fp2).opaque as *const core::ffi::c_void,
                        (*fp2).size as usize) == 0) as i32
}

unsafe fn erofs_ishare_iget5_set(inode: *mut inode, data: *mut core::ffi::c_void) -> i32 {
    let vi: *mut erofs_inode = EROFS_I(inode);

    (*vi).fingerprint = *(data as *const erofs_inode_fingerprint);
    INIT_LIST_HEAD(&mut (*vi).ishare_list);
    spin_lock_init(&mut (*vi).ishare_lock);
    0
}

unsafe fn erofs_ishare_fill_inode(inode: *mut inode) -> bool {
    static EMPTY_FOPS: file_operations = file_operations {};
    let sbi: *mut erofs_sb_info = EROFS_SB((*inode).i_sb);
    let aops: *const address_space_operations;
    let vi: *mut erofs_inode = EROFS_I(inode);
    let mut fp: erofs_inode_fingerprint = core::mem::zeroed();
    let sd: *mut dentry;
    let si: *mut inode;

    aops = erofs_get_aops(inode);
    if IS_ERR(aops) {
        return false;
    }
    if erofs_xattr_fill_inode_fingerprint(&mut fp, inode, (*sbi).domain_id) != 0 {
        return false;
    }

    si = iget5_locked((*(*EROFS_ISHARE_MNT).mnt_sb).s_bdi,
                      xxh32(fp.opaque, fp.size, 0),
                      erofs_ishare_iget5_eq,
                      erofs_ishare_iget5_set,
                      &mut fp as *mut _ as *mut core::ffi::c_void);
    if !si.is_null() && (inode_state_read_once(si) & I_NEW) != 0 {
        (*si).i_fop = &EMPTY_FOPS;
        (*(*si).i_mapping).a_ops = aops;
        (*si).i_mode = 0o444 | S_IFREG;
        (*si).i_size = (*inode).i_size;
        mapping_set_large_folios((*si).i_mapping);
        unlock_new_inode(si);
    } else {
        kfree(fp.opaque);
        if si.is_null() || aops != (*(*si).i_mapping).a_ops {
            iput(si);
            return false;
        }
        if (*si).i_size != (*inode).i_size {
            erofs_warn((*inode).i_sb, "i_size mismatch (%lld != %lld) for the same fingerprint", (*inode).i_size, (*si).i_size);
            iput(si);
            return false;
        }
    }
    sd = d_obtain_alias(si); // disconnected denties for sharedinodes
    if IS_ERR(sd) {
        return false;
    }
    (*vi).sharedentry = sd;
    INIT_LIST_HEAD(&mut (*vi).ishare_list);
    spin_lock(&mut (*EROFS_I(d_inode(sd))).ishare_lock);
    list_add(&mut (*vi).ishare_list, &mut (*EROFS_I(si)).ishare_list);
    spin_unlock(&mut (*EROFS_I(si)).ishare_lock);
    true
}

unsafe fn erofs_ishare_free_inode(inode: *mut inode) {
    let vi: *mut erofs_inode = EROFS_I(inode);
    let svi: *mut erofs_inode;

    if (*vi).sharedentry.is_null() {
        return;
    }
    svi = EROFS_I(d_inode((*vi).sharedentry));
    spin_lock(&mut (*svi).ishare_lock);
    list_del(&mut (*vi).ishare_list);
    spin_unlock(&mut (*svi).ishare_lock);
    dput((*vi).sharedentry);
    (*vi).sharedentry = core::ptr::null_mut();
}

unsafe fn erofs_ishare_file_open(inode: *mut inode, file: *mut file) -> i32 {
    let sharedpath = path {
        mnt: EROFS_ISHARE_MNT,
        dentry: (*EROFS_I(inode)).sharedentry,
    };
    let rf: *mut file;

    if (*file).f_flags & O_DIRECT != 0 {
        return -EINVAL;
    }
    rf = backing_file_open(file, (*file).f_flags | O_NOATIME, &sharedpath, current_cred());
    if IS_ERR(rf) {
        return PTR_ERR(rf);
    }
    (*file).private_data = rf as *mut core::ffi::c_void;
    0
}

unsafe fn erofs_ishare_file_release(_inode: *mut inode, file: *mut file) -> i32 {
    fput((*file).private_data as *mut file);
    (*file).private_data = core::ptr::null_mut();
    0
}

unsafe fn erofs_ishare_file_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize {
    let realfile: *mut file = (*iocb).ki_filp.as_ref().unwrap().private_data as *mut file;
    let mut dedup_iocb: kiocb = core::mem::zeroed();
    let nread: isize;

    if iov_iter_count(to) == 0 {
        return 0;
    }
    kiocb_clone(&mut dedup_iocb, iocb, realfile);
    nread = filemap_read(&mut dedup_iocb, to, 0);
    (*iocb).ki_pos = dedup_iocb.ki_pos;
    nread
}

unsafe fn erofs_ishare_mmap(file: *mut file, vma: *mut vm_area_struct) -> i32 {
    let realfile: *mut file = (*file).private_data as *mut file;
    let err: i32;

    vma_set_file(vma, realfile);
    err = security_mmap_backing_file(vma, realfile, file);
    if err != 0 {
        return err;
    }
    generic_file_readonly_mmap(file, vma)
}

unsafe fn erofs_ishare_splice_read(in_: *mut file, ppos: *mut loff_t, pipe: *mut pipe_inode_info, len: usize, flags: u32) -> isize {
    filemap_splice_read((*in_).private_data as *mut file, ppos, pipe, len, flags)
}

unsafe fn erofs_ishare_fadvise(file: *mut file, offset: loff_t, len: loff_t, advice: i32) -> i32 {
    vfs_fadvise((*file).private_data as *mut file, offset, len, advice)
}

const EROFS_ISHARE_FOPS: file_operations = file_operations {
    open: Some(erofs_ishare_file_open),
    llseek: Some(erofs_file_llseek),
    read_iter: Some(erofs_ishare_file_read_iter),
    mmap: Some(erofs_ishare_mmap),
    release: Some(erofs_ishare_file_release),
    get_unmapped_area: Some(thp_get_unmapped_area),
    splice_read: Some(erofs_ishare_splice_read),
    fadvise: Some(erofs_ishare_fadvise),
};

unsafe fn erofs_real_inode(inode: *mut inode, need_iput: *mut bool) -> *mut inode {
    let vi: *mut erofs_inode;
    let vi_share: *mut erofs_inode;
    let mut realinode: *mut inode = core::ptr::null_mut();

    *need_iput = false;
    if (*inode).i_sb != (*EROFS_ISHARE_MNT).mnt_sb {
        return inode;
    }
    vi_share = EROFS_I(inode);
    spin_lock(&mut (*vi_share).ishare_lock);
    // fetch any one as real inode
    DBG_BUGON(list_empty(&(*vi_share).ishare_list));
    list_for_each_entry!(vi, &(*vi_share).ishare_list, ishare_list, {
        realinode = igrab(&mut (*vi).vfs_inode);
        if !realinode.is_null() {
            *need_iput = true;
            break;
        }
    });
    spin_unlock(&mut (*vi_share).ishare_lock);
    DBG_BUGON(realinode.is_null());
    realinode
}

unsafe fn erofs_init_ishare() -> i32 {
    let mnt: *mut vfsmount;
    let ret: i32;

    mnt = kern_mount(&mut erofs_anon_fs_type);
    if IS_ERR(mnt) {
        return PTR_ERR(mnt);
    }
    // generic_fadvise() doesn't work if s_bdi == &noop_backing_dev_info
    ret = super_setup_bdi((*mnt).mnt_sb);
    if ret != 0 {
        kern_unmount(mnt);
    } else {
        EROFS_ISHARE_MNT = mnt;
    }
    ret
}

unsafe fn erofs_exit_ishare() {
    kern_unmount(EROFS_ISHARE_MNT);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
