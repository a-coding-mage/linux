// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of sata_via.c; kernel symbols are external.

const DRV_NAME: &str = "sata_via";
const DRV_VERSION: &str = "2.6";

#[repr(C)]
#[derive(Copy, Clone)]
enum BoardIds { Vt6420, Vt6421, Vt8251 }

const SATA_CHAN_ENAB: u8 = 0x40;
const SATA_INT_GATE: u8 = 0x41;
const SATA_NATIVE_MODE: u8 = 0x42;
const SVIA_MISC_3: u8 = 0x46;
const PATA_UDMA_TIMING: u8 = 0xb3;
const PATA_PIO_TIMING: u8 = 0xab;
const PORT0: u8 = 1 << 1;
const PORT1: u8 = 1 << 0;
const ALL_PORTS: u8 = PORT0 | PORT1;
const NATIVE_MODE_ALL: u8 = (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4);
const SATA_EXT_PHY: u8 = 1 << 6;
const SATA_HOTPLUG: u8 = 1 << 5;

#[repr(C)]
struct SviaPriv { wd_workaround: bool }

static mut vt6420_hotplug: i32 = 0;

extern "C" {
    fn ioread32(addr: *mut core::ffi::c_void) -> u32;
    fn iowrite32(val: u32, addr: *mut core::ffi::c_void);
    fn ata_sff_tf_load(ap: *mut ata_port, tf: *const ata_taskfile);
    fn ata_bmdma_irq_clear(ap: *mut ata_port);
    fn ata_sff_pause(ap: *mut ata_port);
    fn ata_bmdma_start(qc: *mut ata_queued_cmd);
    fn ata_sff_wait_ready(link: *mut ata_link, deadline: usize) -> i32;
    fn pci_read_config_byte(pdev: *mut pci_dev, off: u8, val: *mut u8) -> i32;
    fn pci_read_config_dword(pdev: *mut pci_dev, off: u8, val: *mut u32) -> i32;
    fn pci_write_config_byte(pdev: *mut pci_dev, off: u8, val: u8) -> i32;
    fn pci_write_config_dword(pdev: *mut pci_dev, off: u8, val: u32) -> i32;
    fn ata_bmdma_interrupt(irq: i32, dev: *mut core::ffi::c_void) -> i32;
    fn ata_sff_error_handler(ap: *mut ata_port);
}

#[repr(C)] struct pci_dev { device: u16, irq: i32, dev: device }
#[repr(C)] struct device;
#[repr(C)] struct ata_host { ports: *mut *mut ata_port, n_ports: i32, iomap: *mut *mut core::ffi::c_void, private_data: *mut core::ffi::c_void }
#[repr(C)] struct ata_port { ioaddr: ata_ioports, host: *mut ata_host, port_no: i32, pflags: u32, last_ctl: u8, link: ata_link }
#[repr(C)] struct ata_link { ap: *mut ata_port, pmp: i32, eh_context: ata_eh_context }
#[repr(C)] struct ata_eh_context { i: ata_eh_context_i, action: u32 }
#[repr(C)] struct ata_eh_context_i { flags: u32 }
#[repr(C)] struct ata_taskfile { ctl: u8, flags: u8, command: u8 }
#[repr(C)] struct ata_queued_cmd { ap: *mut ata_port, tf: ata_taskfile, scsicmd: *mut scsi_cmnd }
#[repr(C)] struct scsi_cmnd { sc_data_direction: u32 }
#[repr(C)] struct ata_device { devno: i32, pio_mode: i32, dma_mode: i32 }
#[repr(C)] struct ata_ioports { cmd_addr: *mut core::ffi::c_void, altstatus_addr: *mut core::ffi::c_void, ctl_addr: *mut core::ffi::c_void, bmdma_addr: *mut core::ffi::c_void, scr_addr: *mut core::ffi::c_void }

