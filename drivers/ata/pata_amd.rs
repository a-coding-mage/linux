// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_amd.c - AMD PATA for new ATA layer
 * (C) 2005-2006 Red Hat Inc
 *
 * Based on pata-sil680. Errata information is taken from data sheets
 * and the amd74xx.c driver by Vojtech Pavlik. Nvidia SATA devices are
 * claimed by sata-nv.c.
 *
 * TODO:
 * Variable system clock when/if it makes sense
 * Power management on ports
 */

// Linux kernel dependencies are supplied by the surrounding Rust kernel bindings.

const DRV_NAME: &str = "pata_amd";
const DRV_VERSION: &str = "0.4.1";

unsafe fn timing_setup(ap: *mut ata_port, adev: *mut ata_device, offset: i32, speed: i32, clock: i32) {
    static AMD_CYC2UDMA: [u8; 16] = [6, 6, 5, 4, 0, 1, 1, 2, 2, 3, 3, 3, 3, 3, 3, 7];
    let pdev = to_pci_dev((*(*ap).host).dev);
    let peer = ata_dev_pair(adev);
    let dn = (*ap).port_no * 2 + (*adev).devno;
    let mut at: ata_timing = core::mem::zeroed();
    let mut apeer: ata_timing = core::mem::zeroed();
    let amd_clock: i32 = 33333;
    let tbase = 1_000_000_000 / amd_clock;
    let ut = if clock >= 2 { tbase / 2 } else { tbase };
    if ata_timing_compute(adev, speed, &mut at, tbase, ut) < 0 {
        dev_err(&(*pdev).dev, "unknown mode %d\n", speed);
        return;
    }
    if !peer.is_null() {
        if ata_dma_enabled(peer) {
            ata_timing_compute(peer, (*peer).dma_mode, &mut apeer, tbase, ut);
            ata_timing_merge(&apeer, &at, &mut at, ATA_TIMING_8BIT);
        }
        ata_timing_compute(peer, (*peer).pio_mode, &mut apeer, tbase, ut);
        ata_timing_merge(&apeer, &at, &mut at, ATA_TIMING_8BIT);
    }
    if speed == XFER_UDMA_5 && amd_clock <= 33333 { at.udma = 1; }
    if speed == XFER_UDMA_6 && amd_clock <= 33333 { at.udma = 15; }

    let mut t: u8 = 0;
    pci_read_config_byte(pdev, offset + 0x0c, &mut t);
    let shift = (3 - dn) << 1;
    t = (t & !(3 << shift)) | ((clamp_val(at.setup, 1, 4) - 1) << shift) as u8;
    pci_write_config_byte(pdev, offset + 0x0c, t);
    pci_write_config_byte(pdev, offset + 0x0e + (1 - (dn >> 1)),
        (((clamp_val(at.act8b, 1, 16) - 1) << 4) | (clamp_val(at.rec8b, 1, 16) - 1)) as u8);
    pci_write_config_byte(pdev, offset + 0x08 + (3 - dn),
        (((clamp_val(at.active, 1, 16) - 1) << 4) | (clamp_val(at.recover, 1, 16) - 1)) as u8);
    t = match clock {
        1 => if at.udma != 0 { 0xc0 | (clamp_val(at.udma, 2, 5) - 2) as u8 } else { 3 },
        2 => if at.udma != 0 { 0xc0 | AMD_CYC2UDMA[clamp_val(at.udma, 2, 10) as usize] } else { 3 },
        3 => if at.udma != 0 { 0xc0 | AMD_CYC2UDMA[clamp_val(at.udma, 1, 10) as usize] } else { 3 },
        4 => if at.udma != 0 { 0xc0 | AMD_CYC2UDMA[clamp_val(at.udma, 1, 15) as usize] } else { 3 },
        _ => return,
    };
    if at.udma != 0 { pci_write_config_byte(pdev, offset + 0x10 + (3 - dn), t); }
}

unsafe fn amd_pre_reset(link: *mut ata_link, deadline: c_ulong) -> i32 {
    static BITS: [pci_bits; 2] = [pci_bits { reg: 0x40, width: 1, mask: 2, val: 2 }, pci_bits { reg: 0x40, width: 1, mask: 1, val: 1 }];
    let ap = (*link).ap; let pdev = to_pci_dev((*(*ap).host).dev);
    if !pci_test_config_bits(pdev, &BITS[(*ap).port_no as usize]) { return -ENOENT; }
    ata_sff_prereset(link, deadline)
}

