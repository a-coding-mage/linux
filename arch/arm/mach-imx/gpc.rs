// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011-2013 Freescale Semiconductor, Inc.
 * Copyright 2011 Linaro Ltd.
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const GPC_CNTR: usize = 0x0;
const GPC_IMR1: usize = 0x008;
const GPC_PGC_CPU_PDN: usize = 0x2a0;
const GPC_PGC_CPU_PUPSCR: usize = 0x2a4;
const GPC_PGC_CPU_PDNSCR: usize = 0x2a8;
const GPC_PGC_SW2ISO_SHIFT: u32 = 0x8;
const GPC_PGC_SW_SHIFT: u32 = 0x0;
const GPC_CNTR_L2_PGE_SHIFT: u32 = 22;
const IMR_NUM: usize = 4;
const GPC_MAX_IRQS: usize = IMR_NUM * 32;

type U32 = u32;

#[repr(C)]
pub struct irq_data { pub hwirq: u32 }
#[repr(C)]
pub struct irq_chip {
    pub name: *const u8,
    pub irq_eoi: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_retrigger: Option<unsafe extern "C" fn(*mut irq_data) -> i32>,
    pub irq_set_wake: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>,
    pub irq_set_type: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>,
    #[cfg(feature = "SMP")]
    pub irq_set_affinity: Option<unsafe extern "C" fn(*mut irq_data, *const u8, bool) -> i32>,
}
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct irq_domain { pub parent: *mut irq_domain }
#[repr(C)] pub struct irq_fwnode;
#[repr(C)] pub struct irq_fwspec { pub fwnode: *mut irq_fwnode, pub param_count: u32, pub param: [u32; 3] }
extern "C" {
    static mut gpc_base: *mut u8;
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn irq_chip_unmask_parent(d: *mut irq_data);
    fn irq_chip_mask_parent(d: *mut irq_data);
    fn irq_chip_eoi_parent(d: *mut irq_data);
    fn irq_chip_retrigger_hierarchy(d: *mut irq_data) -> i32;
    fn irq_chip_set_type_parent(d: *mut irq_data, ty: u32) -> i32;
    fn irq_domain_set_hwirq_and_chip(d: *mut irq_domain, irq: u32, hwirq: u32, chip: *mut irq_chip, data: *mut u8);
    fn irq_domain_alloc_irqs_parent(d: *mut irq_domain, irq: u32, nr: u32, fwspec: *mut irq_fwspec) -> i32;
    fn irq_domain_free_irqs_common(d: *mut irq_domain, irq: u32, nr: u32);
    fn irq_find_host(node: *mut device_node) -> *mut irq_domain;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn of_fwnode_handle(node: *mut device_node) -> *mut irq_fwnode;
    fn of_node_clear_flag(node: *mut device_node, flag: u32);
    fn of_find_compatible_node(from: *mut device_node, ty: *const u8, compatible: *const u8) -> *mut device_node;
    fn of_property_read_bool(node: *mut device_node, property: *const u8) -> bool;
    fn of_node_put(node: *mut device_node);
}

static mut GPC_WAKE_IRQS: [U32; IMR_NUM] = [0; IMR_NUM];
static mut GPC_SAVED_IMRS: [U32; IMR_NUM] = [0; IMR_NUM];

#[inline] unsafe fn reg(offset: usize) -> *mut u8 { gpc_base.add(offset) }

