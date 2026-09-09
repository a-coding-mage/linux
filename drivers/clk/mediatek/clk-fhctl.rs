// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Edward-JW Yang <edward-jw.yang@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const fn percent_to_ddslmt(dds: u32, percent_m10: u32) -> u32 {
    (((dds.wrapping_mul(percent_m10)) >> 5) / 100)
}

static FHCTL_OFFSET_V1: fhctl_offset = fhctl_offset {
    offset_hp_en: 0x0,
    offset_clk_con: 0x4,
    offset_rst_con: 0x8,
    offset_slope0: 0xc,
    offset_slope1: 0x10,
    offset_cfg: 0x0,
    offset_updnlmt: 0x4,
    offset_dds: 0x8,
    offset_dvfs: 0xc,
    offset_mon: 0x10,
};

static FHCTL_OFFSET_V2: fhctl_offset = fhctl_offset {
    offset_hp_en: 0x0,
    offset_clk_con: 0x8,
    offset_rst_con: 0xc,
    offset_slope0: 0x10,
    offset_slope1: 0x14,
    offset_cfg: 0x0,
    offset_updnlmt: 0x4,
    offset_dds: 0x8,
    offset_dvfs: 0xc,
    offset_mon: 0x10,
};

pub unsafe fn fhctl_get_offset_table(v: fhctl_variant) -> *const fhctl_offset {
    match v {
        FHCTL_PLLFH_V1 => &FHCTL_OFFSET_V1,
        FHCTL_PLLFH_V2 => &FHCTL_OFFSET_V2,
        _ => ERR_PTR(-EINVAL),
    }
}

unsafe fn dump_hw(pll: *mut mtk_clk_pll, regs: *mut fh_pll_regs, data: *const fh_pll_data) {
    pr_info!("hp_en<%x>,clk_con<%x>,slope0<%x>,slope1<%x>\n",
        readl((*regs).reg_hp_en), readl((*regs).reg_clk_con),
        readl((*regs).reg_slope0), readl((*regs).reg_slope1));
    pr_info!("cfg<%x>,lmt<%x>,dds<%x>,dvfs<%x>,mon<%x>\n",
        readl((*regs).reg_cfg), readl((*regs).reg_updnlmt),
        readl((*regs).reg_dds), readl((*regs).reg_dvfs),
        readl((*regs).reg_mon));
    pr_info!("pcw<%x>\n", readl((*pll).pcw_addr));
}

unsafe fn fhctl_set_ssc_regs(pll: *mut mtk_clk_pll, regs: *mut fh_pll_regs,
                             data: *const fh_pll_data, rate: u32) -> i32 {
    let mut updnlmt_val: u32;
    let mut r: u32;

    writel(readl((*regs).reg_cfg) & !(*data).frddsx_en, (*regs).reg_cfg);
    writel(readl((*regs).reg_cfg) & !(*data).sfstrx_en, (*regs).reg_cfg);
    writel(readl((*regs).reg_cfg) & !(*data).fhctlx_en, (*regs).reg_cfg);

    if rate > 0 {
        r = readl((*regs).reg_cfg);
        r &= !(*data).msk_frddsx_dys;
        r |= (*data).df_val << (ffs((*data).msk_frddsx_dys) - 1);
        writel(r, (*regs).reg_cfg);

        r = readl((*regs).reg_cfg);
        r &= !(*data).msk_frddsx_dts;
        r |= (*data).dt_val << (ffs((*data).msk_frddsx_dts) - 1);
        writel(r, (*regs).reg_cfg);

        writel((readl((*pll).pcw_addr) & (*data).dds_mask) | (*data).tgl_org,
               (*regs).reg_dds);
        updnlmt_val = percent_to_ddslmt(readl((*regs).reg_dds) & (*data).dds_mask, rate)
            << (*data).updnlmt_shft;
        writel(updnlmt_val, (*regs).reg_updnlmt);
        writel(readl((*regs).reg_hp_en) | BIT((*data).fh_id), (*regs).reg_hp_en);
        writel(readl((*regs).reg_cfg) | (*data).frddsx_en, (*regs).reg_cfg);
        writel(readl((*regs).reg_cfg) | (*data).fhctlx_en, (*regs).reg_cfg);
    } else {
        writel(readl((*regs).reg_hp_en) & !BIT((*data).fh_id), (*regs).reg_hp_en);
        udelay(30);
    }
    0
}

