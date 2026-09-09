// SPDX-License-Identifier: GPL-2.0
// Translated from C; kernel dependencies are supplied externally.

const MXC_PLL_DP_CTL: u32 = 0x00;
const MXC_PLL_DP_CONFIG: u32 = 0x04;
const MXC_PLL_DP_OP: u32 = 0x08;
const MXC_PLL_DP_MFD: u32 = 0x0C;
const MXC_PLL_DP_MFN: u32 = 0x10;
const MXC_PLL_DP_MFNMINUS: u32 = 0x14;
const MXC_PLL_DP_MFNPLUS: u32 = 0x18;
const MXC_PLL_DP_HFS_OP: u32 = 0x1C;
const MXC_PLL_DP_HFS_MFD: u32 = 0x20;
const MXC_PLL_DP_HFS_MFN: u32 = 0x24;
const MXC_PLL_DP_MFN_TOGC: u32 = 0x28;
const MXC_PLL_DP_DESTAT: u32 = 0x2c;

const MXC_PLL_DP_CTL_MUL_CTRL: u32 = 0x2000;
const MXC_PLL_DP_CTL_DPDCK0_2_EN: u32 = 0x1000;
const MXC_PLL_DP_CTL_DPDCK0_2_OFFSET: u32 = 12;
const MXC_PLL_DP_CTL_ADE: u32 = 0x800;
const MXC_PLL_DP_CTL_REF_CLK_DIV: u32 = 0x400;
const MXC_PLL_DP_CTL_REF_CLK_SEL_MASK: u32 = 3 << 8;
const MXC_PLL_DP_CTL_REF_CLK_SEL_OFFSET: u32 = 8;
const MXC_PLL_DP_CTL_HFSM: u32 = 0x80;
const MXC_PLL_DP_CTL_PRE: u32 = 0x40;
const MXC_PLL_DP_CTL_UPEN: u32 = 0x20;
const MXC_PLL_DP_CTL_RST: u32 = 0x10;
const MXC_PLL_DP_CTL_RCP: u32 = 0x8;
const MXC_PLL_DP_CTL_PLM: u32 = 0x4;
const MXC_PLL_DP_CTL_BRM0: u32 = 0x2;
const MXC_PLL_DP_CTL_LRF: u32 = 0x1;

const MXC_PLL_DP_CONFIG_BIST: u32 = 0x8;
const MXC_PLL_DP_CONFIG_SJC_CE: u32 = 0x4;
const MXC_PLL_DP_CONFIG_AREN: u32 = 0x2;
const MXC_PLL_DP_CONFIG_LDREQ: u32 = 0x1;
const MXC_PLL_DP_OP_MFI_OFFSET: u32 = 4;
const MXC_PLL_DP_OP_MFI_MASK: u32 = 0xF << 4;
const MXC_PLL_DP_OP_PDF_OFFSET: u32 = 0;
const MXC_PLL_DP_OP_PDF_MASK: u32 = 0xF;
const MXC_PLL_DP_MFD_OFFSET: u32 = 0;
const MXC_PLL_DP_MFD_MASK: u32 = 0x07FFFFFF;
const MXC_PLL_DP_MFN_OFFSET: u32 = 0x0;
const MXC_PLL_DP_MFN_MASK: u32 = 0x07FFFFFF;
const MXC_PLL_DP_MFN_TOGC_TOG_DIS: u32 = 1 << 17;
const MXC_PLL_DP_MFN_TOGC_TOG_EN: u32 = 1 << 16;
const MXC_PLL_DP_MFN_TOGC_CNT_OFFSET: u32 = 0x0;
const MXC_PLL_DP_MFN_TOGC_CNT_MASK: u32 = 0xFFFF;
const MXC_PLL_DP_DESTAT_TOG_SEL: u32 = 1 << 31;
const MXC_PLL_DP_DESTAT_MFN: u32 = 0x07FFFFFF;
const MAX_DPLL_WAIT_TRIES: i32 = 1000;

#[repr(C)]
pub struct clk_pllv2 {
    pub hw: clk_hw,
    pub base: *mut core::ffi::c_void,
}

unsafe fn __clk_pllv2_recalc_rate(parent_rate: usize, dp_ctl: u32, dp_op: u32,
                                  dp_mfd: u32, dp_mfn: u32) -> usize {
    let dbl = dp_ctl & MXC_PLL_DP_CTL_DPDCK0_2_EN;
    let pdf = (dp_op & MXC_PLL_DP_OP_PDF_MASK) as isize;
    let mut mfi = ((dp_op & MXC_PLL_DP_OP_MFI_MASK) >> MXC_PLL_DP_OP_MFI_OFFSET) as isize;
    mfi = if mfi <= 5 { 5 } else { mfi };
    let mfd = (dp_mfd & MXC_PLL_DP_MFD_MASK) as isize;
    let raw_mfn = dp_mfn & MXC_PLL_DP_MFN_MASK;
    let mfn = ((raw_mfn << 6) as i32 >> 6) as isize;
    let mut ref_clk = 2 * parent_rate as isize;
    if dbl != 0 { ref_clk *= 2; }
    ref_clk /= pdf + 1;
    let mut temp = (ref_clk.unsigned_abs() * mfn.unsigned_abs()) / (mfd as usize + 1);
    if mfn < 0 { temp = (ref_clk * mfi) as usize - temp; }
    else { temp = (ref_clk * mfi) as usize + temp; }
    temp
}

