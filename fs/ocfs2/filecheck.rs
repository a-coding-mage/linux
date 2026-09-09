// SPDX-License-Identifier: GPL-2.0-only
/*
 * filecheck.c
 *
 * Code which implements online file check.
 *
 * Copyright (C) 2016 SuSE.  All rights reserved.
 */

// Kernel and OCFS2 declarations are supplied by the surrounding translation unit.

/* File check error strings, must correspond with error number in header file. */
static OCFS2_FILECHECK_ERRS: [&[u8]; 11] = [
    b"SUCCESS\0", b"FAILED\0", b"INPROGRESS\0", b"READONLY\0",
    b"INJBD\0", b"INVALIDINO\0", b"BLOCKECC\0", b"BLOCKNO\0",
    b"VALIDFLAG\0", b"GENERATION\0", b"UNSUPPORTED\0",
];

#[repr(C)]
pub struct ocfs2_filecheck_entry {
    pub fe_list: list_head,
    pub fe_ino: c_ulong,
    pub fe_type: c_uint,
    pub fe_done: c_uint,
    pub fe_status: c_uint,
}

#[repr(C)]
pub union ocfs2_filecheck_args_value {
    pub fa_ino: c_ulong,
    pub fa_len: c_uint,
}

#[repr(C)]
pub struct ocfs2_filecheck_args {
    pub fa_type: c_uint,
    pub value: ocfs2_filecheck_args_value,
}

unsafe fn ocfs2_filecheck_error(errno: c_int) -> *const u8 {
    if errno == 0 { return OCFS2_FILECHECK_ERRS[0].as_ptr(); }
    BUG_ON(errno < OCFS2_FILECHECK_ERR_START || errno > OCFS2_FILECHECK_ERR_END);
    OCFS2_FILECHECK_ERRS[(errno - OCFS2_FILECHECK_ERR_START + 1) as usize].as_ptr()
}

unsafe extern "C" {
    fn ocfs2_filecheck_attr_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t;
    fn ocfs2_filecheck_attr_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *const c_char, count: size_t) -> ssize_t;
}

unsafe fn ocfs2_filecheck_release(kobj: *mut kobject) {
    let entry = container_of!(kobj, ocfs2_filecheck_sysfs_entry, fs_kobj);
    complete(&mut (*entry).fs_kobj_unregister);
}

unsafe fn ocfs2_filecheck_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> ssize_t {
    let mut ret: ssize_t = -EIO;
    let kattr = container_of!(attr, kobj_attribute, attr);
    kobject_get(kobj);
    if !(*kattr).show.is_null() { ret = ((*kattr).show)(kobj, kattr, buf); }
    kobject_put(kobj);
    ret
}

unsafe fn ocfs2_filecheck_store(kobj: *mut kobject, attr: *mut attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let mut ret: ssize_t = -EIO;
    let kattr = container_of!(attr, kobj_attribute, attr);
    kobject_get(kobj);
    if !(*kattr).store.is_null() { ret = ((*kattr).store)(kobj, kattr, buf, count); }
    kobject_put(kobj);
    ret
}

unsafe fn ocfs2_filecheck_sysfs_free(entry: *mut ocfs2_filecheck_sysfs_entry) {
    spin_lock(&mut (*(*entry).fs_fcheck).fc_lock);
    while !list_empty(&(*(*entry).fs_fcheck).fc_head) {
        let p = list_first_entry!(&mut (*(*entry).fs_fcheck).fc_head, ocfs2_filecheck_entry, fe_list);
        list_del(&mut (*p).fe_list);
        BUG_ON((*p).fe_done == 0);
        kfree(p as *mut c_void);
    }
    spin_unlock(&mut (*(*entry).fs_fcheck).fc_lock);
    kfree((*entry).fs_fcheck as *mut c_void);
    (*entry).fs_fcheck = core::ptr::null_mut();
}

pub unsafe fn ocfs2_filecheck_create_sysfs(osb: *mut ocfs2_super) -> c_int {
    let fcheck = kmalloc_obj!(ocfs2_filecheck, GFP_NOFS);
    if fcheck.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD(&mut (*fcheck).fc_head);
    spin_lock_init(&mut (*fcheck).fc_lock);
    (*fcheck).fc_max = OCFS2_FILECHECK_MINSIZE;
    (*fcheck).fc_size = 0;
    (*fcheck).fc_done = 0;
    let entry = &mut (*osb).osb_fc_ent;
    (*entry).fs_kobj.kset = (*osb).osb_dev_kset;
    init_completion(&mut (*entry).fs_kobj_unregister);
    let ret = kobject_init_and_add(&mut (*entry).fs_kobj, &ocfs2_ktype_filecheck, core::ptr::null_mut(), b"filecheck\0".as_ptr() as *const c_char);
    if ret != 0 { kobject_put(&mut (*entry).fs_kobj); kfree(fcheck as *mut c_void); return ret; }
    (*entry).fs_fcheck = fcheck;
    0
}

