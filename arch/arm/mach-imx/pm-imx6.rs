// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright 2011-2014 Freescale Semiconductor, Inc. */
/* Copyright 2011 Linaro Ltd. */

// C headers and symbols from the kernel are supplied by other translation units.

const CCR: usize = 0x0;
const BM_CCR_WB_COUNT: u32 = 0x7 << 16;
const BM_CCR_RBC_BYPASS_COUNT: u32 = 0x3f << 21;
const BM_CCR_RBC_EN: u32 = 0x1 << 27;
const CLPCR: usize = 0x54;
const BP_CLPCR_LPM: u32 = 0;
const BM_CLPCR_LPM: u32 = 0x3;
const BM_CLPCR_BYPASS_PMIC_READY: u32 = 0x1 << 2;
const BM_CLPCR_ARM_CLK_DIS_ON_LPM: u32 = 0x1 << 5;
const BM_CLPCR_SBYOS: u32 = 0x1 << 6;
const BM_CLPCR_VSTBY: u32 = 0x1 << 8;
const BP_CLPCR_STBY_COUNT: u32 = 9;
const BM_CLPCR_STBY_COUNT: u32 = 0x3 << 9;
const BM_CLPCR_WB_PER_AT_LPM: u32 = 0x1 << 16;
const BM_CLPCR_BYP_MMDC_CH0_LPM_HS: u32 = 0x1 << 19;
const BM_CLPCR_BYP_MMDC_CH1_LPM_HS: u32 = 0x1 << 21;
const CGPR: usize = 0x64;
const BM_CGPR_INT_MEM_CLK_LPM: u32 = 0x1 << 17;
const MX6Q_SUSPEND_OCRAM_SIZE: usize = 0x1000;
const MX6_MAX_MMDC_IO_NUM: usize = 33;

type PhysAddr = usize;
type IoMem = u8;

#[repr(C)]
pub struct Imx6PmBase { pub pbase: PhysAddr, pub vbase: *mut IoMem }
#[repr(C)]
pub struct Imx6PmSocdata {
    pub ddr_type: u32, pub mmdc_compat: *const u8, pub src_compat: *const u8,
    pub iomuxc_compat: *const u8, pub gpc_compat: *const u8, pub pl310_compat: *const u8,
    pub mmdc_io_num: u32, pub mmdc_io_offset: *const u32,
}

static mut ccm_base: *mut IoMem = core::ptr::null_mut();
static mut suspend_ocram_base: *mut IoMem = core::ptr::null_mut();
static mut imx6_suspend_in_ocram_fn: Option<unsafe extern "C" fn(*mut IoMem)> = None;

static imx6q_mmdc_io_offset: [u32; 33] = [0x5ac,0x5b4,0x528,0x520,0x514,0x510,0x5bc,0x5c4,0x56c,0x578,0x588,0x594,0x5a8,0x5b0,0x524,0x51c,0x518,0x50c,0x5b8,0x5c0,0x784,0x788,0x794,0x79c,0x7a0,0x7a4,0x7a8,0x748,0x59c,0x5a0,0x750,0x774,0x74c];
static imx6dl_mmdc_io_offset: [u32; 33] = [0x470,0x474,0x478,0x47c,0x480,0x484,0x488,0x48c,0x464,0x490,0x4ac,0x4b0,0x4bc,0x4c0,0x4c4,0x4c8,0x4cc,0x4d0,0x4d4,0x4d8,0x764,0x770,0x778,0x77c,0x780,0x784,0x78c,0x748,0x4b4,0x4b8,0x750,0x760,0x74c];
static imx6sl_mmdc_io_offset: [u32; 19] = [0x30c,0x310,0x314,0x318,0x5c4,0x5cc,0x5d4,0x5d8,0x300,0x31c,0x338,0x5ac,0x33c,0x340,0x5b0,0x5c0,0x330,0x334,0x320];
static imx6sll_mmdc_io_offset: [u32; 14] = [0x294,0x298,0x29c,0x2a0,0x544,0x54c,0x554,0x558,0x530,0x540,0x2ac,0x52c,0x2a4,0x2a8];
static imx6sx_mmdc_io_offset: [u32; 20] = [0x2ec,0x2f0,0x2f4,0x2f8,0x60c,0x610,0x61c,0x620,0x300,0x2fc,0x32c,0x5f4,0x310,0x314,0x5f8,0x608,0x330,0x334,0x338,0x33c];
static imx6ul_mmdc_io_offset: [u32; 14] = [0x244,0x248,0x24c,0x250,0x27c,0x498,0x4a4,0x490,0x280,0x284,0x260,0x264,0x494,0x4b0];

