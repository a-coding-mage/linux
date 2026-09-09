// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Hisilicon Reset Controller Driver
 *
 * Copyright (c) 2015-2016 HiSilicon Technologies Co., Ltd.
 */

// Dependencies are supplied by the surrounding kernel translation.

const HISI_RESET_BIT_MASK: u32 = 0x1f;
const HISI_RESET_OFFSET_SHIFT: u32 = 8;
const HISI_RESET_OFFSET_MASK: u32 = 0xffff00;

#[repr(C)]
pub struct HisiResetController {
    pub lock: spinlock_t,
    pub membase: *mut core::ffi::c_void,
    pub rcdev: reset_controller_dev,
}

#[inline]
unsafe fn to_hisi_reset_controller(
    rcdev: *mut reset_controller_dev,
) -> *mut HisiResetController {
    container_of!(rcdev, HisiResetController, rcdev)
}

unsafe extern "C" fn hisi_reset_of_xlate(
    rcdev: *mut reset_controller_dev,
    reset_spec: *const of_phandle_args,
) -> i32 {
    let _ = rcdev;
    let offset: u32 = ((*reset_spec).args[0] << HISI_RESET_OFFSET_SHIFT)
        & HISI_RESET_OFFSET_MASK;
    let bit: u8 = ((*reset_spec).args[1] & HISI_RESET_BIT_MASK) as u8;

    (offset | bit as u32) as i32
}

unsafe extern "C" fn hisi_reset_assert(
    rcdev: *mut reset_controller_dev,
    id: core::ffi::c_ulong,
) -> i32 {
    let rstc = to_hisi_reset_controller(rcdev);
    let mut flags: core::ffi::c_ulong = 0;
    let offset: u32 = ((id as u32 & HISI_RESET_OFFSET_MASK) >> HISI_RESET_OFFSET_SHIFT) as u32;
    let bit: u8 = (id as u32 & HISI_RESET_BIT_MASK) as u8;

    spin_lock_irqsave(&mut (*rstc).lock, &mut flags);

    let reg: u32 = readl((*rstc).membase.add(offset as usize));
    writel(reg | (1u32 << bit), (*rstc).membase.add(offset as usize));

    spin_unlock_irqrestore(&mut (*rstc).lock, flags);

    0
}

unsafe extern "C" fn hisi_reset_deassert(
    rcdev: *mut reset_controller_dev,
    id: core::ffi::c_ulong,
) -> i32 {
    let rstc = to_hisi_reset_controller(rcdev);
    let mut flags: core::ffi::c_ulong = 0;
    let offset: u32 = ((id as u32 & HISI_RESET_OFFSET_MASK) >> HISI_RESET_OFFSET_SHIFT) as u32;
    let bit: u8 = (id as u32 & HISI_RESET_BIT_MASK) as u8;

    spin_lock_irqsave(&mut (*rstc).lock, &mut flags);

    let reg: u32 = readl((*rstc).membase.add(offset as usize));
    writel(reg & !(1u32 << bit), (*rstc).membase.add(offset as usize));

    spin_unlock_irqrestore(&mut (*rstc).lock, flags);

    0
}

#[repr(C)]
static HISI_RESET_OPS: reset_control_ops = reset_control_ops {
    assert_: Some(hisi_reset_assert),
    deassert: Some(hisi_reset_deassert),
};

pub unsafe extern "C" fn hisi_reset_init(
    pdev: *mut platform_device,
) -> *mut HisiResetController {
    let rstc = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<HisiResetController>(),
        GFP_KERNEL,
    ) as *mut HisiResetController;
    if rstc.is_null() {
        return core::ptr::null_mut();
    }

    (*rstc).membase = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*rstc).membase) {
        return core::ptr::null_mut();
    }

    spin_lock_init(&mut (*rstc).lock);
    (*rstc).rcdev.owner = THIS_MODULE;
    (*rstc).rcdev.ops = &HISI_RESET_OPS;
    (*rstc).rcdev.of_node = (*pdev).dev.of_node;
    (*rstc).rcdev.of_reset_n_cells = 2;
    (*rstc).rcdev.of_xlate = Some(hisi_reset_of_xlate);
    reset_controller_register(&mut (*rstc).rcdev);

    rstc
}

pub unsafe extern "C" fn hisi_reset_exit(rstc: *mut HisiResetController) {
    reset_controller_unregister(&mut (*rstc).rcdev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
