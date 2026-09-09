// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2005-2008, PA Semi, Inc
 *
 * Maintained by: Olof Johansson <olof@lixom.net>
 */

// Dependencies supplied by the surrounding kernel translation.

const IOBMAP_PAGE_SHIFT: u32 = 12;
const IOBMAP_PAGE_SIZE: usize = 1usize << IOBMAP_PAGE_SHIFT;
const IOBMAP_PAGE_MASK: usize = IOBMAP_PAGE_SIZE - 1;

const IOB_BASE: usize = 0xe0000000;
const IOB_SIZE: usize = 0x3000;
const IOBCAP_REG: usize = 0x40;
const IOBCOM_REG: usize = 0x100;
const IOBCOM_ATEN: u32 = 0x00000100;

const IOB_AD_REG: usize = 0x14c;
const IOB_AD_VGPRT: u32 = 0x00000e00;
const IOB_AD_VGAEN: u32 = 0x00000100;
const IOB_AD_MPSEL_MASK: u32 = 0x00000030;
const IOB_AD_MPSEL_B38: u32 = 0x00000000;
const IOB_AD_MPSEL_B40: u32 = 0x00000010;
const IOB_AD_MPSEL_B42: u32 = 0x00000020;
const IOB_AD_TRNG_MASK: u32 = 0x00000003;
const IOB_AD_TRNG_256M: u32 = 0x00000000;
const IOB_AD_TRNG_2G: u32 = 0x00000001;
const IOB_AD_TRNG_128G: u32 = 0x00000003;

const IOB_TABLEBASE_REG: usize = 0x154;
const IOB_XLT_L1_REGBASE: usize = 0x2b00;
const IOB_AT_INVAL_TLB_REG: usize = 0x2d00;

const IOBMAP_L1E_V: u32 = 0x40000000;
const IOBMAP_L1E_V_B: u32 = 0x80000000;
const IOBMAP_L1E_BIG_CACHED: u32 = 0x00000002;
const IOBMAP_L1E_BIG_PRIORITY: u32 = 0x00000001;
const IOBMAP_L2E_V: u32 = 0x80000000;
const IOBMAP_L2E_V_CACHED: u32 = 0xc0000000;

static mut iob: *mut core::ffi::c_void = core::ptr::null_mut();
static mut iob_l1_emptyval: u32 = 0;
static mut iob_l2_emptyval: u32 = 0;
static mut iob_l2_base: *mut u32 = core::ptr::null_mut();

static mut iommu_table_iobmap: iommu_table = unsafe { core::mem::zeroed() };
static mut iommu_table_iobmap_inited: i32 = 0;

unsafe fn iobmap_build(
    tbl: *mut iommu_table,
    mut index: isize,
    mut npages: isize,
    mut uaddr: usize,
    _direction: dma_data_direction,
    _attrs: usize,
) -> i32 {
    pr_debug!("iobmap: build at: {:x}, {:x}, addr: {:x}\n", index, npages, uaddr);
    let mut bus_addr = ((*tbl).it_offset + index as usize) << IOBMAP_PAGE_SHIFT;
    let mut ip = (*tbl).it_base as *mut u32;
    ip = ip.add(index as usize);
    while npages != 0 {
        let rpn = __pa(uaddr) >> IOBMAP_PAGE_SHIFT;
        *ip = IOBMAP_L2E_V | rpn as u32;
        out_le32(iob.add(IOB_AT_INVAL_TLB_REG), (bus_addr >> 14) as u32);
        ip = ip.add(1);
        uaddr += IOBMAP_PAGE_SIZE;
        bus_addr += IOBMAP_PAGE_SIZE;
        npages -= 1;
    }
    0
}

unsafe fn iobmap_free(tbl: *mut iommu_table, mut index: isize, mut npages: isize) {
    pr_debug!("iobmap: free at: {:x}, {:x}\n", index, npages);
    let mut bus_addr = ((*tbl).it_offset + index as usize) << IOBMAP_PAGE_SHIFT;
    let mut ip = ((*tbl).it_base as *mut u32).add(index as usize);
    while npages != 0 {
        *ip = iob_l2_emptyval;
        out_le32(iob.add(IOB_AT_INVAL_TLB_REG), (bus_addr >> 14) as u32);
        ip = ip.add(1);
        bus_addr += IOBMAP_PAGE_SIZE;
        npages -= 1;
    }
}

static mut iommu_table_iobmap_ops: iommu_table_ops = iommu_table_ops {
    set: Some(iobmap_build),
    clear: Some(iobmap_free),
};

