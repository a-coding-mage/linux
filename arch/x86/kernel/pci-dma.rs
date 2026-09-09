// SPDX-License-Identifier: GPL-2.0
// External kernel dependencies are supplied by other translation units.

static mut DISABLE_DAC_QUIRK: bool = false;

static mut DMA_OPS: *const dma_map_ops = core::ptr::null();

#[cfg(CONFIG_IOMMU_DEBUG)]
static mut PANIC_ON_OVERFLOW: i32 = 1;
#[cfg(not(CONFIG_IOMMU_DEBUG))]
static mut PANIC_ON_OVERFLOW: i32 = 0;
#[cfg(CONFIG_IOMMU_DEBUG)]
static mut FORCE_IOMMU: i32 = 1;
#[cfg(not(CONFIG_IOMMU_DEBUG))]
static mut FORCE_IOMMU: i32 = 0;

static mut IOMMU_MERGE: i32 = 0;
static mut NO_IOMMU: i32 = 0;
// Set this to 1 if there is a HW IOMMU in the system
static mut IOMMU_DETECTED: i32 = 0;

#[cfg(CONFIG_SWIOTLB)]
static mut X86_SWIOTLB_ENABLE: bool = false;
#[cfg(CONFIG_SWIOTLB)]
static mut X86_SWIOTLB_FLAGS: u32 = 0;

#[cfg(CONFIG_SWIOTLB)]
unsafe fn pci_swiotlb_detect() {
    // don't initialize swiotlb if iommu=off (no_iommu=1)
    if NO_IOMMU == 0 && max_possible_pfn > MAX_DMA32_PFN {
        X86_SWIOTLB_ENABLE = true;
    }

    // Set swiotlb to 1 so that bounce buffers are allocated and used for
    // devices that can't support DMA to encrypted memory.
    if cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) {
        X86_SWIOTLB_ENABLE = true;
    }

    // Guest with guest memory encryption currently perform all DMA through
    // bounce buffers as the hypervisor can't access arbitrary VM memory
    // that is not explicitly shared with it.
    if cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) {
        X86_SWIOTLB_ENABLE = true;
    }
}

#[cfg(not(CONFIG_SWIOTLB))]
#[inline]
unsafe fn pci_swiotlb_detect() {}

#[cfg(CONFIG_SWIOTLB_XEN)]
unsafe fn xen_swiotlb_enabled() -> bool {
    xen_initial_domain() || X86_SWIOTLB_ENABLE ||
        (cfg!(CONFIG_XEN_PCIDEV_FRONTEND) && xen_pv_pci_possible)
}

#[cfg(CONFIG_SWIOTLB_XEN)]
unsafe fn pci_xen_swiotlb_init() {
    if !xen_swiotlb_enabled() {
        return;
    }
    X86_SWIOTLB_ENABLE = true;
    X86_SWIOTLB_FLAGS |= SWIOTLB_ANY;
    swiotlb_init_remap(true, X86_SWIOTLB_FLAGS, xen_swiotlb_fixup);
    DMA_OPS = &xen_swiotlb_dma_ops;
    if cfg!(CONFIG_PCI) {
        pci_request_acs();
    }
}

#[cfg(not(CONFIG_SWIOTLB_XEN))]
#[inline]
unsafe fn pci_xen_swiotlb_init() {}

unsafe fn pci_iommu_alloc() {
    if xen_pv_domain() {
        pci_xen_swiotlb_init();
        return;
    }
    pci_swiotlb_detect();
    gart_iommu_hole_init();
    amd_iommu_detect();
    detect_intel_iommu();
    swiotlb_init(X86_SWIOTLB_ENABLE, X86_SWIOTLB_FLAGS);
}

unsafe fn iommu_setup(mut p: *mut u8) -> i32 {
    IOMMU_MERGE = 1;
    if p.is_null() {
        return -EINVAL;
    }

    while *p != 0 {
        if !strncmp(p, b"off\0".as_ptr(), 3) { NO_IOMMU = 1; }
        // gart_parse_options has more force support
        if !strncmp(p, b"force\0".as_ptr(), 5) { FORCE_IOMMU = 1; }
        if !strncmp(p, b"noforce\0".as_ptr(), 7) { IOMMU_MERGE = 0; FORCE_IOMMU = 0; }
        if !strncmp(p, b"biomerge\0".as_ptr(), 8) { IOMMU_MERGE = 1; FORCE_IOMMU = 1; }
        if !strncmp(p, b"panic\0".as_ptr(), 5) { PANIC_ON_OVERFLOW = 1; }
        if !strncmp(p, b"nopanic\0".as_ptr(), 7) { PANIC_ON_OVERFLOW = 0; }
        if !strncmp(p, b"merge\0".as_ptr(), 5) { IOMMU_MERGE = 1; FORCE_IOMMU = 1; }
        if !strncmp(p, b"nomerge\0".as_ptr(), 7) { IOMMU_MERGE = 0; }
        if !strncmp(p, b"forcesac\0".as_ptr(), 8) { pr_warn(b"forcesac option ignored.\n\0".as_ptr()); }
        if !strncmp(p, b"allowdac\0".as_ptr(), 8) { pr_warn(b"allowdac option ignored.\n\0".as_ptr()); }
        if !strncmp(p, b"nodac\0".as_ptr(), 5) { pr_warn(b"nodac option ignored.\n\0".as_ptr()); }
        if !strncmp(p, b"usedac\0".as_ptr(), 6) { DISABLE_DAC_QUIRK = true; return 1; }
        #[cfg(CONFIG_SWIOTLB)]
        if !strncmp(p, b"soft\0".as_ptr(), 4) { X86_SWIOTLB_ENABLE = true; }
        if !strncmp(p, b"pt\0".as_ptr(), 2) { iommu_set_default_passthrough(true); }
        if !strncmp(p, b"nopt\0".as_ptr(), 4) { iommu_set_default_translated(true); }
        gart_parse_options(p);
        p = p.add(strcspn(p, b",\0".as_ptr()));
        if *p == b',' { p = p.add(1); }
    }
    0
}

unsafe fn pci_iommu_init() -> i32 {
    x86_init.iommu.iommu_init();
    #[cfg(CONFIG_SWIOTLB)]
    if X86_SWIOTLB_ENABLE {
        pr_info(b"PCI-DMA: Using software bounce buffering for IO (SWIOTLB)\n\0".as_ptr());
        swiotlb_print_info();
    } else {
        swiotlb_exit();
    }
    0
}

#[cfg(CONFIG_PCI)]
unsafe fn via_no_dac_cb(pdev: *mut pci_dev, _data: *mut core::ffi::c_void) -> i32 {
    (*pdev).dev.bus_dma_limit = DMA_BIT_MASK(32);
    0
}

#[cfg(CONFIG_PCI)]
unsafe fn via_no_dac(dev: *mut pci_dev) {
    if !DISABLE_DAC_QUIRK {
        dev_info(&mut (*dev).dev, b"disabling DAC on VIA PCI bridge\n\0".as_ptr());
        pci_walk_bus((*dev).subordinate, via_no_dac_cb, core::ptr::null_mut());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
