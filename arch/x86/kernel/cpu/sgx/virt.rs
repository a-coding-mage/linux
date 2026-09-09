// SPDX-License-Identifier: GPL-2.0
/*
 * Device driver to expose SGX enclave memory to KVM guests.
 *
 * Copyright(c) 2021 Intel Corporation.
 */

// Dependencies are supplied by the surrounding kernel Rust environment.

#[repr(C)]
struct sgx_vepc {
    page_array: xarray,
    lock: mutex,
}

/* Temporary SECS pages that cannot be EREMOVE'd due to having children in
 * other virtual EPC instances, and the lock to protect it. */
static mut zombie_secs_pages_lock: mutex = mutex::new();
static mut zombie_secs_pages: list_head = list_head::new();

unsafe fn __sgx_vepc_fault(vepc: *mut sgx_vepc, vma: *mut vm_area_struct,
                            addr: c_ulong) -> c_int {
    let mut epc_page: *mut sgx_epc_page;
    let index: c_ulong;
    let pfn: c_ulong;
    let mut ret: c_int;

    WARN_ON(!mutex_is_locked(&mut (*vepc).lock));

    /* Calculate index of EPC page in virtual EPC's page_array */
    index = linear_page_index(vma, addr);
    epc_page = xa_load(&mut (*vepc).page_array, index);
    if !epc_page.is_null() { return 0; }

    epc_page = sgx_alloc_epc_page(vepc, false);
    if IS_ERR(epc_page) { return PTR_ERR(epc_page); }

    ret = xa_err(xa_store(&mut (*vepc).page_array, index, epc_page, GFP_KERNEL));
    if ret != 0 { goto err_free; }

    pfn = PFN_DOWN(sgx_get_epc_phys_addr(epc_page));
    ret = vmf_insert_pfn(vma, addr, pfn);
    if ret != VM_FAULT_NOPAGE {
        ret = -EFAULT;
        goto err_delete;
    }
    return 0;

err_delete:
    xa_erase(&mut (*vepc).page_array, index);
err_free:
    sgx_free_epc_page(epc_page);
    ret
}

unsafe extern "C" fn sgx_vepc_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let vma = (*vmf).vma;
    let vepc = (*vma).vm_private_data as *mut sgx_vepc;
    let ret: c_int;

    mutex_lock(&mut (*vepc).lock);
    ret = __sgx_vepc_fault(vepc, vma, (*vmf).address);
    mutex_unlock(&mut (*vepc).lock);
    if ret == 0 { return VM_FAULT_NOPAGE; }
    if ret == -EBUSY && ((*vmf).flags & FAULT_FLAG_ALLOW_RETRY) != 0 {
        mmap_read_unlock((*vma).vm_mm);
        return VM_FAULT_RETRY;
    }
    VM_FAULT_SIGBUS
}

static sgx_vepc_vm_ops: vm_operations_struct = vm_operations_struct {
    fault: Some(sgx_vepc_fault),
};

unsafe extern "C" fn sgx_vepc_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    let vepc = (*file).private_data as *mut sgx_vepc;
    if ((*vma).vm_flags & VM_SHARED) == 0 { return -EINVAL; }
    (*vma).vm_ops = &sgx_vepc_vm_ops;
    vm_flags_set(vma, VM_PFNMAP | VM_IO | VM_DONTDUMP | VM_DONTCOPY);
    (*vma).vm_private_data = vepc as *mut c_void;
    0
}

unsafe fn sgx_vepc_remove_page(epc_page: *mut sgx_epc_page) -> c_int {
    __eremove(sgx_get_epc_virt_addr(epc_page))
}

unsafe fn sgx_vepc_free_page(epc_page: *mut sgx_epc_page) -> c_int {
    let ret = sgx_vepc_remove_page(epc_page);
    if ret != 0 {
        WARN_ONCE(ret != SGX_CHILD_PRESENT, EREMOVE_ERROR_MESSAGE, ret, ret);
        return ret;
    }
    sgx_free_epc_page(epc_page);
    0
}

unsafe fn sgx_vepc_remove_all(vepc: *mut sgx_vepc) -> c_long {
    let mut entry: *mut sgx_epc_page;
    let mut index: c_ulong;
    let mut failures: c_long = 0;
    xa_for_each!(&mut (*vepc).page_array, index, entry, {
        let ret = sgx_vepc_remove_page(entry);
        if ret != 0 {
            if ret == SGX_CHILD_PRESENT { failures += 1; }
            else {
                WARN_ON_ONCE(encls_faulted(ret) && ENCLS_TRAPNR(ret) != X86_TRAP_GP);
                return -EBUSY;
            }
        }
        cond_resched();
    });
    failures
}

