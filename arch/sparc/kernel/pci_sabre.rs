// SPDX-License-Identifier: GPL-2.0
/* pci_sabre.c: Sabre specific PCI controller support. */

// Kernel and architecture dependencies supplied by the surrounding tree.

const DRIVER_NAME: &str = "sabre";
const PFX: &str = "sabre: ";

const SABRE_UE_AFSR: u64 = 0x0030;
const SABRE_UEAFSR_PDRD: u64 = 0x4000000000000000;
const SABRE_UEAFSR_PDWR: u64 = 0x2000000000000000;
const SABRE_UEAFSR_SDRD: u64 = 0x0800000000000000;
const SABRE_UEAFSR_SDWR: u64 = 0x0400000000000000;
const SABRE_UEAFSR_SDTE: u64 = 0x0200000000000000;
const SABRE_UEAFSR_PDTE: u64 = 0x0100000000000000;
const SABRE_UEAFSR_BMSK: u64 = 0x0000ffff00000000;
const SABRE_UEAFSR_OFF: u64 = 0x00000000e0000000;
const SABRE_UEAFSR_BLK: u64 = 0x0000000000800000;
const SABRE_UECE_AFAR: u64 = 0x0038;
const SABRE_CE_AFSR: u64 = 0x0040;
const SABRE_CEAFSR_PDRD: u64 = 0x4000000000000000;
const SABRE_CEAFSR_PDWR: u64 = 0x2000000000000000;
const SABRE_CEAFSR_SDRD: u64 = 0x0800000000000000;
const SABRE_CEAFSR_SDWR: u64 = 0x0400000000000000;
const SABRE_CEAFSR_ESYND: u64 = 0x00ff000000000000;
const SABRE_CEAFSR_BMSK: u64 = 0x0000ffff00000000;
const SABRE_CEAFSR_OFF: u64 = 0x00000000e0000000;
const SABRE_CEAFSR_BLK: u64 = 0x0000000000800000;
const SABRE_IOMMU_CONTROL: u64 = 0x0200;
const SABRE_IOMMU_TSBBASE: u64 = 0x0208;
const SABRE_IOMMU_FLUSH: u64 = 0x0210;
const SABRE_IMAP_A_SLOT0: u64 = 0x0c00;
const SABRE_IMAP_B_SLOT0: u64 = 0x0c20;
const SABRE_ICLR_A_SLOT0: u64 = 0x1400;
const SABRE_ICLR_B_SLOT0: u64 = 0x1480;
const SABRE_ICLR_SCSI: u64 = 0x1800;
const SABRE_WRSYNC: u64 = 0x1c20;
const SABRE_PCICTRL: u64 = 0x2000;
const SABRE_PCICTRL_MRLEN: u64 = 0x0000001000000000;
const SABRE_PCICTRL_SERR: u64 = 0x0000000400000000;
const SABRE_PCICTRL_ARBPARK: u64 = 0x0000000000200000;
const SABRE_PCICTRL_ERREN: u64 = 0x0000000000000100;
const SABRE_PCICTRL_AEN: u64 = 0x000000000000000f;
const SABRE_PIOAFSR: u64 = 0x2010;
const SABRE_PIOAFAR: u64 = 0x2018;
const SABRE_CONFIGSPACE: u64 = 0x001000000;

static mut hummingbird_p: i32 = 0;
static mut sabre_root_bus: *mut pci_bus = core::ptr::null_mut();

unsafe fn sabre_ue_intr(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let pbm = dev_id as *mut pci_pbm_info;
    let afsr_reg = (*pbm).controller_regs + SABRE_UE_AFSR;
    let afar_reg = (*pbm).controller_regs + SABRE_UECE_AFAR;
    let afar = upa_readq(afar_reg);
    let afsr = upa_readq(afsr_reg);
    let error_bits = afsr & (SABRE_UEAFSR_PDRD | SABRE_UEAFSR_PDWR | SABRE_UEAFSR_SDRD |
        SABRE_UEAFSR_SDWR | SABRE_UEAFSR_SDTE | SABRE_UEAFSR_PDTE);
    if error_bits == 0 { return IRQ_NONE; }
    upa_writeq(error_bits, afsr_reg);
    printk("%s: Uncorrectable Error, primary error type[%s%s]\n", (*pbm).name,
        if error_bits & SABRE_UEAFSR_PDRD != 0 { "DMA Read" } else if error_bits & SABRE_UEAFSR_PDWR != 0 { "DMA Write" } else { "???" },
        if error_bits & SABRE_UEAFSR_PDTE != 0 { ":Translation Error" } else { "" });
    printk("%s: bytemask[%04lx] dword_offset[%lx] was_block(%d)\n", (*pbm).name,
        (afsr & SABRE_UEAFSR_BMSK) >> 32, (afsr & SABRE_UEAFSR_OFF) >> 29,
        if afsr & SABRE_UEAFSR_BLK != 0 { 1 } else { 0 });
    printk("%s: UE AFAR [%016lx]\n", (*pbm).name, afar);
    printk("%s: UE Secondary errors [", (*pbm).name);
    let mut reported = 0;
    if afsr & SABRE_UEAFSR_SDRD != 0 { reported += 1; printk("(DMA Read)"); }
    if afsr & SABRE_UEAFSR_SDWR != 0 { reported += 1; printk("(DMA Write)"); }
    if afsr & SABRE_UEAFSR_SDTE != 0 { reported += 1; printk("(Translation Error)"); }
    if reported == 0 { printk("(none)"); }
    printk("]\n");
    psycho_check_iommu_error(pbm, afsr, afar, UE_ERR);
    IRQ_HANDLED
}

