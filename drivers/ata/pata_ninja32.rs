// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_ninja32.c - Ninja32 PATA for new ATA layer
 *
 * Direct low-level translation of the original Linux driver.  Kernel types,
 * functions, constants, and structure layouts are supplied externally.
 */

const DRV_NAME: &[u8] = b"pata_ninja32\0";
const DRV_VERSION: &[u8] = b"0.1.5\0";

extern "C" {
    fn iowrite8(value: u8, address: *mut core::ffi::c_void);
    fn ata_sff_dev_select(ap: *mut ata_port, device: u32);
    fn ata_host_alloc(dev: *mut device, n_ports: i32) -> *mut ata_host;
    fn pcim_enable_device(dev: *mut pci_dev) -> i32;
    fn pcim_iomap_regions(dev: *mut pci_dev, mask: u32, name: *const u8) -> i32;
    fn pcim_pin_device(dev: *mut pci_dev);
    fn pcim_iomap_table(dev: *mut pci_dev) -> *mut *mut core::ffi::c_void;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> i32;
    fn pci_set_master(dev: *mut pci_dev);
    fn ata_sff_std_ports(ioaddr: *mut ata_ioports);
    fn ata_bmdma_interrupt(irq: i32, dev: *mut core::ffi::c_void) -> i32;
    fn ata_host_activate(host: *mut ata_host, irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32,
                         flags: u32, sht: *const scsi_host_template) -> i32;
    fn pci_get_drvdata(dev: *mut pci_dev) -> *mut ata_host;
    fn ata_pci_device_do_resume(dev: *mut pci_dev) -> i32;
    fn ata_host_resume(host: *mut ata_host);
    fn ata_pci_remove_one(dev: *mut pci_dev);
    fn ata_pci_device_suspend(dev: *mut pci_dev, state: u32) -> i32;
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub dev: device, pub irq: i32 }
#[repr(C)] pub struct pci_device_id { pub vendor: u16, pub device: u16 }
#[repr(C)] pub struct ata_device { pub pio_mode: u8 }
#[repr(C)] pub struct ata_link { pub device: [ata_device; 2] }
#[repr(C)] pub struct ata_ioports { pub bmdma_addr: *mut core::ffi::c_void, pub cmd_addr: *mut core::ffi::c_void, pub ctl_addr: *mut core::ffi::c_void, pub altstatus_addr: *mut core::ffi::c_void }
#[repr(C)] pub struct ata_port { pub link: ata_link, pub private_data: *mut core::ffi::c_void, pub ioaddr: ata_ioports, pub ops: *const ata_port_operations, pub pio_mask: u32, pub flags: u32, pub pflags: u32 }
#[repr(C)] pub struct ata_host { pub ports: *mut *mut ata_port, pub iomap: *mut *mut core::ffi::c_void }
#[repr(C)] pub struct scsi_host_template { _private: [u8; 0] }
#[repr(C)] pub struct ata_port_operations { pub inherits: *const ata_port_operations, pub sff_dev_select: Option<unsafe extern "C" fn(*mut ata_port, u32)>, pub cable_detect: Option<unsafe extern "C" fn() -> i32>, pub set_piomode: Option<unsafe extern "C" fn(*mut ata_port, *mut ata_device)>, pub sff_data_xfer: Option<unsafe extern "C" fn()> }

const XFER_PIO_0: u8 = 0;
const ATA_PIO4: u32 = 4;
const ATA_FLAG_SLAVE_POSS: u32 = 1 << 0;
const ATA_PFLAG_PIO32: u32 = 1 << 1;
const ATA_PFLAG_PIO32CHANGE: u32 = 1 << 2;
const ATA_DMA_MASK: u64 = !0;
const IRQF_SHARED: u32 = 1 << 0;

unsafe extern "C" fn ninja32_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static PIO_TIMING: [u16; 5] = [0xd6, 0x85, 0x44, 0x33, 0x13];
    let timing = PIO_TIMING[((*adev).pio_mode - XFER_PIO_0) as usize] as u8;
    iowrite8(timing, (*ap).ioaddr.bmdma_addr.add(0x1f));
    (*ap).private_data = adev.cast();
}

unsafe extern "C" fn ninja32_dev_select(ap: *mut ata_port, device: u32) {
    let adev = (*ap).link.device.as_mut_ptr().add(device as usize);
    if (*ap).private_data != adev.cast() {
        iowrite8(0xd6, (*ap).ioaddr.bmdma_addr.add(0x1f));
        ata_sff_dev_select(ap, device);
        ninja32_set_piomode(ap, adev);
    }
}

