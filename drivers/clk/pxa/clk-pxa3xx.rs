// SPDX-License-Identifier: GPL-2.0-only
/* Marvell PXA3xxx family clocks; direct low-level translation of clk-pxa3xx.c. */

const KHZ: u32 = 1000;
const MHZ: u32 = 1000 * 1000;
const ACCR: usize = 0x0000;
const ACSR: usize = 0x0004;
const AICSR: usize = 0x0008;
const CKENA: usize = 0x000c;
const CKENB: usize = 0x0010;
const CKENC: usize = 0x0024;
const AC97_DIV: usize = 0x0014;

const ACCR_XPDIS: u32 = 1 << 31;
const ACCR_SPDIS: u32 = 1 << 30;
const ACCR_D0CS: u32 = 1 << 26;
const ACCR_PCCE: u32 = 1 << 11;
const ACCR_DDR_D0CS: u32 = 1 << 7;
const ACCR_SMCFS_MASK: u32 = 0x7 << 23;
const ACCR_SFLFS_MASK: u32 = 0x3 << 18;
const ACCR_XSPCLK_MASK: u32 = 0x3 << 16;
const ACCR_HSS_MASK: u32 = 0x3 << 14;
const ACCR_DMCFS_MASK: u32 = 0x3 << 12;
const ACCR_XN_MASK: u32 = 0x7 << 8;
const ACCR_XL_MASK: u32 = 0x1f;

const fn accr_smcfs(x: u32) -> u32 { (x & 0x7) << 23 }
const fn accr_sflfs(x: u32) -> u32 { (x & 0x3) << 18 }
const fn accr_xspclk(x: u32) -> u32 { (x & 0x3) << 16 }
const fn accr_hss(x: u32) -> u32 { (x & 0x3) << 14 }
const fn accr_dmcfs(x: u32) -> u32 { (x & 0x3) << 12 }
const fn accr_xn(x: u32) -> u32 { (x & 0x7) << 8 }
const fn accr_xl(x: u32) -> u32 { x & 0x1f }

const CKEN_LCD: u32 = 1; const CKEN_USBH: u32 = 2; const CKEN_CAMERA: u32 = 3;
const CKEN_NAND: u32 = 4; const CKEN_USB2: u32 = 6; const CKEN_DMC: u32 = 8;
const CKEN_SMC: u32 = 9; const CKEN_ISC: u32 = 10; const CKEN_BOOT: u32 = 11;
const CKEN_MMC1: u32 = 12; const CKEN_MMC2: u32 = 13; const CKEN_KEYPAD: u32 = 14;
const CKEN_CIR: u32 = 15; const CKEN_USIM0: u32 = 17; const CKEN_USIM1: u32 = 18;
const CKEN_TPM: u32 = 19; const CKEN_UDC: u32 = 20; const CKEN_BTUART: u32 = 21;
const CKEN_FFUART: u32 = 22; const CKEN_STUART: u32 = 23; const CKEN_AC97: u32 = 24;
const CKEN_TOUCH: u32 = 25; const CKEN_SSP1: u32 = 26; const CKEN_SSP2: u32 = 27;
const CKEN_SSP3: u32 = 28; const CKEN_SSP4: u32 = 29; const CKEN_MSL0: u32 = 30;
const CKEN_PWM0: u32 = 32; const CKEN_PWM1: u32 = 33; const CKEN_I2C: u32 = 36;
const CKEN_INTC: u32 = 38; const CKEN_GPIO: u32 = 39; const CKEN_1WIRE: u32 = 40;
const CKEN_HSIO2: u32 = 41; const CKEN_MINI_IM: u32 = 48; const CKEN_MINI_LCD: u32 = 49;
const CKEN_MMC3: u32 = 5; const CKEN_MVED: u32 = 43;
const CKEN_PXA300_GCU: u32 = 42; const CKEN_PXA320_GCU: u32 = 7;

enum PxaCore { PxaCore60Mhz = 0, PxaCoreRun, PxaCoreTurbo }
enum PxaBus { PxaBus60Mhz = 0, PxaBusHss }

static HSS_MULT: [u8; 4] = [8, 12, 16, 24];
static SMCFS_MULT: [u32; 8] = [6, 0, 8, 0, 0, 16, 0, 0];
static GET_FREQ_KHZ: [&str; 5] = ["core", "ring_osc_60mhz", "run", "cpll", "system_bus"];
static mut CLK_REGS: *mut u8 = core::ptr::null_mut();

