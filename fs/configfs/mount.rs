// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * mount.c - operations for initializing and mounting configfs.
 *
 * Based on sysfs:
 *	sysfs is Copyright (C) 2001, 2002, 2003 Patrick Mochel
 *
 * configfs Copyright (C) 2005 Oracle.  All rights reserved.
 */

// Linux kernel headers and "configfs_internal.h" provide the declarations
// referenced below.

pub const CONFIGFS_MAGIC: u32 = 0x62656570;

static mut configfs_mount: *mut vfsmount = core::ptr::null_mut();
pub static mut configfs_dir_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut configfs_mnt_count: core::ffi::c_int = 0;

unsafe fn configfs_free_inode(inode: *mut inode) {
    if S_ISLNK((*inode).i_mode) {
        kfree((*inode).i_link);
    }
    free_inode_nonrcu(inode);
}

static configfs_ops: super_operations = super_operations {
    statfs: Some(simple_statfs),
    drop_inode: Some(inode_just_drop),
    free_inode: Some(configfs_free_inode),
};

static mut configfs_root_group: config_group = config_group {
    cg_item: config_item {
        ci_namebuf: *b"root\0",
        ci_name: core::ptr::null(),
    },
};

pub unsafe fn configfs_is_root(item: *mut config_item) -> bool {
    item == core::ptr::addr_of_mut!(configfs_root_group.cg_item)
}

static mut configfs_root: configfs_dirent = configfs_dirent {
    s_sibling: LIST_HEAD_INIT(),
    s_children: LIST_HEAD_INIT(),
    s_element: core::ptr::addr_of_mut!(configfs_root_group.cg_item),
    s_type: CONFIGFS_ROOT,
    s_iattr: core::ptr::null_mut(),
};

unsafe fn configfs_fill_super(sb: *mut super_block, _fc: *mut fs_context) -> core::ffi::c_int {
    (*sb).s_blocksize = PAGE_SIZE;
    (*sb).s_blocksize_bits = PAGE_SHIFT;
    (*sb).s_magic = CONFIGFS_MAGIC;
    (*sb).s_op = core::ptr::addr_of!(configfs_ops);
    (*sb).s_time_gran = 1;

    let inode = configfs_new_inode(S_IFDIR | S_IRWXU | S_IRUGO | S_IXUGO,
                                   core::ptr::addr_of_mut!(configfs_root), sb);
    if !inode.is_null() {
        (*inode).i_op = core::ptr::addr_of!(configfs_root_inode_operations);
        (*inode).i_fop = core::ptr::addr_of!(configfs_dir_operations);
        // directory inodes start off with i_nlink == 2 (for "." entry)
        inc_nlink(inode);
    } else {
        pr_debug!("could not get root inode\n");
        return -ENOMEM;
    }

    let root = d_make_root(inode);
    if root.is_null() {
        pr_debug!("%s: could not get root dentry!\n", c"configfs_fill_super");
        return -ENOMEM;
    }
    config_group_init(core::ptr::addr_of_mut!(configfs_root_group));
    (*core::ptr::addr_of_mut!(configfs_root_group)).cg_item.ci_dentry = root;
    (*root).d_fsdata = core::ptr::addr_of_mut!(configfs_root) as *mut core::ffi::c_void;
    (*sb).s_root = root;
    set_default_d_op(sb, core::ptr::addr_of!(configfs_dentry_ops));
    (*sb).s_d_flags |= DCACHE_DONTCACHE;
    0
}

unsafe fn configfs_get_tree(fc: *mut fs_context) -> core::ffi::c_int {
    get_tree_single(fc, Some(configfs_fill_super))
}

static configfs_context_ops: fs_context_operations = fs_context_operations {
    get_tree: Some(configfs_get_tree),
};

unsafe fn configfs_init_fs_context(fc: *mut fs_context) -> core::ffi::c_int {
    (*fc).ops = core::ptr::addr_of!(configfs_context_ops);
    0
}

static mut configfs_fs_type: file_system_type = file_system_type {
    owner: THIS_MODULE,
    name: *b"configfs\0",
    init_fs_context: Some(configfs_init_fs_context),
    kill_sb: Some(kill_anon_super),
};

pub unsafe fn configfs_pin_fs() -> *mut dentry {
    let err = simple_pin_fs(core::ptr::addr_of_mut!(configfs_fs_type),
                            core::ptr::addr_of_mut!(configfs_mount),
                            core::ptr::addr_of_mut!(configfs_mnt_count));
    if err != 0 {
        ERR_PTR(err)
    } else {
        (*configfs_mount).mnt_root
    }
}

pub unsafe fn configfs_release_fs() {
    simple_release_fs(core::ptr::addr_of_mut!(configfs_mount),
                      core::ptr::addr_of_mut!(configfs_mnt_count));
}

unsafe fn configfs_init() -> core::ffi::c_int {
    let mut err = -ENOMEM;
    configfs_dir_cachep = kmem_cache_create(c"configfs_dir_cache",
                                             core::mem::size_of::<configfs_dirent>(),
                                             0, 0, core::ptr::null_mut());
    if configfs_dir_cachep.is_null() {
        return err;
    }

    err = sysfs_create_mount_point(kernel_kobj, c"config");
    if err != 0 {
        kmem_cache_destroy(configfs_dir_cachep);
        configfs_dir_cachep = core::ptr::null_mut();
        return err;
    }

    err = register_filesystem(core::ptr::addr_of_mut!(configfs_fs_type));
    if err != 0 {
        pr_err!("Unable to register filesystem!\n");
        sysfs_remove_mount_point(kernel_kobj, c"config");
        kmem_cache_destroy(configfs_dir_cachep);
        configfs_dir_cachep = core::ptr::null_mut();
        return err;
    }
    0
}

unsafe fn configfs_exit() {
    unregister_filesystem(core::ptr::addr_of_mut!(configfs_fs_type));
    sysfs_remove_mount_point(kernel_kobj, c"config");
    kmem_cache_destroy(configfs_dir_cachep);
    configfs_dir_cachep = core::ptr::null_mut();
}

// MODULE_ALIAS_FS("configfs");
// MODULE_AUTHOR("Oracle");
// MODULE_LICENSE("GPL");
// MODULE_VERSION("0.0.2");
// MODULE_DESCRIPTION("Simple RAM filesystem for user driven kernel subsystem configuration.");
// core_initcall(configfs_init);
// module_exit(configfs_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