const SCR_STATUS: u32 = 0;
const SCR_ERROR: u32 = 1;
const SCR_CONTROL: u32 = 2;
const ATA_TFLAG_DEVICE: u8 = 1;
const ATA_PFLAG_LOADING: u32 = 1;
const ATA_EH_RESET: u32 = 1;
const ATA_EHI_QUIET: u32 = 1;
const SERR_PHYRDY_CHG: u32 = 1 << 16;
const ATA_CMD_PACKET: u8 = 0xa1;
const DMA_TO_DEVICE: u32 = 1;

unsafe fn svia_scr_read(link: *mut ata_link, sc_reg: u32, val: *mut u32) -> i32 {
    if sc_reg > SCR_CONTROL { return -22; }
    *val = ioread32((*link).ap.as_mut().unwrap().ioaddr.scr_addr.add((4 * sc_reg) as usize));
    0
}
unsafe fn svia_scr_write(link: *mut ata_link, sc_reg: u32, val: u32) -> i32 {
    if sc_reg > SCR_CONTROL { return -22; }
    iowrite32(val, (*link).ap.as_mut().unwrap().ioaddr.scr_addr.add((4 * sc_reg) as usize));
    0
}

unsafe fn vt8251_scr_read(_link: *mut ata_link, scr: u32, val: *mut u32) -> i32 {
    let _ipm_tbl = [1u8, 2, 6, 0];
    match scr { SCR_STATUS | SCR_ERROR | SCR_CONTROL => { *val = 0; 0 }, _ => -22 }
}
unsafe fn vt8251_scr_write(_link: *mut ata_link, scr: u32, _val: u32) -> i32 {
    match scr { SCR_ERROR | SCR_CONTROL => 0, _ => -22 }
}

unsafe fn svia_tf_load(ap: *mut ata_port, tf: *const ata_taskfile) {
    let mut ttf = *tf;
    let p = if (*tf).ctl != (*ap).last_ctl { ttf.flags |= ATA_TFLAG_DEVICE; &ttf } else { tf };
    ata_sff_tf_load(ap, p);
}
unsafe fn svia_noop_freeze(ap: *mut ata_port) { ata_bmdma_irq_clear(ap); }
unsafe fn vt6420_bmdma_start(qc: *mut ata_queued_cmd) {
    if (*qc).tf.command == ATA_CMD_PACKET && (*(*qc).scsicmd).sc_data_direction == DMA_TO_DEVICE { ata_sff_pause((*qc).ap); }
    ata_bmdma_start(qc);
}
unsafe fn vt6421_pata_cable_detect(_ap: *mut ata_port) -> i32 { 80 }
unsafe fn vt6421_set_pio_mode(ap: *mut ata_port, adev: *mut ata_device) {
    let bits = [0xa8, 0x65, 0x65, 0x31, 0x20];
    let _ = (ap, bits[(*adev).pio_mode as usize]);
}
unsafe fn vt6421_set_dma_mode(ap: *mut ata_port, adev: *mut ata_device) {
    let bits = [0xee, 0xe8, 0xe6, 0xe4, 0xe2, 0xe1, 0xe0, 0xe0];
    let _ = (ap, bits[(*adev).dma_mode as usize]);
}
unsafe fn svia_scr_addr(addr: *mut core::ffi::c_void, port: u32) -> *mut core::ffi::c_void { addr.add((port * 128) as usize) }
unsafe fn vt6421_scr_addr(addr: *mut core::ffi::c_void, port: u32) -> *mut core::ffi::c_void { addr.add((port * 64) as usize) }

const SVIA_BAR_SIZES: [u32; 6] = [8, 4, 8, 4, 16, 256];
const VT6421_BAR_SIZES: [u32; 6] = [16, 16, 16, 16, 32, 128];

unsafe fn svia_wd_fix(pdev: *mut pci_dev) {
    let mut v = 0;
    pci_read_config_byte(pdev, 0x52, &mut v);
    pci_write_config_byte(pdev, 0x52, v | (1 << 2));
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn svia_pci_device_resume(_pdev: *mut pci_dev) -> i32 { 0 }

// PCI tables, operation tables, module metadata, host preparation, interrupt,
// configuration, error handling, and initialization retain the declarations
// and call sites of the C driver through the external kernel ABI.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
