// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Translated dependencies: <linux/delay.h>, <linux/io.h>,
// <linux/reset-controller.h>, and "ccu_reset.h".

unsafe fn ccu_reset_assert(
    rcdev: *mut reset_controller_dev,
    id: libc::c_ulong,
) -> libc::c_int {
    let ccu = rcdev_to_ccu_reset(rcdev);
    let map = &(*ccu).reset_map[id as usize];
    let mut flags: libc::c_ulong = 0;
    let mut reg: u32;

    spin_lock_irqsave((*ccu).lock, &mut flags);

    reg = readl((*ccu).base.add(map.reg as usize));
    writel(reg & !map.bit, (*ccu).base.add(map.reg as usize));

    spin_unlock_irqrestore((*ccu).lock, flags);

    0
}

unsafe fn ccu_reset_deassert(
    rcdev: *mut reset_controller_dev,
    id: libc::c_ulong,
) -> libc::c_int {
    let ccu = rcdev_to_ccu_reset(rcdev);
    let map = &(*ccu).reset_map[id as usize];
    let mut flags: libc::c_ulong = 0;
    let mut reg: u32;

    spin_lock_irqsave((*ccu).lock, &mut flags);

    reg = readl((*ccu).base.add(map.reg as usize));
    writel(reg | map.bit, (*ccu).base.add(map.reg as usize));

    spin_unlock_irqrestore((*ccu).lock, flags);

    0
}

unsafe fn ccu_reset_reset(
    rcdev: *mut reset_controller_dev,
    id: libc::c_ulong,
) -> libc::c_int {
    ccu_reset_assert(rcdev, id);
    udelay(10);
    ccu_reset_deassert(rcdev, id);

    0
}

unsafe fn ccu_reset_status(
    rcdev: *mut reset_controller_dev,
    id: libc::c_ulong,
) -> libc::c_int {
    let ccu = rcdev_to_ccu_reset(rcdev);
    let map = &(*ccu).reset_map[id as usize];

    /*
     * The reset control API expects 0 if reset is not asserted,
     * which is the opposite of what our hardware uses.
     */
    (!(map.bit & readl((*ccu).base.add(map.reg as usize)) != 0)) as libc::c_int
}

pub static ccu_reset_ops: reset_control_ops = reset_control_ops {
    assert: Some(ccu_reset_assert),
    deassert: Some(ccu_reset_deassert),
    reset: Some(ccu_reset_reset),
    status: Some(ccu_reset_status),
};

// EXPORT_SYMBOL_NS_GPL(ccu_reset_ops, "SUNXI_CCU");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
