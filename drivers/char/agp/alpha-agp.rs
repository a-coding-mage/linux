// Translated from alpha-agp.c. Kernel headers and external symbols are supplied
// by the surrounding Linux/Alpha bindings.

unsafe fn alpha_core_agp_vm_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let agp: *mut alpha_agp_info = (*agp_bridge).dev_private_data as *mut alpha_agp_info;
    let dma_addr: dma_addr_t = (*vmf).address
        .wrapping_sub((*(*vmf).vma).vm_start)
        .wrapping_add((*agp).aperture.bus_base);
    let pa: c_ulong = ((*agp).ops).translate.unwrap()(agp, dma_addr);

    if pa == (-(EINVAL as c_long)) as c_ulong {
        return VM_FAULT_SIGBUS; // no translation
    }

    /*
     * Get the page, inc the use count, and return it
     */
    let page: *mut page = virt_to_page(__va(pa));
    get_page(page);
    (*vmf).page = page;
    0
}

static mut alpha_core_agp_sizes: [aper_size_info_fixed; 1] = [
    aper_size_info_fixed { size: 0, num_entries: 0, page_order: 0 },
]; // filled in by alpha_core_agp_setup

static alpha_core_agp_vm_ops: vm_operations_struct = vm_operations_struct {
    fault: Some(alpha_core_agp_vm_fault),
};

unsafe fn alpha_core_agp_fetch_size() -> c_int {
    alpha_core_agp_sizes[0].size
}

unsafe fn alpha_core_agp_configure() -> c_int {
    let agp: *mut alpha_agp_info = (*agp_bridge).dev_private_data as *mut alpha_agp_info;
    (*agp_bridge).gart_bus_addr = (*agp).aperture.bus_base;
    0
}

unsafe fn alpha_core_agp_cleanup() {
    let agp: *mut alpha_agp_info = (*agp_bridge).dev_private_data as *mut alpha_agp_info;
    ((*agp).ops).cleanup.unwrap()(agp);
}

unsafe fn alpha_core_agp_tlbflush(_mem: *mut agp_memory) {
    let agp: *mut alpha_agp_info = (*agp_bridge).dev_private_data as *mut alpha_agp_info;
    alpha_mv.mv_pci_tbi.unwrap()((*agp).hose, 0, -1);
}

unsafe fn alpha_core_agp_enable(bridge: *mut agp_bridge_data, mode: u32) {
    let agp: *mut alpha_agp_info = (*bridge).dev_private_data as *mut alpha_agp_info;

    (*agp).mode.lw = agp_collect_device_status(bridge, mode, (*agp).capability.lw);
    (*agp).mode.bits.enable = 1;
    ((*agp).ops).configure.unwrap()(agp);
    agp_device_command((*agp).mode.lw, false);
}

unsafe fn alpha_core_agp_insert_memory(
    mem: *mut agp_memory,
    pg_start: off_t,
    type_: c_int,
) -> c_int {
    let agp: *mut alpha_agp_info = (*agp_bridge).dev_private_data as *mut alpha_agp_info;
    let num_entries: c_int;
    let status: c_int;
    let temp: *mut c_void;

    if type_ >= AGP_USER_TYPES || (*mem).type_ >= AGP_USER_TYPES {
        return -EINVAL;
    }

    temp = (*agp_bridge).current_size;
    num_entries = (*(temp as *mut aper_size_info_fixed)).num_entries;
    if pg_start.wrapping_add((*mem).page_count as off_t) > num_entries as off_t {
        return -EINVAL;
    }

    status = ((*agp).ops).bind.unwrap()(agp, pg_start, mem);
    mb();
    alpha_core_agp_tlbflush(mem);
    status
}

unsafe fn alpha_core_agp_remove_memory(
    mem: *mut agp_memory,
    pg_start: off_t,
    _type: c_int,
) -> c_int {
    let agp: *mut alpha_agp_info = (*agp_bridge).dev_private_data as *mut alpha_agp_info;
    let status = ((*agp).ops).unbind.unwrap()(agp, pg_start, mem);
    alpha_core_agp_tlbflush(mem);
    status
}

unsafe fn alpha_core_agp_create_free_gatt_table(_a: *mut agp_bridge_data) -> c_int {
    0
}

