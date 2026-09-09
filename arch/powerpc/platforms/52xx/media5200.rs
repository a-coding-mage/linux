// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Support for 'media5200-platform' compatible boards.
 *
 * Copyright (C) 2008 Secret Lab Technologies Ltd.
 *
 * Description:
 * This code implements support for the Freescape Media5200 platform
 * (built around the MPC5200 SoC).
 *
 * Notable characteristic of the Media5200 is the presence of an FPGA
 * that has all external IRQ lines routed through it.  This file implements
 * a cascaded interrupt controller driver which attaches itself to the
 * Virtual IRQ subsystem after the primary mpc5200 interrupt controller
 * is initialized.
 */

// C build-time DEBUG is undefined in the source.

use core::ffi::c_void;

const MEDIA5200_IRQ_ENABLE: usize = 0x40c;
const MEDIA5200_IRQ_STATUS: usize = 0x410;
const MEDIA5200_NUM_IRQS: usize = 6;
const MEDIA5200_IRQ_SHIFT: usize = 32 - MEDIA5200_NUM_IRQS;

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const u8,
}

#[repr(C)]
pub struct Media5200Irq {
    pub regs: *mut c_void,
    pub lock: Spinlock,
    pub irqhost: *mut IrqDomain,
}

#[repr(C)]
pub struct Spinlock {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct IrqDomain {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct IrqData {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct IrqDesc {
    pub lock: RawSpinlock,
    pub irq_data: IrqData,
}

#[repr(C)]
pub struct RawSpinlock {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct IrqChip {
    pub name: *const u8,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut IrqData)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut IrqData)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut IrqData)>,
}

#[repr(C)]
pub struct IrqDomainOps {
    pub map: Option<unsafe extern "C" fn(*mut IrqDomain, u32, usize) -> i32>,
    pub xlate: Option<unsafe extern "C" fn(*mut IrqDomain, *mut DeviceNode, *const u32, u32, *mut usize, *mut u32) -> i32>,
}

#[repr(C)]
pub struct DeviceNode {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Mpc52xxGpio {
    pub port_config: u32,
}

#[repr(C)]
pub struct PpcMd {
    pub progress: Option<unsafe extern "C" fn(*const u8, u32)>,
}

extern "C" {
    static mut media5200_irq: Media5200Irq;
    static mut ppc_md: PpcMd;

    fn in_be32(addr: *const u32) -> u32;
    fn out_be32(addr: *mut u32, value: u32);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn raw_spin_lock(lock: *mut RawSpinlock);
    fn raw_spin_unlock(lock: *mut RawSpinlock);
    fn irqd_to_hwirq(data: *const IrqData) -> usize;
    fn irqd_irq_disabled(data: *const IrqData) -> bool;
    fn irq_desc_get_chip(desc: *mut IrqDesc) -> *mut IrqChip;
    fn generic_handle_domain_irq(domain: *mut IrqDomain, hwirq: usize);
    fn irq_set_chip_data(virq: u32, data: *mut c_void);
    fn irq_set_chip_and_handler(virq: u32, chip: *const IrqChip, handler: unsafe extern "C" fn(*mut IrqData));
    fn irq_set_status_flags(virq: u32, flags: u32);
    fn handle_level_irq(data: *mut IrqData);
    fn mpc52xx_init_irq();
    fn of_find_compatible_node(from: *mut DeviceNode, ty: *const u8, compatible: *const u8) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: i32) -> *mut c_void;
    fn irq_of_parse_and_map(node: *mut DeviceNode, index: i32) -> i32;
    fn spin_lock_init(lock: *mut Spinlock);
    fn of_fwnode_handle(node: *mut DeviceNode) -> *mut c_void;
    fn irq_domain_create_linear(fwnode: *mut c_void, size: usize, ops: *const IrqDomainOps, host_data: *mut Media5200Irq) -> *mut IrqDomain;
    fn of_node_put(node: *mut DeviceNode);
    fn irq_set_chained_handler_and_data(virq: i32, handler: unsafe extern "C" fn(*mut IrqDesc), data: *mut c_void);
    fn mpc52xx_map_common_devices();
    fn mpc5200_setup_xlb_arbiter();
    fn of_find_matching_node(from: *mut DeviceNode, matches: *const OfDeviceId) -> *mut DeviceNode;
    fn printk(fmt: *const u8, ...);
    fn iounmap(addr: *mut c_void);
    fn mpc52xx_setup_pci();
    fn mpc52xx_declare_of_platform_devices();
    fn mpc52xx_get_irq() -> i32;
    fn mpc52xx_restart();
}

#[no_mangle]
pub static mut mpc5200_gpio_ids: [OfDeviceId; 3] = [
    OfDeviceId { compatible: b"fsl,mpc5200-gpio\0".as_ptr() },
    OfDeviceId { compatible: b"mpc5200-gpio\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe extern "C" fn media5200_irq_unmask(d: *mut IrqData) {
    let mut flags = 0usize;
    spin_lock_irqsave(&mut media5200_irq.lock, &mut flags);
    let addr = (media5200_irq.regs as *mut u8).add(MEDIA5200_IRQ_ENABLE) as *mut u32;
    let mut val = in_be32(addr);
    val |= 1u32 << (MEDIA5200_IRQ_SHIFT + irqd_to_hwirq(d));
    out_be32(addr, val);
    spin_unlock_irqrestore(&mut media5200_irq.lock, flags);
}

unsafe extern "C" fn media5200_irq_mask(d: *mut IrqData) {
    let mut flags = 0usize;
    spin_lock_irqsave(&mut media5200_irq.lock, &mut flags);
    let addr = (media5200_irq.regs as *mut u8).add(MEDIA5200_IRQ_ENABLE) as *mut u32;
    let mut val = in_be32(addr);
    val &= !(1u32 << (MEDIA5200_IRQ_SHIFT + irqd_to_hwirq(d)));
    out_be32(addr, val);
    spin_unlock_irqrestore(&mut media5200_irq.lock, flags);
}

static mut MEDIA5200_IRQ_CHIP: IrqChip = IrqChip {
    name: b"Media5200 FPGA\0".as_ptr(),
    irq_unmask: Some(media5200_irq_unmask),
    irq_mask: Some(media5200_irq_mask),
    irq_mask_ack: Some(media5200_irq_mask),
};

unsafe extern "C" fn media5200_irq_cascade(desc: *mut IrqDesc) {
    let chip = irq_desc_get_chip(desc);
    raw_spin_lock(&mut (*desc).lock);
    ((*chip).irq_mask.unwrap())(&mut (*desc).irq_data);
    raw_spin_unlock(&mut (*desc).lock);

    let enable_addr = (media5200_irq.regs as *mut u8).add(MEDIA5200_IRQ_ENABLE) as *const u32;
    let status_addr = (media5200_irq.regs as *mut u8).add(MEDIA5200_IRQ_STATUS) as *const u32;
    let status = in_be32(enable_addr);
    let enable = in_be32(status_addr);
    let pending = (status & enable) >> MEDIA5200_IRQ_SHIFT;
    let val = if pending == 0 { 0 } else { pending.trailing_zeros() + 1 };
    if val != 0 {
        generic_handle_domain_irq(media5200_irq.irqhost, (val - 1) as usize);
    }

    raw_spin_lock(&mut (*desc).lock);
    ((*chip).irq_mask_ack.unwrap())(&mut (*desc).irq_data);
    if !irqd_irq_disabled(&(*desc).irq_data) {
        ((*chip).irq_unmask.unwrap())(&mut (*desc).irq_data);
    }
    raw_spin_unlock(&mut (*desc).lock);
}

unsafe extern "C" fn media5200_irq_map(_h: *mut IrqDomain, virq: u32, _hw: usize) -> i32 {
    irq_set_chip_data(virq, &mut media5200_irq as *mut _ as *mut c_void);
    irq_set_chip_and_handler(virq, &MEDIA5200_IRQ_CHIP, handle_level_irq);
    irq_set_status_flags(virq, 1);
    0
}

unsafe extern "C" fn media5200_irq_xlate(_h: *mut IrqDomain, _ct: *mut DeviceNode, intspec: *const u32, intsize: u32, out_hwirq: *mut usize, out_flags: *mut u32) -> i32 {
    if intsize != 2 { return -1; }
    *out_hwirq = *intspec.add(1) as usize;
    *out_flags = 0;
    0
}

static MEDIA5200_IRQ_OPS: IrqDomainOps = IrqDomainOps { map: Some(media5200_irq_map), xlate: Some(media5200_irq_xlate) };

unsafe extern "C" fn media5200_init_irq() {
    mpc52xx_init_irq();
    let fpga_np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,media5200-fpga\0".as_ptr());
    if fpga_np.is_null() { printk(b"Could not find Media5200 FPGA; PCI interrupts will not work\n\0".as_ptr()); return; }
    media5200_irq.regs = of_iomap(fpga_np, 0);
    if media5200_irq.regs.is_null() { of_node_put(fpga_np); return; }
    let cascade_virq = irq_of_parse_and_map(fpga_np, 0);
    if cascade_virq == 0 { of_node_put(fpga_np); return; }
    out_be32((media5200_irq.regs as *mut u8).add(MEDIA5200_IRQ_ENABLE) as *mut u32, 0);
    spin_lock_init(&mut media5200_irq.lock);
    media5200_irq.irqhost = irq_domain_create_linear(of_fwnode_handle(fpga_np), MEDIA5200_NUM_IRQS, &MEDIA5200_IRQ_OPS, &mut media5200_irq);
    if media5200_irq.irqhost.is_null() { of_node_put(fpga_np); return; }
    of_node_put(fpga_np);
    irq_set_chained_handler_and_data(cascade_virq, media5200_irq_cascade, &mut media5200_irq as *mut _ as *mut c_void);
}

unsafe extern "C" fn media5200_setup_arch() {
    if let Some(progress) = ppc_md.progress { progress(b"media5200_setup_arch()\0".as_ptr(), 0); }
    mpc52xx_map_common_devices();
    mpc5200_setup_xlb_arbiter();
    let np = of_find_matching_node(core::ptr::null_mut(), mpc5200_gpio_ids.as_ptr());
    let gpio = of_iomap(np, 0) as *mut Mpc52xxGpio;
    of_node_put(np);
    if gpio.is_null() { printk(b"%s() failed. expect abnormal behavior\n\0".as_ptr()); return; }
    let mut port_config = in_be32(&(*gpio).port_config);
    port_config &= !0x03000000;
    port_config |= 0x01000000;
    out_be32(&mut (*gpio).port_config, port_config);
    iounmap(gpio as *mut c_void);
}

#[repr(C)]
pub struct MachineDesc {
    pub name: *const u8,
    pub compatible: *const u8,
    pub setup_arch: Option<unsafe extern "C" fn()>,
    pub discover_phbs: Option<unsafe extern "C" fn()>,
    pub init: Option<unsafe extern "C" fn()>,
    pub init_irq: Option<unsafe extern "C" fn()>,
    pub get_irq: Option<unsafe extern "C" fn() -> i32>,
    pub restart: Option<unsafe extern "C" fn()>,
}

#[no_mangle]
pub static MEDIA5200_PLATFORM: MachineDesc = MachineDesc {
    name: b"media5200-platform\0".as_ptr(),
    compatible: b"fsl,media5200\0".as_ptr(),
    setup_arch: Some(media5200_setup_arch),
    discover_phbs: Some(mpc52xx_setup_pci),
    init: Some(mpc52xx_declare_of_platform_devices),
    init_irq: Some(media5200_init_irq),
    get_irq: Some(mpc52xx_get_irq),
    restart: Some(mpc52xx_restart),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
