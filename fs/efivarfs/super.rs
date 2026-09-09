// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Red Hat, Inc.
 * Copyright (C) 2012 Jeremy Kerr <jeremy.kerr@canonical.com>
 */

/* Linux kernel dependencies are supplied by the surrounding translation. */

unsafe fn efivarfs_ops_notifier(nb: *mut notifier_block, event: c_ulong, _data: *mut c_void) -> c_int {
    let sfi = container_of!(nb, efivarfs_fs_info, nb);
    match event {
        EFIVAR_OPS_RDONLY => {
            (*(*sfi).sb).s_flags |= SB_RDONLY;
        }
        EFIVAR_OPS_RDWR => {
            (*(*sfi).sb).s_flags &= !SB_RDONLY;
        }
        _ => return NOTIFY_DONE,
    }
    NOTIFY_OK
}

unsafe fn efivarfs_alloc_inode(sb: *mut super_block) -> *mut inode {
    let entry = kzalloc_obj!(efivar_entry);
    if entry.is_null() { return core::ptr::null_mut(); }
    inode_init_once(&mut (*entry).vfs_inode);
    (*entry).removed = false;
    &mut (*entry).vfs_inode
}

unsafe fn efivarfs_free_inode(inode: *mut inode) {
    let entry = efivar_entry(inode);
    kfree(entry as *mut c_void);
}

unsafe fn efivarfs_show_options(m: *mut seq_file, root: *mut dentry) -> c_int {
    let sb = (*root).d_sb;
    let sbi = (*sb).s_fs_info as *mut efivarfs_fs_info;
    let opts = &mut (*sbi).mount_opts;
    if !uid_eq(opts.uid, GLOBAL_ROOT_UID) { seq_printf(m, c",uid=%u", from_kuid_munged(&init_user_ns, opts.uid)); }
    if !gid_eq(opts.gid, GLOBAL_ROOT_GID) { seq_printf(m, c",gid=%u", from_kgid_munged(&init_user_ns, opts.gid)); }
    0
}

unsafe fn efivarfs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int {
    let attr: u32 = EFI_VARIABLE_NON_VOLATILE | EFI_VARIABLE_BOOTSERVICE_ACCESS | EFI_VARIABLE_RUNTIME_ACCESS;
    let mut storage_space: u64 = 0;
    let mut remaining_space: u64 = 0;
    let mut max_variable_size: u64 = 0;
    let id = huge_encode_dev((*(*dentry).d_sb).s_dev);
    let mut status: efi_status_t;

    if efi_rt_services_supported(EFI_RT_SUPPORTED_QUERY_VARIABLE_INFO) {
        static mut STORAGE: u64 = 0;
        static mut REMAINING: u64 = 0;
        static mut RS: ratelimit_state = DEFINE_RATELIMIT_STATE!(2 * HZ, 5);
        static mut LOCK: spinlock_t = DEFINE_SPINLOCK!();
        if !__ratelimit(&mut RS) {
            ratelimit_set_flags(&mut RS, RATELIMIT_MSG_ON_RELEASE);
            spin_lock(&mut LOCK);
            storage_space = STORAGE;
            remaining_space = REMAINING;
            spin_unlock(&mut LOCK);
        } else {
            status = efivar_query_variable_info(attr, &mut storage_space, &mut remaining_space, &mut max_variable_size);
            if status != EFI_SUCCESS && status != EFI_UNSUPPORTED { pr_warn!(c"query_variable_info() failed: 0x%lx\n", status); }
            spin_lock(&mut LOCK);
            STORAGE = storage_space;
            REMAINING = remaining_space;
            spin_unlock(&mut LOCK);
        }
    }
    (*buf).f_bsize = 1;
    (*buf).f_namelen = NAME_MAX;
    (*buf).f_blocks = storage_space;
    (*buf).f_bfree = remaining_space;
    (*buf).f_type = (*(*dentry).d_sb).s_magic;
    (*buf).f_fsid = u64_to_fsid(id);
    (*buf).f_bavail = if remaining_space > efivar_reserved_space() { remaining_space - efivar_reserved_space() } else { 0 };
    0
}

unsafe fn efivarfs_freeze_fs(_sb: *mut super_block) -> c_int { 0 }

unsafe fn efivarfs_d_compare(_dentry: *const dentry, len: c_uint, str_: *const c_char, name: *const qstr) -> c_int {
    let guid = len as c_int - EFI_VARIABLE_GUID_LEN as c_int;
    if guid <= 0 || (*name).len != len { return 1; }
    if memcmp(str_ as *const c_void, (*name).name as *const c_void, guid as usize) != 0 { return 1; }
    strncasecmp((*name).name.add(guid as usize), str_.add(guid as usize), EFI_VARIABLE_GUID_LEN)
}