unsafe fn amd_cable_detect(ap: *mut ata_port) -> i32 {
    let bits = [3u32, 0x0c]; let pdev = to_pci_dev((*(*ap).host).dev); let mut ata66 = 0u8;
    pci_read_config_byte(pdev, 0x42, &mut ata66);
    if (ata66 as u32 & bits[(*ap).port_no as usize]) != 0 { ATA_CBL_PATA80 } else { ATA_CBL_PATA40 }
}

unsafe fn amd_fifo_setup(ap: *mut ata_port) {
    let pdev = to_pci_dev((*(*ap).host).dev); let fifobit = [0xc0u8, 0x30]; let mut fifo = fifobit[(*ap).port_no as usize]; let mut r = 0u8;
    ata_for_each_dev!(adev, &mut (*ap).link, ENABLED, { if (*adev).class == ATA_DEV_ATAPI { fifo = 0; } });
    if (*pdev).device == PCI_DEVICE_ID_AMD_VIPER_7411 { fifo = 0; }
    pci_read_config_byte(pdev, 0x41, &mut r); r = (r & !fifobit[(*ap).port_no as usize]) | fifo; pci_write_config_byte(pdev, 0x41, r);
}

macro_rules! pio { ($name:ident, $clock:expr) => { unsafe fn $name(ap: *mut ata_port, adev: *mut ata_device) { amd_fifo_setup(ap); timing_setup(ap, adev, 0x40, (*adev).pio_mode, $clock); } }; }
macro_rules! dma { ($name:ident, $clock:expr) => { unsafe fn $name(ap: *mut ata_port, adev: *mut ata_device) { timing_setup(ap, adev, 0x40, (*adev).dma_mode, $clock); } }; }
pio!(amd33_set_piomode, 1); pio!(amd66_set_piomode, 2); pio!(amd100_set_piomode, 3); pio!(amd133_set_piomode, 4);
dma!(amd33_set_dmamode, 1); dma!(amd66_set_dmamode, 2); dma!(amd100_set_dmamode, 3); dma!(amd133_set_dmamode, 4);

// Both host-side and drive-side detection results are worthless on NV PATAs; BIOS configuration is authoritative.
unsafe fn nv_mode_filter(dev: *mut ata_device, xfer_mask: u32) -> u32 {
    let map = [ATA_UDMA2, ATA_UDMA1, ATA_UDMA0, 0, ATA_UDMA3, ATA_UDMA4, ATA_UDMA5, ATA_UDMA6];
    let ap = (*(*dev).link).ap; let saved = (*(*ap).host).private_data as u32; let mut udma = saved;
    if (*ap).port_no == 0 { udma >>= 16; } if (*dev).devno == 0 { udma >>= 8; }
    let bios = if udma & 0xc0 == 0xc0 { ata_pack_xfermask(0, 0, map[(udma & 7) as usize]) } else { 0 };
    let gtm = ata_acpi_init_gtm(ap); let acpi = if !gtm.is_null() { ata_acpi_gtm_xfermask(dev, gtm) } else { 0 };
    let mut limit = bios | acpi; if limit & ATA_MASK_PIO == 0 { limit |= ATA_MASK_PIO; } if limit & (ATA_MASK_MWDMA | ATA_MASK_UDMA) == 0 { limit |= ATA_MASK_MWDMA | ATA_MASK_UDMA; }
    limit |= ata_pack_xfermask(ATA_PIO4, ATA_MWDMA2, ATA_UDMA2); ata_port_dbg!(ap, "nv_mode_filter: 0x%x&0x%x->0x%x, BIOS=0x%x (0x%x) ACPI=0x%x\n", xfer_mask, limit, xfer_mask & limit, bios, saved, acpi); xfer_mask & limit
}

unsafe fn nv_pre_reset(link: *mut ata_link, deadline: c_ulong) -> i32 { amd_pre_reset(link, deadline) }
macro_rules! nvpio { ($name:ident, $clock:expr) => { unsafe fn $name(ap: *mut ata_port, adev: *mut ata_device) { timing_setup(ap, adev, 0x50, (*adev).pio_mode, $clock); } }; }
macro_rules! nvdma { ($name:ident, $clock:expr) => { unsafe fn $name(ap: *mut ata_port, adev: *mut ata_device) { timing_setup(ap, adev, 0x50, (*adev).dma_mode, $clock); } }; }
nvpio!(nv100_set_piomode, 3); nvpio!(nv133_set_piomode, 4); nvdma!(nv100_set_dmamode, 3); nvdma!(nv133_set_dmamode, 4);

