// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Dependencies supplied by the Linux clock-provider and Sunxi CCU headers.

unsafe fn ccu_div_determine_rate_helper(
    mux: *mut ccu_mux_internal,
    req: *mut clk_rate_request,
    data: *mut core::ffi::c_void,
) -> i32 {
    let cd = data as *mut ccu_div;
    let mut ret: i32;

    unsafe {
        if ((*cd).common.features & CCU_FEATURE_FIXED_POSTDIV) != 0 {
            (*req).rate *= (*cd).fixed_post_div;
        }

        if ((*cd).div.flags & CLK_DIVIDER_READ_ONLY) != 0 {
            let val: u64;
            let reg: u32;

            reg = readl((*cd).common.base.add((*cd).common.reg as usize));
            val = (reg >> (*cd).div.shift) as u64;
            let val = val & (((1u64 << (*cd).div.width) - 1) as u64);

            ret = divider_ro_determine_rate(
                &mut (*cd).common.hw,
                &mut *req,
                (*cd).div.table,
                (*cd).div.width,
                (*cd).div.flags,
                val,
            );
        } else {
            ret = divider_determine_rate(
                &mut (*cd).common.hw,
                &mut *req,
                (*cd).div.table,
                (*cd).div.width,
                (*cd).div.flags,
            );
        }

        if ret != 0 {
            return ret;
        }

        if ((*cd).common.features & CCU_FEATURE_FIXED_POSTDIV) != 0 {
            (*req).rate /= (*cd).fixed_post_div;
        }
    }

    0
}

unsafe fn ccu_div_disable(hw: *mut clk_hw) {
    let cd = unsafe { hw_to_ccu_div(hw) };
    unsafe { ccu_gate_helper_disable(&mut (*cd).common, (*cd).enable); }
}

unsafe fn ccu_div_enable(hw: *mut clk_hw) -> i32 {
    let cd = unsafe { hw_to_ccu_div(hw) };
    unsafe { ccu_gate_helper_enable(&mut (*cd).common, (*cd).enable) }
}

unsafe fn ccu_div_is_enabled(hw: *mut clk_hw) -> i32 {
    let cd = unsafe { hw_to_ccu_div(hw) };
    unsafe { ccu_gate_helper_is_enabled(&mut (*cd).common, (*cd).enable) }
}

unsafe fn ccu_div_recalc_rate(hw: *mut clk_hw, mut parent_rate: u64) -> u64 {
    let cd = unsafe { hw_to_ccu_div(hw) };
    let mut val: u64;
    let reg: u32;

    unsafe {
        reg = readl((*cd).common.base.add((*cd).common.reg as usize));
        val = (reg >> (*cd).div.shift) as u64;
        val &= (1u64 << (*cd).div.width) - 1;

        parent_rate = ccu_mux_helper_apply_prediv(
            &mut (*cd).common, &mut (*cd).mux, -1, parent_rate,
        );

        val = divider_recalc_rate(
            hw, parent_rate, val, (*cd).div.table, (*cd).div.flags, (*cd).div.width,
        );

        if ((*cd).common.features & CCU_FEATURE_FIXED_POSTDIV) != 0 {
            val /= (*cd).fixed_post_div;
        }
    }

    val
}

unsafe fn ccu_div_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let cd = unsafe { hw_to_ccu_div(hw) };
    unsafe {
        ccu_mux_helper_determine_rate(
            &mut (*cd).common, &mut (*cd).mux, &mut *req,
            ccu_div_determine_rate_helper, cd as *mut core::ffi::c_void,
        )
    }
}

unsafe fn ccu_div_set_rate(hw: *mut clk_hw, mut rate: u64, mut parent_rate: u64) -> i32 {
    let cd = unsafe { hw_to_ccu_div(hw) };
    let mut flags: u64;
    let val: u64;
    let mut reg: u32;

    unsafe {
        parent_rate = ccu_mux_helper_apply_prediv(
            &mut (*cd).common, &mut (*cd).mux, -1, parent_rate,
        );

        if ((*cd).common.features & CCU_FEATURE_FIXED_POSTDIV) != 0 {
            rate *= (*cd).fixed_post_div;
        }

        val = divider_get_val(
            rate, parent_rate, (*cd).div.table, (*cd).div.width, (*cd).div.flags,
        );

        spin_lock_irqsave((*cd).common.lock, &mut flags);

        reg = readl((*cd).common.base.add((*cd).common.reg as usize));
        reg &= !genmask((*cd).div.width + (*cd).div.shift - 1, (*cd).div.shift);
        if ((*cd).common.features & CCU_FEATURE_UPDATE_BIT) != 0 {
            reg |= CCU_SUNXI_UPDATE_BIT;
        }

        writel(
            reg | ((val as u32) << (*cd).div.shift),
            (*cd).common.base.add((*cd).common.reg as usize),
        );

        spin_unlock_irqrestore((*cd).common.lock, flags);
    }

    0
}

unsafe fn ccu_div_get_parent(hw: *mut clk_hw) -> u8 {
    let cd = unsafe { hw_to_ccu_div(hw) };
    unsafe { ccu_mux_helper_get_parent(&mut (*cd).common, &mut (*cd).mux) }
}

unsafe fn ccu_div_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let cd = unsafe { hw_to_ccu_div(hw) };
    unsafe { ccu_mux_helper_set_parent(&mut (*cd).common, &mut (*cd).mux, index) }
}

pub static ccu_div_ops: clk_ops = clk_ops {
    disable: Some(ccu_div_disable),
    enable: Some(ccu_div_enable),
    is_enabled: Some(ccu_div_is_enabled),
    get_parent: Some(ccu_div_get_parent),
    set_parent: Some(ccu_div_set_parent),
    determine_rate: Some(ccu_div_determine_rate),
    recalc_rate: Some(ccu_div_recalc_rate),
    set_rate: Some(ccu_div_set_rate),
};

pub static ccu_rodiv_ops: clk_ops = clk_ops {
    disable: Some(ccu_div_disable),
    enable: Some(ccu_div_enable),
    is_enabled: Some(ccu_div_is_enabled),
    get_parent: Some(ccu_div_get_parent),
    set_parent: Some(ccu_div_set_parent),
    determine_rate: Some(ccu_div_determine_rate),
    recalc_rate: Some(ccu_div_recalc_rate),
    set_rate: None,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
