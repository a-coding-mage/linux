// SPDX-License-Identifier: GPL-2.0-only
/* Translated from pata_serverworks.c. Linux headers and supplied symbols are external dependencies. */

const DRV_NAME: &str = "pata_serverworks";
const DRV_VERSION: &str = "0.4.3";
const SVWKS_CSB5_REVISION_NEW: u8 = 0x92;
const SVWKS_CSB6_REVISION: u8 = 0xa0;

static CSB_BAD_ATA100: [&[u8]; 5] = [b"ST320011A", b"ST340016A", b"ST360021A", b"ST380021A", b""];

#[repr(C)]
struct SvCableTable {
    device: i32,
    subvendor: i32,
    cable_detect: unsafe extern "C" fn(*mut ata_port) -> i32,
}

unsafe extern "C" fn oem_cable(ap: *mut ata_port) -> i32 {
    let pdev = to_pci_dev((*(*ap).host).dev);
    if (*pdev).subsystem_device & (1 << ((*ap).port_no + 14)) != 0 { ATA_CBL_PATA80 } else { ATA_CBL_PATA40 }
}

static mut CABLE_DETECT: [SvCableTable; 9] = [
    SvCableTable { device: PCI_DEVICE_ID_SERVERWORKS_CSB5IDE, subvendor: PCI_VENDOR_ID_DELL, cable_detect: oem_cable },
    SvCableTable { device: PCI_DEVICE_ID_SERVERWORKS_CSB6IDE, subvendor: PCI_VENDOR_ID_DELL, cable_detect: oem_cable },
    SvCableTable { device: PCI_DEVICE_ID_SERVERWORKS_CSB5IDE, subvendor: PCI_VENDOR_ID_SUN, cable_detect: oem_cable },
    SvCableTable { device: PCI_DEVICE_ID_SERVERWORKS_OSB4IDE, subvendor: PCI_ANY_ID, cable_detect: ata_cable_40wire },
    SvCableTable { device: PCI_DEVICE_ID_SERVERWORKS_CSB5IDE, subvendor: PCI_ANY_ID, cable_detect: ata_cable_unknown },
    SvCableTable { device: PCI_DEVICE_ID_SERVERWORKS_CSB6IDE, subvendor: PCI_ANY_ID, cable_detect: ata_cable_unknown },
    SvCableTable { device: PCI_DEVICE_ID_SERVERWORKS_CSB6IDE2, subvendor: PCI_ANY_ID, cable_detect: ata_cable_unknown },
    SvCableTable { device: PCI_DEVICE_ID_SERVERWORKS_HT1000IDE, subvendor: PCI_ANY_ID, cable_detect: ata_cable_unknown },
    SvCableTable { device: 0, subvendor: 0, cable_detect: ata_cable_unknown },
];

unsafe extern "C" fn serverworks_cable_detect(ap: *mut ata_port) -> i32 {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut cb = CABLE_DETECT.as_mut_ptr();
    while (*cb).device != 0 {
        if (*cb).device == (*pdev).device && ((*cb).subvendor == (*pdev).subsystem_vendor || (*cb).subvendor == PCI_ANY_ID) { return ((*cb).cable_detect)(ap); }
        cb = cb.add(1);
    }
    BUG();
    -1
}

unsafe extern "C" fn serverworks_is_csb(pdev: *mut pci_dev) -> u8 {
    match (*pdev).device {
        PCI_DEVICE_ID_SERVERWORKS_CSB5IDE | PCI_DEVICE_ID_SERVERWORKS_CSB6IDE | PCI_DEVICE_ID_SERVERWORKS_CSB6IDE2 | PCI_DEVICE_ID_SERVERWORKS_HT1000IDE => 1,
        _ => 0,
    }
}

unsafe extern "C" fn serverworks_osb4_filter(adev: *mut ata_device, mut mask: u32) -> u32 {
    if (*adev).class == ATA_DEV_ATA { mask &= !ATA_MASK_UDMA; } mask
}

unsafe extern "C" fn serverworks_csb_filter(adev: *mut ata_device, mut mask: u32) -> u32 {
    if (*adev).class != ATA_DEV_ATA { return mask; }
    let mut model_num = [0i8; ATA_ID_PROD_LEN + 1];
    ata_id_c_string((*adev).id, model_num.as_mut_ptr(), ATA_ID_PROD, model_num.len());
    let p = model_num.as_ptr();
    for name in CSB_BAD_ATA100.iter().take(4) {
        if strcmp(name.as_ptr() as *const i8, p) == 0 { mask &= !(0xE0 << ATA_SHIFT_UDMA); }
    }
    mask
}

unsafe extern "C" fn serverworks_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pio_mode: [u8; 5] = [0x5d, 0x47, 0x34, 0x22, 0x20];
    let offset = 1 + 2 * (*ap).port_no - (*adev).devno;
    let devbits = (2 * (*ap).port_no + (*adev).devno) * 4;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let pio = (*adev).pio_mode - XFER_PIO_0;
    pci_write_config_byte(pdev, 0x40 + offset, pio_mode[pio as usize]);
    if serverworks_is_csb(pdev) != 0 {
        let mut csb5_pio = 0u16; pci_read_config_word(pdev, 0x4A, &mut csb5_pio);
        csb5_pio &= !(0x0F << devbits); pci_write_config_word(pdev, 0x4A, csb5_pio | ((pio as u16) << devbits));
    }
}

