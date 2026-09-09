// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corporation, 2021
 *
 * Author: Mike Rapoport <rppt@linux.ibm.com>
 */

// Kernel dependencies supplied by the surrounding translation unit.

const SECRETMEM_MODE_MASK: u32 = 0x0;
const SECRETMEM_FLAGS_MASK: u32 = SECRETMEM_MODE_MASK;

static mut secretmem_enable: bool = true;
static mut secretmem_users: atomic_t = atomic_t { counter: 0 };

pub unsafe fn secretmem_active() -> bool {
    atomic_read(&raw const secretmem_users) != 0
}

unsafe fn secretmem_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let mapping = (*(*vmf).vma).vm_file.as_ref().unwrap().f_mapping;
    let inode = file_inode((*vmf).vma.as_ref().unwrap().vm_file);
    let offset = (*vmf).pgoff;
    let gfp = (*vmf).gfp_mask;
    let mut addr: unsigned_long;
    let mut folio: *mut folio;
    let ret: vm_fault_t;
    let err: i32;

    if (((*vmf).pgoff as loff_t) << PAGE_SHIFT) >= i_size_read(inode) {
        return vmf_error(-EINVAL);
    }

    filemap_invalidate_lock_shared(mapping);

    'retry: loop {
        folio = filemap_lock_folio(mapping, offset);
        if !IS_ERR(folio) {
            break;
        }

        folio = folio_alloc(gfp | __GFP_ZERO, 0);
        if folio.is_null() {
            ret = VM_FAULT_OOM;
            break;
        }

        err = set_direct_map_invalid_noflush(folio_page(folio, 0));
        if err != 0 {
            folio_put(folio);
            ret = vmf_error(err);
            break;
        }

        __folio_mark_uptodate(folio);
        err = filemap_add_folio(mapping, folio, offset, gfp);
        if err != 0 {
            // If a split of a large page was required, it already happened
            // when we marked the page invalid, guaranteeing that this call
            // will not fail.
            set_direct_map_default_noflush(folio_page(folio, 0));
            folio_put(folio);
            if err == -EEXIST {
                continue 'retry;
            }
            ret = vmf_error(err);
            break;
        }

        addr = folio_address(folio) as unsigned_long;
        flush_tlb_kernel_range(addr, addr + PAGE_SIZE);
        break;
    }

    if !IS_ERR(folio) && !folio.is_null() {
        (*vmf).page = folio_file_page(folio, (*vmf).pgoff);
        ret = VM_FAULT_LOCKED;
    }

    filemap_invalidate_unlock_shared(mapping);
    ret
}

static const secretmem_vm_ops: vm_operations_struct = vm_operations_struct {
    fault: Some(secretmem_fault),
};

unsafe fn secretmem_release(_inode: *mut inode, _file: *mut file) -> i32 {
    atomic_dec(&raw mut secretmem_users);
    0
}

unsafe fn secretmem_mmap_prepare(desc: *mut vm_area_desc) -> i32 {
    let len = vma_desc_size(desc);

    if !vma_desc_test_any(desc, VMA_SHARED_BIT, VMA_MAYSHARE_BIT) {
        return -EINVAL;
    }

    vma_desc_set_flags(desc, VMA_LOCKED_BIT, VMA_DONTDUMP_BIT);
    if !mlock_future_ok((*desc).mm, true, len) {
        return -EAGAIN;
    }
    (*desc).vm_ops = &raw const secretmem_vm_ops;
    0
}

pub unsafe fn vma_is_secretmem(vma: *mut vm_area_struct) -> bool {
    (*vma).vm_ops == &raw const secretmem_vm_ops
}

static const secretmem_fops: file_operations = file_operations {
    release: Some(secretmem_release),
    mmap_prepare: Some(secretmem_mmap_prepare),
};

