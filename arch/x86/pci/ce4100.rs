// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright(c) 2010 Intel Corporation. All rights reserved.
 *
 *  Contact Information:
 *    Intel Corporation
 *    2200 Mission College Blvd.
 *    Santa Clara, CA  97052
 *
 * This provides access methods for PCI registers that mis-behave on
 * the CE4100. Each register can be assigned a private init, read and
 * write routine. The exception to this is the bridge device.  The
 * bridge device is the only device on bus zero (0) that requires any
 * fixup so it is a special case ATM
 */

// External kernel/architecture symbols supplied by other translation units.
extern "C" {
    static mut pci_direct_conf1: PciRawOps;
    static mut raw_pci_ops: *const PciRawOps;
    static mut pci_config_lock: RawSpinLock;
    fn raw_spin_lock_irqsave(lock: *mut RawSpinLock, flags: *mut u64);
    fn raw_spin_unlock_irqrestore(lock: *mut RawSpinLock, flags: u64);
    fn warn_on(condition: bool);
}

#[repr(C)]
pub struct RawSpinLock {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PciRawOps {
    pub read: unsafe extern "C" fn(u32, u32, u32, i32, i32, *mut u32) -> i32,
    pub write: unsafe extern "C" fn(u32, u32, u32, i32, i32, u32) -> i32,
}

#[repr(C)]
pub struct SimReg {
    value: u32,
    mask: u32,
}

#[repr(C)]
pub struct SimDevReg {
    dev_func: i32,
    reg: i32,
    init: Option<unsafe extern "C" fn(*mut SimDevReg)>,
    read: Option<unsafe extern "C" fn(*mut SimDevReg, *mut u32)>,
    write: Option<unsafe extern "C" fn(*mut SimDevReg, u32)>,
    sim_reg: SimReg,
}

const MB: u32 = 1024 * 1024;
const KB: u32 = 1024;
const fn size_to_mask(size: u32) -> u32 { !(size - 1) }
const fn pci_devfn(device: i32, func: i32) -> i32 { (device << 3) | func }

unsafe extern "C" fn reg_init(reg: *mut SimDevReg) {
    ((*pci_direct_conf1.read)(0, 1, (*reg).dev_func as u32, (*reg).reg, 4,
        &mut (*reg).sim_reg.value));
}

unsafe extern "C" fn reg_read(reg: *mut SimDevReg, value: *mut u32) {
    *value = (*reg).sim_reg.value;
}

unsafe extern "C" fn reg_write(reg: *mut SimDevReg, value: u32) {
    (*reg).sim_reg.value = (value & (*reg).sim_reg.mask)
        | ((*reg).sim_reg.value & !(*reg).sim_reg.mask);
}

unsafe extern "C" fn sata_reg_init(reg: *mut SimDevReg) {
    ((*pci_direct_conf1.read)(0, 1, pci_devfn(14, 0) as u32, 0x10, 4,
        &mut (*reg).sim_reg.value));
    (*reg).sim_reg.value = (*reg).sim_reg.value.wrapping_add(0x400);
}

unsafe extern "C" fn ehci_reg_read(reg: *mut SimDevReg, value: *mut u32) {
    reg_read(reg, value);
    if *value != (*reg).sim_reg.mask { *value |= 0x100; }
}

unsafe extern "C" fn sata_revid_init(reg: *mut SimDevReg) {
    (*reg).sim_reg.value = 0x01060100;
    (*reg).sim_reg.mask = 0;
}

unsafe extern "C" fn sata_revid_read(reg: *mut SimDevReg, value: *mut u32) { reg_read(reg, value); }

unsafe extern "C" fn reg_noirq_read(reg: *mut SimDevReg, value: *mut u32) {
    // force interrupt pin value to 0
    *value = (*reg).sim_reg.value & 0xfff00ff;
}

macro_rules! define_reg {
    ($d:expr, $f:expr, $r:expr, $s:expr, $i:expr, $rd:expr, $w:expr) => {
        SimDevReg { dev_func: pci_devfn($d, $f), reg: $r, init: $i, read: $rd,
            write: $w, sim_reg: SimReg { value: 0, mask: size_to_mask($s) } }
    };
}

static mut bus1_fixups: [SimDevReg; 51] = [
    define_reg!(2,0,0x10,16*MB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(2,0,0x14,256,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(2,1,0x10,64*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(3,0,0x10,64*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(4,0,0x10,128*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(4,1,0x10,128*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(6,0,0x10,512*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(6,1,0x10,512*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(6,2,0x10,64*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(8,0,0x10,MB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(8,1,0x10,64*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(8,2,0x10,64*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(9,0,0x10,MB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(9,0,0x14,64*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(10,0,0x10,256,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(10,0,0x14,256*MB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(11,0,0x10,256,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(11,0,0x14,256,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(11,1,0x10,256,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(11,2,0x10,256,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(11,2,0x14,256,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(11,2,0x18,256,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(11,3,0x10,256,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(11,3,0x14,256,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(11,4,0x10,256,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(11,5,0x10,64*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(11,6,0x10,256,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(11,7,0x10,64*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(11,7,0x3c,256,Some(reg_init),Some(reg_noirq_read),Some(reg_write)),
    define_reg!(12,0,0x10,128*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(12,0,0x14,256,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(12,1,0x10,1024,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(13,0,0x10,32*KB,Some(reg_init),Some(ehci_reg_read),Some(reg_write)),
    define_reg!(13,1,0x10,32*KB,Some(reg_init),Some(ehci_reg_read),Some(reg_write)),
    define_reg!(14,0,0x8,0,Some(sata_revid_init),Some(sata_revid_read),None),
    define_reg!(14,0,0x10,0,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(14,0,0x14,0,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(14,0,0x18,0,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(14,0,0x1c,0,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(14,0,0x20,0,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(14,0,0x24,0x200,Some(sata_reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(15,0,0x10,64*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(15,0,0x14,64*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(16,0,0x10,64*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(16,0,0x14,64*MB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(16,0,0x18,64*MB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(16,0,0x3c,256,Some(reg_init),Some(reg_noirq_read),Some(reg_write)),
    define_reg!(17,0,0x10,128*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(18,0,0x10,1*KB,Some(reg_init),Some(reg_read),Some(reg_write)),
    define_reg!(18,0,0x3c,256,Some(reg_init),Some(reg_noirq_read),Some(reg_write)),
];

unsafe fn init_sim_regs() {
    for i in 0..bus1_fixups.len() {
        if let Some(init) = bus1_fixups[i].init { init(&mut bus1_fixups[i]); }
    }
}

unsafe fn extract_bytes(value: *mut u32, reg: i32, len: i32) {
    *value >>= ((reg & 3) * 8) as u32;
    let mask = 0xffff_ffffu32 >> ((4 - len) * 8) as u32;
    *value &= mask;
}

const PCI_BASE_ADDRESS_0: i32 = 0x10;
const PCI_PRIMARY_BUS: i32 = 0x18;
const PCI_SUBORDINATE_BUS: i32 = 0x1a;
const PCI_MEMORY_BASE: i32 = 0x20;
const PCI_MEMORY_LIMIT: i32 = 0x22;
const PCI_PREF_MEMORY_BASE: i32 = 0x24;
const PCI_PREF_MEMORY_LIMIT: i32 = 0x26;
const PCI_IO_BASE: i32 = 0x1c;
const PCI_IO_LIMIT: i32 = 0x1d;

unsafe fn bridge_read(devfn: u32, reg: i32, len: i32, value: *mut u32) -> i32 {
    let mut av_bridge_base = 0u32;
    let mut av_bridge_limit: u32;
    match reg {
        0x10 | 0x11 | 0x12 | 0x13 => *value = 0,
        PCI_PRIMARY_BUS => if len == 4 { *value = 0x00010100 },
        PCI_SUBORDINATE_BUS => *value = 1,
        PCI_MEMORY_BASE | PCI_MEMORY_LIMIT => {
            ((*pci_direct_conf1.read)(0, 0, devfn, PCI_BASE_ADDRESS_0, 4, &mut av_bridge_base));
            av_bridge_limit = av_bridge_base.wrapping_add(512 * MB - 1);
            av_bridge_limit = (av_bridge_limit >> 16) & 0xfff0;
            av_bridge_base = (av_bridge_base >> 16) & 0xfff0;
            if reg == PCI_MEMORY_LIMIT { *value = av_bridge_limit; }
            else if len == 2 { *value = av_bridge_base; }
            else { *value = (av_bridge_limit << 16) | av_bridge_base; }
        }
        PCI_PREF_MEMORY_BASE => *value = 0xfff0,
        PCI_PREF_MEMORY_LIMIT => *value = 0,
        PCI_IO_BASE => *value = 0xf0,
        PCI_IO_LIMIT => *value = 0,
        _ => return 1,
    }
    0
}

unsafe fn ce4100_bus1_read(devfn: u32, reg: i32, len: i32, value: *mut u32) -> i32 {
    for i in 0..bus1_fixups.len() {
        let item = &mut bus1_fixups[i];
        if item.dev_func as u32 == devfn && item.reg == (reg & !3) {
            if let Some(read) = item.read {
                let mut flags = 0u64;
                raw_spin_lock_irqsave(&mut pci_config_lock, &mut flags);
                read(item, value);
                raw_spin_unlock_irqrestore(&mut pci_config_lock, flags);
                extract_bytes(value, reg, len);
                return 0;
            }
        }
    }
    -1
}

unsafe fn ce4100_conf_read(seg: u32, bus: u32, devfn: u32, reg: i32, len: i32, value: *mut u32) -> i32 {
    warn_on(seg != 0);
    if bus == 1 && ce4100_bus1_read(devfn, reg, len, value) == 0 { return 0; }
    if bus == 0 && pci_devfn(1, 0) as u32 == devfn && bridge_read(devfn, reg, len, value) == 0 { return 0; }
    (pci_direct_conf1.read)(seg, bus, devfn, reg, len, value)
}

unsafe fn ce4100_bus1_write(devfn: u32, reg: i32, _len: i32, value: u32) -> i32 {
    for i in 0..bus1_fixups.len() {
        let item = &mut bus1_fixups[i];
        if item.dev_func as u32 == devfn && item.reg == (reg & !3) {
            if let Some(write) = item.write {
                let mut flags = 0u64;
                raw_spin_lock_irqsave(&mut pci_config_lock, &mut flags);
                write(item, value);
                raw_spin_unlock_irqrestore(&mut pci_config_lock, flags);
                return 0;
            }
        }
    }
    -1
}

unsafe fn ce4100_conf_write(seg: u32, bus: u32, devfn: u32, reg: i32, len: i32, value: u32) -> i32 {
    warn_on(seg != 0);
    if bus == 1 && ce4100_bus1_write(devfn, reg, len, value) == 0 { return 0; }
    // Discard writes to A/V bridge BAR.
    if bus == 0 && pci_devfn(1, 0) as u32 == devfn && (reg & !3) == PCI_BASE_ADDRESS_0 { return 0; }
    (pci_direct_conf1.write)(seg, bus, devfn, reg, len, value)
}

static ce4100_pci_conf: PciRawOps = PciRawOps { read: ce4100_conf_read, write: ce4100_conf_write };

pub unsafe fn ce4100_pci_init() -> i32 {
    init_sim_regs();
    raw_pci_ops = &ce4100_pci_conf;
    // Indicate caller that it should invoke pci_legacy_init()
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
