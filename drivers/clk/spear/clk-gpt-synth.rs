// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 *
 * General Purpose Timer Synthesizer clock implementation
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/clk-provider.h, linux/slab.h, linux/io.h, linux/err.h, and clk.h.

const GPT_MSCALE_MASK: u32 = 0xFFF;
const GPT_NSCALE_SHIFT: u32 = 12;
const GPT_NSCALE_MASK: u32 = 0xF;

/*
 * DOC: General Purpose Timer Synthesizer clock
 *
 * Calculates gpt synth clk rate for different values of mscale and nscale
 *
 * Fout= Fin/((2 ^ (N+1)) * (M+1))
 */

unsafe fn gpt_calc_rate(hw: *mut clk_hw, mut prate: libc::c_ulong, index: libc::c_int) -> libc::c_ulong {
    let gpt = container_of_clk_gpt(hw);
    let rtbl = (*gpt).rtbl;

    prate /= ((1i32 << ((*rtbl.add(index as usize)).nscale + 1))
        * ((*rtbl.add(index as usize)).mscale + 1)) as libc::c_ulong;

    prate
}

unsafe fn clk_gpt_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> libc::c_int {
    let gpt = container_of_clk_gpt(hw);
    let mut unused: libc::c_int = 0;

    (*req).rate = clk_round_rate_index(
        hw,
        (*req).rate,
        (*req).best_parent_rate,
        Some(gpt_calc_rate),
        (*gpt).rtbl_cnt,
        &mut unused,
    );

    0
}

unsafe fn clk_gpt_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: libc::c_ulong,
) -> libc::c_ulong {
    let gpt = container_of_clk_gpt(hw);
    let mut flags: libc::c_ulong = 0;
    let mut div: libc::c_uint = 1;
    let val: libc::c_uint;

    if !(*gpt).lock.is_null() {
        spin_lock_irqsave((*gpt).lock, &mut flags);
    }

    val = readl_relaxed((*gpt).reg);

    if !(*gpt).lock.is_null() {
        spin_unlock_irqrestore((*gpt).lock, flags);
    }

    div += val & GPT_MSCALE_MASK;
    div *= 1 << (((val >> GPT_NSCALE_SHIFT) & GPT_NSCALE_MASK) + 1);

    if div == 0 {
        return 0;
    }

    parent_rate / div as libc::c_ulong
}

/* Configures new clock rate of gpt */
unsafe fn clk_gpt_set_rate(
    hw: *mut clk_hw,
    drate: libc::c_ulong,
    prate: libc::c_ulong,
) -> libc::c_int {
    let gpt = container_of_clk_gpt(hw);
    let rtbl = (*gpt).rtbl;
    let mut flags: libc::c_ulong = 0;
    let mut val: libc::c_uint;
    let mut i: libc::c_int = 0;

    clk_round_rate_index(
        hw,
        drate,
        prate,
        Some(gpt_calc_rate),
        (*gpt).rtbl_cnt,
        &mut i,
    );

    if !(*gpt).lock.is_null() {
        spin_lock_irqsave((*gpt).lock, &mut flags);
    }

    val = readl((*gpt).reg) & !GPT_MSCALE_MASK;
    val &= !(GPT_NSCALE_MASK << GPT_NSCALE_SHIFT);

    val |= (*rtbl.add(i as usize)).mscale & GPT_MSCALE_MASK;
    val |= ((*rtbl.add(i as usize)).nscale & GPT_NSCALE_MASK) << GPT_NSCALE_SHIFT;

    writel_relaxed(val, (*gpt).reg);

    if !(*gpt).lock.is_null() {
        spin_unlock_irqrestore((*gpt).lock, flags);
    }

    0
}

static clk_gpt_ops: clk_ops = clk_ops {
    recalc_rate: Some(clk_gpt_recalc_rate),
    determine_rate: Some(clk_gpt_determine_rate),
    set_rate: Some(clk_gpt_set_rate),
};

pub unsafe fn clk_register_gpt(
    name: *const libc::c_char,
    parent_name: *const libc::c_char,
    flags: libc::c_ulong,
    reg: *mut libc::c_void,
    rtbl: *mut gpt_rate_tbl,
    rtbl_cnt: u8,
    lock: *mut spinlock_t,
) -> *mut clk {
    let mut init: clk_init_data = core::mem::zeroed();
    let gpt: *mut clk_gpt;
    let clk: *mut clk;

    if name.is_null() || parent_name.is_null() || reg.is_null() || rtbl.is_null() || rtbl_cnt == 0 {
        pr_err(c"Invalid arguments passed\n");
        return err_ptr(-libc::EINVAL);
    }

    gpt = kzalloc_clk_gpt();
    if gpt.is_null() {
        return err_ptr(-libc::ENOMEM);
    }

    /* struct clk_gpt assignments */
    (*gpt).reg = reg;
    (*gpt).rtbl = rtbl;
    (*gpt).rtbl_cnt = rtbl_cnt;
    (*gpt).lock = lock;
    (*gpt).hw.init = &mut init;

    init.name = name;
    init.ops = &clk_gpt_ops;
    init.flags = flags;
    init.parent_names = &parent_name;
    init.num_parents = 1;

    clk = clk_register(core::ptr::null_mut(), &mut (*gpt).hw);
    if !is_err_or_null(clk) {
        return clk;
    }

    pr_err(c"clk register failed\n");
    kfree(gpt);

    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
