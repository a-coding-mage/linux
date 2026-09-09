// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2014 Google, Inc
 * Author: Alexandru M Stan <amstan@chromium.org>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left as external names.

#[repr(C)]
struct RockchipMmcClock {
    hw: clk_hw,
    reg: *mut core::ffi::c_void,
    grf: *mut regmap,
    grf_reg: i32,
    shift: i32,
    cached_phase: i32,
    clk_rate_change_nb: notifier_block,
}

const RK3288_MMC_CLKGEN_DIV: u64 = 2;

unsafe fn rockchip_mmc_recalc(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let _ = hw;
    parent_rate / RK3288_MMC_CLKGEN_DIV
}

const ROCKCHIP_MMC_DELAY_SEL: u32 = 1 << 10;
const ROCKCHIP_MMC_DEGREE_MASK: u32 = 0x3;
const ROCKCHIP_MMC_DELAYNUM_OFFSET: u32 = 2;
const ROCKCHIP_MMC_DELAYNUM_MASK: u32 = 0xff << ROCKCHIP_MMC_DELAYNUM_OFFSET;
const PSECS_PER_SEC: i64 = 1_000_000_000_000;
const ROCKCHIP_MMC_DELAY_ELEMENT_PSEC: u64 = 60;

unsafe fn rockchip_mmc_get_phase(hw: *mut clk_hw) -> i32 {
    let mmc_clock = container_of!(hw, RockchipMmcClock, hw);
    let rate = clk_hw_get_rate(hw);
    let mut raw_value: u32;
    let mut degrees: u16;
    let mut delay_num: u32 = 0;

    /* Constant signal, no measurable phase shift */
    if rate == 0 {
        return 0;
    }

    if !(*mmc_clock).grf.is_null() {
        regmap_read((*mmc_clock).grf, (*mmc_clock).grf_reg, &mut raw_value);
    } else {
        raw_value = readl((*mmc_clock).reg);
    }

    raw_value >>= (*mmc_clock).shift;
    degrees = ((raw_value & ROCKCHIP_MMC_DEGREE_MASK) * 90) as u16;

    if raw_value & ROCKCHIP_MMC_DELAY_SEL != 0 {
        /* degrees/delaynum * 1000000 */
        let factor = (ROCKCHIP_MMC_DELAY_ELEMENT_PSEC / 10) * 36 * (rate / 10000);
        delay_num = (raw_value & ROCKCHIP_MMC_DELAYNUM_MASK) >> ROCKCHIP_MMC_DELAYNUM_OFFSET;
        degrees = degrees.wrapping_add(div_round_closest_u32(
            delay_num.wrapping_mul(factor as u32),
            1_000_000,
        ) as u16);
    }

    (degrees as i32) % 360
}

unsafe fn rockchip_mmc_set_phase(hw: *mut clk_hw, degrees: i32) -> i32 {
    let mmc_clock = container_of!(hw, RockchipMmcClock, hw);
    let rate = clk_hw_get_rate(hw);
    let nineties: u8;
    let remainder: u8;
    let delay_num: u8;
    let mut raw_value: u32;
    let mut delay: u32;

    /*
     * The below calculation is based on the output clock from
     * MMC host to the card, which expects the phase clock inherits
     * the clock rate from its parent, namely the output clock
     * provider of MMC host. However, things may go wrong if
     * (1) It is orphan.
     * (2) It is assigned to the wrong parent.
     *
     * This check help debug the case (1), which seems to be the
     * most likely problem we often face and which makes it difficult
     * for people to debug unstable mmc tuning results.
     */
    if rate == 0 {
        pr_err!("{}: invalid clk rate\n", __func__);
        return -EINVAL;
    }

    nineties = (degrees / 90) as u8;
    remainder = (degrees % 90) as u8;

    /*
     * Due to the inexact nature of the "fine" delay, we might
     * actually go non-monotonic.  We don't go _too_ monotonic
     * though, so we should be OK.  Here are options of how we may
     * work:
     *
     * Ideally we end up with:
     *   1.0, 2.0, ..., 69.0, 70.0, ...,  89.0, 90.0
     *
     * On one extreme (if delay is actually 44ps):
     *   .73, 1.5, ..., 50.6, 51.3, ...,  65.3, 90.0
     * The other (if delay is actually 77ps):
     *   1.3, 2.6, ..., 88.6. 89.8, ..., 114.0, 90
     *
     * It's possible we might make a delay that is up to 25
     * degrees off from what we think we're making.  That's OK
     * though because we should be REALLY far from any bad range.
     */

    /*
     * Convert to delay; do a little extra work to make sure we
     * don't overflow 32-bit / 64-bit numbers.
     */
    delay = 10_000_000; /* PSECS_PER_SEC / 10000 / 10 */
    delay = delay.wrapping_mul(remainder as u32);
    delay = div_round_closest_u32(
        delay,
        ((rate / 1000) * 36 * (ROCKCHIP_MMC_DELAY_ELEMENT_PSEC / 10)) as u32,
    );

    delay_num = core::cmp::min(delay, 255) as u8;
    raw_value = if delay_num != 0 { ROCKCHIP_MMC_DELAY_SEL } else { 0 };
    raw_value |= (delay_num as u32) << ROCKCHIP_MMC_DELAYNUM_OFFSET;
    raw_value |= nineties as u32;
    raw_value = HIWORD_UPDATE!(raw_value, 0x07ff, (*mmc_clock).shift);

    if !(*mmc_clock).grf.is_null() {
        regmap_write((*mmc_clock).grf, (*mmc_clock).grf_reg, raw_value);
    } else {
        writel(raw_value, (*mmc_clock).reg);
    }

    pr_debug!(
        "{}->set_phase({}) delay_nums={} reg[0x{:p}]=0x{:03x} actual_degrees={}\n",
        clk_hw_get_name(hw), degrees, delay_num, (*mmc_clock).reg,
        raw_value >> (*mmc_clock).shift, rockchip_mmc_get_phase(hw)
    );
    0
}

