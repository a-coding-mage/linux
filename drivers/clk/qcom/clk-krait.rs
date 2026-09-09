// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

// C dependencies supplied by the surrounding kernel translation unit are intentionally
// referenced here rather than reimplemented.

/* Secondary and primary muxes share the same cp15 register */
static mut KRAIT_CLOCK_REG_LOCK: spinlock_t = spinlock_t::new();

const LPL_SHIFT: u32 = 8;
const SECCLKAGD: u32 = 1 << 4;

unsafe fn __krait_mux_set_sel(mux: *mut krait_mux_clk, sel: i32) {
    let mut flags: unsigned_long = 0;
    let mut regval: u32;

    spin_lock_irqsave(&raw mut KRAIT_CLOCK_REG_LOCK, &mut flags);

    regval = krait_get_l2_indirect_reg((*mux).offset);

    /* apq/ipq8064 Errata: disable sec_src clock gating during switch. */
    if (*mux).disable_sec_src_gating {
        regval |= SECCLKAGD;
        krait_set_l2_indirect_reg((*mux).offset, regval);
    }

    regval &= !((*mux).mask << (*mux).shift);
    regval |= ((sel as u32) & (*mux).mask) << (*mux).shift;
    if (*mux).lpl {
        regval &= !((*mux).mask << ((*mux).shift + LPL_SHIFT));
        regval |= ((sel as u32) & (*mux).mask) << ((*mux).shift + LPL_SHIFT);
    }
    krait_set_l2_indirect_reg((*mux).offset, regval);

    /* apq/ipq8064 Errata: re-enabled sec_src clock gating. */
    if (*mux).disable_sec_src_gating {
        regval &= !SECCLKAGD;
        krait_set_l2_indirect_reg((*mux).offset, regval);
    }

    /* Wait for switch to complete. */
    mb();
    udelay(1);

    /*
     * Unlock now to make sure the mux register is not
     * modified while switching to the new parent.
     */
    spin_unlock_irqrestore(&raw mut KRAIT_CLOCK_REG_LOCK, flags);
}

unsafe fn krait_mux_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let mux: *mut krait_mux_clk = to_krait_mux_clk(hw);
    let sel: u32;

    sel = clk_mux_index_to_val((*mux).parent_map, 0, index);
    (*mux).en_mask = sel;
    /* Don't touch mux if CPU is off as it won't work */
    if clk_hw_is_enabled(hw) {
        __krait_mux_set_sel(mux, sel as i32);
    }

    (*mux).reparent = true;

    0
}

unsafe fn krait_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let mux: *mut krait_mux_clk = to_krait_mux_clk(hw);
    let mut sel: u32;

    sel = krait_get_l2_indirect_reg((*mux).offset);
    sel >>= (*mux).shift;
    sel &= (*mux).mask;
    (*mux).en_mask = sel;

    clk_mux_val_to_index(hw, (*mux).parent_map, 0, sel)
}

#[no_mangle]
pub static krait_mux_clk_ops: clk_ops = clk_ops {
    .set_parent = Some(krait_mux_set_parent),
    .get_parent = Some(krait_mux_get_parent),
    .determine_rate = Some(__clk_mux_determine_rate_closest),
};

/* The divider can divide by 2, 4, 6 and 8. But we only really need div-2. */
unsafe fn krait_div2_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    (*req).best_parent_rate = clk_hw_round_rate(
        clk_hw_get_parent(hw),
        (*req).rate.wrapping_mul(2),
    );
    (*req).rate = div_round_up((*req).best_parent_rate, 2);
    0
}

unsafe fn krait_div2_set_rate(
    hw: *mut clk_hw,
    _rate: unsigned_long,
    _parent_rate: unsigned_long,
) -> i32 {
    let d: *mut krait_div2_clk = to_krait_div2_clk(hw);
    let mut flags: unsigned_long = 0;
    let val: u32;
    let mut mask: u32 = (1u32 << (*d).width) - 1;

    if (*d).lpl {
        mask = (mask << ((*d).shift + LPL_SHIFT)) | (mask << (*d).shift);
    } else {
        mask <<= (*d).shift;
    }

    spin_lock_irqsave(&raw mut KRAIT_CLOCK_REG_LOCK, &mut flags);
    val = krait_get_l2_indirect_reg((*d).offset) & !mask;
    krait_set_l2_indirect_reg((*d).offset, val);
    spin_unlock_irqrestore(&raw mut KRAIT_CLOCK_REG_LOCK, flags);

    0
}

unsafe fn krait_div2_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: unsigned_long,
) -> unsigned_long {
    let d: *mut krait_div2_clk = to_krait_div2_clk(hw);
    let mask: u32 = (1u32 << (*d).width) - 1;
    let mut div: u32;

    div = krait_get_l2_indirect_reg((*d).offset);
    div >>= (*d).shift;
    div &= mask;
    div = (div + 1) * 2;

    div_round_up(parent_rate, div as unsigned_long)
}

#[no_mangle]
pub static krait_div2_clk_ops: clk_ops = clk_ops {
    .determine_rate = Some(krait_div2_determine_rate),
    .set_rate = Some(krait_div2_set_rate),
    .recalc_rate = Some(krait_div2_recalc_rate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