unsafe fn iommu_table_iobmap_setup() {
    pr_debug!(" -> {}\n", module_path!());
    iommu_table_iobmap.it_busno = 0;
    iommu_table_iobmap.it_offset = 0;
    iommu_table_iobmap.it_page_shift = IOBMAP_PAGE_SHIFT;
    iommu_table_iobmap.it_size = 0x80000000usize >> iommu_table_iobmap.it_page_shift;
    iommu_table_iobmap.it_base = iob_l2_base as usize;
    iommu_table_iobmap.it_index = 0;
    // XXXOJN tune this to avoid IOB cache invals. Should probably be 8 (64 bytes)
    iommu_table_iobmap.it_blocksize = 4;
    iommu_table_iobmap.it_ops = &raw mut iommu_table_iobmap_ops;
    if !iommu_init_table(&raw mut iommu_table_iobmap, 0, 0, 0) {
        panic!("Failed to initialize iommu table");
    }
    pr_debug!(" <- {}\n", module_path!());
}

unsafe fn pci_dma_bus_setup_pasemi(bus: *mut pci_bus) {
    pr_debug!("pci_dma_bus_setup, bus {:p}, bus->self {:p}\n", bus, (*bus).self_);
    if iommu_table_iobmap_inited == 0 {
        iommu_table_iobmap_inited = 1;
        iommu_table_iobmap_setup();
    }
}

unsafe fn pci_dma_dev_setup_pasemi(dev: *mut pci_dev) {
    pr_debug!("pci_dma_dev_setup, dev {:p} ({})\n", dev, pci_name(dev));
    // CONFIG_PPC_PASEMI_IOMMU_DMA_FORCE is a build-time condition.
    if (*dev).vendor == 0x1959 && (*dev).device == 0xa007
        && !firmware_has_feature(FW_FEATURE_LPAR)
    {
        (*dev).dev.dma_ops = core::ptr::null();
        (*dev).dev.coherent_dma_mask = DMA_BIT_MASK(44);
        return;
    }
    set_iommu_table_base(&mut (*dev).dev, &raw mut iommu_table_iobmap);
}

unsafe fn iob_init(_dn: *mut device_node) -> i32 {
    let mut tmp: usize;
    let mut regword: u32;
    pr_debug!(" -> {}\n", module_path!());
    iob_l2_base = memblock_alloc_try_nid_raw(1usize << 21, 1usize << 21, MEMBLOCK_LOW_LIMIT, 0x80000000, NUMA_NO_NODE) as *mut u32;
    if iob_l2_base.is_null() {
        panic!("{}: Failed to allocate {} bytes align=0x{:x} max_addr={:x}\n", module_path!(), 1usize << 21, 1usize << 21, 0x80000000u32);
    }
    pr_info!("IOBMAP L2 allocated at: {:p}\n", iob_l2_base);
    tmp = memblock_phys_alloc(IOBMAP_PAGE_SIZE, IOBMAP_PAGE_SIZE);
    if tmp == 0 { panic!("IOBMAP: Cannot allocate spare page!"); }
    iob_l1_emptyval = 0;
    iob_l2_emptyval = IOBMAP_L2E_V | (tmp >> IOBMAP_PAGE_SHIFT) as u32;
    iob = ioremap(IOB_BASE, IOB_SIZE);
    if iob.is_null() { panic!("IOBMAP: Cannot map registers!"); }
    for i in 0..64usize {
        regword = IOBMAP_L1E_V | (__pa(iob_l2_base.add(i * 0x2000) as usize) >> 12) as u32;
        out_le32(iob.add(IOB_XLT_L1_REGBASE + i * 4), regword);
    }
    regword = in_le32(iob.add(IOB_AD_REG));
    regword &= !IOB_AD_TRNG_MASK;
    regword |= IOB_AD_TRNG_2G;
    out_le32(iob.add(IOB_AD_REG), regword);
    regword = in_le32(iob.add(IOBCOM_REG));
    regword |= IOBCOM_ATEN;
    out_le32(iob.add(IOBCOM_REG), regword);
    pr_debug!(" <- {}\n", module_path!());
    0
}

pub unsafe fn iommu_init_early_pasemi() {
    // CONFIG_PPC_PASEMI_IOMMU controls the compile-time branch.
    let iommu_off = of_chosen != core::ptr::null_mut() && of_property_read_bool(of_chosen, "linux,iommu-off");
    if iommu_off { return; }
    iob_init(core::ptr::null_mut());
    pasemi_pci_controller_ops.dma_dev_setup = Some(pci_dma_dev_setup_pasemi);
    pasemi_pci_controller_ops.dma_bus_setup = Some(pci_dma_bus_setup_pasemi);
    set_pci_dma_ops(&dma_iommu_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
