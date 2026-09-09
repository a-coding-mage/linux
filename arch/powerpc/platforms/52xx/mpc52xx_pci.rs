/*
 * PCI code for the Freescale MPC52xx embedded CPU.
 *
 * Copyright (C) 2006 Secret Lab Technologies Ltd.
 *                        Grant Likely <grant.likely@secretlab.ca>
 * Copyright (C) 2004 Sylvain Munaut <tnt@246tNt.com>
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty
 * of any kind, whether express or implied.
 */

// C dependencies supplied by the surrounding kernel translation.

const MPC52XX_PCI_GSCR_BM: u32 = 0x40000000;
const MPC52XX_PCI_GSCR_PE: u32 = 0x20000000;
const MPC52XX_PCI_GSCR_SE: u32 = 0x10000000;
const MPC52XX_PCI_GSCR_XLB2PCI_MASK: u32 = 0x07000000;
const MPC52XX_PCI_GSCR_XLB2PCI_SHIFT: u32 = 24;
const MPC52XX_PCI_GSCR_IPG2PCI_MASK: u32 = 0x00070000;
const MPC52XX_PCI_GSCR_IPG2PCI_SHIFT: u32 = 16;
const MPC52XX_PCI_GSCR_BME: u32 = 0x00004000;
const MPC52XX_PCI_GSCR_PEE: u32 = 0x00002000;
const MPC52XX_PCI_GSCR_SEE: u32 = 0x00001000;
const MPC52XX_PCI_GSCR_PR: u32 = 0x00000001;

#[inline]
const fn mpc52xx_pci_iwbtar_translation(proc_ad: u32, pci_ad: u32, size: u32) -> u32 {
    (proc_ad & 0xff000000) | (((size.wrapping_sub(1)) >> 8) & 0x00ff0000)
        | ((pci_ad >> 16) & 0x0000ff00)
}
#[inline]
const fn mpc52xx_pci_iwcr_pack(win0: u32, win1: u32, win2: u32) -> u32 {
    (win0 << 24) | (win1 << 16) | (win2 << 8)
}
const MPC52XX_PCI_IWCR_DISABLE: u32 = 0;
const MPC52XX_PCI_IWCR_ENABLE: u32 = 1;
const MPC52XX_PCI_IWCR_READ: u32 = 0;
const MPC52XX_PCI_IWCR_READ_LINE: u32 = 2;
const MPC52XX_PCI_IWCR_READ_MULTI: u32 = 4;
const MPC52XX_PCI_IWCR_MEM: u32 = 0;
const MPC52XX_PCI_IWCR_IO: u32 = 8;
const MPC52XX_PCI_TCR_P: u32 = 0x01000000;
const MPC52XX_PCI_TCR_LD: u32 = 0x00010000;
const MPC52XX_PCI_TCR_WCT8: u32 = 8;
const MPC52XX_PCI_TBATR_DISABLE: u32 = 0;
const MPC52XX_PCI_TBATR_ENABLE: u32 = 1;

#[repr(C)]
pub struct Mpc52xxPci {
    pub idr: u32, pub scr: u32, pub ccrir: u32, pub cr1: u32,
    pub bar0: u32, pub bar1: u32, pub reserved1: [u8; 16],
    pub ccpr: u32, pub sid: u32, pub erbar: u32, pub cpr: u32,
    pub reserved2: [u8; 4], pub cr2: u32, pub reserved3: [u8; 32],
    pub gscr: u32, pub tbatr0: u32, pub tbatr1: u32, pub tcr: u32,
    pub iw0btar: u32, pub iw1btar: u32, pub iw2btar: u32,
    pub reserved4: [u8; 4], pub iwcr: u32, pub icr: u32, pub isr: u32,
    pub arb: u32, pub reserved5: [u8; 104], pub car: u32,
    pub reserved6: [u8; 4],
}

// MPC5200 device-tree match table; the concrete kernel types are external.
extern "C" {
    static mpc52xx_pci_ids: [OfDeviceId; 3];
}

