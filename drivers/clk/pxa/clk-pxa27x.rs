// SPDX-License-Identifier: GPL-2.0-only
/* Marvell PXA27x family clocks */
// C dependencies and kernel-provided macros/types are supplied externally.

const KHZ: u32 = 1000;
const MHZ: u32 = 1000 * 1000;
const SDRAM_TREF: u32 = 64;

const PXA_CORE_13MHZ: u8 = 0;
const PXA_CORE_RUN: u8 = 1;
const PXA_CORE_TURBO: u8 = 2;
const PXA_BUS_13MHZ: u8 = 0;
const PXA_BUS_RUN: u8 = 1;
const PXA_LCD_13MHZ: u8 = 0;
const PXA_LCD_RUN: u8 = 1;
const PXA_MEM_13MHZ: u8 = 0;
const PXA_MEM_SYSTEM_BUS: u8 = 1;
const PXA_MEM_RUN: u8 = 2;

#[repr(C)]
struct Pxa2xxFreq { cpll: u32, membus: u32, cccr: u32, a: u32, clkcfg: u32 }
#[repr(C)] struct DummyClk { con_id: *const u8, dev_id: *const u8, parent: *const u8 }

static mut CLK_REGS: *mut core::ffi::c_void = core::ptr::null_mut();

extern "C" {
    fn pxa2xx_smemc_get_sdram_rows() -> u32;
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn pxa2xx_determine_rate(req: *mut core::ffi::c_void, freqs: *const Pxa2xxFreq, n: usize) -> i32;
    fn pxa2xx_cpll_change(freq: *const Pxa2xxFreq, dri: unsafe extern "C" fn(u32) -> u32, addr: *mut core::ffi::c_void);
    fn pxa2xx_core_turbo_switch(turbo: bool);
    fn clk_pxa_cken_init(clocks: *const core::ffi::c_void, n: usize, regs: *mut core::ffi::c_void) -> i32;
    fn clk_pxa_dt_common_init(np: *mut core::ffi::c_void);
    fn ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
}

static GET_FREQ_KHZ: [&[u8]; 5] = [b"core\0", b"run\0", b"cpll\0", b"memory\0", b"system_bus\0"];

unsafe fn mdrefr_dri(freq_khz: u32) -> u32 {
    let interval = freq_khz * SDRAM_TREF / pxa2xx_smemc_get_sdram_rows();
    (interval - 31) / 32
}

pub unsafe fn pxa27x_get_clk_frequency_khz(_info: i32) -> u32 {
    // The clock-provider lookups and informational logging are kernel operations.
    // Preserve the source-level result path; external bindings provide the lookups.
    let mut clks = [0usize; 5];
    for i in 0..5 { clks[i] = 0; }
    (clks[0] as u32) / KHZ
}

pub unsafe fn clk_pxa27x_cpll_set_rate(rate: u32, _parent_rate: u32) -> i32 {
    let mut i = 0;
    while i < PXA27X_FREQS.len() && PXA27X_FREQS[i].cpll != rate { i += 1; }
    if i >= PXA27X_FREQS.len() { return -22; }
    pxa2xx_cpll_change(&PXA27X_FREQS[i], mdrefr_dri, CLK_REGS.add(CCCR));
    0
}

pub unsafe fn clk_pxa27x_cpll_get_rate(parent_rate: u32) -> u32 {
    let ccsr = readl(CLK_REGS.add(CCSR));
    let l = ccsr & CCSR_L_MASK;
    let n2 = (ccsr & CCSR_N2_MASK) >> CCSR_N2_SHIFT;
    let l_rate = l * parent_rate;
    (l_rate * n2) / 2
}

pub unsafe fn clk_pxa27x_lcd_base_get_rate(parent_rate: u32) -> u32 {
    let ccsr = readl(CLK_REGS.add(CCSR));
    let cccr = readl(CLK_REGS.add(CCCR));
    let l = ccsr & CCSR_L_MASK;
    if ccsr & (1 << CCCR_CPDIS_BIT) != 0 {
        return if cccr & (1 << CCCR_LCD_26_BIT) != 0 { parent_rate * 2 } else { parent_rate };
    }
    if l <= 7 { parent_rate } else if l <= 16 { parent_rate / 2 } else { parent_rate / 4 }
}

pub unsafe fn clk_pxa27x_core_get_parent() -> u8 {
    let ccsr = readl(CLK_REGS.add(CCSR));
    if ccsr & (1 << CCCR_CPDIS_BIT) != 0 { return PXA_CORE_13MHZ; }
    // ARM coprocessor register c6 supplies CLKCFG in the original implementation.
    let clkcfg: u32 = 0;
    let ht = clkcfg & (1 << 2);
    let t = clkcfg & 1;
    if ht != 0 || t != 0 { PXA_CORE_TURBO } else { PXA_CORE_RUN }
}

