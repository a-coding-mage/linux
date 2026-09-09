// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Red Hat, Inc.
 * Copyright (C) 2012 Jeremy Kerr <jeremy.kerr@canonical.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

extern "C" {
    static efivarfs_file_inode_operations: inode_operations;
    static efivarfs_file_operations: file_operations;
    static efivarfs_dir_inode_operations: inode_operations;
    static simple_dir_operations: file_operations;
    static LINUX_EFI_RANDOM_SEED_TABLE_GUID: efi_guid_t;

    fn new_inode(sb: *mut super_block) -> *mut inode;
    fn get_next_ino() -> u64;
    fn inc_nlink(inode: *mut inode);
    fn uuid_is_valid(uuid: *const i8) -> bool;
    fn guid_parse(name: *const i8, guid: *mut efi_guid_t) -> i32;
    fn guid_equal(a: *const efi_guid_t, b: *const efi_guid_t) -> bool;
    fn efivar_variable_is_removable(guid: efi_guid_t, name: *const i8, len: i32) -> bool;
    fn efivar_entry(inode: *mut inode) -> *mut efivar_entry;
    fn d_make_persistent(dentry: *mut dentry, inode: *mut inode);
    fn d_inode(dentry: *mut dentry) -> *mut inode;
    fn efivar_entry_delete(var: *mut efivar_entry) -> i32;
    fn simple_unlink(dir: *mut inode, dentry: *mut dentry) -> i32;
    fn fileattr_fill_flags(fa: *mut file_kattr, flags: u32);
    fn fileattr_has_fsx(fa: *mut file_kattr) -> bool;
    fn inode_set_flags(inode: *mut inode, flags: u32, mask: u32);
    fn setattr_prepare(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> i32;
    fn setattr_copy(idmap: *mut mnt_idmap, inode: *mut inode, attr: *mut iattr);
    fn mark_inode_dirty(inode: *mut inode);
    fn simple_lookup(dir: *mut inode, dentry: *mut dentry, flags: u32) -> *mut dentry;
}

pub unsafe fn efivarfs_get_inode(
    sb: *mut super_block,
    _dir: *const inode,
    mode: umode_t,
    _dev: dev_t,
    is_removable: bool,
) -> *mut inode {
    let inode = new_inode(sb);
    let fsi = (*sb).s_fs_info as *mut efivarfs_fs_info;
    let opts = &mut (*fsi).mount_opts;

    if !inode.is_null() {
        (*inode).i_uid = opts.uid;
        (*inode).i_gid = opts.gid;
        (*inode).i_ino = get_next_ino();
        (*inode).i_mode = mode;
        simple_inode_init_ts(inode);
        (*inode).i_flags = if is_removable { 0 } else { S_IMMUTABLE };
        match mode & S_IFMT {
            S_IFREG => {
                (*inode).i_op = &efivarfs_file_inode_operations;
                (*inode).i_fop = &efivarfs_file_operations;
            }
            S_IFDIR => {
                (*inode).i_op = &efivarfs_dir_inode_operations;
                (*inode).i_fop = &simple_dir_operations;
                inc_nlink(inode);
            }
            _ => {}
        }
    }
    inode
}

/* Return true if `str` is a valid efivarfs filename of the documented form. */
unsafe fn efivarfs_valid_name(str_: *const i8, len: i32) -> bool {
    let s = str_.add((len - EFI_VARIABLE_GUID_LEN) as usize);
    if len < EFI_VARIABLE_GUID_LEN + 2 {
        return false;
    }
    if *s.sub(1) != b'-' as i8 {
        return false;
    }
    uuid_is_valid(s)
}

unsafe fn efivarfs_create(
    _idmap: *mut mnt_idmap,
    dir: *mut inode,
    dentry: *mut dentry,
    mode: umode_t,
) -> i32 {
    let mut inode: *mut inode = core::ptr::null_mut();
    let mut var: *mut efivar_entry;
    let namelen: i32;
    let mut i: i32 = 0;
    let mut err: i32 = 0;
    let mut is_removable = false;
    let mut vendor: efi_guid_t = core::mem::zeroed();

    if !efivarfs_valid_name((*dentry).d_name.name, (*dentry).d_name.len) {
        return -EINVAL;
    }
    namelen = (*dentry).d_name.len - EFI_VARIABLE_GUID_LEN - 1;
    err = guid_parse((*dentry).d_name.name.add((namelen + 1) as usize), &mut vendor);
    if err != 0 { return err; }
    if guid_equal(&vendor, &LINUX_EFI_RANDOM_SEED_TABLE_GUID) { return -EPERM; }
    if efivar_variable_is_removable(vendor, (*dentry).d_name.name, namelen) { is_removable = true; }
    inode = efivarfs_get_inode((*dir).i_sb, dir, mode, 0, is_removable);
    if inode.is_null() { return -ENOMEM; }
    var = efivar_entry(inode);
    (*var).var.VendorGuid = vendor;
    while i < namelen {
        (*var).var.VariableName[i as usize] = *(*dentry).d_name.name.add(i as usize) as _;
        i += 1;
    }
    (*var).var.VariableName[i as usize] = 0;
    (*inode).i_private = var as *mut core::ffi::c_void;
    d_make_persistent(dentry, inode);
    0
}

unsafe fn efivarfs_unlink(dir: *mut inode, dentry: *mut dentry) -> i32 {
    let var = (*d_inode(dentry)).i_private as *mut efivar_entry;
    if efivar_entry_delete(var) != 0 { return -EINVAL; }
    simple_unlink(dir, dentry)
}

/* The operation tables are defined by the kernel's inode-operation type. */
#[no_mangle]
pub static efivarfs_dir_inode_operations_rust: inode_operations = inode_operations {
    lookup: Some(simple_lookup), unlink: Some(efivarfs_unlink), create: Some(efivarfs_create),
};

unsafe fn efivarfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> i32 {
    let i_flags = (*d_inode(dentry)).i_flags;
    let mut flags = 0;
    if i_flags & S_IMMUTABLE != 0 { flags |= FS_IMMUTABLE_FL; }
    fileattr_fill_flags(fa, flags);
    0
}

unsafe fn efivarfs_fileattr_set(_idmap: *mut mnt_idmap, dentry: *mut dentry, fa: *mut file_kattr) -> i32 {
    if fileattr_has_fsx(fa) || (*fa).flags & !FS_IMMUTABLE_FL != 0 { return -EOPNOTSUPP; }
    let i_flags = if (*fa).flags & FS_IMMUTABLE_FL != 0 { S_IMMUTABLE } else { 0 };
    inode_set_flags(d_inode(dentry), i_flags, S_IMMUTABLE);
    0
}

/* copy of simple_setattr except that it doesn't do i_size updates */
unsafe fn efivarfs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, iattr: *mut iattr) -> i32 {
    let inode = d_inode(dentry);
    let error = setattr_prepare(idmap, dentry, iattr);
    if error != 0 { return error; }
    setattr_copy(idmap, inode, iattr);
    mark_inode_dirty(inode);
    0
}

static efivarfs_file_inode_operations_rust: inode_operations = inode_operations {
    fileattr_get: Some(efivarfs_fileattr_get),
    fileattr_set: Some(efivarfs_fileattr_set),
    setattr: Some(efivarfs_setattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
