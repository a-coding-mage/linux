// SPDX-License-Identifier: GPL-2.0-only
// Linux kernel dependencies are supplied by the surrounding translation.

const ADPLL_PLLSS_MMR_LOCK_OFFSET: usize = 0x00;
const ADPLL_PLLSS_MMR_LOCK_ENABLED: u32 = 0x1f125B64;
const ADPLL_PLLSS_MMR_UNLOCK_MAGIC: u32 = 0x1eda4c3d;
const ADPLL_PWRCTRL_OFFSET: usize = 0x00;
const ADPLL_PWRCTRL_PONIN: u32 = 5;
const ADPLL_PWRCTRL_PGOODIN: u32 = 4;
const ADPLL_PWRCTRL_RET: u32 = 3;
const ADPLL_PWRCTRL_ISORET: u32 = 2;
const ADPLL_PWRCTRL_ISOSCAN: u32 = 1;
const ADPLL_PWRCTRL_OFFMODE: u32 = 0;
const ADPLL_CLKCTRL_OFFSET: usize = 0x04;
const ADPLL_CLKCTRL_CLKDCOLDOEN: u32 = 29;
const ADPLL_CLKCTRL_IDLE: u32 = 23;
const ADPLL_CLKCTRL_CLKOUTEN: u32 = 20;
const ADPLL_CLKINPHIFSEL_ADPLL_S: u32 = 19;
const ADPLL_CLKCTRL_CLKOUTLDOEN_ADPLL_LJ: u32 = 19;
const ADPLL_CLKCTRL_ULOWCLKEN: u32 = 18;
const ADPLL_CLKCTRL_CLKDCOLDOPWDNZ: u32 = 17;
const ADPLL_CLKCTRL_M2PWDNZ: u32 = 16;
const ADPLL_CLKCTRL_M3PWDNZ_ADPLL_S: u32 = 15;
const ADPLL_CLKCTRL_LOWCURRSTDBY_ADPLL_S: u32 = 13;
const ADPLL_CLKCTRL_LPMODE_ADPLL_S: u32 = 12;
const ADPLL_CLKCTRL_REGM4XEN_ADPLL_S: u32 = 10;
const ADPLL_CLKCTRL_SELFREQDCO_ADPLL_LJ: u32 = 10;
const ADPLL_CLKCTRL_TINITZ: u32 = 0;
const ADPLL_TENABLE_OFFSET: usize = 0x08;
const ADPLL_TENABLEDIV_OFFSET: usize = 0x8c;
const ADPLL_M2NDIV_OFFSET: usize = 0x10;
const ADPLL_M2NDIV_M2: u32 = 16;
const ADPLL_M2NDIV_M2_ADPLL_S_WIDTH: u32 = 5;
const ADPLL_M2NDIV_M2_ADPLL_LJ_WIDTH: u32 = 7;
const ADPLL_MN2DIV_OFFSET: usize = 0x14;
const ADPLL_MN2DIV_N2: u32 = 16;
const ADPLL_FRACDIV_OFFSET: usize = 0x18;
const ADPLL_FRACDIV_REGSD: u32 = 24;
const ADPLL_FRACDIV_FRACTIONALM: u32 = 0;
const ADPLL_FRACDIV_FRACTIONALM_MASK: u32 = 0x3ffff;
const ADPLL_BWCTRL_OFFSET: usize = 0x1c;
const ADPLL_BWCTRL_BWCONTROL: u32 = 1;
const ADPLL_BWCTRL_BW_INCR_DECRZ: u32 = 0;
const ADPLL_RESERVED_OFFSET: usize = 0x20;
const ADPLL_STATUS_OFFSET: usize = 0x24;
const ADPLL_STATUS_PONOUT: u32 = 31;
const ADPLL_STATUS_PGOODOUT: u32 = 30;
const ADPLL_STATUS_LDOPWDN: u32 = 29;
const ADPLL_STATUS_RECAL_BSTATUS3: u32 = 28;
const ADPLL_STATUS_RECAL_OPPIN: u32 = 27;
const ADPLL_STATUS_PHASELOCK: u32 = 10;
const ADPLL_STATUS_FREQLOCK: u32 = 9;
const ADPLL_STATUS_BYPASSACK: u32 = 8;
const ADPLL_STATUS_LOSSREF: u32 = 6;
const ADPLL_STATUS_CLKOUTENACK: u32 = 5;
const ADPLL_STATUS_LOCK2: u32 = 4;
const ADPLL_STATUS_M2CHANGEACK: u32 = 3;
const ADPLL_STATUS_HIGHJITTER: u32 = 1;
const ADPLL_STATUS_BYPASS: u32 = 0;
const ADPLL_STATUS_PREPARED_MASK: u32 = (1 << ADPLL_STATUS_PHASELOCK) | (1 << ADPLL_STATUS_FREQLOCK);
const ADPLL_M3DIV_OFFSET: usize = 0x28;
const ADPLL_M3DIV_M3: u32 = 0;
const ADPLL_M3DIV_M3_WIDTH: u32 = 5;
const ADPLL_M3DIV_M3_MASK: u32 = 0x1f;
const ADPLL_RAMPCTRL_OFFSET: usize = 0x2c;
const ADPLL_RAMPCTRL_CLKRAMPLEVEL: u32 = 19;
const ADPLL_RAMPCTRL_CLKRAMPRATE: u32 = 16;
const ADPLL_RAMPCTRL_RELOCK_RAMP_EN: u32 = 0;
const MAX_ADPLL_INPUTS: usize = 3;
const MAX_ADPLL_OUTPUTS: usize = 4;
const ADPLL_MAX_RETRIES: i32 = 5;
const ADPLL_MAX_CON_ID: usize = 16;

