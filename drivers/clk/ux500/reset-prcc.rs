// SPDX-License-Identifier: GPL-2.0-only
/*
 * Reset controller portions for the U8500 PRCC
 * Copyright (C) 2021 Linus Walleij <linus.walleij@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation.

const PRCC_K_SOFTRST_SET: usize = 0x018;
const PRCC_K_SOFTRST_CLEAR: usize = 0x01c;
const PRCC_K_RST_STATUS: usize = 0x020;

/* This macro flattens the 2-dimensional PRCC numberspace. */
#[inline]
fn prcc_reset_line(prcc_num: usize, bit: usize) -> usize {
    (prcc_num * PRCC_PERIPHS_PER_CLUSTER) + bit
}

unsafe fn prcc_num_to_index(num: u32) -> i32 {
    match num {
        1 => CLKRST1_INDEX,
        2 => CLKRST2_INDEX,
        3 => CLKRST3_INDEX,
        5 => CLKRST5_INDEX,
        6 => CLKRST6_INDEX,
        _ => -EINVAL,
    }
}

unsafe fn u8500_prcc_reset_base(
    ur: *mut u8500_prcc_reset,
    id: usize,
) -> *mut core::ffi::c_void {
    let prcc_num: u32 = (id / PRCC_PERIPHS_PER_CLUSTER) as u32;
    let index: i32 = prcc_num_to_index(prcc_num);

    if (index as usize) >= core::mem::size_of_val(&(*ur).base) / core::mem::size_of::<*mut core::ffi::c_void>() {
        return core::ptr::null_mut();
    }

    (*ur).base[index as usize]
}

unsafe fn u8500_prcc_reset(
    rcdev: *mut reset_controller_dev,
    id: usize,
) -> i32 {
    let ur: *mut u8500_prcc_reset = container_of!(rcdev, u8500_prcc_reset, rcdev);
    let base: *mut core::ffi::c_void = u8500_prcc_reset_base(ur, id);
    let bit: usize = id % PRCC_PERIPHS_PER_CLUSTER;

    pr_debug!("PRCC cycle reset id {}, bit {}\n", id, bit);
    writel(bit!(bit), base.add(PRCC_K_SOFTRST_CLEAR));
    udelay(1);
    writel(bit!(bit), base.add(PRCC_K_SOFTRST_SET));
    udelay(1);
    0
}

unsafe fn u8500_prcc_reset_assert(
    rcdev: *mut reset_controller_dev,
    id: usize,
) -> i32 {
    let ur: *mut u8500_prcc_reset = container_of!(rcdev, u8500_prcc_reset, rcdev);
    let base: *mut core::ffi::c_void = u8500_prcc_reset_base(ur, id);
    let bit: usize = id % PRCC_PERIPHS_PER_CLUSTER;

    pr_debug!("PRCC assert reset id {}, bit {}\n", id, bit);
    writel(bit!(bit), base.add(PRCC_K_SOFTRST_CLEAR));
    0
}

unsafe fn u8500_prcc_reset_deassert(
    rcdev: *mut reset_controller_dev,
    id: usize,
) -> i32 {
    let ur: *mut u8500_prcc_reset = container_of!(rcdev, u8500_prcc_reset, rcdev);
    let base: *mut core::ffi::c_void = u8500_prcc_reset_base(ur, id);
    let bit: usize = id % PRCC_PERIPHS_PER_CLUSTER;

    pr_debug!("PRCC deassert reset id {}, bit {}\n", id, bit);
    writel(bit!(bit), base.add(PRCC_K_SOFTRST_SET));
    0
}

unsafe fn u8500_prcc_reset_status(
    rcdev: *mut reset_controller_dev,
    id: usize,
) -> i32 {
    let ur: *mut u8500_prcc_reset = container_of!(rcdev, u8500_prcc_reset, rcdev);
    let base: *mut core::ffi::c_void = u8500_prcc_reset_base(ur, id);
    let bit: usize = id % PRCC_PERIPHS_PER_CLUSTER;

    pr_debug!("PRCC check status on reset line id {}, bit {}\n", id, bit);
    let val: u32 = readl(base.add(PRCC_K_RST_STATUS));

    /* Active low so return the inverse value of the bit. */
    (!(val & bit!(bit))) as i32
}

static U8500_PRCC_RESET_OPS: reset_control_ops = reset_control_ops {
    reset: Some(u8500_prcc_reset),
    assert: Some(u8500_prcc_reset_assert),
    deassert: Some(u8500_prcc_reset_deassert),
    status: Some(u8500_prcc_reset_status),
};

unsafe fn u8500_prcc_reset_xlate(
    _rcdev: *mut reset_controller_dev,
    reset_spec: *const of_phandle_args,
) -> i32 {
    if (*reset_spec).args_count != 2 {
        return -EINVAL;
    }

    let prcc_num: usize = (*reset_spec).args[0] as usize;
    let bit: usize = (*reset_spec).args[1] as usize;

    if prcc_num != 1 && prcc_num != 2 && prcc_num != 3 && prcc_num != 5 && prcc_num != 6 {
        pr_err!("{}: invalid PRCC {}\n", module_path!(), prcc_num);
        return -EINVAL;
    }

    pr_debug!("located reset line {} at PRCC {} bit {}\n", prcc_reset_line(prcc_num, bit), prcc_num, bit);
    prcc_reset_line(prcc_num, bit) as i32
}

pub unsafe fn u8500_prcc_reset_init(
    np: *mut device_node,
    ur: *mut u8500_prcc_reset,
) {
    let rcdev: *mut reset_controller_dev = &mut (*ur).rcdev;
    let mut ret: i32;
    let mut i: usize = 0;

    while i < CLKRST_MAX as usize {
        (*ur).base[i] = ioremap((*ur).phy_base[i], SZ_4K);
        if (*ur).base[i].is_null() {
            pr_err!("PRCC failed to remap for reset base {} ({:08x})\n", i, (*ur).phy_base[i]);
        }
        i += 1;
    }

    (*rcdev).owner = THIS_MODULE;
    (*rcdev).ops = &U8500_PRCC_RESET_OPS;
    (*rcdev).of_node = np;
    (*rcdev).of_reset_n_cells = 2;
    (*rcdev).of_xlate = Some(u8500_prcc_reset_xlate);

    ret = reset_controller_register(rcdev);
    if ret != 0 {
        pr_err!("PRCC failed to register reset controller\n");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
