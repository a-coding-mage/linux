// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
 */

// Kernel dependencies supplied by the surrounding translation unit.

const UTMI_RATE: u64 = 480000000;

#[repr(C)]
pub struct clk_utmi {
    pub hw: clk_hw,
    pub regmap_pmc: *mut regmap,
    pub regmap_sfr: *mut regmap,
    pub pms: at91_clk_pms,
}

unsafe fn to_clk_utmi(hw: *mut clk_hw) -> *mut clk_utmi {
    hw as *mut clk_utmi
}

#[inline]
unsafe fn clk_utmi_ready(regmap: *mut regmap) -> bool {
    let mut status: u32 = 0;
    regmap_read(regmap, AT91_PMC_SR, &mut status);
    (status & AT91_PMC_LOCKU) != 0
}

unsafe fn clk_utmi_prepare(hw: *mut clk_hw) -> i32 {
    let utmi = &mut *to_clk_utmi(hw);
    let mut uckr: u32 = AT91_PMC_UPLLEN | AT91_PMC_UPLLCOUNT | AT91_PMC_BIASEN;
    let utmi_ref_clk_freq: u32;
    let hw_parent: *mut clk_hw;
    let parent_rate: u64;

    /*
     * If mainck rate is different from 12 MHz, we have to configure the
     * FREQ field of the SFR_UTMICKTRIM register to generate properly
     * the utmi clock.
     */
    hw_parent = clk_hw_get_parent(hw);
    parent_rate = clk_hw_get_rate(hw_parent);

    match parent_rate {
        12000000 => utmi_ref_clk_freq = 0,
        16000000 => utmi_ref_clk_freq = 1,
        24000000 => utmi_ref_clk_freq = 2,
        /* Not supported on SAMA5D2 but it is not an issue since MAINCK
         * maximum value is 24 MHz. */
        48000000 => utmi_ref_clk_freq = 3,
        _ => {
            pr_err("UTMICK: unsupported mainck rate\n");
            return -EINVAL;
        }
    }

    if !utmi.regmap_sfr.is_null() {
        regmap_update_bits(utmi.regmap_sfr, AT91_SFR_UTMICKTRIM,
                           AT91_UTMICKTRIM_FREQ, utmi_ref_clk_freq);
    } else if utmi_ref_clk_freq != 0 {
        pr_err("UTMICK: sfr node required\n");
        return -EINVAL;
    }

    regmap_update_bits(utmi.regmap_pmc, AT91_CKGR_UCKR, uckr, uckr);

    while !clk_utmi_ready(utmi.regmap_pmc) {
        cpu_relax();
    }

    0
}

unsafe fn clk_utmi_is_prepared(hw: *mut clk_hw) -> i32 {
    let utmi = &mut *to_clk_utmi(hw);
    clk_utmi_ready(utmi.regmap_pmc) as i32
}

unsafe fn clk_utmi_unprepare(hw: *mut clk_hw) {
    let utmi = &mut *to_clk_utmi(hw);
    regmap_update_bits(utmi.regmap_pmc, AT91_CKGR_UCKR, AT91_PMC_UPLLEN, 0);
}

unsafe fn clk_utmi_recalc_rate(_hw: *mut clk_hw, _parent_rate: u64) -> u64 {
    /* UTMI clk rate is fixed. */
    UTMI_RATE
}

unsafe fn clk_utmi_save_context(hw: *mut clk_hw) -> i32 {
    let utmi = &mut *to_clk_utmi(hw);
    utmi.pms.status = clk_utmi_is_prepared(hw);
    0
}

unsafe fn clk_utmi_restore_context(hw: *mut clk_hw) {
    let utmi = &mut *to_clk_utmi(hw);
    if utmi.pms.status != 0 {
        clk_utmi_prepare(hw);
    }
}

static utmi_ops: clk_ops = clk_ops {
    prepare: Some(clk_utmi_prepare),
    unprepare: Some(clk_utmi_unprepare),
    is_prepared: Some(clk_utmi_is_prepared),
    recalc_rate: Some(clk_utmi_recalc_rate),
    save_context: Some(clk_utmi_save_context),
    restore_context: Some(clk_utmi_restore_context),
};