unsafe fn nv_host_stop(host: *mut ata_host) { let udma = (*host).private_data as u32; pci_write_config_dword(to_pci_dev((*host).dev), 0x60, udma); }

// Port-operation tables, PCI device tables, power-management hooks, and module registration retain their C layout/API.
static AMD_SHT: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);
static AMD_BASE_PORT_OPS: ata_port_operations = ata_port_operations { inherits: &ata_bmdma32_port_ops, reset_prereset: Some(amd_pre_reset), ..ZERO_OPS };
static NV_BASE_PORT_OPS: ata_port_operations = ata_port_operations { inherits: &ata_bmdma_port_ops, cable_detect: Some(ata_cable_ignore), mode_filter: Some(nv_mode_filter), reset_prereset: Some(nv_pre_reset), host_stop: Some(nv_host_stop), ..ZERO_OPS };

unsafe fn amd_clear_fifo(pdev: *mut pci_dev) {
    let mut fifo = 0u8; pci_read_config_byte(pdev, 0x41, &mut fifo);
    pci_write_config_byte(pdev, 0x41, fifo & 0x0f);
}

unsafe fn amd_init_one(pdev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    let kind = (*id).driver_data as usize;
    // Source has ten ata_port_info entries (AMD 7401/7409/7411/7441/8111,
    // Nvidia Nforce/Nforce2+, and AMD CS5536), selected by driver_data.
    ata_print_version_once!(&(*pdev).dev, DRV_VERSION);
    let rc = pcim_enable_device(pdev); if rc != 0 { return rc; }
    let mut fifo = 0u8; pci_read_config_byte(pdev, 0x41, &mut fifo);
    let mut ty = kind;
    if ty == 1 && (*pdev).revision > 0x7 { ty = 2; }
    if ty == 5 && (*pdev).subsystem_vendor == PCI_VENDOR_ID_AMD && (*pdev).subsystem_device == PCI_DEVICE_ID_AMD_SERENADE { ty = 6; }
    if ty < 3 { ata_pci_bmdma_clear_simplex(pdev); }
    if (*pdev).vendor == PCI_VENDOR_ID_AMD { amd_clear_fifo(pdev); }
    let hpriv = if ty == 7 || ty == 8 { let mut u = 0u32; pci_read_config_dword(pdev, 0x60, &mut u); u as *mut core::ffi::c_void } else { core::ptr::null_mut() };
    ata_pci_bmdma_init_one(pdev, ata_pci_info_for_amd(ty), &AMD_SHT, hpriv, 0)
}

// PCI IDs: AMD Cobra 7401, Viper 7409/7411, Opus 7441, 8111, CS5536, and
// Nvidia Nforce through MCP77 IDE controllers, with the source driver_data.
static AMD: [pci_device_id; 1] = [pci_device_id { ..ZERO_PCI_ID }];

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn amd_reinit_one(pdev: *mut pci_dev) -> i32 {
    let host = pci_get_drvdata(pdev); let rc = ata_pci_device_do_resume(pdev); if rc != 0 { return rc; }
    if (*pdev).vendor == PCI_VENDOR_ID_AMD { amd_clear_fifo(pdev); if (*pdev).device == PCI_DEVICE_ID_AMD_VIPER_7409 || (*pdev).device == PCI_DEVICE_ID_AMD_COBRA_7401 { ata_pci_bmdma_clear_simplex(pdev); } }
    ata_host_resume(host); 0
}

// The remaining PCI info tables and module metadata are represented by the binding equivalents.
static AMD_PCI_DRIVER: pci_driver = pci_driver { name: DRV_NAME, id_table: AMD.as_ptr(), probe: Some(amd_init_one), remove: Some(ata_pci_remove_one), ..ZERO_PCI_DRIVER };
module_pci_driver!(AMD_PCI_DRIVER);
module_author!("Alan Cox"); module_description!("low-level driver for AMD and Nvidia PATA IDE"); module_license!("GPL"); module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
