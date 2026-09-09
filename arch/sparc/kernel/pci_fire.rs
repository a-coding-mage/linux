// SPDX-License-Identifier: GPL-2.0
/* pci_fire.c: Sun4u platform PCI-E controller support. */

const DRIVER_NAME: &str = "fire";
const PFX: &str = "fire: ";

const FIRE_IOMMU_CONTROL: usize = 0x40000;
const FIRE_IOMMU_TSBBASE: usize = 0x40008;
const FIRE_IOMMU_FLUSH: usize = 0x40100;
const FIRE_IOMMU_FLUSHINV: usize = 0x40108;

unsafe fn pci_fire_pbm_iommu_init(pbm: *mut pci_pbm_info) -> i32 {
    let iommu = (*pbm).iommu;
    let vdma = [0xc0000000u32, 0x40000000u32];
    let dma_mask = 0xffff_ffffu32;
    let tsbsize = 128;

    (*iommu).iommu_control = (*pbm).pbm_regs + FIRE_IOMMU_CONTROL;
    (*iommu).iommu_tsbbase = (*pbm).pbm_regs + FIRE_IOMMU_TSBBASE;
    (*iommu).iommu_flush = (*pbm).pbm_regs + FIRE_IOMMU_FLUSH;
    (*iommu).iommu_flushinv = (*pbm).pbm_regs + FIRE_IOMMU_FLUSHINV;
    (*iommu).write_complete_reg = (*pbm).controller_regs + 0x410000;
    upa_writeq(!0u64, (*iommu).iommu_flushinv);
    let err = iommu_table_init(iommu, tsbsize * 8 * 1024, vdma[0], dma_mask, (*pbm).numa_node);
    if err != 0 { return err; }
    upa_writeq(__pa((*iommu).page_table) | 0x7, (*iommu).iommu_tsbbase);
    let mut control = upa_readq((*iommu).iommu_control);
    control |= 0x00000400 | 0x00000300 | 0x00000002 | 0x00000001;
    upa_writeq(control, (*iommu).iommu_control);
    0
}

#[repr(C)]
struct pci_msiq_entry { word0: u64, word1: u64, resv: [u64; 6] }

const MSIQ_WORD0_FMT_TYPE: u64 = 0x7f00000000000000;
const MSIQ_WORD0_FMT_TYPE_SHIFT: u32 = 56;
const MSIQ_WORD0_DATA0: u64 = 0xffff;
const MSIQ_WORD0_DATA0_SHIFT: u32 = 0;
const MSIQ_TYPE_MSI32: u64 = 0xb;
const MSIQ_TYPE_MSI64: u64 = 0xf;
const EVENT_QUEUE_BASE_ADDR_REG: usize = 0x010000;
const EVENT_QUEUE_BASE_ADDR_ALL_ONES: u64 = 0xfffc000000000000;
const EVENT_QUEUE_CONTROL_SET_EN: u64 = 0x0000100000000000;
const EVENT_QUEUE_HEAD_VAL: u64 = 0x7;
const MSI_MAP_VALID: u64 = 0x8000000000000000;
const MSI_MAP_EQNUM: u64 = 0x3f;
const MSI_CLEAR_EQWR_N: u64 = 0x4000000000000000;
const IMONDO_DATA0: usize = 0x02c000;
const IMONDO_DATA1: usize = 0x02c008;
const MSI_32BIT_ADDR: usize = 0x034000;
const MSI_64BIT_ADDR: usize = 0x034008;

const fn event_queue_head(eq: usize) -> usize { 0x011800 + eq * 8 }
const fn event_queue_tail(eq: usize) -> usize { 0x011600 + eq * 8 }
const fn event_queue_control_set(eq: usize) -> usize { 0x011000 + eq * 8 }
const fn msi_map(msi: usize) -> usize { 0x020000 + msi * 8 }
const fn msi_clear(msi: usize) -> usize { 0x028000 + msi * 8 }

unsafe fn pci_fire_get_head(pbm: *mut pci_pbm_info, msiqid: usize, head: *mut usize) -> i32 {
    *head = upa_readq((*pbm).pbm_regs + event_queue_head(msiqid)) as usize; 0
}

