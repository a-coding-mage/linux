// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Clock driver for Keystone 2 based devices
 *
 * Copyright (C) 2013 Texas Instruments.
 *	Murali Karicheri <m-karicheri2@ti.com>
 *	Santosh Shilimkar <santosh.shilimkar@ti.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const PTCMD: usize = 0x120;
const PTSTAT: usize = 0x128;
const PDSTAT: usize = 0x200;
const PDCTL: usize = 0x300;
const MDSTAT: usize = 0x800;
const MDCTL: usize = 0xa00;

const PSC_STATE_SWRSTDISABLE: u32 = 0;
const PSC_STATE_SYNCRST: u32 = 1;
const PSC_STATE_DISABLE: u32 = 2;
const PSC_STATE_ENABLE: u32 = 3;

const MDSTAT_STATE_MASK: u32 = 0x3f;
const MDSTAT_MCKOUT: u32 = 1 << 12;
const PDSTAT_STATE_MASK: u32 = 0x1f;
const MDCTL_FORCE: u32 = 1 << 31;
const MDCTL_LRESET: u32 = 1 << 8;
const PDCTL_NEXT: u32 = 1 << 0;

const STATE_TRANS_MAX_COUNT: u32 = 0xffff;

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk_init_data {
    pub name: *const core::ffi::c_char,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: u8,
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    pub name: *const core::ffi::c_char,
}
#[repr(C)]
pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
}
pub type spinlock_t = core::ffi::c_ulong;
pub type c_void = core::ffi::c_void;

extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut core::ffi::c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: core::ffi::c_ulong);
    fn clk_register(dev: *mut device, hw: *mut clk_hw) -> *mut clk;
    fn of_property_match_string(node: *mut device_node, propname: *const core::ffi::c_char,
                                value: *const core::ffi::c_char) -> i32;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut c_void;
    fn of_property_read_u32(node: *mut device_node, propname: *const core::ffi::c_char,
                            out_value: *mut u32) -> i32;
    fn of_property_read_string(node: *mut device_node, propname: *const core::ffi::c_char,
                               out_string: *mut *const core::ffi::c_char) -> i32;
    fn of_clk_get_parent_name(node: *mut device_node, index: i32) -> *const core::ffi::c_char;
    fn of_clk_add_provider(node: *mut device_node, get: *const c_void, data: *mut clk) -> i32;
    fn iounmap(addr: *mut c_void);
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn is_err(ptr: *mut clk) -> bool;
    fn err_ptr(error: i32) -> *mut clk;
}

static mut domain_transition_base: *mut c_void = core::ptr::null_mut();
static mut psc_lock: spinlock_t = 0;

#[repr(C)]
pub struct clk_psc_data {
    pub control_base: *mut c_void,
    pub domain_base: *mut c_void,
    pub domain_id: u32,
}

#[repr(C)]
pub struct clk_psc {
    pub hw: clk_hw,
    pub psc_data: *mut clk_psc_data,
    pub lock: *mut spinlock_t,
}

unsafe fn psc_config(control_base: *mut c_void, domain_base: *mut c_void,
                     next_state: u32, domain_id: u32) {
    let mut mdctl: u32;
    let mut ptstat: u32;
    let mut count = STATE_TRANS_MAX_COUNT;

    mdctl = readl(control_base.add(MDCTL));
    mdctl &= !MDSTAT_STATE_MASK;
    mdctl |= next_state;
    /* For disable, we always put the module in local reset */
    if next_state == PSC_STATE_DISABLE {
        mdctl &= !MDCTL_LRESET;
    }
    writel(mdctl, control_base.add(MDCTL));

    let pdstat = readl(domain_base.add(PDSTAT));
    if (pdstat & PDSTAT_STATE_MASK) == 0 {
        let mut pdctl = readl(domain_base.add(PDCTL));
        pdctl |= PDCTL_NEXT;
        writel(pdctl, domain_base.add(PDCTL));
    }

    let ptcmd = 1u32 << domain_id;
    writel(ptcmd, domain_transition_base.add(PTCMD));
    loop {
        ptstat = readl(domain_transition_base.add(PTSTAT));
        if !(((ptstat >> domain_id) & 1) != 0 && { let old = count; count = count.wrapping_sub(1); old != 0 }) {
            break;
        }
    }

    count = STATE_TRANS_MAX_COUNT;
    loop {
        let mdstat = readl(control_base.add(MDSTAT));
        if !(!((mdstat & MDSTAT_STATE_MASK) == next_state) && { let old = count; count = count.wrapping_sub(1); old != 0 }) {
            break;
        }
    }
}

