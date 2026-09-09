// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2013, The Linux Foundation. All rights reserved.
 * Copyright (c) 2023, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn clk_branch_in_hwcg_mode(br: *const clk_branch) -> bool {
    let mut val: u32 = 0;

    if (*br).hwcg_reg == 0 {
        return false;
    }

    regmap_read((*br).clkr.regmap, (*br).hwcg_reg, &mut val);

    (val & BIT((*br).hwcg_bit)) != 0
}

unsafe fn clk_branch_check_halt(br: *const clk_branch, enabling: bool) -> bool {
    let invert = ((*br).halt_check & BRANCH_HALT_ENABLE) != 0;
    let mut val: u32 = 0;

    regmap_read((*br).clkr.regmap, (*br).halt_reg, &mut val);

    val &= BIT((*br).halt_bit);
    if invert {
        val = if val != 0 { 0 } else { 1 };
    }

    (val != 0) == !enabling
}

unsafe fn clk_branch2_check_halt(br: *const clk_branch, enabling: bool) -> bool {
    let mut val: u32 = 0;
    let mut mask: u32;
    let invert = ((*br).halt_check & BRANCH_HALT_ENABLE) != 0;

    mask = CBCR_NOC_FSM_STATUS;
    mask |= CBCR_CLK_OFF;

    regmap_read((*br).clkr.regmap, (*br).halt_reg, &mut val);

    if enabling {
        val &= mask;
        return (val & CBCR_CLK_OFF) == (if invert { CBCR_CLK_OFF } else { 0 })
            || FIELD_GET(CBCR_NOC_FSM_STATUS, val) == FSM_STATUS_ON;
    }
    (val & CBCR_CLK_OFF) == (if invert { 0 } else { CBCR_CLK_OFF })
}

unsafe fn clk_branch_wait(
    br: *const clk_branch,
    enabling: bool,
    check_halt: unsafe fn(*const clk_branch, bool) -> bool,
) -> i32 {
    let voted = ((*br).halt_check & BRANCH_VOTED) != 0;
    let name = clk_hw_get_name(&(*br).clkr.hw);

    /*
     * Skip checking halt bit if we're explicitly ignoring the bit or the
     * clock is in hardware gated mode
     */
    if (*br).halt_check == BRANCH_HALT_SKIP || clk_branch_in_hwcg_mode(br) {
        return 0;
    }

    if (*br).halt_check == BRANCH_HALT_DELAY || (!enabling && voted) {
        udelay(10);
    } else if (*br).halt_check == BRANCH_HALT_ENABLE
        || (*br).halt_check == BRANCH_HALT
        || (enabling && voted)
    {
        let mut count = 200;

        while count > 0 {
            count -= 1;
            if check_halt(br, enabling) {
                return 0;
            }
            udelay(1);
        }
        WARN(1, "%s status stuck at 'o%s'", name, if enabling { "ff" } else { "n" });
        return -EBUSY;
    }
    0
}

unsafe fn clk_branch_toggle(
    hw: *mut clk_hw,
    en: bool,
    check_halt: unsafe fn(*const clk_branch, bool) -> bool,
) -> i32 {
    let br = to_clk_branch(hw);
    let ret: i32;

    if en {
        ret = clk_enable_regmap(hw);
        if ret != 0 {
            return ret;
        }
    } else {
        clk_disable_regmap(hw);
    }

    clk_branch_wait(br, en, check_halt)
}

unsafe fn clk_branch_enable(hw: *mut clk_hw) -> i32 {
    clk_branch_toggle(hw, true, clk_branch_check_halt)
}

unsafe fn clk_branch_disable(hw: *mut clk_hw) {
    clk_branch_toggle(hw, false, clk_branch_check_halt);
}

#[no_mangle]
pub static clk_branch_ops: clk_ops = clk_ops {
    enable: Some(clk_branch_enable),
    disable: Some(clk_branch_disable),
    is_enabled: Some(clk_is_enabled_regmap),
};

unsafe fn clk_branch2_enable(hw: *mut clk_hw) -> i32 {
    clk_branch_toggle(hw, true, clk_branch2_check_halt)
}

unsafe fn clk_branch2_disable(hw: *mut clk_hw) {
    clk_branch_toggle(hw, false, clk_branch2_check_halt);
}

unsafe fn clk_branch2_mem_enable(hw: *mut clk_hw) -> i32 {
    let mem_br = to_clk_mem_branch(hw);
    let branch = (*mem_br).branch;
    let mut val: u32 = 0;
    let ret: i32;

    regmap_assign_bits(
        branch.clkr.regmap,
        (*mem_br).mem_enable_reg,
        (*mem_br).mem_enable_mask,
        !(*mem_br).mem_enable_invert,
    );

    ret = regmap_read_poll_timeout(
        branch.clkr.regmap,
        (*mem_br).mem_ack_reg,
        &mut val,
        val & (*mem_br).mem_enable_ack_mask,
        0,
        200,
    );
    if ret != 0 {
        WARN(1, "%s mem enable failed\n", clk_hw_get_name(&branch.clkr.hw));
        return ret;
    }

    clk_branch2_enable(hw)
}

unsafe fn clk_branch2_mem_disable(hw: *mut clk_hw) {
    let mem_br = to_clk_mem_branch(hw);

    regmap_assign_bits(
        (*mem_br).branch.clkr.regmap,
        (*mem_br).mem_enable_reg,
        (*mem_br).mem_enable_mask,
        (*mem_br).mem_enable_invert,
    );

    clk_branch2_disable(hw);
}

#[no_mangle]
pub static clk_branch2_mem_ops: clk_ops = clk_ops {
    enable: Some(clk_branch2_mem_enable),
    disable: Some(clk_branch2_mem_disable),
    is_enabled: Some(clk_is_enabled_regmap),
};

#[no_mangle]
pub static clk_branch2_ops: clk_ops = clk_ops {
    enable: Some(clk_branch2_enable),
    disable: Some(clk_branch2_disable),
    is_enabled: Some(clk_is_enabled_regmap),
};

#[no_mangle]
pub static clk_branch2_aon_ops: clk_ops = clk_ops {
    enable: Some(clk_branch2_enable),
    disable: None,
    is_enabled: Some(clk_is_enabled_regmap),
};

#[no_mangle]
pub static clk_branch_simple_ops: clk_ops = clk_ops {
    enable: Some(clk_enable_regmap),
    disable: Some(clk_disable_regmap),
    is_enabled: Some(clk_is_enabled_regmap),
};

#[no_mangle]
pub static clk_branch2_prepare_ops: clk_ops = clk_ops {
    prepare: Some(clk_branch2_enable),
    unprepare: Some(clk_branch2_disable),
    is_prepared: Some(clk_is_enabled_regmap),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
