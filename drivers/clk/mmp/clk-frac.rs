// SPDX-License-Identifier: GPL-2.0-only
/*
 * mmp factor clock operation source file
 *
 * Copyright (C) 2012 Marvell
 * Chao Xie <xiechao.mail@gmail.com>
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * It is M/N clock
 *
 * Fout from synthesizer can be given from two equations:
 * numerator/denominator = Fin / (Fout * factor)
 */

unsafe fn to_clk_factor(hw: *mut clk_hw) -> *mut mmp_clk_factor {
    // C equivalent: container_of(hw, struct mmp_clk_factor, hw)
    (hw as *mut u8).sub(core::mem::offset_of!(mmp_clk_factor, hw)) as *mut mmp_clk_factor
}

unsafe fn clk_factor_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let factor = &mut *to_clk_factor(hw);
    let mut rate: u64 = 0;
    let mut prev_rate: u64;
    let mut d: *mut u32_fract;
    let mut i: i32 = 0;

    while i < factor.ftbl_cnt as i32 {
        d = factor.ftbl.add(i as usize);

        prev_rate = rate;
        rate = (*req).best_parent_rate as u64 * (*d).denominator as u64;
        rate /= ((*d).numerator as u64) * factor.masks.as_ref().unwrap().factor as u64;
        if rate > (*req).rate as u64 {
            break;
        }
        i += 1;
    }

    if i == 0 || i == factor.ftbl_cnt as i32 {
        (*req).rate = rate as _;
    } else if ((*req).rate as u64 - prev_rate) > (rate - (*req).rate as u64) {
        (*req).rate = rate as _;
    } else {
        (*req).rate = prev_rate as _;
    }

    0
}

unsafe fn clk_factor_recalc_rate(hw: *mut clk_hw, parent_rate: libc::c_ulong) -> libc::c_ulong {
    let factor = &mut *to_clk_factor(hw);
    let masks = &*factor.masks;
    let mut d = u32_fract { numerator: 0, denominator: 0 };
    let val: u32 = readl_relaxed(factor.base);

    // calculate numerator
    d.numerator = (val >> masks.num_shift) & masks.num_mask;

    // calculate denominator
    d.denominator = (val >> masks.den_shift) & masks.den_mask;
    if d.denominator == 0 {
        return 0;
    }

    let mut rate = parent_rate as u64 * d.denominator as u64;
    rate /= d.numerator as u64 * factor.masks.as_ref().unwrap().factor as u64;
    rate as _
}

// Configures new clock rate
unsafe fn clk_factor_set_rate(
    hw: *mut clk_hw,
    drate: libc::c_ulong,
    prate: libc::c_ulong,
) -> i32 {
    let factor = &mut *to_clk_factor(hw);
    let masks = &*factor.masks;
    let mut i: i32 = 0;
    let mut flags: libc::c_ulong = 0;
    let mut rate: u64 = 0;

    while i < factor.ftbl_cnt as i32 {
        let d = &*factor.ftbl.add(i as usize);
        rate = prate as u64 * d.denominator as u64;
        rate /= d.numerator as u64 * factor.masks.as_ref().unwrap().factor as u64;
        if rate > drate as u64 {
            break;
        }
        i += 1;
    }
    let d = &*factor.ftbl.add(if i != 0 { (i - 1) as usize } else { 0 });

    if !factor.lock.is_null() {
        spin_lock_irqsave(factor.lock, &mut flags);
    }

    let mut val = readl_relaxed(factor.base);
    val &= !(masks.num_mask << masks.num_shift);
    val |= (d.numerator & masks.num_mask) << masks.num_shift;
    val &= !(masks.den_mask << masks.den_shift);
    val |= (d.denominator & masks.den_mask) << masks.den_shift;
    writel_relaxed(val, factor.base);

    if !factor.lock.is_null() {
        spin_unlock_irqrestore(factor.lock, flags);
    }
    0
}

unsafe fn clk_factor_init(hw: *mut clk_hw) -> i32 {
    let factor = &mut *to_clk_factor(hw);
    let masks = &*factor.masks;
    let mut d = u32_fract { numerator: 0, denominator: 0 };
    let mut flags: libc::c_ulong = 0;

    if !factor.lock.is_null() {
        spin_lock_irqsave(factor.lock, &mut flags);
    }

    let mut val = readl(factor.base);
    // calculate numerator
    d.numerator = (val >> masks.num_shift) & masks.num_mask;
    // calculate denominator
    d.denominator = (val >> masks.den_shift) & masks.den_mask;

    let mut i: i32 = 0;
    while i < factor.ftbl_cnt as i32 {
        let entry = &*factor.ftbl.add(i as usize);
        if d.denominator == entry.denominator && d.numerator == entry.numerator {
            break;
        }
        i += 1;
    }

    if i >= factor.ftbl_cnt as i32 {
        let first = &*factor.ftbl;
        val &= !(masks.num_mask << masks.num_shift);
        val |= (first.numerator & masks.num_mask) << masks.num_shift;
        val &= !(masks.den_mask << masks.den_shift);
        val |= (first.denominator & masks.den_mask) << masks.den_shift;
    }

    if (val & masks.enable_mask) == 0 || i >= factor.ftbl_cnt as i32 {
        val |= masks.enable_mask;
        writel(val, factor.base);
    }

    if !factor.lock.is_null() {
        spin_unlock_irqrestore(factor.lock, flags);
    }
    0
}

static clk_factor_ops: clk_ops = clk_ops {
    recalc_rate: Some(clk_factor_recalc_rate),
    determine_rate: Some(clk_factor_determine_rate),
    set_rate: Some(clk_factor_set_rate),
    init: Some(clk_factor_init),
};

unsafe fn mmp_clk_register_factor(
    name: *const libc::c_char,
    parent_name: *const libc::c_char,
    flags: libc::c_ulong,
    base: *mut core::ffi::c_void,
    masks: *mut mmp_clk_factor_masks,
    ftbl: *mut u32_fract,
    ftbl_cnt: libc::c_uint,
    lock: *mut spinlock_t,
) -> *mut clk {
    if masks.is_null() {
        pr_err!("%s: must pass a clk_factor_mask\n", c_str!("__func__"));
        return ERR_PTR(-EINVAL);
    }

    let factor = kzalloc_obj::<mmp_clk_factor>();
    if factor.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*factor).base = base;
    (*factor).masks = masks;
    (*factor).ftbl = ftbl;
    (*factor).ftbl_cnt = ftbl_cnt;
    (*factor).lock = lock;

    let init = clk_init_data {
        name,
        ops: &clk_factor_ops,
        flags,
        parent_names: &parent_name,
        num_parents: 1,
    };
    (*factor).hw.init = &init;

    let clk = clk_register(core::ptr::null_mut(), &mut (*factor).hw);
    if IS_ERR_OR_NULL(clk) {
        kfree(factor);
    }
    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
