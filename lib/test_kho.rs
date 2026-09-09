// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Test module for KHO
 * Copyright (c) 2025 Microsoft Corporation.
 *
 * Authors:
 *   Saurabh Sengar <ssengar@microsoft.com>
 *   Mike Rapoport <rppt@kernel.org>
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Kernel dependencies supplied by the surrounding build.

const KHO_TEST_MAGIC: u32 = 0x4b484f21; // KHO!
const KHO_TEST_FDT: &str = "kho_test";
const KHO_TEST_COMPAT: &str = "kho-test-v1";

static mut max_mem: c_long = (PAGE_SIZE << MAX_PAGE_ORDER) * 2;

#[repr(C)]
struct kho_test_state {
    nr_folios: c_uint,
    folios: *mut *mut folio,
    folios_info: *mut phys_addr_t,
    folios_info_phys: kho_vmalloc,
    nr_folios_preserved: c_int,
    fdt: *mut folio,
    csum: __wsum,
}

static mut kho_test_state: kho_test_state = kho_test_state {
    nr_folios: 0,
    folios: core::ptr::null_mut(),
    folios_info: core::ptr::null_mut(),
    folios_info_phys: kho_vmalloc::default(),
    nr_folios_preserved: 0,
    fdt: core::ptr::null_mut(),
    csum: 0,
};

unsafe fn kho_test_unpreserve_data(state: *mut kho_test_state) {
    for i in 0..(*state).nr_folios_preserved {
        kho_unpreserve_folio(*(*state).folios.add(i as usize));
    }

    kho_unpreserve_vmalloc(&mut (*state).folios_info_phys);
    vfree((*state).folios_info.cast());
}

unsafe fn kho_test_preserve_data(state: *mut kho_test_state) -> c_int {
    let mut folios_info_phys: kho_vmalloc = kho_vmalloc::default();
    let folios_info = vmalloc_array((*state).nr_folios as usize, core::mem::size_of::<phys_addr_t>());
    if folios_info.is_null() {
        return -ENOMEM;
    }

    let mut err = kho_preserve_vmalloc(folios_info, &mut folios_info_phys);
    if err != 0 {
        vfree(folios_info.cast());
        return err;
    }

    (*state).folios_info_phys = folios_info_phys;
    (*state).folios_info = folios_info.cast();

    for i in 0..(*state).nr_folios {
        let folio = *(*state).folios.add(i as usize);
        let order = folio_order(folio);
        *(*state).folios_info.add(i as usize) = virt_to_phys(folio_address(folio)) | order as phys_addr_t;
        err = kho_preserve_folio(folio);
        if err != 0 {
            // kho_test_unpreserve_data frees folio_info; bail out immediately
            // to avoid double free.
            kho_test_unpreserve_data(state);
            return err;
        }
        (*state).nr_folios_preserved += 1;
    }

    0
}

unsafe fn kho_test_prepare_fdt(state: *mut kho_test_state, fdt_size: ssize_t) -> c_int {
    let compatible = KHO_TEST_COMPAT.as_bytes();
    let magic = KHO_TEST_MAGIC;
    let fdt = folio_address((*state).fdt);
    let mut err = fdt_create(fdt, fdt_size);
    err |= fdt_finish_reservemap(fdt);
    err |= fdt_begin_node(fdt, cstr!(""));
    err |= fdt_property(fdt, cstr!("compatible"), compatible.as_ptr().cast(), compatible.len());
    err |= fdt_property(fdt, cstr!("magic"), (&magic as *const _).cast(), core::mem::size_of_val(&magic));
    err |= fdt_begin_node(fdt, cstr!("data"));
    err |= fdt_property(fdt, cstr!("nr_folios"), &(*state).nr_folios as *const _ as *const c_void, core::mem::size_of_val(&(*state).nr_folios));
    err |= fdt_property(fdt, cstr!("folios_info"), &(*state).folios_info_phys as *const _ as *const c_void, core::mem::size_of_val(&(*state).folios_info_phys));
    err |= fdt_property(fdt, cstr!("csum"), &(*state).csum as *const _ as *const c_void, core::mem::size_of_val(&(*state).csum));
    err |= fdt_end_node(fdt);
    err |= fdt_end_node(fdt);
    err |= fdt_finish(fdt);
    err
}

unsafe fn kho_test_preserve(state: *mut kho_test_state) -> c_int {
    let fdt_size = (*state).nr_folios as ssize_t * core::mem::size_of::<phys_addr_t>() as ssize_t + PAGE_SIZE as ssize_t;
    (*state).fdt = folio_alloc(GFP_KERNEL, get_order(fdt_size));
    if (*state).fdt.is_null() { return -ENOMEM; }
    let mut err = kho_preserve_folio((*state).fdt);
    if err != 0 { folio_put((*state).fdt); return err; }
    err = kho_test_preserve_data(state);
    if err != 0 { kho_unpreserve_folio((*state).fdt); folio_put((*state).fdt); return err; }
    err = kho_test_prepare_fdt(state, fdt_size);
    if err != 0 { kho_test_unpreserve_data(state); kho_unpreserve_folio((*state).fdt); folio_put((*state).fdt); return err; }
    err = kho_add_subtree(KHO_TEST_FDT, folio_address((*state).fdt), fdt_totalsize(folio_address((*state).fdt)));
    if err != 0 { kho_test_unpreserve_data(state); kho_unpreserve_folio((*state).fdt); folio_put((*state).fdt); }
    err
}