unsafe fn pci_fire_dequeue_msi(pbm: *mut pci_pbm_info, msiqid: usize, head: *mut usize, msi: *mut usize) -> i32 {
    let base = ((*pbm).msi_queues as *mut pci_msiq_entry).add((msiqid - (*pbm).msiq_first) * 8192);
    let ep = &mut *base.add(*head);
    if ep.word0 & MSIQ_WORD0_FMT_TYPE == 0 { return 0; }
    let type_fmt = (ep.word0 & MSIQ_WORD0_FMT_TYPE) >> MSIQ_WORD0_FMT_TYPE_SHIFT;
    let typ = type_fmt >> 3;
    if typ != MSIQ_TYPE_MSI32 && typ != MSIQ_TYPE_MSI64 { return -22; }
    *msi = ((ep.word0 & MSIQ_WORD0_DATA0) >> MSIQ_WORD0_DATA0_SHIFT) as usize;
    upa_writeq(MSI_CLEAR_EQWR_N, (*pbm).pbm_regs + msi_clear(*msi));
    ep.word0 &= !MSIQ_WORD0_FMT_TYPE;
    *head += 1;
    if *head >= (*pbm).msiq_ent_count as usize { *head = 0; }
    1
}

unsafe fn pci_fire_set_head(pbm: *mut pci_pbm_info, msiqid: usize, head: usize) -> i32 {
    upa_writeq(head as u64, (*pbm).pbm_regs + event_queue_head(msiqid)); 0
}
unsafe fn pci_fire_msi_setup(pbm: *mut pci_pbm_info, msiqid: usize, msi: usize, _is_msi64: i32) -> i32 {
    let reg = (*pbm).pbm_regs + msi_map(msi); let mut val = upa_readq(reg);
    val = (val & !MSI_MAP_EQNUM) | msiqid as u64; upa_writeq(val, reg);
    upa_writeq(MSI_CLEAR_EQWR_N, (*pbm).pbm_regs + msi_clear(msi));
    upa_writeq(upa_readq(reg) | MSI_MAP_VALID, reg); 0
}
unsafe fn pci_fire_msi_teardown(pbm: *mut pci_pbm_info, msi: usize) -> i32 {
    let reg = (*pbm).pbm_regs + msi_map(msi); upa_writeq(upa_readq(reg) & !MSI_MAP_VALID, reg); 0
}

unsafe fn pci_fire_msiq_alloc(pbm: *mut pci_pbm_info) -> i32 {
    let order = get_order(512 * 1024); let pages = __get_free_pages(GFP_KERNEL | __GFP_COMP, order);
    if pages == 0 { printk(KERN_ERR, "MSI: Cannot allocate MSI queues (o=%lu).\n", order); return -12; }
    memset(pages as *mut core::ffi::c_void, 0, PAGE_SIZE << order); (*pbm).msi_queues = pages as *mut core::ffi::c_void;
    upa_writeq(EVENT_QUEUE_BASE_ADDR_ALL_ONES | __pa((*pbm).msi_queues), (*pbm).pbm_regs + EVENT_QUEUE_BASE_ADDR_REG);
    upa_writeq((*pbm).portid as u64 << 6, (*pbm).pbm_regs + IMONDO_DATA0); upa_writeq(0, (*pbm).pbm_regs + IMONDO_DATA1);
    upa_writeq((*pbm).msi32_start, (*pbm).pbm_regs + MSI_32BIT_ADDR); upa_writeq((*pbm).msi64_start, (*pbm).pbm_regs + MSI_64BIT_ADDR);
    for i in 0..(*pbm).msiq_num as usize { upa_writeq(0, (*pbm).pbm_regs + event_queue_head(i)); upa_writeq(0, (*pbm).pbm_regs + event_queue_tail(i)); } 0
}
unsafe fn pci_fire_msiq_free(pbm: *mut pci_pbm_info) { let order = get_order(512 * 1024); free_pages((*pbm).msi_queues as usize, order); (*pbm).msi_queues = core::ptr::null_mut(); }

