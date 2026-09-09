// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MMP PLL clock rate calculation
 *
 * Copyright (C) 2020 Lubomir Rintel <lkundrak@v3.sk>
 */

// Linux kernel dependencies supplied by other translation units.

#[repr(C)]
pub struct MmpClkPll {
    pub hw: ClkHw,
    pub default_rate: c_ulong,
    pub enable_reg: *mut c_void,
    pub enable: u32,
    pub reg: *mut c_void,
    pub shift: u8,
    pub input_rate: c_ulong,
    pub postdiv_reg: *mut c_void,
    pub postdiv_shift: u8,
}

unsafe fn mmp_clk_pll_is_enabled(hw: *mut ClkHw) -> c_int {
    let pll = container_of_mmp_clk_pll(hw);
    let val = readl_relaxed((*pll).enable_reg);
    if (val & (*pll).enable) == (*pll).enable {
        return 1;
    }

    /* Some PLLs, if not software controlled, output default clock. */
    if (*pll).default_rate > 0 {
        return 1;
    }

    0
}

unsafe fn mmp_clk_pll_recalc_rate(hw: *mut ClkHw, _parent_rate: c_ulong) -> c_ulong {
    let pll = container_of_mmp_clk_pll(hw);
    let mut fbdiv: u32;
    let mut refdiv: u32;
    let postdiv: u32;
    let mut rate: u64;
    let mut val: u32;

    val = readl_relaxed((*pll).enable_reg);
    if (val & (*pll).enable) != (*pll).enable {
        return (*pll).default_rate;
    }

    if !(*pll).reg.is_null() {
        val = readl_relaxed((*pll).reg);
        fbdiv = (val >> (*pll).shift) & 0x1ff;
        refdiv = (val >> ((*pll).shift + 9)) & 0x1f;
    } else {
        fbdiv = 2;
        refdiv = 1;
    }

    if !(*pll).postdiv_reg.is_null() {
        /* MMP3 clock rate calculation */
        const POSTDIVS: [u8; 9] = [2, 3, 4, 5, 6, 8, 10, 12, 16];

        val = readl_relaxed((*pll).postdiv_reg);
        postdiv = (val >> (*pll).postdiv_shift) & 0x7;

        rate = (*pll).input_rate as u64;
        rate = rate.wrapping_mul((2u32.wrapping_mul(fbdiv)) as u64);
        rate /= refdiv as u64;
        rate /= POSTDIVS[postdiv as usize] as u64;
    } else {
        /* MMP2 clock rate calculation */
        rate = if refdiv == 3 {
            19200000
        } else if refdiv == 4 {
            26000000
        } else {
            pr_err!("bad refdiv: %d (0x%08x)\n", refdiv, val);
            return 0;
        };

        rate = rate.wrapping_mul((fbdiv + 2) as u64);
        rate /= (refdiv + 2) as u64;
    }

    rate as c_ulong
}

static MMP_CLK_PLL_OPS: ClkOps = ClkOps {
    is_enabled: Some(mmp_clk_pll_is_enabled),
    recalc_rate: Some(mmp_clk_pll_recalc_rate),
};

pub unsafe fn mmp_clk_register_pll(
    name: *mut c_char,
    default_rate: c_ulong,
    enable_reg: *mut c_void,
    enable: u32,
    reg: *mut c_void,
    shift: u8,
    input_rate: c_ulong,
    postdiv_reg: *mut c_void,
    postdiv_shift: u8,
) -> *mut Clk {
    let pll = kzalloc_mmp_clk_pll();
    if pll.is_null() {
        return err_ptr(-12);
    }

    let init = ClkInitData {
        name,
        ops: &MMP_CLK_PLL_OPS,
        flags: 0,
        parent_names: core::ptr::null(),
        num_parents: 0,
    };

    (*pll).default_rate = default_rate;
    (*pll).enable_reg = enable_reg;
    (*pll).enable = enable;
    (*pll).reg = reg;
    (*pll).shift = shift;
    (*pll).input_rate = input_rate;
    (*pll).postdiv_reg = postdiv_reg;
    (*pll).postdiv_shift = postdiv_shift;
    (*pll).hw.init = &init;

    let clk = clk_register(core::ptr::null_mut(), &mut (*pll).hw);
    if is_err(clk) {
        kfree(pll);
    }
    clk
}

pub unsafe fn mmp_register_pll_clks(
    unit: *mut MmpClkUnit,
    clks: *mut MmpParamPllClk,
    base: *mut c_void,
    size: c_int,
) {
    for i in 0..size {
        let desc = &*clks.add(i as usize);
        let reg = if desc.offset != 0 {
            base.add(desc.offset as usize)
        } else {
            core::ptr::null_mut()
        };

        let clk = mmp_clk_register_pll(
            desc.name,
            desc.default_rate,
            base.add(desc.enable_offset as usize),
            desc.enable,
            reg,
            desc.shift,
            desc.input_rate,
            base.add(desc.postdiv_offset as usize),
            desc.postdiv_shift,
        );
        if is_err(clk) {
            pr_err!("%s: failed to register clock %s\n", "mmp_register_pll_clks", desc.name);
            continue;
        }
        if desc.id != 0 {
            (*unit).clk_table[desc.id as usize] = clk;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