unsafe fn efivarfs_d_hash(_dentry: *const dentry, qstr: *mut qstr) -> c_int {
    let mut hash = init_name_hash(_dentry);
    let mut s = (*qstr).name;
    let mut len = (*qstr).len;
    while len > EFI_VARIABLE_GUID_LEN { hash = partial_name_hash(*s, hash); s = s.add(1); len -= 1; }
    while len > 0 { hash = partial_name_hash(tolower(*s), hash); s = s.add(1); len -= 1; }
    (*qstr).hash = end_name_hash(hash); 0
}

unsafe fn efivarfs_alloc_dentry(parent: *mut dentry, name: *mut c_char) -> *mut dentry {
    let mut q = QSTR!(name);
    let err = efivarfs_d_hash(parent, &mut q);
    if err != 0 { return ERR_PTR(err as isize); }
    let d = d_alloc(parent, &q);
    if !d.is_null() { d } else { ERR_PTR(-ENOMEM as isize) }
}

pub unsafe fn efivarfs_variable_is_present(variable_name: *mut efi_char16_t, vendor: *mut efi_guid_t, data: *mut c_void) -> bool {
    let name = efivar_get_utf8name(variable_name, vendor);
    if name.is_null() { return true; }
    let sb = data as *mut super_block;
    let dentry = try_lookup_noperm(&QSTR!(name), (*sb).s_root);
    kfree(name as *mut c_void);
    if !IS_ERR_OR_NULL(dentry) { dput(dentry); }
    !dentry.is_null()
}

unsafe fn efivarfs_create_dentry(sb: *mut super_block, name16: *mut efi_char16_t, name_size: c_ulong, vendor: efi_guid_t, name: *mut c_char) -> c_int {
    let mut size: c_ulong = 0;
    let len = strlen(name) as c_int - EFI_VARIABLE_GUID_LEN as c_int - 1;
    let is_removable = efivar_variable_is_removable(vendor, name, len);
    let inode = efivarfs_get_inode(sb, (*(*sb).s_root).d_inode, S_IFREG | 0o644, 0, is_removable);
    if inode.is_null() { kfree(name as *mut c_void); return -ENOMEM; }
    let entry = efivar_entry(inode);
    memcpy((*entry).var.VariableName.as_mut_ptr() as *mut c_void, name16 as *const c_void, name_size as usize);
    memcpy(&mut (*entry).var.VendorGuid as *mut _ as *mut c_void, &vendor as *const _ as *const c_void, core::mem::size_of::<efi_guid_t>());
    let dentry = efivarfs_alloc_dentry((*sb).s_root, name);
    if IS_ERR(dentry) { let err = PTR_ERR(dentry); iput(inode); kfree(name as *mut c_void); return err; }
    __efivar_entry_get(entry, core::ptr::null_mut(), &mut size, core::ptr::null_mut());
    kfree(name as *mut c_void);
    inode_lock(inode); (*inode).i_private = entry as *mut c_void; i_size_write(inode, size + core::mem::size_of::<u32>() as u64); inode_unlock(inode);
    d_make_persistent(dentry, inode); dput(dentry); 0
}

unsafe fn efivarfs_callback(name16: *mut efi_char16_t, vendor: efi_guid_t, name_size: c_ulong, data: *mut c_void) -> c_int {
    if guid_equal(&vendor, &LINUX_EFI_RANDOM_SEED_TABLE_GUID) { return 0; }
    let name = efivar_get_utf8name(name16, &vendor);
    if name.is_null() { return -ENOMEM; }
    efivarfs_create_dentry(data as *mut super_block, name16, name_size, vendor, name)
}

enum { Opt_uid, Opt_gid }

static EFIVARFS_PARAMETERS: [fs_parameter_spec; 3] = [fsparam_uid!(c"uid", Opt_uid), fsparam_gid!(c"gid", Opt_gid), fsparam_empty!()];

unsafe fn efivarfs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let sbi = (*fc).s_fs_info as *mut efivarfs_fs_info;
    let opts = &mut (*sbi).mount_opts;
    let mut result = core::mem::zeroed::<fs_parse_result>();
    let opt = fs_parse(fc, EFIVARFS_PARAMETERS.as_ptr(), param, &mut result);
    if opt < 0 { return opt; }
    match opt { Opt_uid => opts.uid = result.uid, Opt_gid => opts.gid = result.gid, _ => return -EINVAL }
    0
}

