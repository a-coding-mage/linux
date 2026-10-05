// SPDX-License-Identifier: GPL-2.0-only
// Original !CONFIG_SHMEM ramfs alternatives, not placeholders.
use b::generic_file_vm_ops as rust_shmem_data_shmem_anon_vm_ops;
use b::ramfs_file_operations as rust_shmem_data_shmem_file_operations;
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RUST_SHMEM_INIT_COLD, cold)]
pub unsafe extern "C" fn shmem_init() {
    rust_shmem_bug(register_filesystem(addr_of_mut!(rust_shmem_data_shmem_fs_type)) != 0);
    rust_shmem_data_shm_mnt = kern_mount(addr_of_mut!(rust_shmem_data_shmem_fs_type));
    rust_shmem_bug(IS_ERR(rust_shmem_data_shm_mnt.cast()));
}
#[no_mangle]
pub unsafe extern "C" fn shmem_unuse(_ty: c_uint) -> c_int {
    0
}
#[no_mangle]
pub unsafe extern "C" fn shmem_lock(
    _file: *mut file,
    _lock: c_int,
    _ucounts: *mut ucounts,
) -> c_int {
    0
}
#[no_mangle]
pub unsafe extern "C" fn shmem_unlock_mapping(_mapping: *mut address_space) {}
#[cfg(CONFIG_MMU)]
#[no_mangle]
pub unsafe extern "C" fn shmem_get_unmapped_area(
    file: *mut file,
    addr: c_ulong,
    len: c_ulong,
    index: c_ulong,
    flags: c_ulong,
) -> c_ulong {
    mm_get_unmapped_area(file, addr, len, index, flags)
}
#[no_mangle]
pub unsafe extern "C" fn shmem_truncate_range(inode: *mut inode, start: loff_t, end: uoff_t) {
    truncate_inode_pages_range((*inode).i_mapping, start, end);
}
unsafe fn shmem_acct_size(_flags: c_ulong, _size: loff_t) -> c_int {
    0
}
unsafe fn shmem_unacct_size(_flags: c_ulong, _size: loff_t) {}
unsafe fn shmem_get_inode(
    _idmap: *mut mnt_idmap,
    sb: *mut super_block,
    dir: *mut inode,
    mode: umode_t,
    dev: dev_t,
    _flags: vma_flags_t,
) -> *mut inode {
    let inode = ramfs_get_inode(sb, dir, mode, dev);
    if inode.is_null() {
        err_ptr(-(RUST_SHMEM_ENOSPC as c_int))
    } else {
        inode
    }
}