static ROCKCHIP_MMC_CLK_OPS: clk_ops = clk_ops {
    recalc_rate: Some(rockchip_mmc_recalc),
    get_phase: Some(rockchip_mmc_get_phase),
    set_phase: Some(rockchip_mmc_set_phase),
};

unsafe fn rockchip_mmc_clk_rate_notify(
    nb: *mut notifier_block,
    event: u64,
    data: *mut core::ffi::c_void,
) -> i32 {
    let mmc_clock = container_of!(nb, RockchipMmcClock, clk_rate_change_nb);
    let ndata = data as *mut clk_notifier_data;

    /*
     * rockchip_mmc_clk is mostly used by mmc controllers to sample
     * the input data, which expects the fixed phase after the tuning
     * process. However if the clock rate is changed, the phase is stale
     * and may break the data sampling. So here we try to restore the phase
     * for that case, except that
     * (1) cached_phase is invalid since we inevitably cached it when the
     * clock provider be reparented from orphan to its real parent in the
     * first place. Otherwise we may mess up the initialization of MMC cards
     * since we only set the default sample phase and drive phase later on.
     * (2) the new coming rate is higher than the older one since mmc driver
     * set the max-frequency to match the boards' ability but we can't go
     * over the heads of that, otherwise the tests smoke out the issue.
     */
    if (*ndata).old_rate <= (*ndata).new_rate {
        return NOTIFY_DONE;
    }

    if event == PRE_RATE_CHANGE {
        (*mmc_clock).cached_phase = rockchip_mmc_get_phase(&mut (*mmc_clock).hw);
    } else if (*mmc_clock).cached_phase != -EINVAL && event == POST_RATE_CHANGE {
        rockchip_mmc_set_phase(&mut (*mmc_clock).hw, (*mmc_clock).cached_phase);
    }
    NOTIFY_DONE
}

unsafe fn rockchip_clk_register_mmc(
    name: *const core::ffi::c_char,
    parent_names: *const *const core::ffi::c_char,
    num_parents: u8,
    reg: *mut core::ffi::c_void,
    grf: *mut regmap,
    grf_reg: i32,
    shift: i32,
) -> *mut clk {
    let mut init: clk_init_data = core::mem::zeroed();
    let mmc_clock = kmalloc_obj!(RockchipMmcClock);
    let clk: *mut clk;
    let ret: i32;

    if mmc_clock.is_null() {
        return ERR_PTR!(-ENOMEM);
    }

    init.name = name;
    init.flags = 0;
    init.num_parents = num_parents;
    init.parent_names = parent_names;
    init.ops = &ROCKCHIP_MMC_CLK_OPS;

    (*mmc_clock).hw.init = &init;
    (*mmc_clock).reg = reg;
    (*mmc_clock).grf = grf;
    (*mmc_clock).grf_reg = grf_reg;
    (*mmc_clock).shift = shift;

    clk = clk_register(core::ptr::null_mut(), &mut (*mmc_clock).hw);
    if IS_ERR!(clk) {
        ret = PTR_ERR!(clk);
        goto!(err_register);
    }

    (*mmc_clock).clk_rate_change_nb.notifier_call = Some(rockchip_mmc_clk_rate_notify);
    ret = clk_notifier_register(clk, &mut (*mmc_clock).clk_rate_change_nb);
    if ret != 0 {
        goto!(err_notifier);
    }
    return clk;

err_notifier:
    clk_unregister(clk);
err_register:
    kfree(mmc_clock);
    ERR_PTR!(ret)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