unsafe fn clk_pllv2_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let pll = &*(hw as *mut clk_pllv2);
    let base = pll.base as *mut u8;
    let dp_ctl = __raw_readl(base.add(MXC_PLL_DP_CTL as usize));
    let dp_op = __raw_readl(base.add(MXC_PLL_DP_OP as usize));
    let dp_mfd = __raw_readl(base.add(MXC_PLL_DP_MFD as usize));
    let dp_mfn = __raw_readl(base.add(MXC_PLL_DP_MFN as usize));
    __clk_pllv2_recalc_rate(parent_rate, dp_ctl, dp_op, dp_mfd, dp_mfn)
}

unsafe fn __clk_pllv2_set_rate(rate: usize, parent_rate: usize, dp_op: *mut u32,
                               dp_mfd: *mut u32, dp_mfn: *mut u32) -> i32 {
    let quad_parent_rate = 4 * parent_rate;
    let mut pdf: isize = -1;
    let mut mfi: isize = -1;
    while { pdf += 1; pdf < 16 && mfi < 5 } {
        mfi = (rate * (pdf as usize + 1) / quad_parent_rate) as isize;
    }
    if mfi > 15 { return -22; }
    pdf -= 1;
    let temp64 = (rate * (pdf as usize + 1) - quad_parent_rate * mfi as usize)
        / (quad_parent_rate / 1_000_000);
    *dp_op = (mfi as u32) << 4 | pdf as u32;
    *dp_mfd = 999999;
    *dp_mfn = temp64 as u32;
    0
}

unsafe fn clk_pllv2_set_rate(hw: *mut clk_hw, rate: usize, parent_rate: usize) -> i32 {
    let pll = &*(hw as *mut clk_pllv2);
    let base = pll.base as *mut u8;
    let mut dp_op = 0; let mut dp_mfd = 0; let mut dp_mfn = 0;
    let ret = __clk_pllv2_set_rate(rate, parent_rate, &mut dp_op, &mut dp_mfd, &mut dp_mfn);
    if ret != 0 { return ret; }
    let dp_ctl = __raw_readl(base.add(MXC_PLL_DP_CTL as usize));
    __raw_writel(dp_ctl | 0x1000, base.add(MXC_PLL_DP_CTL as usize));
    __raw_writel(dp_op, base.add(MXC_PLL_DP_OP as usize));
    __raw_writel(dp_mfd, base.add(MXC_PLL_DP_MFD as usize));
    __raw_writel(dp_mfn, base.add(MXC_PLL_DP_MFN as usize));
    0
}

unsafe fn clk_pllv2_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let mut dp_op = 0; let mut dp_mfd = 0; let mut dp_mfn = 0;
    let ret = __clk_pllv2_set_rate((*req).rate, (*req).best_parent_rate,
                                   &mut dp_op, &mut dp_mfd, &mut dp_mfn);
    if ret != 0 { (*req).rate = ret as usize; return 0; }
    (*req).rate = __clk_pllv2_recalc_rate((*req).best_parent_rate,
        MXC_PLL_DP_CTL_DPDCK0_2_EN, dp_op, dp_mfd, dp_mfn);
    0
}

unsafe fn clk_pllv2_prepare(hw: *mut clk_hw) -> i32 {
    let pll = &*(hw as *mut clk_pllv2); let base = pll.base as *mut u8;
    let mut reg = __raw_readl(base.add(MXC_PLL_DP_CTL as usize)) | MXC_PLL_DP_CTL_UPEN;
    __raw_writel(reg, base.add(MXC_PLL_DP_CTL as usize));
    let mut i = 0;
    loop {
        reg = __raw_readl(base.add(MXC_PLL_DP_CTL as usize));
        if reg & MXC_PLL_DP_CTL_LRF != 0 { break; }
        udelay(1);
        i += 1;
        if i >= MAX_DPLL_WAIT_TRIES { pr_err("MX5: pll locking failed\n"); return -22; }
    }
    0
}

unsafe fn clk_pllv2_unprepare(hw: *mut clk_hw) {
    let pll = &*(hw as *mut clk_pllv2); let base = pll.base as *mut u8;
    let reg = __raw_readl(base.add(MXC_PLL_DP_CTL as usize)) & !MXC_PLL_DP_CTL_UPEN;
    __raw_writel(reg, base.add(MXC_PLL_DP_CTL as usize));
}

static clk_pllv2_ops: clk_ops = clk_ops {
    prepare: Some(clk_pllv2_prepare),
    unprepare: Some(clk_pllv2_unprepare),
    recalc_rate: Some(clk_pllv2_recalc_rate),
    determine_rate: Some(clk_pllv2_determine_rate),
    set_rate: Some(clk_pllv2_set_rate),
};

pub unsafe fn imx_clk_hw_pllv2(name: *const core::ffi::c_char,
                               parent: *const core::ffi::c_char,
                               base: *mut core::ffi::c_void) -> *mut clk_hw {
    let pll = kzalloc_obj::<clk_pllv2>();
    if pll.is_null() { return ERR_PTR(-12); }
    (*pll).base = base;
    let init = clk_init_data { name, ops: &clk_pllv2_ops, flags: 0,
        parent_names: &parent, num_parents: 1 };
    (*pll).hw.init = &init;
    let ret = clk_hw_register(core::ptr::null_mut(), &mut (*pll).hw);
    if ret != 0 { kfree(pll); return ERR_PTR(ret); }
    &mut (*pll).hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
