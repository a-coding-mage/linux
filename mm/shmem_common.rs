// SPDX-License-Identifier: GPL-2.0-only
// Common SHMEM/tiny-ramfs API bodies, C:5807-5998.
unsafe fn __shmem_file_setup(
    mnt: *mut vfsmount,
    name: *const c_char,
    size: loff_t,
    flags: vma_flags_t,
    iflags: c_uint,
) -> *mut file {
    let shmem_flags = if vma_flags_test(&flags, VMA_NORESERVE_BIT) {
        RUST_SHMEM_SHMEM_F_NORESERVE as c_ulong
    } else {
        0
    };
    if IS_ERR(mnt.cast()) {
        return mnt.cast();
    }
    if size < 0 || size > RUST_SHMEM_MAX_LFS_FILESIZE as loff_t || is_idmapped_mnt(mnt) {
        return err_ptr(-(RUST_SHMEM_EINVAL as c_int));
    }
    if shmem_acct_size(shmem_flags, size) != 0 {
        return err_ptr(-(RUST_SHMEM_ENOMEM as c_int));
    }
    let inode = shmem_get_inode(
        addr_of_mut!(nop_mnt_idmap),
        (*mnt).mnt_sb,
        null_mut(),
        (RUST_SHMEM_S_IFREG | RUST_SHMEM_S_IRWXUGO) as umode_t,
        0,
        flags,
    );
    if IS_ERR(inode.cast()) {
        shmem_unacct_size(shmem_flags, size);
        return inode.cast();
    }
    (*inode).i_flags |= iflags;
    (*inode).i_size = size;
    clear_nlink(inode);
    let mut res = err_ptr(ramfs_nommu_expand_for_mapping(inode, size));
    if !IS_ERR(res) {
        res = alloc_file_pseudo(
            inode,
            mnt,
            name,
            RUST_SHMEM_O_RDWR as c_int,
            addr_of!(rust_shmem_data_shmem_file_operations),
        )
        .cast();
    }
    if IS_ERR(res) {
        iput(inode);
    }
    res.cast()
}
#[no_mangle]
pub unsafe extern "C" fn shmem_kernel_file_setup(
    name: *const c_char,
    size: loff_t,
    flags: vma_flags_t,
) -> *mut file {
    __shmem_file_setup(
        rust_shmem_data_shm_mnt,
        name,
        size,
        flags,
        RUST_SHMEM_S_PRIVATE as c_uint,
    )
}
#[no_mangle]
pub unsafe extern "C" fn shmem_file_setup(
    name: *const c_char,
    size: loff_t,
    flags: vma_flags_t,
) -> *mut file {
    __shmem_file_setup(rust_shmem_data_shm_mnt, name, size, flags, 0)
}
#[no_mangle]
pub unsafe extern "C" fn shmem_file_setup_with_mnt(
    mnt: *mut vfsmount,
    name: *const c_char,
    size: loff_t,
    flags: vma_flags_t,
) -> *mut file {
    __shmem_file_setup(mnt, name, size, flags, 0)
}
unsafe fn __shmem_zero_setup(start: c_ulong, end: c_ulong, flags: vma_flags_t) -> *mut file {
    shmem_kernel_file_setup(
        c"dev/zero".as_char_ptr(),
        end.wrapping_sub(start) as loff_t,
        flags,
    )
}
#[no_mangle]
pub unsafe extern "C" fn shmem_zero_setup(vma: *mut vm_area_struct) -> c_int {
    let f = __shmem_zero_setup(
        rust_shmem_vma_vm_start(vma),
        rust_shmem_vma_vm_end(vma),
        rust_shmem_vma_flags(vma),
    );
    if IS_ERR(f.cast()) {
        return ptr_err(f);
    }
    if !(*vma).vm_file.is_null() {
        fput((*vma).vm_file);
    }
    (*vma).vm_file = f;
    (*vma).vm_ops = addr_of!(rust_shmem_data_shmem_anon_vm_ops);
    0
}
#[no_mangle]
pub unsafe extern "C" fn shmem_zero_setup_desc(desc: *mut vm_area_desc) -> c_int {
    let f = __shmem_zero_setup((*desc).start, (*desc).end, (*desc).vma_flags);
    if IS_ERR(f.cast()) {
        return ptr_err(f);
    }
    (*desc).vm_file = f;
    (*desc).vm_ops = addr_of!(rust_shmem_data_shmem_anon_vm_ops);
    0
}
#[no_mangle]
pub unsafe extern "C" fn shmem_read_folio_gfp(
    mapping: *mut address_space,
    index: Pgoff,
    gfp: gfp_t,
) -> *mut folio {
    #[cfg(CONFIG_SHMEM)]
    {
        let inode = (*mapping).host;
        let mut f = null_mut();
        let err = shmem_get_folio_gfp(
            inode,
            index,
            i_size_read(inode),
            &mut f,
            SGP_CACHE,
            gfp,
            null_mut(),
            null_mut(),
        );
        if err != 0 {
            return err_ptr(err);
        }
        folio_unlock(f);
        f
    }
    #[cfg(not(CONFIG_SHMEM))]
    {
        mapping_read_folio_gfp(mapping, index, gfp)
    }
}
#[no_mangle]
pub unsafe extern "C" fn shmem_read_mapping_page_gfp(
    mapping: *mut address_space,
    index: Pgoff,
    gfp: gfp_t,
) -> *mut page {
    let f = shmem_read_folio_gfp(mapping, index, gfp);
    if IS_ERR(f.cast()) {
        return rust_shmem_folio_page_address(f);
    }
    let p = folio_file_page(f, index);
    if rust_shmem_PageHWPoison(p) {
        folio_put(f);
        return err_ptr(-(RUST_SHMEM_EIO as c_int));
    }
    p
}