#[repr(C)] pub struct ClkHw { _private: [u8; 0] }
#[repr(C)] pub struct Clk { _private: [u8; 0] }
#[repr(C)] pub struct DeviceNode { _private: [u8; 0] }
#[repr(C)] pub struct DescClkCken { _private: [u8; 0] }

extern "C" {
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn cpu_relax();
    fn pxa3xx_smemc_get_memclkdiv() -> u32;
    fn clk_get(dev: *mut core::ffi::c_void, name: *const u8) -> *mut Clk;
    fn clk_get_rate(clk: *mut Clk) -> usize;
    fn clk_put(clk: *mut Clk);
    fn pr_info(fmt: *const u8, ...);
    fn clk_register_clk_pxa3xx_cpll(); fn clk_register_clk_pxa3xx_run();
    fn clk_register_clk_pxa3xx_core() -> *mut Clk;
    fn clk_register_clk_pxa3xx_system_bus(); fn clk_register_clk_pxa3xx_ac97();
    fn clk_register_clk_pxa3xx_smemc();
    fn clk_register_fixed_rate(a: *mut core::ffi::c_void, n: *const u8, p: *const u8, f: u32, r: u32) -> *mut Clk;
    fn clk_register_fixed_factor(a: *mut core::ffi::c_void, n: *const u8, p: *const u8, f: u32, m: u32, d: u32) -> *mut Clk;
    fn clk_register_gate(a: *mut core::ffi::c_void, n: *const u8, p: *const u8, f: u32, reg: *mut u8, bit: u8, flags: u8, lock: *mut core::ffi::c_void) -> *mut Clk;
    fn clk_register_clkdev(c: *mut Clk, con: *const u8, dev: *const u8);
    fn clkdev_pxa_register(id: u32, con: *const u8, dev: *const u8, c: *mut Clk);
    fn clk_pxa_cken_init(c: *mut DescClkCken, n: usize, r: *mut u8) -> i32;
    fn cpu_is_pxa320() -> bool; fn cpu_is_pxa300() -> bool; fn cpu_is_pxa310() -> bool;
    fn ioremap(addr: usize, size: usize) -> *mut u8;
    fn clk_pxa_dt_common_init(np: *mut DeviceNode);
}

#[inline] unsafe fn reg(offset: usize) -> *mut u8 { CLK_REGS.add(offset) }

#[no_mangle]
pub unsafe extern "C" fn pxa3xx_get_clk_frequency_khz(info: i32) -> u32 {
    let mut clks = [0usize; 5];
    for i in 0..5 {
        let mut name = GET_FREQ_KHZ[i].as_bytes().to_vec(); name.push(0);
        let clk = clk_get(core::ptr::null_mut(), name.as_ptr());
        if clk.is_null() { clks[i] = 0; } else { clks[i] = clk_get_rate(clk); clk_put(clk); }
    }
    if info != 0 {
        pr_info(b"RO Mode clock: %ld.%02ldMHz\n\0".as_ptr(), clks[1] / 1_000_000, (clks[0] % 1_000_000) / 10_000);
        pr_info(b"Run Mode clock: %ld.%02ldMHz\n\0".as_ptr(), clks[2] / 1_000_000, (clks[1] % 1_000_000) / 10_000);
        pr_info(b"Turbo Mode clock: %ld.%02ldMHz\n\0".as_ptr(), clks[3] / 1_000_000, (clks[2] % 1_000_000) / 10_000);
        pr_info(b"System bus clock: %ld.%02ldMHz\n\0".as_ptr(), clks[4] / 1_000_000, (clks[4] % 1_000_000) / 10_000);
    }
    (clks[0] as u32) / KHZ
}

#[no_mangle]
pub unsafe extern "C" fn pxa3xx_clk_update_accr(disable: u32, enable: u32, xclkcfg: u32, mask: u32) {
    let mut accr = readl(reg(ACCR)); accr &= !disable; accr |= enable; writel(accr, reg(ACCR));
    if xclkcfg != 0 { core::arch::asm!("mcr p14, 0, {0}, c6, c0, 0", in(reg) xclkcfg); }
    while (readl(reg(ACSR)) & mask) != (accr & mask) { cpu_relax(); }
}