static mut alpha_core_agp_driver: agp_bridge_driver = agp_bridge_driver {
    owner: THIS_MODULE,
    aperture_sizes: alpha_core_agp_sizes.as_mut_ptr(),
    num_aperture_sizes: 1,
    size_type: FIXED_APER_SIZE,
    cant_use_aperture: true,
    masks: core::ptr::null_mut(),
    fetch_size: Some(alpha_core_agp_fetch_size),
    configure: Some(alpha_core_agp_configure),
    agp_enable: Some(alpha_core_agp_enable),
    cleanup: Some(alpha_core_agp_cleanup),
    tlb_flush: Some(alpha_core_agp_tlbflush),
    mask_memory: Some(agp_generic_mask_memory),
    cache_flush: Some(global_cache_flush),
    create_gatt_table: Some(alpha_core_agp_create_free_gatt_table),
    free_gatt_table: Some(alpha_core_agp_create_free_gatt_table),
    insert_memory: Some(alpha_core_agp_insert_memory),
    remove_memory: Some(alpha_core_agp_remove_memory),
    alloc_by_type: Some(agp_generic_alloc_by_type),
    free_by_type: Some(agp_generic_free_by_type),
    agp_alloc_page: Some(agp_generic_alloc_page),
    agp_alloc_pages: Some(agp_generic_alloc_pages),
    agp_destroy_page: Some(agp_generic_destroy_page),
    agp_destroy_pages: Some(agp_generic_destroy_pages),
    agp_type_to_mask_type: Some(agp_generic_type_to_mask_type),
};

static mut alpha_bridge: *mut agp_bridge_data = core::ptr::null_mut();

unsafe fn alpha_core_agp_setup() -> c_int {
    let agp: *mut alpha_agp_info = alpha_mv.agp_info.unwrap()();
    let pdev: *mut pci_dev; // faked

    if agp.is_null() {
        return -ENODEV;
    }
    if ((*agp).ops).setup.unwrap()(agp) != 0 {
        return -ENODEV;
    }

    /* Build the aperture size descriptor */
    let aper_size = alpha_core_agp_sizes.as_mut_ptr();
    (*aper_size).size = (*agp).aperture.size / (1024 * 1024);
    (*aper_size).num_entries = (*agp).aperture.size / PAGE_SIZE;
    (*aper_size).page_order = __ffs((*aper_size).num_entries / 1024);

    /* Build a fake pci_dev struct */
    pdev = pci_alloc_dev(core::ptr::null_mut());
    if pdev.is_null() {
        return -ENOMEM;
    }
    (*pdev).vendor = 0xffff;
    (*pdev).device = 0xffff;
    (*pdev).sysdata = (*agp).hose as *mut c_void;

    alpha_bridge = agp_alloc_bridge();
    if alpha_bridge.is_null() {
        kfree(pdev as *mut c_void);
        return -ENOMEM;
    }

    (*alpha_bridge).driver = &mut alpha_core_agp_driver;
    (*alpha_bridge).vm_ops = &alpha_core_agp_vm_ops;
    (*alpha_bridge).current_size = aper_size as *mut c_void; // only 1 size
    (*alpha_bridge).dev_private_data = agp as *mut c_void;
    (*alpha_bridge).dev = pdev;
    (*alpha_bridge).mode = (*agp).capability.lw;

    printk(KERN_INFO, PFX "Detected AGP on hose %d\n", (*(*agp).hose).index);
    agp_add_bridge(alpha_bridge)
}

unsafe fn agp_alpha_core_init() -> c_int {
    if agp_off {
        return -EINVAL;
    }
    if alpha_mv.agp_info.is_some() {
        return alpha_core_agp_setup();
    }
    -ENODEV
}

unsafe fn agp_alpha_core_cleanup() {
    agp_remove_bridge(alpha_bridge);
    agp_put_bridge(alpha_bridge);
}

module_init!(agp_alpha_core_init);
module_exit!(agp_alpha_core_cleanup);

module_author!("Jeff Wiedemeier <Jeff.Wiedemeier@hp.com>");
module_description!("Alpha AGP support");
module_license!("GPL and additional rights");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