pub unsafe fn ocfs2_filecheck_remove_sysfs(osb: *mut ocfs2_super) {
    if (*osb).osb_fc_ent.fs_fcheck.is_null() { return; }
    kobject_del(&mut (*osb).osb_fc_ent.fs_kobj);
    kobject_put(&mut (*osb).osb_fc_ent.fs_kobj);
    wait_for_completion(&mut (*osb).osb_fc_ent.fs_kobj_unregister);
    ocfs2_filecheck_sysfs_free(&mut (*osb).osb_fc_ent);
}

const OCFS2_FILECHECK_ARGS_LEN: usize = 24;

unsafe fn ocfs2_filecheck_args_get_long(buf: *const c_char, count: usize, val: *mut c_ulong) -> c_int {
    let mut buffer = [0i8; OCFS2_FILECHECK_ARGS_LEN];
    core::ptr::copy_nonoverlapping(buf, buffer.as_mut_ptr(), count);
    buffer[count] = 0;
    if kstrtoul(buffer.as_ptr(), 0, val) != 0 { return 1; }
    0
}

unsafe fn ocfs2_filecheck_type_parse(name: *const c_char, ty: *mut c_uint) -> c_int {
    if !strncmp(name, b"fix\0".as_ptr() as *const c_char, 4) { *ty = OCFS2_FILECHECK_TYPE_FIX; }
    else if !strncmp(name, b"check\0".as_ptr() as *const c_char, 6) { *ty = OCFS2_FILECHECK_TYPE_CHK; }
    else if !strncmp(name, b"set\0".as_ptr() as *const c_char, 4) { *ty = OCFS2_FILECHECK_TYPE_SET; }
    else { return 1; }
    0
}

unsafe fn ocfs2_filecheck_args_parse(name: *const c_char, buf: *const c_char, count: usize, args: *mut ocfs2_filecheck_args) -> c_int {
    let mut val: c_ulong = 0; let mut ty = 0;
    if count < 1 || count >= OCFS2_FILECHECK_ARGS_LEN { return 1; }
    if ocfs2_filecheck_type_parse(name, &mut ty) != 0 || ocfs2_filecheck_args_get_long(buf, count, &mut val) != 0 || val <= 0 { return 1; }
    (*args).fa_type = ty;
    if ty == OCFS2_FILECHECK_TYPE_SET { (*args).value.fa_len = val as c_uint; } else { (*args).value.fa_ino = val; }
    0
}

unsafe fn ocfs2_filecheck_is_dup_entry(ent: *mut ocfs2_filecheck_sysfs_entry, ino: c_ulong) -> c_int {
    let mut p: *mut ocfs2_filecheck_entry;
    list_for_each_entry!(p, &mut (*(*ent).fs_fcheck).fc_head, fe_list) {
        if (*p).fe_done == 0 && (*p).fe_ino == ino { return 1; }
    }
    0
}

unsafe fn ocfs2_filecheck_erase_entry(ent: *mut ocfs2_filecheck_sysfs_entry) -> c_int {
    let mut p: *mut ocfs2_filecheck_entry;
    list_for_each_entry!(p, &mut (*(*ent).fs_fcheck).fc_head, fe_list) {
        if (*p).fe_done != 0 {
            list_del(&mut (*p).fe_list); kfree(p as *mut c_void);
            (*(*ent).fs_fcheck).fc_size -= 1; (*(*ent).fs_fcheck).fc_done -= 1;
            return 1;
        }
    }
    0
}

unsafe fn ocfs2_filecheck_erase_entries(ent: *mut ocfs2_filecheck_sysfs_entry, count: c_uint) -> c_int {
    let mut i = 0; let mut ret = 0;
    while { i += 1; i <= count } { if ocfs2_filecheck_erase_entry(ent) != 0 { ret += 1; } else { break; } }
    if ret == count { 1 } else { 0 }
}