#[no_mangle] pub unsafe extern "C" fn imx_gpc_set_arm_power_up_timing(sw2iso: u32, sw: u32) {
    writel_relaxed((sw2iso << GPC_PGC_SW2ISO_SHIFT) | (sw << GPC_PGC_SW_SHIFT), reg(GPC_PGC_CPU_PUPSCR));
}
#[no_mangle] pub unsafe extern "C" fn imx_gpc_set_arm_power_down_timing(sw2iso: u32, sw: u32) {
    writel_relaxed((sw2iso << GPC_PGC_SW2ISO_SHIFT) | (sw << GPC_PGC_SW_SHIFT), reg(GPC_PGC_CPU_PDNSCR));
}
#[no_mangle] pub unsafe extern "C" fn imx_gpc_set_arm_power_in_lpm(power_off: bool) { writel_relaxed(power_off as u32, reg(GPC_PGC_CPU_PDN)); }
#[no_mangle] pub unsafe extern "C" fn imx_gpc_set_l2_mem_power_in_lpm(power_off: bool) {
    let mut val = readl_relaxed(reg(GPC_CNTR)); val &= !(1u32 << GPC_CNTR_L2_PGE_SHIFT);
    if power_off { val |= 1u32 << GPC_CNTR_L2_PGE_SHIFT; } writel_relaxed(val, reg(GPC_CNTR));
}
#[no_mangle] pub unsafe extern "C" fn imx_gpc_pre_suspend(arm_power_off: bool) {
    if arm_power_off { imx_gpc_set_arm_power_in_lpm(arm_power_off); }
    for i in 0..IMR_NUM { GPC_SAVED_IMRS[i] = readl_relaxed(reg(GPC_IMR1 + i * 4)); writel_relaxed(!GPC_WAKE_IRQS[i], reg(GPC_IMR1 + i * 4)); }
}
#[no_mangle] pub unsafe extern "C" fn imx_gpc_post_resume() {
    imx_gpc_set_arm_power_in_lpm(false);
    for i in 0..IMR_NUM { writel_relaxed(GPC_SAVED_IMRS[i], reg(GPC_IMR1 + i * 4)); }
}
unsafe extern "C" fn imx_gpc_irq_set_wake(d: *mut irq_data, on: u32) -> i32 {
    let idx = ((*d).hwirq / 32) as usize; let mask = 1u32 << ((*d).hwirq % 32);
    GPC_WAKE_IRQS[idx] = if on != 0 { GPC_WAKE_IRQS[idx] | mask } else { GPC_WAKE_IRQS[idx] & !mask }; 0
}
#[no_mangle] pub unsafe extern "C" fn imx_gpc_mask_all() { for i in 0..IMR_NUM { GPC_SAVED_IMRS[i] = readl_relaxed(reg(GPC_IMR1+i*4)); writel_relaxed(!0, reg(GPC_IMR1+i*4)); } }
#[no_mangle] pub unsafe extern "C" fn imx_gpc_restore_all() { for i in 0..IMR_NUM { writel_relaxed(GPC_SAVED_IMRS[i], reg(GPC_IMR1+i*4)); } }
#[no_mangle] pub unsafe extern "C" fn imx_gpc_hwirq_unmask(hwirq: u32) { let p=reg(GPC_IMR1+(hwirq/32) as usize*4); let mut v=readl_relaxed(p); v &= !(1 << (hwirq%32)); writel_relaxed(v,p); }
#[no_mangle] pub unsafe extern "C" fn imx_gpc_hwirq_mask(hwirq: u32) { let p=reg(GPC_IMR1+(hwirq/32) as usize*4); let mut v=readl_relaxed(p); v |= 1 << (hwirq%32); writel_relaxed(v,p); }

unsafe extern "C" fn imx_gpc_irq_unmask(d: *mut irq_data) { imx_gpc_hwirq_unmask((*d).hwirq); irq_chip_unmask_parent(d); }
unsafe extern "C" fn imx_gpc_irq_mask(d: *mut irq_data) { imx_gpc_hwirq_mask((*d).hwirq); irq_chip_mask_parent(d); }
static mut IMX_GPC_CHIP: irq_chip = irq_chip { name: b"GPC\0".as_ptr(), irq_eoi: Some(irq_chip_eoi_parent), irq_mask: Some(imx_gpc_irq_mask), irq_unmask: Some(imx_gpc_irq_unmask), irq_retrigger: Some(irq_chip_retrigger_hierarchy), irq_set_wake: Some(imx_gpc_irq_set_wake), irq_set_type: Some(irq_chip_set_type_parent), #[cfg(feature="SMP")] irq_set_affinity: None };

unsafe fn imx_gpc_domain_translate(_d: *mut irq_domain, f: *mut irq_fwspec, h: *mut usize, ty: *mut u32) -> i32 { if (*f).param_count != 3 || (*f).param[0] != 0 { return -22; } *h=(*f).param[1] as usize; *ty=(*f).param[2]; 0 }
unsafe fn imx_gpc_domain_alloc(d: *mut irq_domain, irq: u32, nr: u32, f: *mut irq_fwspec) -> i32 { if (*f).param_count != 3 || (*f).param[0] != 0 || (*f).param[1] as usize >= GPC_MAX_IRQS { return -22; } for i in 0..nr { irq_domain_set_hwirq_and_chip(d,irq+i,(*f).param[1]+i,&mut IMX_GPC_CHIP,core::ptr::null_mut()); } irq_domain_alloc_irqs_parent(d,irq,nr,f) }
#[no_mangle] pub unsafe extern "C" fn imx_gpc_init(node: *mut device_node, parent: *mut device_node) -> i32 { if parent.is_null() { return -19; } if irq_find_host(parent).is_null() { return -6; } gpc_base=of_iomap(node,0); if gpc_base.is_null() { return -12; } for i in 0..IMR_NUM { writel_relaxed(!0,reg(GPC_IMR1+i*4)); } of_node_clear_flag(node,1); 0 }
#[no_mangle] pub unsafe extern "C" fn imx_gpc_check_dt() { let np=of_find_compatible_node(core::ptr::null_mut(),core::ptr::null(),b"fsl,imx6q-gpc\0".as_ptr()); if np.is_null() { return; } if !of_property_read_bool(np,b"interrupt-controller\0".as_ptr()) { gpc_base=of_iomap(np,0); } of_node_put(np); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
