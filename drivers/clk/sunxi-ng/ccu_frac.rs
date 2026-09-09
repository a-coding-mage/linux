// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn ccu_frac_helper_is_enabled(
    common: *mut ccu_common,
    cf: *mut ccu_frac_internal,
) -> bool {
    if ((*common).features & CCU_FEATURE_FRACTIONAL) == 0 {
        return false;
    }

    !(readl((*common).base.add((*common).reg as usize)) & (*cf).enable != 0)
}

// EXPORT_SYMBOL_NS_GPL(ccu_frac_helper_is_enabled, "SUNXI_CCU");

pub unsafe fn ccu_frac_helper_enable(
    common: *mut ccu_common,
    cf: *mut ccu_frac_internal,
) {
    let mut flags: unsigned_long;
    let reg: u32;

    if ((*common).features & CCU_FEATURE_FRACTIONAL) == 0 {
        return;
    }

    spin_lock_irqsave((*common).lock, &mut flags);
    reg = readl((*common).base.add((*common).reg as usize));
    writel(
        reg & !(*cf).enable,
        (*common).base.add((*common).reg as usize),
    );
    spin_unlock_irqrestore((*common).lock, flags);
}

// EXPORT_SYMBOL_NS_GPL(ccu_frac_helper_enable, "SUNXI_CCU");

pub unsafe fn ccu_frac_helper_disable(
    common: *mut ccu_common,
    cf: *mut ccu_frac_internal,
) {
    let mut flags: unsigned_long;
    let reg: u32;

    if ((*common).features & CCU_FEATURE_FRACTIONAL) == 0 {
        return;
    }

    spin_lock_irqsave((*common).lock, &mut flags);
    reg = readl((*common).base.add((*common).reg as usize));
    writel(
        reg | (*cf).enable,
        (*common).base.add((*common).reg as usize),
    );
    spin_unlock_irqrestore((*common).lock, flags);
}

// EXPORT_SYMBOL_NS_GPL(ccu_frac_helper_disable, "SUNXI_CCU");

pub unsafe fn ccu_frac_helper_has_rate(
    common: *mut ccu_common,
    cf: *mut ccu_frac_internal,
    rate: unsigned_long,
) -> bool {
    if ((*common).features & CCU_FEATURE_FRACTIONAL) == 0 {
        return false;
    }

    (*cf).rates[0] == rate || (*cf).rates[1] == rate
}

// EXPORT_SYMBOL_NS_GPL(ccu_frac_helper_has_rate, "SUNXI_CCU");

pub unsafe fn ccu_frac_helper_read_rate(
    common: *mut ccu_common,
    cf: *mut ccu_frac_internal,
) -> unsigned_long {
    let reg: u32;

    pr_debug!("%s: Read fractional\\n", clk_hw_get_name(&(*common).hw));

    if ((*common).features & CCU_FEATURE_FRACTIONAL) == 0 {
        return 0;
    }

    pr_debug!(
        "%s: clock is fractional (rates %lu and %lu)\\n",
        clk_hw_get_name(&(*common).hw),
        (*cf).rates[0],
        (*cf).rates[1]
    );

    reg = readl((*common).base.add((*common).reg as usize));

    pr_debug!(
        "%s: clock reg is 0x%x (select is 0x%x)\\n",
        clk_hw_get_name(&(*common).hw),
        reg,
        (*cf).select
    );

    if (reg & (*cf).select) != 0 {
        (*cf).rates[1]
    } else {
        (*cf).rates[0]
    }
}

// EXPORT_SYMBOL_NS_GPL(ccu_frac_helper_read_rate, "SUNXI_CCU");

pub unsafe fn ccu_frac_helper_set_rate(
    common: *mut ccu_common,
    cf: *mut ccu_frac_internal,
    rate: unsigned_long,
    lock: u32,
) -> i32 {
    let mut flags: unsigned_long;
    let mut reg: u32;
    let sel: u32;

    if ((*common).features & CCU_FEATURE_FRACTIONAL) == 0 {
        return -EINVAL;
    }

    if (*cf).rates[0] == rate {
        sel = 0;
    } else if (*cf).rates[1] == rate {
        sel = (*cf).select;
    } else {
        return -EINVAL;
    }

    spin_lock_irqsave((*common).lock, &mut flags);
    reg = readl((*common).base.add((*common).reg as usize));
    reg &= !(*cf).select;
    writel(
        reg | sel,
        (*common).base.add((*common).reg as usize),
    );
    spin_unlock_irqrestore((*common).lock, flags);

    ccu_helper_wait_for_lock(common, lock);

    0
}

// EXPORT_SYMBOL_NS_GPL(ccu_frac_helper_set_rate, "SUNXI_CCU");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