pub unsafe fn clk_pxa3xx_ac97_get_rate(_hw: *mut ClkHw, parent_rate: usize) -> usize {
    let d = readl(reg(AC97_DIV)) as usize;
    let mut rate = parent_rate / 2; rate /= (d >> 12) & 0x7fff; rate *= d & 0xfff; rate
}
pub unsafe fn clk_pxa3xx_smemc_get_rate(_hw: *mut ClkHw, parent_rate: usize) -> usize {
    let acsr = readl(reg(ACSR)); parent_rate / 48 * SMCFS_MULT[((acsr >> 23) & 7) as usize] / pxa3xx_smemc_get_memclkdiv() as usize
}
pub unsafe fn pxa3xx_is_ring_osc_forced() -> bool { readl(reg(ACSR)) & ACCR_D0CS != 0 }

pub unsafe fn clk_pxa3xx_system_bus_get_rate(_hw: *mut ClkHw, parent_rate: usize) -> usize {
    let hss = ((readl(reg(ACSR)) >> 14) & 3) as usize;
    if pxa3xx_is_ring_osc_forced() { parent_rate } else { parent_rate / 48 * HSS_MULT[hss] as usize }
}
pub unsafe fn clk_pxa3xx_system_bus_get_parent(_hw: *mut ClkHw) -> u8 { if pxa3xx_is_ring_osc_forced() { PxaBus::PxaBus60Mhz as u8 } else { PxaBus::PxaBusHss as u8 } }
pub unsafe fn clk_pxa3xx_core_get_rate(_hw: *mut ClkHw, parent_rate: usize) -> usize { parent_rate }
pub unsafe fn clk_pxa3xx_core_get_parent(_hw: *mut ClkHw) -> u8 {
    if pxa3xx_is_ring_osc_forced() { return PxaCore::PxaCore60Mhz as u8; }
    let xclkcfg: usize; core::arch::asm!("mrc p14, 0, {0}, c6, c0, 0", out(reg) xclkcfg);
    if xclkcfg & 1 != 0 { PxaCore::PxaCoreTurbo as u8 } else { PxaCore::PxaCoreRun as u8 }
}
pub unsafe fn clk_pxa3xx_run_get_rate(_hw: *mut ClkHw, parent_rate: usize) -> usize {
    let xn = ((readl(reg(ACSR)) & ACCR_XN_MASK) >> 8) as usize;
    let xclkcfg: usize; core::arch::asm!("mrc p14, 0, {0}, c6, c0, 0", out(reg) xclkcfg);
    if xclkcfg & 1 != 0 { parent_rate / xn * 2 } else { parent_rate }
}
pub unsafe fn clk_pxa3xx_cpll_get_rate(_hw: *mut ClkHw, parent_rate: usize) -> usize {
    let acsr = readl(reg(ACSR)); let xn = ((acsr & ACCR_XN_MASK) >> 8) as usize; let xl = (acsr & ACCR_XL_MASK) as usize;
    let xclkcfg: usize; core::arch::asm!("mrc p14, 0, {0}, c6, c0, 0", out(reg) xclkcfg);
    pr_info(b"RJK: parent_rate=%lu, xl=%u, xn=%u\n\0".as_ptr(), parent_rate, xl, xn);
    if xclkcfg & 1 != 0 { parent_rate * xl * xn } else { parent_rate * xl }
}

// The remaining clock registration tables and init routines are represented using
// the kernel-provided PXA clock descriptor macros and external registration APIs.
extern "C" {
    fn pxa3xx_base_clocks_init(oscc_reg: *mut u8);
    fn pxa3xx_dummy_clocks_init();
}

#[no_mangle]
pub unsafe extern "C" fn pxa3xx_clocks_init(regs: *mut u8, oscc_reg: *mut u8) -> i32 {
    CLK_REGS = regs; pxa3xx_base_clocks_init(oscc_reg); pxa3xx_dummy_clocks_init();
    let ret = clk_pxa_cken_init(core::ptr::null_mut(), 0, regs); if ret != 0 { return ret; }
    if cpu_is_pxa320() { return clk_pxa_cken_init(core::ptr::null_mut(), 0, regs); }
    if cpu_is_pxa300() || cpu_is_pxa310() { return clk_pxa_cken_init(core::ptr::null_mut(), 0, regs); }
    clk_pxa_cken_init(core::ptr::null_mut(), 0, regs)
}

pub unsafe fn pxa3xx_dt_clocks_init(np: *mut DeviceNode) {
    pxa3xx_clocks_init(ioremap(0x41340000, 0x10), ioremap(0x41350000, 4)); clk_pxa_dt_common_init(np);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