unsafe fn kho_test_generate_data(state: *mut kho_test_state) -> c_int {
    let mut alloc_size: usize = 0;
    let mut csum: __wsum = 0;
    while alloc_size < max_mem as usize {
        let mut order = get_random_u32() % NR_PAGE_ORDERS;
        if alloc_size + (PAGE_SIZE << order) > max_mem as usize {
            order = get_order((max_mem as usize - alloc_size) as ssize_t);
            if order != 0 { order -= 1; }
        }
        let size = PAGE_SIZE << order;
        let folio = folio_alloc(GFP_KERNEL | __GFP_NORETRY, order);
        if folio.is_null() { for i in 0..(*state).nr_folios { folio_put(*(*state).folios.add(i as usize)); } (*state).nr_folios = 0; return -ENOMEM; }
        *(*state).folios.add((*state).nr_folios as usize) = folio;
        (*state).nr_folios += 1;
        let addr = folio_address(folio);
        get_random_bytes(addr, size);
        csum = csum_partial(addr, size, csum);
        alloc_size += size;
    }
    (*state).csum = csum;
    0
}

unsafe fn kho_test_save() -> c_int {
    let state = &raw mut kho_test_state;
    max_mem = PAGE_ALIGN(max_mem);
    let max_nr = max_mem >> PAGE_SHIFT;
    (*state).folios = kvmalloc_objs(max_nr);
    if (*state).folios.is_null() { return -ENOMEM; }
    let err = kho_test_generate_data(state);
    if err != 0 { kvfree((*state).folios.cast()); return err; }
    let err = kho_test_preserve(state);
    if err != 0 { kvfree((*state).folios.cast()); }
    err
}

unsafe fn kho_test_restore_data(fdt: *const c_void, mut node: c_int) -> c_int {
    let mut csum: __wsum = 0;
    let mut len = 0;
    node = fdt_path_offset(fdt, cstr!("/data"));
    let nr_folios = fdt_getprop(fdt, node, cstr!("nr_folios"), &mut len) as *const c_uint;
    if nr_folios.is_null() || len as usize != core::mem::size_of::<c_uint>() { return -EINVAL; }
    let old_csum = fdt_getprop(fdt, node, cstr!("csum"), &mut len) as *const __wsum;
    if old_csum.is_null() || len as usize != core::mem::size_of::<__wsum>() { return -EINVAL; }
    let info_phys = fdt_getprop(fdt, node, cstr!("folios_info"), &mut len) as *const kho_vmalloc;
    if info_phys.is_null() || len as usize != core::mem::size_of::<kho_vmalloc>() { return -EINVAL; }
    let info = kho_restore_vmalloc(info_phys);
    if info.is_null() { return -EINVAL; }
    for i in 0..*nr_folios {
        let value = *info.add(i as usize);
        let order = value & !PAGE_MASK;
        let folio = kho_restore_folio(value & PAGE_MASK);
        if folio.is_null() || folio_order(folio) != order { break; }
        csum = csum_partial(folio_address(folio), PAGE_SIZE << order, csum);
        folio_put(folio);
    }
    vfree(info.cast());
    if csum != *old_csum { return -EINVAL; }
    0
}

unsafe fn kho_test_restore(fdt_phys: phys_addr_t) -> c_int {
    let fdt = phys_to_virt(fdt_phys);
    let mut len = 0;
    let node = fdt_path_offset(fdt, cstr!("/"));
    if node < 0 || fdt_node_check_compatible(fdt, node, KHO_TEST_COMPAT) != 0 { return -EINVAL; }
    let magic = fdt_getprop(fdt, node, cstr!("magic"), &mut len) as *const u32;
    if magic.is_null() || len as usize != core::mem::size_of::<u32>() || *magic != KHO_TEST_MAGIC { return -EINVAL; }
    kho_test_restore_data(fdt, node)
}

unsafe fn kho_test_init() -> c_int {
    if !kho_is_enabled() { return 0; }
    let mut fdt_phys = 0;
    let err = kho_retrieve_subtree(KHO_TEST_FDT, &mut fdt_phys, core::ptr::null_mut());
    if err == 0 { let err = kho_test_restore(fdt_phys); if err != 0 { pr_err!("KHO restore failed\n"); } else { pr_info!("KHO restore succeeded\n"); } return err; }
    if err != -ENOENT { pr_warn!("failed to retrieve {} FDT: {}\n", KHO_TEST_FDT, err); return err; }
    kho_test_save()
}

unsafe fn kho_test_cleanup() {
    kho_test_unpreserve_data(&raw mut kho_test_state);
    for i in 0..kho_test_state.nr_folios { folio_put(*kho_test_state.folios.add(i as usize)); }
    kvfree(kho_test_state.folios.cast());
    kho_unpreserve_folio(kho_test_state.fdt);
    folio_put(kho_test_state.fdt);
}

unsafe fn kho_test_exit() {
    kho_remove_subtree(folio_address(kho_test_state.fdt));
    kho_test_cleanup();
}

// module_init!(kho_test_init);
// module_exit!(kho_test_exit);
// MODULE_AUTHOR!("Mike Rapoport <rppt@kernel.org>");
// MODULE_DESCRIPTION!("KHO test module");
// MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