unsafe fn sabre_ce_intr(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let pbm = dev_id as *mut pci_pbm_info;
    let afsr_reg = (*pbm).controller_regs + SABRE_CE_AFSR;
    let afar_reg = (*pbm).controller_regs + SABRE_UECE_AFAR;
    let afar = upa_readq(afar_reg); let afsr = upa_readq(afsr_reg);
    let error_bits = afsr & (SABRE_CEAFSR_PDRD | SABRE_CEAFSR_PDWR | SABRE_CEAFSR_SDRD | SABRE_CEAFSR_SDWR);
    if error_bits == 0 { return IRQ_NONE; }
    upa_writeq(error_bits, afsr_reg);
    printk("%s: Correctable Error, primary error type[%s]\n", (*pbm).name,
        if error_bits & SABRE_CEAFSR_PDRD != 0 { "DMA Read" } else if error_bits & SABRE_CEAFSR_PDWR != 0 { "DMA Write" } else { "???" });
    printk("%s: syndrome[%02lx] bytemask[%04lx] dword_offset[%lx] was_block(%d)\n", (*pbm).name,
        (afsr & SABRE_CEAFSR_ESYND) >> 48, (afsr & SABRE_CEAFSR_BMSK) >> 32,
        (afsr & SABRE_CEAFSR_OFF) >> 29, if afsr & SABRE_CEAFSR_BLK != 0 { 1 } else { 0 });
    printk("%s: CE AFAR [%016lx]\n", (*pbm).name, afar);
    printk("%s: CE Secondary errors [", (*pbm).name);
    let mut reported = 0;
    if afsr & SABRE_CEAFSR_SDRD != 0 { reported += 1; printk("(DMA Read)"); }
    if afsr & SABRE_CEAFSR_SDWR != 0 { reported += 1; printk("(DMA Write)"); }
    if reported == 0 { printk("(none)"); } printk("]\n"); IRQ_HANDLED
}

// The remaining controller setup is a direct unsafe translation; external kernel
// declarations and structures are intentionally left to the surrounding tree.
unsafe fn sabre_register_error_handlers(pbm: *mut pci_pbm_info) {
    let mut base = (*pbm).controller_regs;
    upa_writeq(SABRE_UEAFSR_PDRD | SABRE_UEAFSR_PDWR | SABRE_UEAFSR_SDRD | SABRE_UEAFSR_SDWR | SABRE_UEAFSR_SDTE | SABRE_UEAFSR_PDTE, base + SABRE_UE_AFSR);
    upa_writeq(SABRE_CEAFSR_PDRD | SABRE_CEAFSR_PDWR | SABRE_CEAFSR_SDRD | SABRE_CEAFSR_SDWR, base + SABRE_CE_AFSR);
    let mut tmp = upa_readq(base + SABRE_PCICTRL); tmp |= SABRE_PCICTRL_ERREN; upa_writeq(tmp, base + SABRE_PCICTRL);
}

unsafe fn sabre_pbm_init(pbm: *mut pci_pbm_info, op: *mut platform_device) {
    psycho_pbm_init_common(pbm, op, "SABRE", PBM_CHIP_TYPE_SABRE);
    (*pbm).pci_afsr = (*pbm).controller_regs + SABRE_PIOAFSR;
    (*pbm).pci_afar = (*pbm).controller_regs + SABRE_PIOAFAR;
    (*pbm).pci_csr = (*pbm).controller_regs + SABRE_PCICTRL;
}