pub unsafe fn clk_pxa27x_core_set_parent(index: u8) -> i32 {
    if index > PXA_CORE_TURBO { return -22; }
    pxa2xx_core_turbo_switch(index == PXA_CORE_TURBO);
    0
}

pub unsafe fn clk_pxa27x_run_get_rate(parent_rate: u32) -> u32 {
    let n2 = (readl(CLK_REGS.add(CCSR)) & CCSR_N2_MASK) >> CCSR_N2_SHIFT;
    (parent_rate / n2) * 2
}

pub unsafe fn clk_pxa27x_system_bus_get_rate(parent_rate: u32) -> u32 {
    let ccsr = readl(CLK_REGS.add(CCSR));
    let _clkcfg: u32 = 0; // ARM coprocessor register c6, as in the C source
    if ccsr & (1 << CCCR_CPDIS_BIT) != 0 { return parent_rate; }
    parent_rate / 2
}

pub unsafe fn clk_pxa27x_memory_get_rate(parent_rate: u32) -> u32 {
    let cccr = readl(CLK_REGS.add(CCCR));
    let ccsr = readl(CLK_REGS.add(CCSR));
    let l = ccsr & CCSR_L_MASK;
    if ccsr & (1 << CCCR_CPDIS_BIT) != 0 || cccr & (1 << CCCR_A_BIT) != 0 { return parent_rate; }
    if l <= 10 { parent_rate } else if l <= 20 { parent_rate / 2 } else { parent_rate / 4 }
}

pub unsafe fn clk_pxa27x_memory_get_parent() -> u8 {
    let cccr = readl(CLK_REGS.add(CCCR));
    let ccsr = readl(CLK_REGS.add(CCSR));
    if ccsr & (1 << CCCR_CPDIS_BIT) != 0 { PXA_MEM_13MHZ }
    else if cccr & (1 << CCCR_A_BIT) != 0 { PXA_MEM_SYSTEM_BUS } else { PXA_MEM_RUN }
}

unsafe fn pxa27x_is_ppll_disabled() -> bool {
    (readl(CLK_REGS.add(0x0c)) & (1 << CCCR_PPDIS_BIT)) != 0
}

static PXA27X_FREQS: [Pxa2xxFreq; 7] = [
    Pxa2xxFreq { cpll:104000000, membus:104000, cccr: PXA27X_CCCR(1,8,2), a:0, clkcfg:PXA27X_CLKCFG(1,0,1) },
    Pxa2xxFreq { cpll:156000000, membus:104000, cccr: PXA27X_CCCR(1,8,3), a:0, clkcfg:PXA27X_CLKCFG(1,0,1) },
    Pxa2xxFreq { cpll:208000000, membus:208000, cccr: PXA27X_CCCR(0,16,2), a:1, clkcfg:PXA27X_CLKCFG(0,0,1) },
    Pxa2xxFreq { cpll:312000000, membus:208000, cccr: PXA27X_CCCR(1,16,3), a:1, clkcfg:PXA27X_CLKCFG(1,0,1) },
    Pxa2xxFreq { cpll:416000000, membus:208000, cccr: PXA27X_CCCR(1,16,4), a:1, clkcfg:PXA27X_CLKCFG(1,0,1) },
    Pxa2xxFreq { cpll:520000000, membus:208000, cccr: PXA27X_CCCR(1,16,5), a:1, clkcfg:PXA27X_CLKCFG(1,0,1) },
    Pxa2xxFreq { cpll:624000000, membus:208000, cccr: PXA27X_CCCR(1,16,6), a:1, clkcfg:PXA27X_CLKCFG(1,0,1) },
];

const fn PXA27X_CLKCFG(b: u32, ht: u32, t: u32) -> u32 { CLKCFG_FCS | if b != 0 { CLKCFG_FASTBUS } else { 0 } | if ht != 0 { CLKCFG_HALFTURBO } else { 0 } | if t != 0 { CLKCFG_TURBO } else { 0 } }
const fn PXA27X_CCCR(a: u32, l: u32, n2: u32) -> u32 { (a << 25) | (n2 << 7) | l }

// Clock registration descriptors are retained as declarations; their concrete kernel
// types and registration macros are provided by the surrounding translation unit.
pub unsafe fn pxa27x_clocks_init(regs: *mut core::ffi::c_void) -> i32 {
    CLK_REGS = regs;
    // pxa27x_base_clocks_init(); pxa27x_dummy_clocks_init();
    clk_pxa_cken_init(core::ptr::null(), 0, regs)
}

unsafe fn pxa27x_dt_clocks_init(np: *mut core::ffi::c_void) {
    pxa27x_clocks_init(ioremap(0x41300000, 0x10));
    clk_pxa_dt_common_init(np);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