unsafe fn at91_clk_register_utmi_internal(
    regmap_pmc: *mut regmap,
    regmap_sfr: *mut regmap,
    name: *const i8,
    parent_name: *const i8,
    parent_hw: *mut clk_hw,
    ops: *const clk_ops,
    flags: u32,
) -> *mut clk_hw {
    let utmi: *mut clk_utmi;
    let mut init: clk_init_data = core::mem::zeroed();
    let mut hw: *mut clk_hw;
    let ret: i32;

    if parent_name.is_null() && parent_hw.is_null() {
        return ERR_PTR(-EINVAL);
    }

    utmi = kzalloc_obj();
    if utmi.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    init.name = name;
    init.ops = ops;
    if !parent_hw.is_null() {
        init.parent_hws = &parent_hw as *const *mut clk_hw;
    } else {
        init.parent_names = &parent_name;
    }
    init.num_parents = 1;
    init.flags = flags;

    (*utmi).hw.init = &init;
    (*utmi).regmap_pmc = regmap_pmc;
    (*utmi).regmap_sfr = regmap_sfr;

    hw = &mut (*utmi).hw;
    ret = clk_hw_register(core::ptr::null_mut(), &mut (*utmi).hw);
    if ret != 0 {
        kfree(utmi as *mut core::ffi::c_void);
        hw = ERR_PTR(ret);
    }

    hw
}

pub unsafe fn at91_clk_register_utmi(
    regmap_pmc: *mut regmap,
    regmap_sfr: *mut regmap,
    name: *const i8,
    parent_name: *const i8,
    parent_hw: *mut clk_hw,
) -> *mut clk_hw {
    at91_clk_register_utmi_internal(regmap_pmc, regmap_sfr, name, parent_name,
                                    parent_hw, &utmi_ops, CLK_SET_RATE_GATE)
}

unsafe fn clk_utmi_sama7g5_prepare(hw: *mut clk_hw) -> i32 {
    let utmi = &mut *to_clk_utmi(hw);
    let hw_parent = clk_hw_get_parent(hw);
    let parent_rate = clk_hw_get_rate(hw_parent);
    let val: u32;

    match parent_rate {
        16000000 => val = 0,
        20000000 => val = 2,
        24000000 => val = 3,
        32000000 => val = 5,
        _ => {
            pr_err("UTMICK: unsupported main_xtal rate\n");
            return -EINVAL;
        }
    }

    regmap_write(utmi.regmap_pmc, AT91_PMC_XTALF, val);
    0
}

unsafe fn clk_utmi_sama7g5_is_prepared(hw: *mut clk_hw) -> i32 {
    let utmi = &mut *to_clk_utmi(hw);
    let parent_rate = clk_hw_get_rate(clk_hw_get_parent(hw));
    let mut val: u32 = 0;

    regmap_read(utmi.regmap_pmc, AT91_PMC_XTALF, &mut val);
    match val & 0x7 {
        0 if parent_rate == 16000000 => 1,
        2 if parent_rate == 20000000 => 1,
        3 if parent_rate == 24000000 => 1,
        5 if parent_rate == 32000000 => 1,
        _ => 0,
    }
}

unsafe fn clk_utmi_sama7g5_save_context(hw: *mut clk_hw) -> i32 {
    let utmi = &mut *to_clk_utmi(hw);
    utmi.pms.status = clk_utmi_sama7g5_is_prepared(hw);
    0
}

unsafe fn clk_utmi_sama7g5_restore_context(hw: *mut clk_hw) {
    let utmi = &mut *to_clk_utmi(hw);
    if utmi.pms.status != 0 {
        clk_utmi_sama7g5_prepare(hw);
    }
}

static sama7g5_utmi_ops: clk_ops = clk_ops {
    prepare: Some(clk_utmi_sama7g5_prepare),
    unprepare: None,
    is_prepared: Some(clk_utmi_sama7g5_is_prepared),
    recalc_rate: Some(clk_utmi_recalc_rate),
    save_context: Some(clk_utmi_sama7g5_save_context),
    restore_context: Some(clk_utmi_sama7g5_restore_context),
};

pub unsafe fn at91_clk_sama7g5_register_utmi(
    regmap_pmc: *mut regmap,
    name: *const i8,
    parent_name: *const i8,
    parent_hw: *mut clk_hw,
) -> *mut clk_hw {
    at91_clk_register_utmi_internal(regmap_pmc, core::ptr::null_mut(), name,
                                    parent_name, parent_hw, &sama7g5_utmi_ops, 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
