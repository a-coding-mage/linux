// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) STMicroelectronics 2022 - All Rights Reserved
 * Author: Gabriel Fernandez <gabriel.fernandez@foss.st.com> for STMicroelectronics.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct stm32_reset_data {
    /* reset lock */
    lock: spinlock_t,
    rcdev: reset_controller_dev,
    membase: *mut core::ffi::c_void,
    clear_offset: u32,
    reset_lines: *const *const stm32_reset_cfg,
}

#[inline]
unsafe fn to_stm32_reset_data(rcdev: *mut reset_controller_dev) -> *mut stm32_reset_data {
    container_of!(rcdev, stm32_reset_data, rcdev)
}

unsafe fn stm32_get_reset_line(
    rcdev: *mut reset_controller_dev,
    id: usize,
    line: *mut stm32_reset_cfg,
) -> *const stm32_reset_cfg {
    let data = &mut *to_stm32_reset_data(rcdev);

    if data.reset_lines.is_null() {
        let reg_width: usize = core::mem::size_of::<u32>();
        let bank = id / (reg_width * BITS_PER_BYTE as usize);
        let offset = id % (reg_width * BITS_PER_BYTE as usize);

        (*line).offset = (bank * reg_width) as u32;
        (*line).bit_idx = offset as u32;
        (*line).set_clr = if data.clear_offset != 0 { true } else { false };

        return line;
    }

    *data.reset_lines.add(id)
}

unsafe fn stm32_reset_update(
    rcdev: *mut reset_controller_dev,
    id: usize,
    assert: bool,
) -> i32 {
    let data = &mut *to_stm32_reset_data(rcdev);
    let mut line_reset: stm32_reset_cfg = core::mem::zeroed();
    let ptr_line = stm32_get_reset_line(rcdev, id, &mut line_reset);

    if ptr_line.is_null() {
        return -EPERM;
    }

    if (*ptr_line).set_clr {
        let mut addr = (data.membase as *mut u8).add((*ptr_line).offset as usize);
        if !assert {
            addr = addr.add(data.clear_offset as usize);
        }

        writel!(BIT((*ptr_line).bit_idx), addr);
    } else {
        let mut flags: unsigned_long = 0;
        let mut reg: u32;

        spin_lock_irqsave!(&mut data.lock, &mut flags);

        reg = readl!((data.membase as *mut u8).add((*ptr_line).offset as usize));

        if assert {
            reg |= BIT((*ptr_line).bit_idx);
        } else {
            reg &= !BIT((*ptr_line).bit_idx);
        }

        writel!(reg, (data.membase as *mut u8).add((*ptr_line).offset as usize));

        spin_unlock_irqrestore!(&mut data.lock, flags);
    }

    0
}

unsafe fn stm32_reset_assert(rcdev: *mut reset_controller_dev, id: usize) -> i32 {
    stm32_reset_update(rcdev, id, true)
}

unsafe fn stm32_reset_deassert(rcdev: *mut reset_controller_dev, id: usize) -> i32 {
    stm32_reset_update(rcdev, id, false)
}

unsafe fn stm32_reset_status(rcdev: *mut reset_controller_dev, id: usize) -> i32 {
    let data = &mut *to_stm32_reset_data(rcdev);
    let mut line_reset: stm32_reset_cfg = core::mem::zeroed();
    let ptr_line = stm32_get_reset_line(rcdev, id, &mut line_reset);
    let reg: u32;

    if ptr_line.is_null() {
        return -EPERM;
    }

    reg = readl!((data.membase as *mut u8).add((*ptr_line).offset as usize));

    if (reg & BIT((*ptr_line).bit_idx)) != 0 { 1 } else { 0 }
}

static stm32_reset_ops: reset_control_ops = reset_control_ops {
    assert: Some(stm32_reset_assert),
    deassert: Some(stm32_reset_deassert),
    status: Some(stm32_reset_status),
};

unsafe fn stm32_rcc_reset_init(
    dev: *mut device,
    data: *mut clk_stm32_reset_data,
    base: *mut core::ffi::c_void,
) -> i32 {
    let reset_data = kzalloc_obj::<stm32_reset_data>();
    if reset_data.is_null() {
        return -ENOMEM;
    }

    spin_lock_init!(&mut (*reset_data).lock);

    (*reset_data).membase = base;
    (*reset_data).rcdev.owner = THIS_MODULE;
    (*reset_data).rcdev.ops = &stm32_reset_ops;
    (*reset_data).rcdev.of_node = dev_of_node(dev);
    (*reset_data).rcdev.nr_resets = (*data).nr_lines;
    (*reset_data).reset_lines = (*data).reset_lines;
    (*reset_data).clear_offset = (*data).clear_offset;

    reset_controller_register(&mut (*reset_data).rcdev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