unsafe fn efivarfs_fill_super(sb: *mut super_block, _fc: *mut fs_context) -> c_int {
    let sfi = (*sb).s_fs_info as *mut efivarfs_fs_info;
    (*sb).s_maxbytes = MAX_LFS_FILESIZE; (*sb).s_blocksize = PAGE_SIZE; (*sb).s_blocksize_bits = PAGE_SHIFT; (*sb).s_magic = EFIVARFS_MAGIC; (*sb).s_op = &efivarfs_ops; set_default_d_op(sb, &efivarfs_d_ops); (*sb).s_d_flags |= DCACHE_DONTCACHE; (*sb).s_time_gran = 1;
    if !efivar_supports_writes() { (*sb).s_flags |= SB_RDONLY; }
    let inode = efivarfs_get_inode(sb, core::ptr::null_mut(), S_IFDIR | 0o755, 0, true);
    if inode.is_null() { return -ENOMEM; }
    (*inode).i_op = &efivarfs_dir_inode_operations;
    let root = d_make_root(inode); (*sb).s_root = root;
    if root.is_null() { return -ENOMEM; }
    (*sfi).sb = sb; (*sfi).nb.notifier_call = Some(efivarfs_ops_notifier);
    let err = blocking_notifier_chain_register(&mut efivar_ops_nh, &mut (*sfi).nb);
    if err != 0 { return err; }
    efivar_init(efivarfs_callback, sb as *mut c_void, true)
}

unsafe fn efivarfs_get_tree(fc: *mut fs_context) -> c_int { get_tree_single(fc, efivarfs_fill_super) }
unsafe fn efivarfs_reconfigure(fc: *mut fs_context) -> c_int { if !efivar_supports_writes() && ((*fc).sb_flags & SB_RDONLY) == 0 { pr_err!(c"Firmware does not support SetVariableRT. Can not remount with rw\n"); return -EINVAL; } 0 }
unsafe fn efivarfs_free(fc: *mut fs_context) { kfree((*fc).s_fs_info); }

unsafe fn efivarfs_check_missing(name16: *mut efi_char16_t, vendor: efi_guid_t, name_size: c_ulong, data: *mut c_void) -> c_int {
    if guid_equal(&vendor, &LINUX_EFI_RANDOM_SEED_TABLE_GUID) { return 0; }
    let name = efivar_get_utf8name(name16, &vendor); if name.is_null() { return -ENOMEM; }
    let sb = data as *mut super_block; let dentry = try_lookup_noperm(&QSTR!(name), (*sb).s_root);
    if IS_ERR(dentry) { let err = PTR_ERR(dentry); kfree(name as *mut c_void); return err; }
    if dentry.is_null() { pr_info!(c"efivarfs: creating variable %s\n", name); return efivarfs_create_dentry(sb, name16, name_size, vendor, name); }
    dput(dentry); kfree(name as *mut c_void); 0
}

unsafe fn efivarfs_unfreeze_fs(sb: *mut super_block) -> c_int {
    let mut child: *mut dentry = core::ptr::null_mut();
    pr_info!(c"efivarfs: resyncing variable state\n");
    loop {
        child = find_next_child((*sb).s_root, child); if child.is_null() { break; }
        let inode = d_inode(child); let entry = efivar_entry(inode); let mut size = 0; if efivar_entry_size(entry, &mut size) != 0 { size = 0; } else { size += core::mem::size_of::<u32>() as u64; }
        inode_lock(inode); i_size_write(inode, size); inode_unlock(inode);
        if size == 0 { pr_info!(c"efivarfs: removing variable %pd\n", child); simple_recursive_removal(child, core::ptr::null_mut()); }
    }
    efivar_init(efivarfs_check_missing, sb as *mut c_void, false); pr_info!(c"efivarfs: finished resyncing variable state\n"); 0
}

unsafe fn efivarfs_init_fs_context(fc: *mut fs_context) -> c_int {
    if !efivar_is_available() { return -EOPNOTSUPP; }
    let sfi = kzalloc_obj!(efivarfs_fs_info); if sfi.is_null() { return -ENOMEM; }
    (*sfi).mount_opts.uid = GLOBAL_ROOT_UID; (*sfi).mount_opts.gid = GLOBAL_ROOT_GID; (*fc).s_fs_info = sfi as *mut c_void; (*fc).ops = &efivarfs_context_ops; 0
}

unsafe fn efivarfs_kill_sb(sb: *mut super_block) { let sfi = (*sb).s_fs_info as *mut efivarfs_fs_info; blocking_notifier_chain_unregister(&mut efivar_ops_nh, &mut (*sfi).nb); kill_anon_super(sb); kfree(sfi as *mut c_void); }

static mut EFIVARFS_TYPE: file_system_type = file_system_type { owner: THIS_MODULE, name: c"efivarfs", init_fs_context: Some(efivarfs_init_fs_context), kill_sb: Some(efivarfs_kill_sb), parameters: EFIVARFS_PARAMETERS.as_ptr(), fs_flags: FS_POWER_FREEZE };

unsafe fn efivarfs_init() -> c_int { register_filesystem(&mut EFIVARFS_TYPE) }
unsafe fn efivarfs_exit() { unregister_filesystem(&mut EFIVARFS_TYPE); }

// MODULE_AUTHOR("Matthew Garrett, Jeremy Kerr");
// MODULE_DESCRIPTION("EFI Variable Filesystem");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_FS("efivarfs");
// module_init(efivarfs_init);
// module_exit(efivarfs_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