unsafe fn ocfs2_filecheck_adjust_max(ent: *mut ocfs2_filecheck_sysfs_entry, len: c_uint) -> c_int {
    if len < OCFS2_FILECHECK_MINSIZE || len > OCFS2_FILECHECK_MAXSIZE { return -EINVAL; }
    spin_lock(&mut (*(*ent).fs_fcheck).fc_lock);
    let pending = (*(*ent).fs_fcheck).fc_size - (*(*ent).fs_fcheck).fc_done;
    let ret;
    if len < pending { ret = -EBUSY; }
    else { if len < (*(*ent).fs_fcheck).fc_size { BUG_ON(ocfs2_filecheck_erase_entries(ent, (*(*ent).fs_fcheck).fc_size-len) == 0); } (*(*ent).fs_fcheck).fc_max=len; ret=0; }
    spin_unlock(&mut (*(*ent).fs_fcheck).fc_lock); ret
}

unsafe fn ocfs2_filecheck_done_entry(ent: *mut ocfs2_filecheck_sysfs_entry, entry: *mut ocfs2_filecheck_entry) {
    spin_lock(&mut (*(*ent).fs_fcheck).fc_lock); (*entry).fe_done=1; (*(*ent).fs_fcheck).fc_done += 1; spin_unlock(&mut (*(*ent).fs_fcheck).fc_lock);
}

unsafe fn ocfs2_filecheck_handle(osb: *mut ocfs2_super, ino: c_ulong, flags: c_uint) -> c_uint {
    let inode = ocfs2_iget(osb, ino, flags, 0);
    if IS_ERR(inode) { OCFS2_FILECHECK_ERR_FAILED } else { iput(inode); OCFS2_FILECHECK_ERR_SUCCESS }
}

unsafe fn ocfs2_filecheck_handle_entry(ent: *mut ocfs2_filecheck_sysfs_entry, entry: *mut ocfs2_filecheck_entry) {
    let osb = container_of!(ent, ocfs2_super, osb_fc_ent);
    (*entry).fe_status = if (*entry).fe_type == OCFS2_FILECHECK_TYPE_CHK { ocfs2_filecheck_handle(osb, (*entry).fe_ino, OCFS2_FI_FLAG_FILECHECK_CHK) }
        else if (*entry).fe_type == OCFS2_FILECHECK_TYPE_FIX { ocfs2_filecheck_handle(osb, (*entry).fe_ino, OCFS2_FI_FLAG_FILECHECK_FIX) }
        else { OCFS2_FILECHECK_ERR_UNSUPPORTED };
    ocfs2_filecheck_done_entry(ent, entry);
}

unsafe fn ocfs2_filecheck_attr_store_impl(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *const c_char, count: usize) -> ssize_t {
    if count == 0 { return count as ssize_t; }
    let ent = container_of!(kobj, ocfs2_filecheck_sysfs_entry, fs_kobj); let mut args = core::mem::MaybeUninit::<ocfs2_filecheck_args>::uninit();
    if ocfs2_filecheck_args_parse((*attr).attr.name, buf, count, args.as_mut_ptr()) != 0 { return -EINVAL; }
    let args = args.assume_init();
    if args.fa_type == OCFS2_FILECHECK_TYPE_SET { return ocfs2_filecheck_adjust_max(ent, args.value.fa_len as c_uint) as ssize_t; }
    let entry = kmalloc_obj!(ocfs2_filecheck_entry, GFP_NOFS); if entry.is_null() { return -ENOMEM; }
    spin_lock(&mut (*(*ent).fs_fcheck).fc_lock); let mut ret=0;
    if ocfs2_filecheck_is_dup_entry(ent,args.value.fa_ino)!=0 { ret=-EEXIST; kfree(entry as *mut c_void); }
    else if (*(*ent).fs_fcheck).fc_size >= (*(*ent).fs_fcheck).fc_max && (*(*ent).fs_fcheck).fc_done==0 { ret=-EAGAIN; kfree(entry as *mut c_void); }
    else { if (*(*ent).fs_fcheck).fc_size >= (*(*ent).fs_fcheck).fc_max { BUG_ON(ocfs2_filecheck_erase_entry(ent)==0); } (*entry).fe_ino=args.value.fa_ino; (*entry).fe_type=args.fa_type; (*entry).fe_done=0; (*entry).fe_status=OCFS2_FILECHECK_ERR_INPROGRESS; list_add_tail(&mut (*entry).fe_list,&mut (*(*ent).fs_fcheck).fc_head); (*(*ent).fs_fcheck).fc_size+=1; }
    spin_unlock(&mut (*(*ent).fs_fcheck).fc_lock); if ret==0 { ocfs2_filecheck_handle_entry(ent,entry); count as ssize_t } else { ret as ssize_t }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
