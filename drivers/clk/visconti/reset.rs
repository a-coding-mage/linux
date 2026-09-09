// SPDX-License-Identifier: GPL-2.0-only
/*
 * Toshiba Visconti ARM SoC reset controller
 *
 * Copyright (c) 2021 TOSHIBA CORPORATION
 * Copyright (c) 2021 Toshiba Electronic Devices & Storage Corporation
 *
 * Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe extern "C" {
    fn udelay(usecs: u32);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn regmap_update_bits(
        map: *mut regmap,
        offset: u32,
        mask: u32,
        value: u32,
    ) -> c_int;
    fn regmap_read(map: *mut regmap, offset: u32, value: *mut u32) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_ulong) -> *mut c_void;
    fn devm_reset_controller_register(
        dev: *mut device,
        rcdev: *mut reset_controller_dev,
    ) -> c_int;
}

const GFP_KERNEL: c_ulong = 0;

#[inline]
unsafe fn to_visconti_reset(rcdev: *mut reset_controller_dev) -> *mut visconti_reset {
    container_of!(rcdev, visconti_reset, rcdev)
}

unsafe fn visconti_reset_assert(
    rcdev: *mut reset_controller_dev,
    id: c_ulong,
) -> c_int {
    let reset = to_visconti_reset(rcdev);
    let data = &(*reset).resets[id as usize];
    let rst: u32 = 1u32.wrapping_shl(data.rs_idx);
    let mut flags: c_ulong = 0;
    let ret: c_int;

    spin_lock_irqsave((*reset).lock, &mut flags);
    ret = regmap_update_bits((*reset).regmap, data.rson_offset, rst, rst);
    spin_unlock_irqrestore((*reset).lock, flags);

    ret
}

unsafe fn visconti_reset_deassert(
    rcdev: *mut reset_controller_dev,
    id: c_ulong,
) -> c_int {
    let reset = to_visconti_reset(rcdev);
    let data = &(*reset).resets[id as usize];
    let rst: u32 = 1u32.wrapping_shl(data.rs_idx);
    let mut flags: c_ulong = 0;
    let ret: c_int;

    spin_lock_irqsave((*reset).lock, &mut flags);
    ret = regmap_update_bits((*reset).regmap, data.rsoff_offset, rst, rst);
    spin_unlock_irqrestore((*reset).lock, flags);

    ret
}

unsafe fn visconti_reset_reset(
    rcdev: *mut reset_controller_dev,
    id: c_ulong,
) -> c_int {
    visconti_reset_assert(rcdev, id);
    udelay(1);
    visconti_reset_deassert(rcdev, id);

    0
}

unsafe fn visconti_reset_status(
    rcdev: *mut reset_controller_dev,
    id: c_ulong,
) -> c_int {
    let reset = to_visconti_reset(rcdev);
    let data = &(*reset).resets[id as usize];
    let mut flags: c_ulong = 0;
    let mut reg: u32 = 0;
    let ret: c_int;

    spin_lock_irqsave((*reset).lock, &mut flags);
    ret = regmap_read((*reset).regmap, data.rson_offset, &mut reg);
    spin_unlock_irqrestore((*reset).lock, flags);
    if ret != 0 {
        return ret;
    }

    if (reg & data.rs_idx) == 0 { 1 } else { 0 }
}

pub static visconti_reset_ops: reset_control_ops = reset_control_ops {
    assert: Some(visconti_reset_assert),
    deassert: Some(visconti_reset_deassert),
    reset: Some(visconti_reset_reset),
    status: Some(visconti_reset_status),
};

pub unsafe fn visconti_register_reset_controller(
    dev: *mut device,
    regmap: *mut regmap,
    resets: *const visconti_reset_data,
    num_resets: c_uint,
    reset_ops: *const reset_control_ops,
    lock: *mut spinlock_t,
) -> c_int {
    let reset: *mut visconti_reset;

    reset = devm_kzalloc(dev, core::mem::size_of::<visconti_reset>(), GFP_KERNEL)
        as *mut visconti_reset;
    if reset.is_null() {
        return -12;
    }

    (*reset).regmap = regmap;
    (*reset).resets = resets;
    (*reset).rcdev.ops = reset_ops;
    (*reset).rcdev.nr_resets = num_resets;
    (*reset).rcdev.of_node = (*dev).of_node;
    (*reset).lock = lock;

    devm_reset_controller_register(dev, &mut (*reset).rcdev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