unsafe fn secretmem_migrate_folio(
    _mapping: *mut address_space,
    _dst: *mut folio,
    _src: *mut folio,
    _mode: migrate_mode,
) -> i32 {
    -EBUSY
}

unsafe fn secretmem_free_folio(folio: *mut folio) {
    set_direct_map_default_noflush(folio_page(folio, 0));
    folio_zero_segment(folio, 0, folio_size(folio));
}

pub const secretmem_aops: address_space_operations = address_space_operations {
    dirty_folio: Some(noop_dirty_folio),
    free_folio: Some(secretmem_free_folio),
    migrate_folio: Some(secretmem_migrate_folio),
};

unsafe fn secretmem_setattr(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    iattr: *mut iattr,
) -> i32 {
    let inode = d_inode(dentry);
    let mapping = (*inode).i_mapping;
    let ia_valid = (*iattr).ia_valid;
    let ret: i32;

    filemap_invalidate_lock(mapping);
    if (ia_valid & ATTR_SIZE) != 0 && (*inode).i_size != 0 {
        ret = -EINVAL;
    } else {
        ret = simple_setattr(idmap, dentry, iattr);
    }
    filemap_invalidate_unlock(mapping);
    ret
}

static const secretmem_iops: inode_operations = inode_operations {
    setattr: Some(secretmem_setattr),
};

static mut secretmem_mnt: *mut vfsmount = core::ptr::null_mut();

unsafe fn secretmem_file_create(_flags: unsigned_long) -> *mut file {
    let anon_name = c"[secretmem]";
    let inode = anon_inode_make_secure_inode((*secretmem_mnt).mnt_sb, anon_name.as_ptr(), core::ptr::null_mut());
    if IS_ERR(inode) {
        return ERR_CAST(inode);
    }

    let file = alloc_file_pseudo(inode, secretmem_mnt, c"secretmem".as_ptr(), O_RDWR | O_LARGEFILE, &raw const secretmem_fops);
    if IS_ERR(file) {
        iput(inode);
        return file;
    }

    mapping_set_gfp_mask((*inode).i_mapping, GFP_USER);
    mapping_set_unevictable((*inode).i_mapping);
    (*inode).i_op = &raw const secretmem_iops;
    (*inode).i_mapping.as_mut().unwrap().a_ops = &raw const secretmem_aops;
    (*inode).i_mode |= S_IFREG;
    (*inode).i_size = 0;
    atomic_inc(&raw mut secretmem_users);
    file
}

pub unsafe fn memfd_secret(flags: u32) -> i64 {
    // Make sure local flags do not conflict with global fcntl.h.
    BUILD_BUG_ON(SECRETMEM_FLAGS_MASK & O_CLOEXEC);
    if !secretmem_enable || !can_set_direct_map() {
        return -ENOSYS as i64;
    }
    if flags & !(SECRETMEM_FLAGS_MASK | O_CLOEXEC) != 0 {
        return -EINVAL as i64;
    }
    if atomic_read(&raw const secretmem_users) < 0 {
        return -ENFILE as i64;
    }
    FD_ADD(flags & O_CLOEXEC, secretmem_file_create(flags))
}

unsafe fn secretmem_init_fs_context(fc: *mut fs_context) -> i32 {
    let ctx = init_pseudo(fc, SECRETMEM_MAGIC);
    if ctx.is_null() {
        return -ENOMEM;
    }
    0
}

static mut secretmem_fs: file_system_type = file_system_type {
    name: c"secretmem".as_ptr(),
    init_fs_context: Some(secretmem_init_fs_context),
    kill_sb: Some(kill_anon_super),
};

unsafe fn secretmem_init() -> i32 {
    if !secretmem_enable || !can_set_direct_map() {
        return 0;
    }
    secretmem_mnt = kern_mount(&raw mut secretmem_fs);
    if IS_ERR(secretmem_mnt) {
        return PTR_ERR(secretmem_mnt);
    }
    0
}

// fs_initcall(secretmem_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