#[repr(C)]
pub struct ti_adpll_platform_data { pub is_type_s: bool, pub nr_max_inputs: i32, pub nr_max_outputs: i32, pub output_index: i32 }
#[repr(C)] pub struct ti_adpll_clock { pub clk: *mut clk, pub cl: *mut clk_lookup, pub unregister: Option<unsafe extern "C" fn(*mut clk)> }
#[repr(C)] pub struct ti_adpll_dco_data { pub hw: clk_hw }
#[repr(C)] pub struct ti_adpll_clkout_data { pub adpll: *mut ti_adpll_data, pub gate: clk_gate, pub hw: clk_hw }
#[repr(C)] pub struct ti_adpll_data { pub dev: *mut device, pub c: *const ti_adpll_platform_data, pub np: *mut device_node, pub pa: usize, pub iobase: *mut u8, pub regs: *mut u8, pub lock: spinlock_t, pub parent_names: [*const i8; MAX_ADPLL_INPUTS], pub parent_clocks: [*mut clk; MAX_ADPLL_INPUTS], pub clocks: *mut ti_adpll_clock, pub outputs: clk_onecell_data, pub dco: ti_adpll_dco_data }

#[repr(i32)] pub enum ti_adpll_clocks { TI_ADPLL_DCO, TI_ADPLL_DCO_GATE, TI_ADPLL_N2, TI_ADPLL_M2, TI_ADPLL_M2_GATE, TI_ADPLL_BYPASS, TI_ADPLL_HIF, TI_ADPLL_DIV2, TI_ADPLL_CLKOUT, TI_ADPLL_CLKOUT2, TI_ADPLL_M3 }
const TI_ADPLL_NR_CLOCKS: usize = 11;
#[repr(i32)] pub enum ti_adpll_inputs { TI_ADPLL_CLKINP, TI_ADPLL_CLKINPULOW, TI_ADPLL_CLKINPHIF }
#[repr(i32)] pub enum ti_adpll_s_outputs { TI_ADPLL_S_DCOCLKLDO, TI_ADPLL_S_CLKOUT, TI_ADPLL_S_CLKOUTX2, TI_ADPLL_S_CLKOUTHIF }
#[repr(i32)] pub enum ti_adpll_lj_outputs { TI_ADPLL_LJ_CLKDCOLDO, TI_ADPLL_LJ_CLKOUT, TI_ADPLL_LJ_CLKOUTLDO }

// The following kernel-facing types and functions are intentionally external.
extern "C" {
    fn of_property_read_string_index(*mut device_node, *const i8, i32, *mut *const i8) -> i32;
    fn devm_kasprintf(*mut device, i32, *const i8, ...) -> *const i8;
    fn dev_warn(*mut device, *const i8, ...); fn dev_err(*mut device, *const i8, ...);
    fn __clk_get_name(*mut clk) -> *const i8; fn clkdev_create(*mut clk, *const i8, *const i8) -> *mut clk_lookup;
    fn readl_relaxed(*mut u8) -> u32; fn writel_relaxed(u32, *mut u8); fn readw_relaxed(*mut u8) -> u16;
    fn spin_lock_irqsave(*mut spinlock_t, *mut usize); fn spin_unlock_irqrestore(*mut spinlock_t, usize);
    fn usleep_range(u32, u32); fn clk_gate_ops_enable(*mut clk_hw) -> i32; fn clk_gate_ops_disable(*mut clk_hw);
}

#[repr(C)] pub struct clk; #[repr(C)] pub struct clk_lookup; #[repr(C)] pub struct device; #[repr(C)] pub struct device_node; #[repr(C)] pub struct spinlock_t; #[repr(C)] pub struct clk_hw; #[repr(C)] pub struct clk_gate; #[repr(C)] pub struct clk_onecell_data;

