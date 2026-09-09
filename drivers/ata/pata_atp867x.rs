// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * pata_atp867x.c - ARTOP 867X 64bit 4-channel UDMA133 ATA controller driver
 *
 * (C) 2009 Google Inc. John(Jung-Ik) Lee <jilee@google.com>
 *
 * Per Atp867 data sheet rev 1.2, Acard.
 * Based in part on early ide code from
 * 2003-2004 by Eric Uhrhane, Google, Inc.
 *
 * TODO:
 *   1. RAID features [comparison, XOR, striping, mirroring, etc.]
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const DRV_NAME: &str = "pata_atp867x";
const DRV_VERSION: &str = "0.7.5";

const ATP867X_IO_CHANNEL_OFFSET: usize = 0x10;
const ATP867X_IO_PIOSPD_ACTIVE_SHIFT: u32 = 4;
const ATP867X_IO_PIOSPD_RECOVER_SHIFT: u32 = 0;
const ATP867X_IO_DMAMODE_MSTR_SHIFT: u32 = 0;
const ATP867X_IO_DMAMODE_MSTR_MASK: u8 = 0x07;
const ATP867X_IO_DMAMODE_SLAVE_SHIFT: u32 = 4;
const ATP867X_IO_DMAMODE_SLAVE_MASK: u8 = 0x70;
const ATP867X_IO_DMAMODE_UDMA_6: u8 = 0x07;
const ATP867X_IO_DMAMODE_UDMA_5: u8 = 0x06;
const ATP867X_IO_DMAMODE_UDMA_4: u8 = 0x05;
const ATP867X_IO_DMAMODE_UDMA_3: u8 = 0x04;
const ATP867X_IO_DMAMODE_UDMA_2: u8 = 0x03;
const ATP867X_IO_DMAMODE_UDMA_1: u8 = 0x02;
const ATP867X_IO_DMAMODE_UDMA_0: u8 = 0x01;
const ATP867X_IO_DMAMODE_DISABLE: u8 = 0x00;
const ATP867X_IO_SYS_INFO_66MHZ: u8 = 0x04;
const ATP867X_IO_SYS_INFO_SLOW_UDMA5: u8 = 0x02;
const ATP867X_IO_SYS_MASK_RESERVED: u8 = !0xf1;
const ATP867X_IO_PORTSPD_VAL: u16 = 0x1143;
const ATP867X_PREREAD_VAL: u16 = 0x0200;
const ATP867X_NUM_PORTS: usize = 4;
const ATP867X_BAR_IOBASE: usize = 0;
const ATP867X_BAR_ROMBASE: usize = 6;

#[repr(C)]
struct atp867x_priv {
    dma_mode: *mut core::ffi::c_void,
    mstr_piospd: *mut core::ffi::c_void,
    slave_piospd: *mut core::ffi::c_void,
    eightb_piospd: *mut core::ffi::c_void,
    pci66mhz: i32,
}

unsafe fn atp867x_iobase(ap: *mut ata_port) -> usize { (*(*ap).host).iomap[0] as usize }
unsafe fn atp867x_sys_info(ap: *mut ata_port) -> usize { 0x3f + atp867x_iobase(ap) }
unsafe fn atp867x_portbase(ap: *mut ata_port, port: usize) -> usize { atp867x_iobase(ap) + port * ATP867X_IO_CHANNEL_OFFSET }
unsafe fn atp867x_dmabase(ap: *mut ata_port, port: usize) -> usize { 0x40 + atp867x_portbase(ap, port) }
unsafe fn atp867x_altstatus(ap: *mut ata_port, port: usize) -> usize { 0x0e + atp867x_portbase(ap, port) }
unsafe fn atp867x_mstrpiospd(ap: *mut ata_port, port: usize) -> usize { 0x08 + atp867x_dmabase(ap, port) }
unsafe fn atp867x_slavepiospd(ap: *mut ata_port, port: usize) -> usize { 0x09 + atp867x_dmabase(ap, port) }
unsafe fn atp867x_8bpiospd(ap: *mut ata_port, port: usize) -> usize { 0x0a + atp867x_dmabase(ap, port) }
unsafe fn atp867x_dmamode(ap: *mut ata_port, port: usize) -> usize { 0x0b + atp867x_dmabase(ap, port) }
unsafe fn atp867x_portspd(ap: *mut ata_port, port: usize) -> usize { 0x4a + atp867x_portbase(ap, port) }
unsafe fn atp867x_preread(ap: *mut ata_port, port: usize) -> usize { 0x4c + atp867x_portbase(ap, port) }

unsafe fn atp867x_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let dp = (*ap).private_data as *mut atp867x_priv;
    let speed = (*adev).dma_mode;
    let mut mode = speed - XFER_UDMA_0 + 1;
    if (*dp).pci66mhz != 0 && mode > ATP867X_IO_DMAMODE_UDMA_0 &&
        ((*pdev).device == PCI_DEVICE_ID_ARTOP_ATP867B || mode < ATP867X_IO_DMAMODE_UDMA_5) { mode -= 1; }
    let mut b = ioread8((*dp).dma_mode);
    if (*adev).devno & 1 != 0 { b = (b & !ATP867X_IO_DMAMODE_SLAVE_MASK) | (mode << ATP867X_IO_DMAMODE_SLAVE_SHIFT); }
    else { b = (b & !ATP867X_IO_DMAMODE_MSTR_MASK) | (mode << ATP867X_IO_DMAMODE_MSTR_SHIFT); }
    iowrite8(b, (*dp).dma_mode);
}

