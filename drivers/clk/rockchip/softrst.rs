// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2014 MundoReader S.L.
 * Author: Heiko Stuebner <heiko@sntech.de>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct RockchipSoftrst {
    pub rcdev: ResetControllerDev,
    pub lut: *const c_int,
    pub reg_base: *mut c_void,
    pub num_regs: c_int,
    pub num_per_reg: c_int,
    pub flags: u8,
    pub lock: SpinlockT,
}

unsafe extern "C" {
    fn spin_lock_init(lock: *mut SpinlockT);
    fn spin_lock_irqsave(lock: *mut SpinlockT, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut SpinlockT, flags: c_ulong);
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn reset_controller_register(rcdev: *mut ResetControllerDev) -> c_int;
    fn kzalloc_softrst() -> *mut RockchipSoftrst;
    fn kfree(ptr: *mut RockchipSoftrst);
    fn pr_err(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct ResetControllerDev {
    pub owner: *mut c_void,
    pub nr_resets: c_uint,
    pub ops: *const ResetControlOps,
    pub of_node: *mut DeviceNode,
}

#[repr(C)]
pub struct ResetControlOps {
    pub assert_: Option<unsafe extern "C" fn(*mut ResetControllerDev, c_ulong) -> c_int>,
    pub deassert: Option<unsafe extern "C" fn(*mut ResetControllerDev, c_ulong) -> c_int>,
}

#[repr(C)]
pub struct SpinlockT {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

pub type c_int = i32;
pub type c_uint = u32;
pub type c_ulong = usize;
pub type c_char = i8;
pub type c_void = core::ffi::c_void;

pub const ROCKCHIP_SOFTRST_HIWORD_MASK: u8 = 1;

unsafe fn bit(offset: c_int) -> u32 {
    1u32.wrapping_shl(offset as u32)
}

unsafe extern "C" fn rockchip_softrst_assert(
    rcdev: *mut ResetControllerDev,
    mut id: c_ulong,
) -> c_int {
    let softrst = (rcdev as *mut u8).sub(core::mem::offset_of!(RockchipSoftrst, rcdev))
        as *mut RockchipSoftrst;
    if !(*softrst).lut.is_null() {
        id = *(*softrst).lut.add(id);
    }

    let bank = id / (*softrst).num_per_reg as c_ulong;
    let offset = id % (*softrst).num_per_reg as c_ulong;
    let addr = (*softrst).reg_base.add(bank * 4);

    if (*softrst).flags & ROCKCHIP_SOFTRST_HIWORD_MASK != 0 {
        writel(bit(offset as c_int) | (bit(offset as c_int) << 16), addr);
    } else {
        let mut flags: c_ulong = 0;
        let mut reg: u32;
        spin_lock_irqsave(&mut (*softrst).lock, &mut flags);
        reg = readl(addr);
        writel(reg | bit(offset as c_int), addr);
        spin_unlock_irqrestore(&mut (*softrst).lock, flags);
    }
    0
}

unsafe extern "C" fn rockchip_softrst_deassert(
    rcdev: *mut ResetControllerDev,
    mut id: c_ulong,
) -> c_int {
    let softrst = (rcdev as *mut u8).sub(core::mem::offset_of!(RockchipSoftrst, rcdev))
        as *mut RockchipSoftrst;
    if !(*softrst).lut.is_null() {
        id = *(*softrst).lut.add(id);
    }

    let bank = id / (*softrst).num_per_reg as c_ulong;
    let offset = id % (*softrst).num_per_reg as c_ulong;
    let addr = (*softrst).reg_base.add(bank * 4);

    if (*softrst).flags & ROCKCHIP_SOFTRST_HIWORD_MASK != 0 {
        writel(bit(offset as c_int) << 16, addr);
    } else {
        let mut flags: c_ulong = 0;
        let mut reg: u32;
        spin_lock_irqsave(&mut (*softrst).lock, &mut flags);
        reg = readl(addr);
        writel(reg & !bit(offset as c_int), addr);
        spin_unlock_irqrestore(&mut (*softrst).lock, flags);
    }
    0
}

static ROCKCHIP_SOFTRST_OPS: ResetControlOps = ResetControlOps {
    assert_: Some(rockchip_softrst_assert),
    deassert: Some(rockchip_softrst_deassert),
};

pub unsafe extern "C" fn rockchip_register_softrst_lut(
    np: *mut DeviceNode,
    lookup_table: *const c_int,
    num_regs: c_uint,
    base: *mut c_void,
    flags: u8,
) {
    let softrst = kzalloc_softrst();
    if softrst.is_null() {
        return;
    }

    spin_lock_init(&mut (*softrst).lock);
    (*softrst).reg_base = base;
    (*softrst).lut = lookup_table;
    (*softrst).flags = flags;
    (*softrst).num_regs = num_regs as c_int;
    (*softrst).num_per_reg = if flags & ROCKCHIP_SOFTRST_HIWORD_MASK != 0 { 16 } else { 32 };
    (*softrst).rcdev.owner = core::ptr::null_mut();
    (*softrst).rcdev.nr_resets = if !lookup_table.is_null() {
        num_regs
    } else {
        num_regs * (*softrst).num_per_reg as c_uint
    };
    (*softrst).rcdev.ops = &ROCKCHIP_SOFTRST_OPS;
    (*softrst).rcdev.of_node = np;
    let ret = reset_controller_register(&mut (*softrst).rcdev);
    if ret != 0 {
        kfree(softrst);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