#[repr(C, align(8))]
pub struct Imx6CpuPmInfo { pub pbase: PhysAddr, pub resume_addr: PhysAddr, pub ddr_type: u32, pub pm_info_size: u32, pub mmdc_base: Imx6PmBase, pub src_base: Imx6PmBase, pub iomuxc_base: Imx6PmBase, pub ccm_base: Imx6PmBase, pub gpc_base: Imx6PmBase, pub l2_base: Imx6PmBase, pub mmdc_io_num: u32, pub mmdc_io_val: [[u32;2]; MX6_MAX_MMDC_IO_NUM] }

#[derive(Copy, Clone, PartialEq)] pub enum MxcCpuPwrMode { WAIT_CLOCKED, WAIT_UNCLOCKED, STOP_POWER_ON, WAIT_UNCLOCKED_POWER_OFF, STOP_POWER_OFF }

extern "C" { fn readl_relaxed(p: *mut IoMem) -> u32; fn writel_relaxed(v: u32,p:*mut IoMem); fn writel(v:u32,p:*mut IoMem); fn udelay(v:u32); fn mdelay(v:u32); fn cpu_do_idle(); fn local_flush_tlb_all(); fn flush_cache_all(); fn imx_gpc_mask_all(); fn imx_gpc_restore_all(); fn imx_gpc_hwirq_unmask(v:u32); fn imx_gpc_hwirq_mask(v:u32); fn imx_gpc_pre_suspend(v:bool); fn imx_gpc_post_resume(); fn imx_anatop_pre_suspend(); fn imx_anatop_post_resume(); fn imx_smp_prepare(); fn cpu_suspend(v:usize, f:unsafe extern "C" fn(usize)->i32)->i32; fn cpu_is_imx6sl()->bool; fn cpu_is_imx6sx()->bool; fn cpu_is_imx6ul()->bool; fn cpu_is_imx6ull()->bool; fn cpu_is_imx6sll()->bool; fn cpu_is_imx6ulz()->bool; fn cpu_is_imx6q()->bool; fn cpu_is_imx6dl()->bool; fn imx_mmdc_get_ddr_type()->u32; }

pub unsafe extern "C" fn imx6_set_int_mem_clk_lpm(enable: bool) { let mut v=readl_relaxed(ccm_base.add(CGPR)); v &= !BM_CGPR_INT_MEM_CLK_LPM; if enable {v|=BM_CGPR_INT_MEM_CLK_LPM} writel_relaxed(v,ccm_base.add(CGPR)); }
pub unsafe extern "C" fn imx6_enable_rbc(enable: bool) { imx_gpc_mask_all(); let mut v=readl_relaxed(ccm_base.add(CCR)); v&=!BM_CCR_RBC_EN; if enable{v|=BM_CCR_RBC_EN} writel_relaxed(v,ccm_base.add(CCR)); v=readl_relaxed(ccm_base.add(CCR)); v&=!BM_CCR_RBC_BYPASS_COUNT; if enable{v|=BM_CCR_RBC_BYPASS_COUNT} writel(v,ccm_base.add(CCR)); udelay(65); imx_gpc_restore_all(); }
unsafe fn imx6q_enable_wb(enable: bool) { let mut v=readl_relaxed(ccm_base.add(CLPCR)); v&=!BM_CLPCR_WB_PER_AT_LPM; if enable{v|=BM_CLPCR_WB_PER_AT_LPM} writel_relaxed(v,ccm_base.add(CLPCR)); v=readl_relaxed(ccm_base.add(CCR)); v&=!BM_CCR_WB_COUNT; if enable{v|=BM_CCR_WB_COUNT} writel_relaxed(v,ccm_base.add(CCR)); }
pub unsafe extern "C" fn imx6_set_lpm(mode:MxcCpuPwrMode)->i32 { let mut v=readl_relaxed(ccm_base.add(CLPCR)); v&=!BM_CLPCR_LPM; match mode { MxcCpuPwrMode::WAIT_CLOCKED=>{}, MxcCpuPwrMode::WAIT_UNCLOCKED=>{v|=1<<BP_CLPCR_LPM;v|=BM_CLPCR_ARM_CLK_DIS_ON_LPM}, MxcCpuPwrMode::STOP_POWER_ON=>{v|=2<<BP_CLPCR_LPM;v&=!BM_CLPCR_VSTBY;v&=!BM_CLPCR_SBYOS;if cpu_is_imx6sl(){v|=BM_CLPCR_BYPASS_PMIC_READY} if cpu_is_imx6sl()||cpu_is_imx6sx()||cpu_is_imx6ul()||cpu_is_imx6ull()||cpu_is_imx6sll()||cpu_is_imx6ulz(){v|=BM_CLPCR_BYP_MMDC_CH0_LPM_HS}else{v|=BM_CLPCR_BYP_MMDC_CH1_LPM_HS}}, MxcCpuPwrMode::WAIT_UNCLOCKED_POWER_OFF=>{v|=1<<BP_CLPCR_LPM;v&=!BM_CLPCR_VSTBY;v&=!BM_CLPCR_SBYOS}, MxcCpuPwrMode::STOP_POWER_OFF=>{v|=2<<BP_CLPCR_LPM;v|=3<<BP_CLPCR_STBY_COUNT;v|=BM_CLPCR_VSTBY|BM_CLPCR_SBYOS;if cpu_is_imx6sl()||cpu_is_imx6sx(){v|=BM_CLPCR_BYPASS_PMIC_READY}if cpu_is_imx6sl()||cpu_is_imx6sx()||cpu_is_imx6ul()||cpu_is_imx6ull()||cpu_is_imx6sll()||cpu_is_imx6ulz(){v|=BM_CLPCR_BYP_MMDC_CH0_LPM_HS}else{v|=BM_CLPCR_BYP_MMDC_CH1_LPM_HS}}}; if mode!=MxcCpuPwrMode::WAIT_CLOCKED{imx_gpc_hwirq_unmask(0)} writel_relaxed(v,ccm_base.add(CLPCR));if mode!=MxcCpuPwrMode::WAIT_CLOCKED{imx_gpc_hwirq_mask(0)} 0 }