unsafe fn atp867x_get_active_clocks_shifted(ap: *mut ata_port, clk: u32) -> i32 {
    let dp = (*ap).private_data as *mut atp867x_priv;
    let mut clocks = clk as u8;
    if (*dp).pci66mhz != 0 { clocks = clocks.wrapping_add(1); }
    match clocks {
        0 => clocks = 1,
        1..=6 => {},
        7|8 => { clocks = 0; return (clocks as i32) << ATP867X_IO_PIOSPD_ACTIVE_SHIFT; },
        _ => { ata_port_warn(ap, "ATP867X: active %dclk is invalid. Using 12clk.\n", clk); clocks = 7; }
    }
    (clocks as i32) << ATP867X_IO_PIOSPD_ACTIVE_SHIFT
}

unsafe fn atp867x_get_recover_clocks_shifted(_ap: *mut ata_port, clk: u32) -> i32 {
    let mut clocks = clk as u8;
    match clocks { 0 => clocks = 1, 1..=11 => {}, 13|14 => clocks -= 1, 15 => {}, _ => { ata_port_warn(_ap, "ATP867X: recover %dclk is invalid. Using default 12clk.\n", clk); clocks = 0; } }
    (clocks as i32) << ATP867X_IO_PIOSPD_RECOVER_SHIFT
}

unsafe fn atp867x_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let peer = ata_dev_pair(adev); let dp = (*ap).private_data as *mut atp867x_priv;
    let speed = (*adev).pio_mode; let mut t = ata_timing::default(); let mut p = ata_timing::default();
    let tbase = 1000000000 / 33333; let ut = tbase / 4;
    ata_timing_compute(adev, speed, &mut t, tbase, ut);
    if !peer.is_null() && (*peer).pio_mode != 0 { ata_timing_compute(peer, (*peer).pio_mode, &mut p, tbase, ut); ata_timing_merge(&mut p, &mut t, &mut t, ATA_TIMING_8BIT); }
    let mut b = ioread8((*dp).dma_mode);
    if (*adev).devno & 1 != 0 { b &= !ATP867X_IO_DMAMODE_SLAVE_MASK; } else { b &= !ATP867X_IO_DMAMODE_MSTR_MASK; }
    iowrite8(b, (*dp).dma_mode);
    b = (atp867x_get_active_clocks_shifted(ap, t.active) | atp867x_get_recover_clocks_shifted(ap, t.recover)) as u8;
    if (*adev).devno & 1 != 0 { iowrite8(b, (*dp).slave_piospd); } else { iowrite8(b, (*dp).mstr_piospd); }
    b = (atp867x_get_active_clocks_shifted(ap, t.act8b) | atp867x_get_recover_clocks_shifted(ap, t.rec8b)) as u8;
    iowrite8(b, (*dp).eightb_piospd);
}

unsafe fn atp867x_cable_override(pdev: *mut pci_dev) -> i32 { if (*pdev).subsystem_vendor == PCI_VENDOR_ID_ARTOP && ((*pdev).subsystem_device == PCI_DEVICE_ID_ARTOP_ATP867A || (*pdev).subsystem_device == PCI_DEVICE_ID_ARTOP_ATP867B) { 1 } else { 0 } }
unsafe fn atp867x_cable_detect(ap: *mut ata_port) -> i32 { let pdev = to_pci_dev((*(*ap).host).dev); if atp867x_cable_override(pdev) != 0 { ATA_CBL_PATA40_SHORT } else { ATA_CBL_PATA_UNK } }

