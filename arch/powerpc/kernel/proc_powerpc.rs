// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2001 Mike Corrigan & Dave Engebretsen IBM Corporation
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[cfg(CONFIG_PPC64_PROC_SYSTEMCFG)]
unsafe fn page_map_seek(file: *mut file, off: loff_t, whence: c_int) -> loff_t {
    fixed_size_llseek(file, off, whence, PAGE_SIZE as loff_t)
}

#[cfg(CONFIG_PPC64_PROC_SYSTEMCFG)]
unsafe fn page_map_read(
    file: *mut file,
    buf: *mut c_char,
    nbytes: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    simple_read_from_buffer(
        buf,
        nbytes,
        ppos,
        pde_data(file_inode(file)),
        PAGE_SIZE as size_t,
    )
}

#[cfg(CONFIG_PPC64_PROC_SYSTEMCFG)]
unsafe fn page_map_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    if ((*vma).vm_end - (*vma).vm_start) > PAGE_SIZE {
        return -EINVAL;
    }

    remap_pfn_range(
        vma,
        (*vma).vm_start,
        __pa(pde_data(file_inode(file))) >> PAGE_SHIFT,
        PAGE_SIZE as c_ulong,
        (*vma).vm_page_prot,
    )
}

#[cfg(CONFIG_PPC64_PROC_SYSTEMCFG)]
static page_map_proc_ops: proc_ops = proc_ops {
    proc_lseek: Some(page_map_seek),
    proc_read: Some(page_map_read),
    proc_mmap: Some(page_map_mmap),
};

#[cfg(CONFIG_PPC64_PROC_SYSTEMCFG)]
#[repr(C)]
union SystemcfgDataStore {
    data: systemcfg,
    page: [u8; PAGE_SIZE],
}

#[cfg(CONFIG_PPC64_PROC_SYSTEMCFG)]
#[no_mangle]
static mut systemcfg_data_store: SystemcfgDataStore = SystemcfgDataStore {
    page: [0; PAGE_SIZE],
};

#[cfg(CONFIG_PPC64_PROC_SYSTEMCFG)]
#[no_mangle]
static mut systemcfg: *mut systemcfg = unsafe { &mut systemcfg_data_store.data };

#[cfg(CONFIG_PPC64_PROC_SYSTEMCFG)]
unsafe fn proc_ppc64_init() -> c_int {
    let pde: *mut proc_dir_entry;

    strscpy((*systemcfg).eye_catcher.as_mut_ptr(), b"SYSTEMCFG:PPC64\0".as_ptr() as *const c_char);
    (*systemcfg).version.major = SYSTEMCFG_MAJOR;
    (*systemcfg).version.minor = SYSTEMCFG_MINOR;
    (*systemcfg).processor = mfspr(SPRN_PVR);
    /*
     * Fake the old platform number for pSeries and add
     * in LPAR bit if necessary
     */
    (*systemcfg).platform = 0x100;
    if firmware_has_feature(FW_FEATURE_LPAR) {
        (*systemcfg).platform |= 1;
    }
    (*systemcfg).physicalMemorySize = memblock_phys_mem_size();
    (*systemcfg).dcache_size = ppc64_caches.l1d.size;
    (*systemcfg).dcache_line_size = ppc64_caches.l1d.line_size;
    (*systemcfg).icache_size = ppc64_caches.l1i.size;
    (*systemcfg).icache_line_size = ppc64_caches.l1i.line_size;

    pde = proc_create_data(
        b"powerpc/systemcfg\0".as_ptr() as *const c_char,
        S_IFREG | 0o444,
        core::ptr::null_mut(),
        &page_map_proc_ops,
        systemcfg as *mut c_void,
    );
    if pde.is_null() {
        return 1;
    }
    proc_set_size(pde, PAGE_SIZE as loff_t);

    0
}

// __initcall(proc_ppc64_init);

/*
 * Create the ppc64 and ppc64/rtas directories early. This allows us to
 * assume that they have been previously created in drivers.
 */
unsafe fn proc_ppc64_create() -> c_int {
    let root: *mut proc_dir_entry;

    root = proc_mkdir(b"powerpc\0".as_ptr() as *const c_char, core::ptr::null_mut());
    if root.is_null() {
        return 1;
    }

    #[cfg(CONFIG_PPC64)]
    if proc_symlink(
        b"ppc64\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
        b"powerpc\0".as_ptr() as *const c_char,
    ).is_null() {
        pr_err!("Failed to create link /proc/ppc64 -> /proc/powerpc\n");
    }

    if of_find_node_by_path(b"/rtas\0".as_ptr() as *const c_char).is_null() {
        return 0;
    }

    if proc_mkdir(b"rtas\0".as_ptr() as *const c_char, root).is_null() {
        return 1;
    }

    if proc_symlink(
        b"rtas\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
        b"powerpc/rtas\0".as_ptr() as *const c_char,
    ).is_null() {
        return 1;
    }

    0
}

// core_initcall(proc_ppc64_create);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
