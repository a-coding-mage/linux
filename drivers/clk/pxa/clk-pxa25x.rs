// SPDX-License-Identifier: GPL-2.0-only
/* Marvell PXA25x family clocks */

// C dependencies supplied by the surrounding kernel translation unit.

const KHz: u32 = 1000;
const MHz: u32 = 1000 * 1000;

const PXA_CORE_RUN: u8 = 0;
const PXA_CORE_TURBO: u8 = 1;

const SDRAM_TREF: u32 = 64;

static mut clk_regs: *mut core::ffi::c_void = core::ptr::null_mut();
static L_clk_mult: [u8; 32] = [0, 27, 32, 36, 40, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
static M_clk_mult: [u8; 4] = [0, 1, 2, 4];
static N2_clk_mult: [u8; 8] = [0, 0, 2, 3, 4, 0, 6, 0];
static get_freq_khz: [&str; 4] = ["core", "run", "cpll", "memory"];

const fn pxa25x_clkcfg(t: bool) -> u32 { CLKCFG_FCS | if t { CLKCFG_TURBO } else { 0 } }
const fn pxa25x_cccr(n2: u32, m: u32, l: u32) -> u32 { n2 << 7 | m << 5 | l }

unsafe fn mdrefr_dri(freq_khz: u32) -> u32 {
    let interval = freq_khz * SDRAM_TREF / pxa2xx_smemc_get_sdram_rows();
    interval / 32
}

pub unsafe fn pxa25x_get_clk_frequency_khz(info: i32) -> u32 {
    let mut clks = [0u64; 5];
    for i in 0..get_freq_khz.len() {
        let clk = clk_get(core::ptr::null_mut(), get_freq_khz[i].as_ptr() as *const i8);
        if IS_ERR(clk) { clks[i] = 0; } else { clks[i] = clk_get_rate(clk); clk_put(clk); }
    }
    if info != 0 {
        pr_info!("Run Mode clock: {}.{:02}MHz\n", clks[1] / 1_000_000, (clks[1] % 1_000_000) / 10_000);
        pr_info!("Turbo Mode clock: {}.{:02}MHz\n", clks[2] / 1_000_000, (clks[2] % 1_000_000) / 10_000);
        pr_info!("Memory clock: {}.{:02}MHz\n", clks[3] / 1_000_000, (clks[3] % 1_000_000) / 10_000);
    }
    (clks[0] as u32) / KHz
}

unsafe fn clk_pxa25x_memory_get_rate(_hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let cccr = readl(clk_regs.add(CCCR));
    let m = M_clk_mult[((cccr >> 5) & 3) as usize];
    parent_rate / m as u64
}

// PARENTS(clk_pxa25x_memory) = { "run" }; RATE_RO_OPS(clk_pxa25x_memory, "memory");
// PARENTS(pxa25x_pbus95) = { "ppll_95_85mhz", "ppll_95_85mhz" };
// PARENTS(pxa25x_pbus147) = { "ppll_147_46mhz", "ppll_147_46mhz" };
// PARENTS(pxa25x_osc3) = { "osc_3_6864mhz", "osc_3_6864mhz" };

static mut pxa25x_clocks: [desc_clk_cken; 16] = [
    PXA25X_PBUS95_CKEN!("pxa2xx-mci.0", None, MMC, 1, 5, 0), PXA25X_PBUS95_CKEN!("pxa2xx-i2c.0", None, I2C, 1, 3, 0),
    PXA25X_PBUS95_CKEN!("pxa2xx-ir", Some("FICPCLK"), FICP, 1, 2, 0), PXA25X_PBUS95_CKEN!("pxa25x-udc", None, USB, 1, 2, 5),
    PXA25X_PBUS147_CKEN!("pxa2xx-uart.0", None, FFUART, 1, 10, 1), PXA25X_PBUS147_CKEN!("pxa2xx-uart.1", None, BTUART, 1, 10, 1),
    PXA25X_PBUS147_CKEN!("pxa2xx-uart.2", None, STUART, 1, 10, 1), PXA25X_PBUS147_CKEN!("pxa2xx-uart.3", None, HWUART, 1, 10, 1),
    PXA25X_PBUS147_CKEN!("pxa2xx-i2s", None, I2S, 1, 10, 0), PXA25X_PBUS147_CKEN!(None, Some("AC97CLK"), AC97, 1, 12, 0),
    PXA25X_OSC3_CKEN!("pxa25x-ssp.0", None, SSP, 1, 1, 0), PXA25X_OSC3_CKEN!("pxa25x-nssp.1", None, NSSP, 1, 1, 0),
    PXA25X_OSC3_CKEN!("pxa25x-nssp.2", None, ASSP, 1, 1, 0), PXA25X_OSC3_CKEN!("pxa25x-pwm.0", None, PWM0, 1, 1, 0),
    PXA25X_OSC3_CKEN!("pxa25x-pwm.1", None, PWM1, 1, 1, 0), PXA25X_CKEN_1RATE!("pxa2xx-fb", None, LCD, clk_pxa25x_memory_parents, 0),
];

static pxa25x_freqs: [pxa2xx_freq; 4] = [
    pxa2xx_freq { cpu: 99532800, membus: 99500, cccr: pxa25x_cccr(2,1,1), div2: 1, cclkcfg: pxa25x_clkcfg(true) },
    pxa2xx_freq { cpu: 199065600, membus: 99500, cccr: pxa25x_cccr(4,1,1), div2: 0, cclkcfg: pxa25x_clkcfg(true) },
    pxa2xx_freq { cpu: 298598400, membus: 99500, cccr: pxa25x_cccr(3,2,1), div2: 0, cclkcfg: pxa25x_clkcfg(true) },
    pxa2xx_freq { cpu: 398131200, membus: 99500, cccr: pxa25x_cccr(4,2,1), div2: 0, cclkcfg: pxa25x_clkcfg(true) },
];

// Remaining clock-provider operation tables and registration glue are direct declarations of
// the external kernel interfaces used by the C implementation.
unsafe fn clk_pxa25x_core_get_parent(_hw: *mut clk_hw) -> u8 { let mut clkcfg: u32 = 0; asm!("mrc p14, 0, {}, c6, c0, 0", out(reg) clkcfg); if clkcfg & 1 != 0 { PXA_CORE_TURBO } else { PXA_CORE_RUN } }
unsafe fn clk_pxa25x_core_set_parent(_hw: *mut clk_hw, index: u8) -> i32 { if index > PXA_CORE_TURBO { return -EINVAL; } pxa2xx_core_turbo_switch(index == PXA_CORE_TURBO); 0 }
unsafe fn clk_pxa25x_core_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { __clk_mux_determine_rate(hw, req) }

unsafe fn clk_pxa25x_run_get_rate(_hw: *mut clk_hw, parent_rate: u64) -> u64 { let cccr = readl(clk_regs.add(CCCR)); let n2 = N2_clk_mult[((cccr >> 7) & 7) as usize]; (parent_rate / n2 as u64) * 2 }
unsafe fn clk_pxa25x_cpll_get_rate(_hw: *mut clk_hw, parent_rate: u64) -> u64 { let cccr = readl(clk_regs.add(CCCR)); let mut clkcfg=0; asm!("mrc p14, 0, {}, c6, c0, 0", out(reg) clkcfg); let l=L_clk_mult[(cccr&31) as usize] as u64; let m=M_clk_mult[((cccr>>5)&3) as usize] as u64; let n2=N2_clk_mult[((cccr>>7)&7) as usize] as u64; m*l*n2*parent_rate/2 }
unsafe fn clk_pxa25x_cpll_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { pxa2xx_determine_rate(req, pxa25x_freqs.as_ptr(), 4) }
unsafe fn clk_pxa25x_cpll_set_rate(_hw: *mut clk_hw, rate: u64, _parent_rate: u64) -> i32 { let mut i=0; while i<4 && pxa25x_freqs[i].cpll != rate { i+=1; } if i>=4 { return -EINVAL; } pxa2xx_cpll_change(&pxa25x_freqs[i], mdrefr_dri, clk_regs.add(CCCR)); 0 }

pub unsafe fn pxa25x_clocks_init(regs: *mut core::ffi::c_void) -> i32 { clk_regs=regs; pxa25x_base_clocks_init(); pxa25x_dummy_clocks_init(); clk_pxa_cken_init(pxa25x_clocks.as_mut_ptr(), 16, clk_regs) }

// Registration helpers and dummy clock table from the original implementation.
#[repr(C)] struct dummy_clk { con_id: *const i8, dev_id: *const i8, parent: *const i8 }
static dummy_clks: [dummy_clk; 7] = [
    dummy_clk { con_id: core::ptr::null(), dev_id: cstr!("pxa25x-gpio"), parent: cstr!("osc_32_768khz") },
    dummy_clk { con_id: core::ptr::null(), dev_id: cstr!("pxa26x-gpio"), parent: cstr!("osc_32_768khz") },
    dummy_clk { con_id: cstr!("GPIO11_CLK"), dev_id: core::ptr::null(), parent: cstr!("osc_3_6864mhz") },
    dummy_clk { con_id: cstr!("GPIO12_CLK"), dev_id: core::ptr::null(), parent: cstr!("osc_32_768khz") },
    dummy_clk { con_id: core::ptr::null(), dev_id: cstr!("sa1100-rtc"), parent: cstr!("osc_32_768khz") },
    dummy_clk { con_id: cstr!("OSTIMER0"), dev_id: core::ptr::null(), parent: cstr!("osc_3_6864mhz") },
    dummy_clk { con_id: cstr!("UARTCLK"), dev_id: cstr!("pxa2xx-ir"), parent: cstr!("STUART") },
];
unsafe fn pxa25x_base_clocks_init() { pxa25x_register_plls(); pxa25x_register_core(); clkdev_pxa_register(CLK_NONE,cstr!("system_bus"),core::ptr::null(),clk_register_clk_pxa25x_memory()); }
unsafe fn pxa25x_dummy_clocks_init() { for d in dummy_clks.iter() { let name=if !d.dev_id.is_null(){d.dev_id}else{d.con_id}; let clk=clk_register_fixed_factor(core::ptr::null(),name,d.parent,0,1,1); clk_register_clkdev(clk,d.con_id,d.dev_id); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