unsafe extern "C" fn sgx_vepc_release(_inode: *mut inode, file: *mut file) -> c_int {
    let vepc = (*file).private_data as *mut sgx_vepc;
    let mut entry: *mut sgx_epc_page;
    let mut index: c_ulong;
    let mut secs_pages = LIST_HEAD_INIT();

    xa_for_each!(&mut (*vepc).page_array, index, entry, {
        if sgx_vepc_free_page(entry) != 0 { continue; }
        xa_erase(&mut (*vepc).page_array, index);
        cond_resched();
    });
    xa_for_each!(&mut (*vepc).page_array, index, entry, {
        if sgx_vepc_free_page(entry) != 0 { list_add_tail(&mut (*entry).list, &mut secs_pages); }
        xa_erase(&mut (*vepc).page_array, index);
        cond_resched();
    });
    mutex_lock(&mut zombie_secs_pages_lock);
    let mut epc_page: *mut sgx_epc_page;
    let mut tmp: *mut sgx_epc_page;
    list_for_each_entry_safe!(epc_page, tmp, &mut zombie_secs_pages, list, {
        list_del(&mut (*epc_page).list);
        if sgx_vepc_free_page(epc_page) != 0 { list_add_tail(&mut (*epc_page).list, &mut secs_pages); }
        cond_resched();
    });
    if !list_empty(&mut secs_pages) { list_splice_tail(&mut secs_pages, &mut zombie_secs_pages); }
    mutex_unlock(&mut zombie_secs_pages_lock);
    xa_destroy(&mut (*vepc).page_array);
    kfree(vepc as *mut c_void);
    sgx_dec_usage_count();
    0
}

unsafe fn __sgx_vepc_open(_inode: *mut inode, file: *mut file) -> c_int {
    let vepc = kzalloc_obj::<sgx_vepc>();
    if vepc.is_null() { return -ENOMEM; }
    mutex_init(&mut (*vepc).lock);
    xa_init(&mut (*vepc).page_array);
    (*file).private_data = vepc as *mut c_void;
    0
}

unsafe extern "C" fn sgx_vepc_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ret = sgx_inc_usage_count();
    if ret != 0 { return ret; }
    ret = __sgx_vepc_open(inode, file);
    if ret != 0 { sgx_dec_usage_count(); }
    ret
}

unsafe extern "C" fn sgx_vepc_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let vepc = (*file).private_data as *mut sgx_vepc;
    match cmd {
        SGX_IOC_VEPC_REMOVE_ALL => if arg != 0 { -EINVAL } else { sgx_vepc_remove_all(vepc) },
        _ => -ENOTTY,
    }
}

static sgx_vepc_fops: file_operations = file_operations {
    owner: THIS_MODULE, open: Some(sgx_vepc_open), unlocked_ioctl: Some(sgx_vepc_ioctl),
    compat_ioctl: Some(sgx_vepc_ioctl), release: Some(sgx_vepc_release), mmap: Some(sgx_vepc_mmap),
};

static mut sgx_vepc_dev: miscdevice = miscdevice {
    minor: MISC_DYNAMIC_MINOR, name: "sgx_vepc", nodename: "sgx_vepc", fops: &sgx_vepc_fops,
};

unsafe extern "C" fn sgx_vepc_init() -> c_int {
    if !cpu_feature_enabled(X86_FEATURE_VMX) { return -ENODEV; }
    INIT_LIST_HEAD(&mut zombie_secs_pages);
    mutex_init(&mut zombie_secs_pages_lock);
    misc_register(&mut sgx_vepc_dev)
}

unsafe extern "C" fn sgx_virt_ecreate(pageinfo: *mut sgx_pageinfo, secs: *mut c_void,
                                       trapnr: *mut c_int) -> c_int {
    if WARN_ON_ONCE(!access_ok(secs, PAGE_SIZE)) { return -EINVAL; }
    __uaccess_begin();
    let ret = __ecreate(pageinfo, secs);
    __uaccess_end();
    if encls_faulted(ret) { *trapnr = ENCLS_TRAPNR(ret); return -EFAULT; }
    WARN_ON_ONCE(ret != 0);
    0
}

unsafe fn __sgx_virt_einit(sigstruct: *mut c_void, token: *mut c_void, secs: *mut c_void) -> c_int {
    const SGX_EINITTOKEN_SIZE: usize = 304;
    if WARN_ON_ONCE(!access_ok(sigstruct, size_of::<sgx_sigstruct>()) ||
                    !access_ok(token, SGX_EINITTOKEN_SIZE) || !access_ok(secs, PAGE_SIZE)) { return -EINVAL; }
    __uaccess_begin();
    let ret = __einit(sigstruct, token, secs);
    __uaccess_end();
    ret
}

unsafe extern "C" fn sgx_virt_einit(sigstruct: *mut c_void, token: *mut c_void,
                                     secs: *mut c_void, lepubkeyhash: *mut u64,
                                     trapnr: *mut c_int) -> c_int {
    let ret;
    if !cpu_feature_enabled(X86_FEATURE_SGX_LC) { ret = __sgx_virt_einit(sigstruct, token, secs); }
    else { preempt_disable(); sgx_update_lepubkeyhash(lepubkeyhash); ret = __sgx_virt_einit(sigstruct, token, secs); preempt_enable(); }
    if ret == -EINVAL { return ret; }
    if encls_faulted(ret) { *trapnr = ENCLS_TRAPNR(ret); return -EFAULT; }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