unsafe fn hopping_hw_flow(pll: *mut mtk_clk_pll, regs: *mut fh_pll_regs,
                          data: *const fh_pll_data, state: *mut fh_pll_state,
                          new_dds: u32) -> i32 {
    let dds_mask = (*data).dds_mask;
    let mut mon_dds = 0;
    let mut con_pcw_tmp: u32;
    let ret: i32;

    if (*state).ssc_rate != 0 { fhctl_set_ssc_regs(pll, regs, data, 0); }
    writel((readl((*pll).pcw_addr) & dds_mask) | (*data).tgl_org, (*regs).reg_dds);
    writel(readl((*regs).reg_cfg) | (*data).sfstrx_en, (*regs).reg_cfg);
    writel(readl((*regs).reg_cfg) | (*data).fhctlx_en, (*regs).reg_cfg);
    writel((*data).slope0_value, (*regs).reg_slope0);
    writel((*data).slope1_value, (*regs).reg_slope1);
    writel(readl((*regs).reg_hp_en) | BIT((*data).fh_id), (*regs).reg_hp_en);
    writel(new_dds | (*data).dvfs_tri, (*regs).reg_dvfs);
    ret = readl_poll_timeout_atomic((*regs).reg_mon, &mut mon_dds,
        (mon_dds & dds_mask) == new_dds, 10, 1000);
    if ret != 0 { pr_warn!("{}: FHCTL hopping timeout\n", (*(*pll).data).name); dump_hw(pll, regs, data); }
    con_pcw_tmp = readl((*pll).pcw_addr) & !dds_mask;
    con_pcw_tmp |= readl((*regs).reg_mon) & dds_mask;
    con_pcw_tmp |= (*data).pcwchg;
    writel(con_pcw_tmp, (*pll).pcw_addr);
    writel(readl((*regs).reg_hp_en) & !BIT((*data).fh_id), (*regs).reg_hp_en);
    if (*state).ssc_rate != 0 { fhctl_set_ssc_regs(pll, regs, data, (*state).ssc_rate); }
    ret
}

unsafe fn __get_postdiv(pll: *mut mtk_clk_pll) -> u32 {
    BIT((readl((*pll).pd_addr) >> (*(*pll).data).pd_shift) & POSTDIV_MASK)
}

unsafe fn __set_postdiv(pll: *mut mtk_clk_pll, postdiv: u32) {
    let mut regval = readl((*pll).pd_addr);
    regval &= !(POSTDIV_MASK << (*(*pll).data).pd_shift);
    regval |= (ffs(postdiv) - 1) << (*(*pll).data).pd_shift;
    writel(regval, (*pll).pd_addr);
}

unsafe fn fhctl_hopping(fh: *mut mtk_fh, new_dds: u32, postdiv: u32) -> i32 {
    let data = &(*(*fh).pllfh_data).data;
    let state = &mut (*(*fh).pllfh_data).state;
    let regs = &mut (*fh).regs;
    let pll = &mut (*fh).clk_pll;
    let lock = (*fh).lock;
    let mut flags: u64 = 0;
    let mut pll_postdiv = 0;
    if postdiv != 0 { pll_postdiv = __get_postdiv(pll); if postdiv > pll_postdiv { __set_postdiv(pll, postdiv); } }
    spin_lock_irqsave(lock, &mut flags);
    let ret = hopping_hw_flow(pll, regs, data, state, new_dds);
    spin_unlock_irqrestore(lock, flags);
    if postdiv != 0 && postdiv < pll_postdiv { __set_postdiv(pll, postdiv); }
    ret
}

unsafe fn fhctl_ssc_enable(fh: *mut mtk_fh, rate: u32) -> i32 {
    let data = &(*(*fh).pllfh_data).data;
    let state = &mut (*(*fh).pllfh_data).state;
    let regs = &mut (*fh).regs;
    let pll = &mut (*fh).clk_pll;
    let lock = (*fh).lock;
    let mut flags: u64 = 0;
    spin_lock_irqsave(lock, &mut flags);
    fhctl_set_ssc_regs(pll, regs, data, rate);
    state.ssc_rate = rate;
    spin_unlock_irqrestore(lock, flags);
    0
}

static FHCTL_OPS: fh_operation = fh_operation {
    hopping: fhctl_hopping,
    ssc_enable: fhctl_ssc_enable,
};

pub fn fhctl_get_ops() -> *const fh_operation { &FHCTL_OPS }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