unsafe fn apb_init(sabre_bus: *mut pci_bus) {
    let mut pdev: *mut pci_dev = core::ptr::null_mut();
    list_for_each_entry!(pdev, (*sabre_bus).devices, bus_list);
    while !pdev.is_null() {
        if (*pdev).vendor == PCI_VENDOR_ID_SUN && (*pdev).device == PCI_DEVICE_ID_SUN_SIMBA {
            let mut word16: u16 = 0;
            pci_read_config_word(pdev, PCI_COMMAND, &mut word16);
            word16 |= PCI_COMMAND_SERR | PCI_COMMAND_PARITY | PCI_COMMAND_MASTER | PCI_COMMAND_MEMORY | PCI_COMMAND_IO;
            pci_write_config_word(pdev, PCI_COMMAND, word16);
            pci_write_config_word(pdev, PCI_STATUS, 0xffff);
            pci_write_config_word(pdev, PCI_SEC_STATUS, 0xffff);
            pci_write_config_byte(pdev, PCI_LATENCY_TIMER, 64);
            pci_write_config_byte(pdev, PCI_SEC_LATENCY_TIMER, 64);
            pci_write_config_byte(pdev, PCI_BRIDGE_CONTROL, PCI_BRIDGE_CTL_PARITY | PCI_BRIDGE_CTL_SERR | PCI_BRIDGE_CTL_MASTER_ABORT);
        }
        pdev = list_next_entry!(pdev, bus_list);
    }
}

unsafe fn sabre_scan_bus(pbm: *mut pci_pbm_info, parent: *mut device) {
    static mut once: i32 = 0;
    (*pbm).is_66mhz_capable = if hummingbird_p != 0 { 1 } else { 0 };
    if once != 0 { printk("%sMultiple controllers unsupported.\n", PFX); return; }
    once += 1;
    let bus = pci_scan_one_pbm(pbm, parent);
    if bus.is_null() { return; }
    sabre_root_bus = bus;
    apb_init(bus);
    sabre_register_error_handlers(pbm);
}

unsafe fn sabre_probe(op: *mut platform_device) -> i32 {
    let pbm = kzalloc_obj::<pci_pbm_info>();
    if pbm.is_null() { printk("%sCannot allocate pci_pbm_info.\n", PFX); return -ENOMEM; }
    let iommu = kzalloc_obj::<iommu>();
    if iommu.is_null() { printk("%sCannot allocate PBM iommu.\n", PFX); kfree(pbm); return -ENOMEM; }
    (*pbm).iommu = iommu;
    (*pbm).portid = of_getintprop_default((*op).dev.of_node, "upa-portid", 0xff);
    let pr_regs = of_get_property((*op).dev.of_node, "reg", core::ptr::null_mut());
    if pr_regs.is_null() { printk("%sNo reg property\n", PFX); kfree(iommu); kfree(pbm); return -ENODEV; }
    (*pbm).controller_regs = (*pr_regs).phys_addr;
    let mut clear_irq = SABRE_ICLR_A_SLOT0;
    while clear_irq < SABRE_ICLR_B_SLOT0 + 0x80 { upa_writeq(0, (*pbm).controller_regs + clear_irq); clear_irq += 8; }
    clear_irq = SABRE_ICLR_SCSI;
    while clear_irq < SABRE_ICLR_SCSI + 0x80 { upa_writeq(0, (*pbm).controller_regs + clear_irq); clear_irq += 8; }
    upa_writeq(SABRE_PCICTRL_MRLEN | SABRE_PCICTRL_SERR | SABRE_PCICTRL_ARBPARK | SABRE_PCICTRL_AEN, (*pbm).controller_regs + SABRE_PCICTRL);
    (*pbm).config_space = (*pbm).controller_regs + SABRE_CONFIGSPACE;
    sabre_pbm_init(pbm, op);
    (*pbm).next = pci_pbm_root; pci_pbm_root = pbm;
    dev_set_drvdata(&mut (*op).dev, pbm as *mut _);
    0
}

static mut sabre_driver: platform_driver = platform_driver { driver: driver { name: DRIVER_NAME, of_match_table: sabre_match }, probe: Some(sabre_probe) };
static sabre_match: [of_device_id; 3] = [
    of_device_id { name: "pci", compatible: "pci108e,a001", data: 1 as *mut _ },
    of_device_id { name: "pci", compatible: "pci108e,a000", data: core::ptr::null_mut() },
    of_device_id { name: "", compatible: "", data: core::ptr::null_mut() },
];

unsafe fn sabre_init() -> i32 { platform_driver_register(&mut sabre_driver) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