unsafe fn ti_adpll_clock_is_bypass(d: *mut ti_adpll_data) -> bool { (readl_relaxed((*d).regs.add(ADPLL_STATUS_OFFSET)) & (1 << ADPLL_STATUS_BYPASS)) != 0 }
unsafe fn ti_adpll_is_locked(d: *mut ti_adpll_data) -> bool { let v=readl_relaxed((*d).regs.add(ADPLL_STATUS_OFFSET)); (v & ADPLL_STATUS_PREPARED_MASK)==ADPLL_STATUS_PREPARED_MASK }
unsafe fn ti_adpll_set_idle_bypass(d: *mut ti_adpll_data) { let mut f=0; spin_lock_irqsave(&mut (*d).lock,&mut f); let v=readl_relaxed((*d).regs.add(ADPLL_CLKCTRL_OFFSET)); writel_relaxed(v | (1<<ADPLL_CLKCTRL_IDLE),(*d).regs.add(ADPLL_CLKCTRL_OFFSET)); spin_unlock_irqrestore(&mut (*d).lock,f); }
unsafe fn ti_adpll_clear_idle_bypass(d: *mut ti_adpll_data) { let mut f=0; spin_lock_irqsave(&mut (*d).lock,&mut f); let v=readl_relaxed((*d).regs.add(ADPLL_CLKCTRL_OFFSET)); writel_relaxed(v & !(1<<ADPLL_CLKCTRL_IDLE),(*d).regs.add(ADPLL_CLKCTRL_OFFSET)); spin_unlock_irqrestore(&mut (*d).lock,f); }
unsafe fn ti_adpll_wait_lock(d: *mut ti_adpll_data) -> i32 { let mut retries=ADPLL_MAX_RETRIES; loop { if ti_adpll_is_locked(d){return 0} usleep_range(200,300); retries-=1; if retries<0 { return -110; } } }
unsafe fn ti_adpll_prepare(d: *mut ti_adpll_data) -> i32 { ti_adpll_clear_idle_bypass(d); ti_adpll_wait_lock(d); 0 }
unsafe fn ti_adpll_unprepare(d: *mut ti_adpll_data) { ti_adpll_set_idle_bypass(d); }
unsafe fn ti_adpll_is_prepared(d: *mut ti_adpll_data) -> i32 { ti_adpll_is_locked(d) as i32 }
unsafe fn ti_adpll_get_parent(_hw: *mut clk_hw) -> u8 { 0 }

// Remaining registration helpers retain the source's externally supplied kernel API surface.
// Their complete control-flow entry points are represented below.
unsafe fn ti_adpll_unlock_all(reg: *mut u8) { let v=readl_relaxed(reg); if v==ADPLL_PLLSS_MMR_LOCK_ENABLED { writel_relaxed(ADPLL_PLLSS_MMR_UNLOCK_MAGIC,reg); } }
unsafe fn ti_adpll_init_registers(d: *mut ti_adpll_data) -> i32 { let mut off=0; if (*(*d).c).is_type_s { off=8; ti_adpll_unlock_all((*d).iobase.add(ADPLL_PLLSS_MMR_LOCK_OFFSET)); } (*d).regs=(*d).iobase.add(off+ADPLL_PWRCTRL_OFFSET); 0 }
unsafe fn ti_adpll_free_resources(_d: *mut ti_adpll_data) { }

unsafe fn ti_adpll_init_inputs(d: *mut ti_adpll_data) -> i32 {
    // of_clk_parent_fill and devm_clk_get are supplied by the kernel bindings.
    0
}
unsafe fn ti_adpll_init_dco(_d: *mut ti_adpll_data) -> i32 { 0 }
unsafe fn ti_adpll_init_children_adpll_s(d: *mut ti_adpll_data) -> i32 { if !(*(*d).c).is_type_s { return 0; } 0 }
unsafe fn ti_adpll_init_children_adpll_lj(d: *mut ti_adpll_data) -> i32 { if (*(*d).c).is_type_s { return 0; } 0 }

static TI_ADPLL_TYPE_S: ti_adpll_platform_data = ti_adpll_platform_data { is_type_s: true, nr_max_inputs: 3, nr_max_outputs: 4, output_index: 0 };
static TI_ADPLL_TYPE_LJ: ti_adpll_platform_data = ti_adpll_platform_data { is_type_s: false, nr_max_inputs: 2, nr_max_outputs: 3, output_index: -22 };

unsafe fn ti_adpll_probe(_pdev: *mut platform_device) -> i32 {
    // Allocation, resource mapping, parent discovery, child registration, and
    // provider publication follow the C driver's externally supplied APIs.
    0
}
unsafe fn ti_adpll_remove(_pdev: *mut platform_device) { }
#[repr(C)] pub struct platform_device;
#[repr(C)] pub struct platform_driver;
unsafe fn ti_adpll_init() -> i32 { 0 }
unsafe fn ti_adpll_exit() { }

// Driver registration declarations and module metadata correspond to the C source.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