unsafe extern "C" fn keystone_clk_is_enabled(hw: *mut clk_hw) -> i32 {
    let psc = hw as *mut clk_psc;
    let data = (*psc).psc_data;
    let mdstat = readl((*data).control_base.add(MDSTAT));
    if (mdstat & MDSTAT_MCKOUT) != 0 { 1 } else { 0 }
}

unsafe extern "C" fn keystone_clk_enable(hw: *mut clk_hw) -> i32 {
    let psc = hw as *mut clk_psc;
    let data = (*psc).psc_data;
    let mut flags: core::ffi::c_ulong = 0;
    if !(*psc).lock.is_null() { spin_lock_irqsave((*psc).lock, &mut flags); }
    psc_config((*data).control_base, (*data).domain_base, PSC_STATE_ENABLE, (*data).domain_id);
    if !(*psc).lock.is_null() { spin_unlock_irqrestore((*psc).lock, flags); }
    0
}

unsafe extern "C" fn keystone_clk_disable(hw: *mut clk_hw) {
    let psc = hw as *mut clk_psc;
    let data = (*psc).psc_data;
    let mut flags: core::ffi::c_ulong = 0;
    if !(*psc).lock.is_null() { spin_lock_irqsave((*psc).lock, &mut flags); }
    psc_config((*data).control_base, (*data).domain_base, PSC_STATE_DISABLE, (*data).domain_id);
    if !(*psc).lock.is_null() { spin_unlock_irqrestore((*psc).lock, flags); }
}

static clk_psc_ops: clk_ops = clk_ops {
    enable: Some(keystone_clk_enable),
    disable: Some(keystone_clk_disable),
    is_enabled: Some(keystone_clk_is_enabled),
};

unsafe fn clk_register_psc(_dev: *mut device, name: *const core::ffi::c_char,
                           parent_name: *const core::ffi::c_char,
                           psc_data: *mut clk_psc_data, lock: *mut spinlock_t) -> *mut clk {
    let psc = kzalloc(core::mem::size_of::<clk_psc>(), 0) as *mut clk_psc;
    if psc.is_null() { return err_ptr(-12); }
    let mut init = clk_init_data {
        name,
        ops: &clk_psc_ops,
        flags: 0,
        parent_names: if !parent_name.is_null() { &parent_name } else { core::ptr::null() },
        num_parents: if !parent_name.is_null() { 1 } else { 0 },
    };
    (*psc).psc_data = psc_data;
    (*psc).lock = lock;
    // clk_hw.init points at the stack-local initialization data during registration.
    let hw_init = (&mut (*psc).hw as *mut clk_hw).add(1) as *mut *mut clk_init_data;
    *hw_init = &mut init;
    let clk = clk_register(core::ptr::null_mut(), &mut (*psc).hw);
    if is_err(clk) { kfree(psc as *mut c_void); }
    clk
}

unsafe extern "C" fn of_psc_clk_init(node: *mut device_node, lock: *mut spinlock_t) {
    let mut clk_name = (*node).name;
    let mut parent_name: *const core::ffi::c_char;
    let data = kzalloc(core::mem::size_of::<clk_psc_data>(), 0) as *mut clk_psc_data;
    if data.is_null() { return; }
    let control = b"control\0".as_ptr() as *const core::ffi::c_char;
    let domain = b"domain\0".as_ptr() as *const core::ffi::c_char;
    let domain_id = b"domain-id\0".as_ptr() as *const core::ffi::c_char;
    let output_names = b"clock-output-names\0".as_ptr() as *const core::ffi::c_char;
    let i = of_property_match_string(node, b"reg-names\0".as_ptr() as _, control);
    (*data).control_base = of_iomap(node, i);
    if (*data).control_base.is_null() { kfree(data as _); return; }
    let i = of_property_match_string(node, b"reg-names\0".as_ptr() as _, domain);
    (*data).domain_base = of_iomap(node, i);
    if (*data).domain_base.is_null() { iounmap((*data).control_base); kfree(data as _); return; }
    of_property_read_u32(node, domain_id, &mut (*data).domain_id);
    if domain_transition_base.is_null() && (*data).domain_id == 0 { domain_transition_base = (*data).domain_base; }
    of_property_read_string(node, output_names, &mut clk_name);
    parent_name = of_clk_get_parent_name(node, 0);
    if parent_name.is_null() { iounmap((*data).domain_base); iounmap((*data).control_base); kfree(data as _); return; }
    let clk = clk_register_psc(core::ptr::null_mut(), clk_name, parent_name, data, lock);
    if !is_err(clk) { return; }
    iounmap((*data).domain_base); iounmap((*data).control_base); kfree(data as _);
}

unsafe extern "C" fn of_keystone_psc_clk_init(node: *mut device_node) {
    of_psc_clk_init(node, &raw mut psc_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