// The remaining declarations and hardware/probe operations retain the C implementation's
// external kernel types and calls; these are supplied by the surrounding sparc PCI subsystem.
unsafe fn pci_fire_msi_init(pbm: *mut pci_pbm_info) { sparc64_pbm_msi_init(pbm, &pci_fire_msiq_ops); }
unsafe fn pci_fire_hw_init(pbm: *mut pci_pbm_info) {
    upa_writeq(0x8000000000000000, (*pbm).controller_regs + 0x470010);
    upa_writeq(0x000000000600c047, (*pbm).controller_regs + 0x471028);
    upa_writeq(!0u64, (*pbm).controller_regs + 0x471800);
    let r = (*pbm).pbm_regs; let mut v = upa_readq(r + 0x80000); upa_writeq(v | 0xda000101, r + 0x80000);
    upa_writeq(0, r + 0x90008); upa_writeq(0x40, r + 0x90020); upa_writeq(0, r + 0xe2008); upa_writeq(0x100, r + 0xe2200);
    upa_writeq(3, r + 0xe2240); upa_writeq(0xffff0000, r + 0xe2430); upa_writeq(3000000, r + 0xe2788); upa_writeq(500000, r + 0xe2790);
    upa_writeq((2 << 16) | (140 << 8), r + 0xe2798); upa_writeq(0, r + 0xe27a0); upa_writeq(!0u64, r + 0x31800);
    upa_writeq(0, r + 0x53000); upa_writeq(0, r + 0x53008); upa_writeq(!0u64, r + 0x51800);
}

unsafe fn pci_fire_pbm_init(pbm: *mut pci_pbm_info, op: *mut platform_device, portid: u32) -> i32 {
    (*pbm).numa_node = NUMA_NO_NODE; (*pbm).pci_ops = &sun4u_pci_ops; (*pbm).config_space_reg_bits = 12;
    (*pbm).index = pci_num_pbms; pci_num_pbms += 1; (*pbm).portid = portid; (*pbm).op = op;
    let dp = (*op).dev.of_node; (*pbm).name = (*dp).full_name;
    let regs = of_get_property(dp, b"reg\0".as_ptr(), core::ptr::null_mut());
    (*pbm).pbm_regs = (*regs).phys_addr; (*pbm).controller_regs = (*regs.add(1)).phys_addr - 0x410000;
    printk(KERN_INFO, "%s: SUN4U PCIE Bus Module\n", (*pbm).name);
    pci_determine_mem_io_space(pbm); pci_get_pbm_props(pbm); pci_fire_hw_init(pbm);
    let err = pci_fire_pbm_iommu_init(pbm); if err != 0 { return err; }
    pci_fire_msi_init(pbm); (*pbm).pci_bus = pci_scan_one_pbm(pbm, &mut (*op).dev);
    (*pbm).next = pci_pbm_root; pci_pbm_root = pbm; 0
}

unsafe fn fire_probe(op: *mut platform_device) -> i32 {
    let dp = (*op).dev.of_node; let portid = of_getintprop_default(dp, b"portid\0".as_ptr(), 0xff);
    let pbm = kzalloc_obj::<pci_pbm_info>(); if pbm.is_null() { printk(KERN_ERR, "%sCannot allocate pci_pbminfo.\n", PFX); return -12; }
    let iommu = kzalloc_obj::<iommu>(); if iommu.is_null() { printk(KERN_ERR, "%sCannot allocate PBM iommu.\n", PFX); kfree(pbm); return -12; }
    (*pbm).iommu = iommu; let err = pci_fire_pbm_init(pbm, op, portid); if err != 0 { kfree(iommu); kfree(pbm); return err; }
    dev_set_drvdata(&mut (*op).dev, pbm as *mut core::ffi::c_void); 0
}

// C build-time CONFIG_PCI_MSI wiring and platform-driver registration are kept as
// declarations because their kernel definitions are supplied by other translation units.
extern "C" {
    static mut pci_fire_msiq_ops: core::ffi::c_void;
    static mut pci_num_pbms: u32;
    static mut pci_pbm_root: *mut pci_pbm_info;
    static sun4u_pci_ops: core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
