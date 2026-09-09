// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation unit.

static mut vdso_initdata: [u8; VDSO_NR_PAGES * PAGE_SIZE] = [0; VDSO_NR_PAGES * PAGE_SIZE];

#[cfg(CONFIG_GENERIC_GETTIMEOFDAY)]
static mut vdso_k_time_data: *mut vdso_time_data = unsafe {
    vdso_initdata.as_mut_ptr().add(VDSO_TIME_PAGE_OFFSET * PAGE_SIZE)
        as *mut vdso_time_data
};

#[cfg(CONFIG_GENERIC_GETTIMEOFDAY)]
const _: () = assert!(core::mem::size_of::<vdso_time_data>() <= PAGE_SIZE);

#[cfg(CONFIG_VDSO_GETRANDOM)]
static mut vdso_k_rng_data: *mut vdso_rng_data = unsafe {
    vdso_initdata.as_mut_ptr().add(VDSO_RNG_PAGE_OFFSET * PAGE_SIZE)
        as *mut vdso_rng_data
};

#[cfg(CONFIG_VDSO_GETRANDOM)]
const _: () = assert!(core::mem::size_of::<vdso_rng_data>() <= PAGE_SIZE);

#[cfg(CONFIG_ARCH_HAS_VDSO_ARCH_DATA)]
static mut vdso_k_arch_data: *mut vdso_arch_data = unsafe {
    vdso_initdata.as_mut_ptr().add(VDSO_ARCH_PAGES_START * PAGE_SIZE)
        as *mut vdso_arch_data
};

static mut vdso_data_pages: *mut page = core::ptr::null_mut();

unsafe fn vdso_setup_data_pages() {
    let order: c_uint = get_order(VDSO_NR_PAGES * PAGE_SIZE);

    /*
     * Allocate the data pages dynamically. SPARC does not support mapping
     * static pages to be mapped into userspace.
     * It is also a requirement for mlockall() support.
     *
     * Do not use folios. In time namespaces the pages are mapped in a different order
     * to userspace, which is not handled by the folio optimizations in finish_fault().
     */
    vdso_data_pages = alloc_pages(GFP_KERNEL, order);
    if vdso_data_pages.is_null() {
        panic("Unable to allocate VDSO storage pages");
    }

    /* The pages are mapped one-by-one into userspace and each one needs to be refcounted. */
    split_page(vdso_data_pages, order);

    /* Move the data already written by other subsystems to the new pages */
    memcpy(page_address(vdso_data_pages), vdso_initdata.as_ptr(), VDSO_NR_PAGES * PAGE_SIZE);

    if IS_ENABLED(CONFIG_GENERIC_GETTIMEOFDAY) {
        vdso_k_time_data = page_address(vdso_data_pages.add(VDSO_TIME_PAGE_OFFSET)) as *mut vdso_time_data;
    }

    if IS_ENABLED(CONFIG_VDSO_GETRANDOM) {
        vdso_k_rng_data = page_address(vdso_data_pages.add(VDSO_RNG_PAGE_OFFSET)) as *mut vdso_rng_data;
    }

    if IS_ENABLED(CONFIG_ARCH_HAS_VDSO_ARCH_DATA) {
        vdso_k_arch_data = page_address(vdso_data_pages.add(VDSO_ARCH_PAGES_START)) as *mut vdso_arch_data;
    }
}

unsafe fn vvar_fault(
    _sm: *const vm_special_mapping,
    vma: *mut vm_area_struct,
    vmf: *mut vm_fault,
) -> vm_fault_t {
    let mut page: *mut page;
    let timens_page: *mut page;

    if unlikely((*vmf).flags & FAULT_FLAG_REMOTE != 0) {
        return VM_FAULT_SIGBUS;
    }

    page = vdso_data_pages.add((*vmf).pgoff as usize);
    timens_page = find_timens_vvar_page(vma);

    match (*vmf).pgoff {
        VDSO_TIME_PAGE_OFFSET => {
            if IS_ENABLED(CONFIG_GENERIC_GETTIMEOFDAY) && !timens_page.is_null() {
                /*
                 * Fault in VVAR page too, since it will be accessed
                 * to get clock data anyway.
                 */
                let addr: c_ulong = (*vmf).address + VDSO_TIMENS_PAGE_OFFSET * PAGE_SIZE;
                let err: vm_fault_t = vmf_insert_page(vma, addr, page);
                if unlikely(err & VM_FAULT_ERROR != 0) {
                    return err;
                }
                page = timens_page;
            }
        }
        VDSO_TIMENS_PAGE_OFFSET => {
            /*
             * If a task belongs to a time namespace then a namespace
             * specific VVAR is mapped with the VVAR_DATA_PAGE_OFFSET and
             * the real VVAR page is mapped with the VVAR_TIMENS_PAGE_OFFSET
             * offset.
             * See also the comment near timens_setup_vdso_data().
             */
            if IS_ENABLED(CONFIG_TIME_NS) && !timens_page.is_null() {
                page = vdso_data_pages.add(VDSO_TIME_PAGE_OFFSET);
            }
        }
        VDSO_RNG_PAGE_OFFSET => {}
        VDSO_ARCH_PAGES_START..=VDSO_ARCH_PAGES_END => {}
        _ => return VM_FAULT_SIGBUS,
    }

    get_page(page);
    (*vmf).page = page;
    0
}

const vdso_vvar_mapping: vm_special_mapping = vm_special_mapping {
    name: c"[vvar]".as_ptr(),
    fault: vvar_fault,
};

unsafe fn vdso_install_vvar_mapping(mm: *mut mm_struct, addr: c_ulong) -> *mut vm_area_struct {
    _install_special_mapping(
        mm,
        addr,
        VDSO_NR_PAGES * PAGE_SIZE,
        VM_READ | VM_MAYREAD | VM_DONTDUMP | VM_MIXEDMAP | VM_SEALED_SYSMAP,
        &vdso_vvar_mapping,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