unsafe extern "C" fn serverworks_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let dma_mode: [u8; 3] = [0x77, 0x21, 0x20];
    let offset = 1 + 2 * (*ap).port_no - (*adev).devno;
    let devbits = 2 * (*ap).port_no + (*adev).devno;
    let pdev = to_pci_dev((*(*ap).host).dev); let mut ultra = 0; let mut ultra_cfg = 0;
    pci_read_config_byte(pdev, 0x54, &mut ultra_cfg); pci_read_config_byte(pdev, 0x56 + (*ap).port_no, &mut ultra);
    ultra &= !(0x0F << ((*adev).devno * 4));
    if (*adev).dma_mode >= XFER_UDMA_0 { pci_write_config_byte(pdev, 0x44 + offset, 0x20); ultra |= ((*adev).dma_mode - XFER_UDMA_0) << ((*adev).devno * 4); ultra_cfg |= 1 << devbits; }
    else { pci_write_config_byte(pdev, 0x44 + offset, dma_mode[((*adev).dma_mode - XFER_MW_DMA_0) as usize]); ultra_cfg &= !(1 << devbits); }
    pci_write_config_byte(pdev, 0x56 + (*ap).port_no, ultra); pci_write_config_byte(pdev, 0x54, ultra_cfg);
}

unsafe extern "C" fn serverworks_fixup_osb4(pdev: *mut pci_dev) -> i32 {
    let isa_dev = pci_get_device(PCI_VENDOR_ID_SERVERWORKS, PCI_DEVICE_ID_SERVERWORKS_OSB4, core::ptr::null_mut());
    if !isa_dev.is_null() { let mut reg = 0u32; pci_read_config_dword(isa_dev, 0x64, &mut reg); reg &= !0x00002000; if reg & 0x00004000 == 0 { dev_info((*pdev).dev, "UDMA not BIOS enabled.\n"); } reg |= 0x00004000; pci_write_config_dword(isa_dev, 0x64, reg); pci_dev_put(isa_dev); return 0; }
    dev_warn((*pdev).dev, "Unable to find bridge.\n"); -ENODEV
}

unsafe extern "C" fn serverworks_fixup_csb(pdev: *mut pci_dev) -> i32 {
    let mut btr = 0u8;
    if PCI_FUNC((*pdev).devfn) & 1 == 0 { let findev = pci_get_device(PCI_VENDOR_ID_SERVERWORKS, PCI_DEVICE_ID_SERVERWORKS_CSB5, core::ptr::null_mut()); if !findev.is_null() { let mut reg4c = 0u32; pci_read_config_dword(findev, 0x4C, &mut reg4c); reg4c &= !0x7ff; reg4c |= 0x60; pci_write_config_dword(findev, 0x4C, reg4c); pci_dev_put(findev); } } else { let findev = pci_get_device(PCI_VENDOR_ID_SERVERWORKS, PCI_DEVICE_ID_SERVERWORKS_CSB6, core::ptr::null_mut()); if !findev.is_null() { let mut reg41 = 0u8; pci_read_config_byte(findev, 0x41, &mut reg41); reg41 &= !0x40; pci_write_config_byte(findev, 0x41, reg41); pci_dev_put(findev); } }
    pci_read_config_byte(pdev, 0x5A, &mut btr); btr &= !0x40; if PCI_FUNC((*pdev).devfn) & 1 == 0 { btr |= 2; } else { btr |= if (*pdev).revision >= SVWKS_CSB5_REVISION_NEW { 3 } else { 2 }; } pci_write_config_byte(pdev, 0x5A, btr); btr as i32
}

unsafe extern "C" fn serverworks_fixup_ht1000(pdev: *mut pci_dev) { let mut btr = 0u8; pci_read_config_byte(pdev, 0x5A, &mut btr); btr = (btr & !0x40) | 3; pci_write_config_byte(pdev, 0x5A, btr); }

unsafe extern "C" fn serverworks_fixup(pdev: *mut pci_dev) -> i32 {
    pci_write_config_byte(pdev, PCI_LATENCY_TIMER, 0x40);
    match (*pdev).device { PCI_DEVICE_ID_SERVERWORKS_OSB4IDE => serverworks_fixup_osb4(pdev), PCI_DEVICE_ID_SERVERWORKS_CSB5IDE => { ata_pci_bmdma_clear_simplex(pdev); serverworks_fixup_csb(pdev) }, PCI_DEVICE_ID_SERVERWORKS_CSB6IDE | PCI_DEVICE_ID_SERVERWORKS_CSB6IDE2 => serverworks_fixup_csb(pdev), PCI_DEVICE_ID_SERVERWORKS_HT1000IDE => { serverworks_fixup_ht1000(pdev); 0 }, _ => 0 }
}

// PCI ID table, port operations, SCSI templates, module registration, and PM callbacks are represented by external kernel binding types.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