unsafe fn atp867x_check_res(pdev: *mut pci_dev) { for i in 0..DEVICE_COUNT_RESOURCE { let start = pci_resource_start(pdev, i); let len = pci_resource_len(pdev, i); dev_dbg(&(*pdev).dev, "ATP867X: resource start:len=%lx:%lx\n", start, len); } }
unsafe fn atp867x_check_ports(ap: *mut ata_port, port: i32) { let ioaddr = &(*ap).ioaddr; let dp = (*ap).private_data as *mut atp867x_priv; ata_port_dbg(ap, "ATP867X: port[%d] addresses cmd=0x%lx, 0x%lx ctl=0x%lx, 0x%lx bmdma=0x%lx, 0x%lx data=0x%lx error=0x%lx feature=0x%lx nsect=0x%lx lbal=0x%lx lbam=0x%lx lbah=0x%lx device=0x%lx status=0x%lx command=0x%lx dma=0x%lx mstr=0x%lx slave=0x%lx eightb=0x%lx pci66=0x%lx\n", port, ioaddr.cmd_addr, atp867x_portbase(ap, port as usize), ioaddr.ctl_addr, atp867x_altstatus(ap, port as usize), ioaddr.bmdma_addr, atp867x_dmabase(ap, port as usize), ioaddr.data_addr, ioaddr.error_addr, ioaddr.feature_addr, ioaddr.nsect_addr, ioaddr.lbal_addr, ioaddr.lbam_addr, ioaddr.lbah_addr, ioaddr.device_addr, ioaddr.status_addr, ioaddr.command_addr, (*dp).dma_mode, (*dp).mstr_piospd, (*dp).slave_piospd, (*dp).eightb_piospd, (*dp).pci66mhz); }
unsafe fn atp867x_set_priv(ap: *mut ata_port) -> i32 { let pdev = to_pci_dev((*(*ap).host).dev); let dp = devm_kzalloc(&(*pdev).dev, core::mem::size_of::<atp867x_priv>(), GFP_KERNEL) as *mut atp867x_priv; if dp.is_null() { return -ENOMEM; } (*ap).private_data = dp as *mut _; let port = (*ap).port_no as usize; (*dp).dma_mode = atp867x_dmamode(ap, port) as *mut _; (*dp).mstr_piospd = atp867x_mstrpiospd(ap, port) as *mut _; (*dp).slave_piospd = atp867x_slavepiospd(ap, port) as *mut _; (*dp).eightb_piospd = atp867x_8bpiospd(ap, port) as *mut _; (*dp).pci66mhz = (ioread8(atp867x_sys_info(ap) as *mut _) & ATP867X_IO_SYS_INFO_66MHZ) as i32; 0 }
unsafe fn atp867x_fixup(host: *mut ata_host) { let pdev = to_pci_dev((*host).dev); let ap = (*host).ports[0]; let mut v = 0u8; pci_read_config_byte(pdev, PCI_LATENCY_TIMER, &mut v); if v < 0x80 { v = 0x80; pci_write_config_byte(pdev, PCI_LATENCY_TIMER, v); dev_dbg(&(*pdev).dev, "ATP867X: set latency timer to %d\n", v); } for i in 0..ATP867X_NUM_PORTS { iowrite16(ATP867X_IO_PORTSPD_VAL, atp867x_portspd(ap, i) as *mut _); iowrite16(ATP867X_PREREAD_VAL, atp867x_preread(ap, i) as *mut _); } v = ioread8((atp867x_iobase(ap) + 0x28) as *mut _); v = (v & 0xcf) | 0xc0; iowrite8(v, (atp867x_iobase(ap) + 0x28) as *mut _); v = ioread8(atp867x_sys_info(ap) as *mut _); v &= ATP867X_IO_SYS_MASK_RESERVED; if (*pdev).device == PCI_DEVICE_ID_ARTOP_ATP867B { v |= ATP867X_IO_SYS_INFO_SLOW_UDMA5; } iowrite8(v, atp867x_sys_info(ap) as *mut _); }
unsafe fn atp867x_ata_pci_sff_init_host(host: *mut ata_host) -> i32 { let pdev = to_pci_dev((*host).dev); let rc = pcim_iomap_regions(pdev, 1 << ATP867X_BAR_IOBASE, DRV_NAME); if rc != 0 { return rc; } (*host).iomap = pcim_iomap_table(pdev); atp867x_check_res(pdev); let mut mask = 0u32; for i in 0..(*host).n_ports { let ap = (*host).ports[i]; (*ap).ioaddr.cmd_addr = atp867x_portbase(ap, i); (*ap).ioaddr.ctl_addr = atp867x_altstatus(ap, i); (*ap).ioaddr.altstatus_addr = (*ap).ioaddr.ctl_addr; (*ap).ioaddr.bmdma_addr = atp867x_dmabase(ap, i); ata_sff_std_ports(&mut (*ap).ioaddr); let rc = atp867x_set_priv(ap); if rc != 0 { return rc; } atp867x_check_ports(ap, i as i32); mask |= 1 << i; } if mask == 0 { return -ENODEV; } atp867x_fixup(host); dma_set_mask_and_coherent(&(*pdev).dev, ATA_DMA_MASK) }
unsafe fn atp867x_init_one(pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 { let rc = pcim_enable_device(pdev); if rc != 0 { return rc; } let host = ata_host_alloc_pinfo(&(*pdev).dev, core::ptr::null(), ATP867X_NUM_PORTS); if host.is_null() { return -ENOMEM; } let rc = atp867x_ata_pci_sff_init_host(host); if rc != 0 { return rc; } pci_set_master(pdev); ata_host_activate(host, (*pdev).irq, ata_bmdma_interrupt, IRQF_SHARED, core::ptr::null()) }

// CONFIG_PM_SLEEP declarations are preserved for the kernel build configuration.
#[cfg(CONFIG_PM_SLEEP)]
unsafe fn atp867x_reinit_one(pdev: *mut pci_dev) -> i32 { let host = pci_get_drvdata(pdev); let rc = ata_pci_device_do_resume(pdev); if rc != 0 { return rc; } atp867x_fixup(host); ata_host_resume(host); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