static NINJA32_SHT: scsi_host_template = scsi_host_template { _private: [] };
static NINJA32_PORT_OPS: ata_port_operations = ata_port_operations {
    inherits: core::ptr::null(), sff_dev_select: Some(ninja32_dev_select), cable_detect: None,
    set_piomode: Some(ninja32_set_piomode), sff_data_xfer: None,
};

unsafe fn ninja32_program(base: *mut core::ffi::c_void) {
    iowrite8(0x05, base.add(0x01));
    iowrite8(0xbe, base.add(0x02));
    iowrite8(0x01, base.add(0x03));
    iowrite8(0x20, base.add(0x04));
    iowrite8(0x8f, base.add(0x05));
    iowrite8(0xa4, base.add(0x1c));
    iowrite8(0x83, base.add(0x1d));
}

unsafe extern "C" fn ninja32_init_one(dev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    let host = ata_host_alloc(&mut (*dev).dev, 1);
    if host.is_null() { return -12; }
    let ap = *(*host).ports;
    let mut rc = pcim_enable_device(dev);
    if rc != 0 { return rc; }
    rc = pcim_iomap_regions(dev, 1, DRV_NAME.as_ptr());
    if rc == -16 { pcim_pin_device(dev); }
    if rc != 0 { return rc; }
    (*host).iomap = pcim_iomap_table(dev);
    rc = dma_set_mask_and_coherent(&mut (*dev).dev, ATA_DMA_MASK);
    if rc != 0 { return rc; }
    pci_set_master(dev);
    let base = *(*host).iomap;
    if base.is_null() { return -12; }
    (*ap).ops = &NINJA32_PORT_OPS;
    (*ap).pio_mask = ATA_PIO4;
    (*ap).flags |= ATA_FLAG_SLAVE_POSS;
    (*ap).ioaddr.cmd_addr = base.add(0x10);
    (*ap).ioaddr.ctl_addr = base.add(0x1e);
    (*ap).ioaddr.altstatus_addr = base.add(0x1e);
    (*ap).ioaddr.bmdma_addr = base;
    ata_sff_std_ports(&mut (*ap).ioaddr);
    (*ap).pflags |= ATA_PFLAG_PIO32 | ATA_PFLAG_PIO32CHANGE;
    ninja32_program(base);
    ata_host_activate(host, (*dev).irq, ata_bmdma_interrupt, IRQF_SHARED, &NINJA32_SHT)
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" fn ninja32_reinit_one(pdev: *mut pci_dev) -> i32 {
    let host = pci_get_drvdata(pdev);
    let rc = ata_pci_device_do_resume(pdev);
    if rc != 0 { return rc; }
    ninja32_program(*(*host).iomap);
    ata_host_resume(host);
    0
}

static NINJA32: [pci_device_id; 7] = [
    pci_device_id { vendor: 0x10fc, device: 0x0003 }, pci_device_id { vendor: 0x1145, device: 0x8008 },
    pci_device_id { vendor: 0x1145, device: 0xf008 }, pci_device_id { vendor: 0x1145, device: 0xf021 },
    pci_device_id { vendor: 0x1145, device: 0xf024 }, pci_device_id { vendor: 0x1145, device: 0xf02c },
    pci_device_id { vendor: 0, device: 0 },
];

#[repr(C)] struct pci_driver {
    name: *const u8,
    id_table: *const pci_device_id,
    probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
    #[cfg(CONFIG_PM_SLEEP)] suspend: Option<unsafe extern "C" fn(*mut pci_dev, u32) -> i32>,
    #[cfg(CONFIG_PM_SLEEP)] resume: Option<unsafe extern "C" fn(*mut pci_dev) -> i32>,
}

static NINJA32_PCI_DRIVER: pci_driver = pci_driver {
    name: DRV_NAME.as_ptr(), id_table: NINJA32.as_ptr(), probe: Some(ninja32_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)] suspend: Some(ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)] resume: Some(ninja32_reinit_one),
};

// Equivalent of module_pci_driver(ninja32_pci_driver).
// MODULE_AUTHOR("Alan Cox");
// MODULE_DESCRIPTION("low-level driver for Ninja32 ATA");
// MODULE_LICENSE("GPL");
// MODULE_DEVICE_TABLE(pci, ninja32);
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
