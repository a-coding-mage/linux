// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * eCryptfs: Linux filesystem encryption layer
 *
 * Copyright (C) 1997-2003 Erez Zadok
 * Copyright (C) 2001-2003 Stony Brook University
 * Copyright (C) 2004-2006 International Business Machines Corp.
 *   Author(s): Michael A. Halcrow <mahalcro@us.ibm.com>
 *              Michael C. Thompson <mcthomps@us.ibm.com>
 */

// Linux kernel headers and "ecryptfs_kernel.h" provide the referenced types,
// constants, macros, functions, and operations below.

extern "C" {
    static mut ecryptfs_inode_info_cache: *mut kmem_cache;
}

unsafe fn ecryptfs_alloc_inode(sb: *mut super_block) -> *mut inode {
    let mut inode_info: *mut ecryptfs_inode_info;
    let mut inode: *mut inode = core::ptr::null_mut();

    inode_info = alloc_inode_sb(sb, ecryptfs_inode_info_cache, GFP_KERNEL);
    if inode_info.is_null() {
        return inode;
    }
    ecryptfs_init_crypt_stat(&mut (*inode_info).crypt_stat);
    mutex_init(&mut (*inode_info).lower_file_mutex);
    atomic_set(&mut (*inode_info).lower_file_count, 0);
    (*inode_info).lower_file = core::ptr::null_mut();
    inode = &mut (*inode_info).vfs_inode;
    inode
}

unsafe fn ecryptfs_free_inode(inode: *mut inode) {
    let inode_info: *mut ecryptfs_inode_info;
    inode_info = ecryptfs_inode_to_private(inode);

    kmem_cache_free(ecryptfs_inode_info_cache, inode_info);
}

unsafe fn ecryptfs_destroy_inode(inode: *mut inode) {
    let inode_info: *mut ecryptfs_inode_info;

    inode_info = ecryptfs_inode_to_private(inode);
    BUG_ON(!(*inode_info).lower_file.is_null());
    ecryptfs_destroy_crypt_stat(&mut (*inode_info).crypt_stat);
}

unsafe fn ecryptfs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> i32 {
    let lower_dentry: *mut dentry = ecryptfs_dentry_to_lower(dentry);
    let rc: i32;

    if (*(*lower_dentry).d_sb).s_op.is_null()
        || (*(*(*lower_dentry).d_sb).s_op).statfs.is_none()
    {
        return -ENOSYS;
    }

    rc = ((*(*(*lower_dentry).d_sb).s_op).statfs.unwrap())(lower_dentry, buf);
    if rc != 0 {
        return rc;
    }

    (*buf).f_type = ECRYPTFS_SUPER_MAGIC;
    ecryptfs_set_f_namelen(
        &mut (*buf).f_namelen,
        (*buf).f_namelen,
        &mut (*ecryptfs_superblock_to_private((*dentry).d_sb)).mount_crypt_stat,
    )
}

unsafe fn ecryptfs_evict_inode(inode: *mut inode) {
    truncate_inode_pages_final(&mut (*inode).i_data);
    clear_inode(inode);
    iput(ecryptfs_inode_to_lower(inode));
}

unsafe fn ecryptfs_show_options(m: *mut seq_file, root: *mut dentry) -> i32 {
    let sb: *mut super_block = (*root).d_sb;
    let mount_crypt_stat: *mut ecryptfs_mount_crypt_stat =
        &mut (*ecryptfs_superblock_to_private(sb)).mount_crypt_stat;
    let mut walker: *mut ecryptfs_global_auth_tok;

    mutex_lock(&mut (*mount_crypt_stat).global_auth_tok_list_mutex);
    // list_for_each_entry(walker, &mount_crypt_stat->global_auth_tok_list,
    //                     mount_crypt_stat_list)
    walker = list_first_entry(
        &mut (*mount_crypt_stat).global_auth_tok_list,
        ecryptfs_global_auth_tok,
        mount_crypt_stat_list,
    );
    while !walker.is_null() {
        if (*walker).flags & ECRYPTFS_AUTH_TOK_FNEK != 0 {
            seq_printf(m, ",ecryptfs_fnek_sig=%s", (*walker).sig);
        } else {
            seq_printf(m, ",ecryptfs_sig=%s", (*walker).sig);
        }
        walker = list_next_entry(walker, mount_crypt_stat_list);
    }
    mutex_unlock(&mut (*mount_crypt_stat).global_auth_tok_list_mutex);

    seq_printf(
        m,
        ",ecryptfs_cipher=%s",
        (*mount_crypt_stat).global_default_cipher_name,
    );

    if (*mount_crypt_stat).global_default_cipher_key_size != 0 {
        seq_printf(
            m,
            ",ecryptfs_key_bytes=%zd",
            (*mount_crypt_stat).global_default_cipher_key_size,
        );
    }
    if (*mount_crypt_stat).flags & ECRYPTFS_GLOBAL_ENCRYPT_FILENAMES != 0 {
        seq_printf(
            m,
            ",ecryptfs_fn_cipher=%s",
            (*mount_crypt_stat).global_default_fn_cipher_name,
        );
        if (*mount_crypt_stat).global_default_fn_cipher_key_bytes != 0 {
            seq_printf(
                m,
                ",ecryptfs_fn_key_bytes=%zd",
                (*mount_crypt_stat).global_default_fn_cipher_key_bytes,
            );
        }
    }
    if (*mount_crypt_stat).flags & ECRYPTFS_PLAINTEXT_PASSTHROUGH_ENABLED != 0 {
        seq_printf(m, ",ecryptfs_passthrough");
    }
    if (*mount_crypt_stat).flags & ECRYPTFS_XATTR_METADATA_ENABLED != 0 {
        seq_printf(m, ",ecryptfs_xattr_metadata");
    }
    if (*mount_crypt_stat).flags & ECRYPTFS_ENCRYPTED_VIEW_ENABLED != 0 {
        seq_printf(m, ",ecryptfs_encrypted_view");
    }
    if (*mount_crypt_stat).flags & ECRYPTFS_UNLINK_SIGS != 0 {
        seq_printf(m, ",ecryptfs_unlink_sigs");
    }
    if (*mount_crypt_stat).flags & ECRYPTFS_GLOBAL_MOUNT_AUTH_TOK_ONLY != 0 {
        seq_printf(m, ",ecryptfs_mount_auth_tok_only");
    }

    0
}

#[repr(C)]
pub struct super_operations {
    pub alloc_inode: Option<unsafe fn(*mut super_block) -> *mut inode>,
    pub destroy_inode: Option<unsafe fn(*mut inode)>,
    pub free_inode: Option<unsafe fn(*mut inode)>,
    pub statfs: Option<unsafe fn(*mut dentry, *mut kstatfs) -> i32>,
    pub evict_inode: Option<unsafe fn(*mut inode)>,
    pub show_options: Option<unsafe fn(*mut seq_file, *mut dentry) -> i32>,
}

pub static ecryptfs_sops: super_operations = super_operations {
    alloc_inode: Some(ecryptfs_alloc_inode),
    destroy_inode: Some(ecryptfs_destroy_inode),
    free_inode: Some(ecryptfs_free_inode),
    statfs: Some(ecryptfs_statfs),
    evict_inode: Some(ecryptfs_evict_inode),
    show_options: Some(ecryptfs_show_options),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