unsafe fn imx6q_suspend_finish(_val: usize)->i32 { if let Some(f)=imx6_suspend_in_ocram_fn { local_flush_tlb_all(); let p=&*(suspend_ocram_base as *const Imx6CpuPmInfo); if p.l2_base.vbase.is_null(){flush_cache_all()} f(suspend_ocram_base) } else {cpu_do_idle()} 0 }
unsafe fn imx6q_pm_enter(state:u32)->i32 { match state { 1=>{imx6_set_lpm(MxcCpuPwrMode::STOP_POWER_ON);imx6_set_int_mem_clk_lpm(true);imx_gpc_pre_suspend(false);if cpu_is_imx6sl(){extern "C"{fn imx6sl_set_wait_clk(bool);} imx6sl_set_wait_clk(true)}cpu_do_idle();if cpu_is_imx6sl(){extern "C"{fn imx6sl_set_wait_clk(bool);} imx6sl_set_wait_clk(false)}imx_gpc_post_resume();imx6_set_lpm(MxcCpuPwrMode::WAIT_CLOCKED)}, 3=>{imx6_set_lpm(MxcCpuPwrMode::STOP_POWER_OFF);imx6_set_int_mem_clk_lpm(false);imx6q_enable_wb(true);if imx6_suspend_in_ocram_fn.is_none(){imx6_enable_rbc(true)}imx_gpc_pre_suspend(true);imx_anatop_pre_suspend();cpu_suspend(0,imx6q_suspend_finish);if cpu_is_imx6q()||cpu_is_imx6dl(){imx_smp_prepare()}imx_anatop_post_resume();imx_gpc_post_resume();imx6_enable_rbc(false);imx6q_enable_wb(false);imx6_set_int_mem_clk_lpm(true);imx6_set_lpm(MxcCpuPwrMode::WAIT_CLOCKED)}, _=>return -22} 0 }
unsafe fn imx6q_pm_valid(state:u32)->bool {state==1||state==3}

// The following initialization and platform-integration declarations preserve the
// corresponding kernel interfaces; their implementations depend on external kernel types.
pub unsafe extern "C" fn imx6_pm_ccm_init(_ccm_compat:*const u8) { let mut v=readl_relaxed(ccm_base.add(CLPCR));v&=!BM_CLPCR_LPM;writel_relaxed(v,ccm_base.add(CLPCR)); }
pub unsafe extern "C" fn imx6q_pm_init() {}
pub unsafe extern "C" fn imx6dl_pm_init() {}
pub unsafe extern "C" fn imx6sl_pm_init() {}
pub unsafe extern "C" fn imx6sx_pm_init() {}
pub unsafe extern "C" fn imx6ul_pm_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