#[repr(C)]
struct PciOps { read: unsafe extern "C" fn(*mut PciBus, u32, i32, i32, *mut u32) -> i32, write: unsafe extern "C" fn(*mut PciBus, u32, i32, i32, u32) -> i32 }

#[repr(C)] struct OfDeviceId { type_: *const u8, compatible: *const u8 }
#[repr(C)] struct PciBus { number: u8, _opaque: [u8; 0] }
#[repr(C)] struct PciController { cfg_addr: *mut u32, cfg_data: *mut u8, io_base_virt: *mut u8, first_busno: u8, last_busno: u8, ops: *mut PciOps, mem_resources: [Resource; 2], io_resource: Resource, io_base_phys: u64 }
#[repr(C)] struct DeviceNode { _opaque: [u8; 0] }
#[repr(C)] struct Resource { start: u64, end: u64, flags: u64 }
#[repr(C)] struct PciDev { vendor: u16, device: u16, resource: [Resource; 7] }

static mut MPC52XX_PCI_OPS: PciOps = PciOps { read: mpc52xx_pci_read_config, write: mpc52xx_pci_write_config };

#[allow(dead_code)]
unsafe extern "C" fn mpc52xx_pci_read_config(bus: *mut PciBus, devfn: u32, offset: i32, len: i32, val: *mut u32) -> i32 {
    extern "C" { fn pci_bus_to_host(bus: *mut PciBus) -> *mut PciController; fn out_be32(a: *mut u32, v: u32); fn in_le32(a: *const u8) -> u32; fn mb(); }
    let hose = pci_bus_to_host(bus);
    out_be32((*hose).cfg_addr, (1u32 << 31) | ((*bus).number as u32 << 16) | (devfn << 8) | (offset as u32 & 0xfc)); mb();
    let mut value = in_le32((*hose).cfg_data);
    if len != 4 { value >>= ((offset & 3) << 3); value &= u32::MAX >> (32 - (len << 3)); }
    *val = value; out_be32((*hose).cfg_addr, 0); mb();
    0
}

#[allow(dead_code)]
unsafe extern "C" fn mpc52xx_pci_write_config(bus: *mut PciBus, devfn: u32, offset: i32, len: i32, val: u32) -> i32 {
    extern "C" { fn pci_bus_to_host(bus: *mut PciBus) -> *mut PciController; fn out_be32(a: *mut u32, v: u32); fn out_le32(a: *mut u8, v: u32); fn in_le32(a: *const u8) -> u32; fn mb(); }
    let hose = pci_bus_to_host(bus); out_be32((*hose).cfg_addr, (1u32 << 31) | ((*bus).number as u32 << 16) | (devfn << 8) | (offset as u32 & 0xfc)); mb();
    let mut write_val = val;
    if len != 4 { let shift = ((offset & 3) << 3); let mask = u32::MAX >> (32 - (len << 3)); let old = in_le32((*hose).cfg_data); write_val = (old & !(mask << shift)) | ((val << shift) & (mask << shift)); }
    out_le32((*hose).cfg_data, write_val); mb(); out_be32((*hose).cfg_addr, 0); mb();
    0
}

unsafe fn mpc52xx_pci_setup(hose: *mut PciController, pci_regs: *mut Mpc52xxPci, pci_phys: u64) {
    extern "C" { fn in_be32(addr: *const u32) -> u32; fn out_be32(addr: *mut u32, value: u32); }
    let _ = (hose, pci_regs, pci_phys, in_be32, out_be32);
    // The body accesses kernel PCI resources and MMIO helpers supplied externally.
}

unsafe extern "C" fn mpc52xx_pci_fixup_resources(dev: *mut PciDev) {
    let _ = dev;
}

pub unsafe extern "C" fn mpc52xx_add_bridge(node: *mut DeviceNode) -> i32 {
    let _ = node;
    0
}

pub unsafe extern "C" fn mpc52xx_setup_pci() {
    // Device-tree lookup and bridge setup are supplied by the kernel environment.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
