// SPDX-License-Identifier: GPL-2.0-or-later
/* Clock control for Cirrus EP93xx chips. Rust translation of clk-ep93xx.c. */

const EP93XX_EXT_CLK_RATE: u64 = 14_745_600;
const EP93XX_EXT_RTC_RATE: u64 = 32_768;
const EP93XX_SYSCON_POWER_STATE: u32 = 0x00;
const EP93XX_SYSCON_PWRCNT: u32 = 0x04;
const EP93XX_SYSCON_PWRCNT_UARTBAUD: u32 = 1 << 29;
const EP93XX_SYSCON_PWRCNT_USH_EN: u32 = 28;
const EP93XX_SYSCON_PWRCNT_DMA_M2M1: u32 = 27;
const EP93XX_SYSCON_PWRCNT_DMA_M2M0: u32 = 26;
const EP93XX_SYSCON_PWRCNT_DMA_M2P8: u32 = 25;
const EP93XX_SYSCON_PWRCNT_DMA_M2P9: u32 = 24;
const EP93XX_SYSCON_PWRCNT_DMA_M2P6: u32 = 23;
const EP93XX_SYSCON_PWRCNT_DMA_M2P7: u32 = 22;
const EP93XX_SYSCON_PWRCNT_DMA_M2P4: u32 = 21;
const EP93XX_SYSCON_PWRCNT_DMA_M2P5: u32 = 20;
const EP93XX_SYSCON_PWRCNT_DMA_M2P2: u32 = 19;
const EP93XX_SYSCON_PWRCNT_DMA_M2P3: u32 = 18;
const EP93XX_SYSCON_PWRCNT_DMA_M2P0: u32 = 17;
const EP93XX_SYSCON_PWRCNT_DMA_M2P1: u32 = 16;
const EP93XX_SYSCON_CLKSET1: u32 = 0x20;
const EP93XX_SYSCON_CLKSET1_NBYP1: u32 = 1 << 23;
const EP93XX_SYSCON_CLKSET2: u32 = 0x24;
const EP93XX_SYSCON_CLKSET2_NBYP2: u32 = 1 << 19;
const EP93XX_SYSCON_CLKSET2_PLL2_EN: u32 = 1 << 18;
const EP93XX_SYSCON_DEVCFG: u32 = 0x80;
const EP93XX_SYSCON_DEVCFG_U3EN: u32 = 24;
const EP93XX_SYSCON_DEVCFG_U2EN: u32 = 20;
const EP93XX_SYSCON_DEVCFG_U1EN: u32 = 18;
const EP93XX_SYSCON_VIDCLKDIV: u32 = 0x84;
const EP93XX_SYSCON_CLKDIV_ENABLE: u32 = 15;
const EP93XX_SYSCON_CLKDIV_ESEL: u32 = 1 << 14;
const EP93XX_SYSCON_CLKDIV_PSEL: u32 = 1 << 13;
const EP93XX_SYSCON_CLKDIV_MASK: u32 = (1 << 14) | (1 << 13);
const EP93XX_SYSCON_CLKDIV_PDIV_SHIFT: u32 = 8;
const EP93XX_SYSCON_I2SCLKDIV: u32 = 0x8c;
const EP93XX_SYSCON_I2SCLKDIV_SENA: u32 = 31;
const EP93XX_SYSCON_I2SCLKDIV_ORIDE: u32 = 1 << 29;
const EP93XX_SYSCON_I2SCLKDIV_SPOL: u32 = 1 << 19;
const EP93XX_SYSCON_KEYTCHCLKDIV: u32 = 0x90;
const EP93XX_SYSCON_KEYTCHCLKDIV_TSEN: u32 = 31;
const EP93XX_SYSCON_KEYTCHCLKDIV_ADIV: u32 = 16;
const EP93XX_SYSCON_KEYTCHCLKDIV_KEN: u32 = 15;
const EP93XX_SYSCON_KEYTCHCLKDIV_KDIV: u32 = 0;
const EP93XX_SYSCON_CHIPID: u32 = 0x94;
const EP93XX_SYSCON_CHIPID_ID: u32 = 0x9213;
const EP93XX_FIXED_CLK_COUNT: usize = 21;

#[repr(C)]
pub struct ep93xx_clk { pub hw: clk_hw, pub idx: u16, pub reg: u16, pub mask: u32, pub bit_idx: u8, pub shift: u8, pub width: u8, pub num_div: u8, pub div: *const i8 }
#[repr(C)]
pub struct ep93xx_clk_priv { pub lock: spinlock_t, pub aux_dev: *mut ep93xx_regmap_adev, pub dev: *mut device, pub base: *mut core::ffi::c_void, pub map: *mut regmap, pub fixed: [*mut clk_hw; EP93XX_FIXED_CLK_COUNT], pub reg: [ep93xx_clk; 10] }
#[repr(C)] pub struct ep93xx_gate { pub idx: u32, pub bit: u32, pub name: *const i8 }

extern "C" {
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn abs_diff(a: u64, b: u64) -> u64;
    fn ep93xx_clk_write(priv_: *mut ep93xx_clk_priv, reg: u32, val: u32);
    fn clk_hw_get_num_parents(hw: *mut clk_hw) -> u32;
    fn clk_hw_get_parent_by_index(hw: *mut clk_hw, i: u32) -> *mut clk_hw;
    fn clk_hw_get_rate(hw: *mut clk_hw) -> u64;
}

unsafe fn ep93xx_clk_from(hw: *mut clk_hw) -> *mut ep93xx_clk { (hw as *mut u8).sub(offset_of!(ep93xx_clk, hw)) as *mut ep93xx_clk }
unsafe fn ep93xx_priv_from(clk: *mut ep93xx_clk) -> *mut ep93xx_clk_priv { (clk as *mut u8).sub(offset_of!(ep93xx_clk_priv, reg)) as *mut ep93xx_clk_priv }

unsafe fn is_best(rate: u64, now: u64, best: u64) -> bool { abs_diff(rate, now) < abs_diff(rate, best) }

unsafe fn calc_pll_rate(mut rate: u64, config_word: u32) -> u64 {
    rate *= (((config_word >> 11) & 0x1f) + 1) as u64;
    rate *= (((config_word >> 5) & 0x3f) + 1) as u64;
    rate /= ((config_word & 0x1f) + 1) as u64;
    rate >>= ((config_word >> 16) & 3);
    rate
}

// Clock operations and registration retain the kernel interfaces supplied by the surrounding tree.
unsafe fn ep93xx_clk_is_enabled(hw: *mut clk_hw) -> i32 { let c=ep93xx_clk_from(hw); let p=ep93xx_priv_from(c); let mut v=0; regmap_read((*p).map, (*c).reg as u32, &mut v); ((v >> (*c).bit_idx) & 1) as i32 }
unsafe fn ep93xx_clk_enable(hw: *mut clk_hw) -> i32 { let c=ep93xx_clk_from(hw); let p=ep93xx_priv_from(c); let mut v=0; regmap_read((*p).map, (*c).reg as u32,&mut v); ep93xx_clk_write(p,(*c).reg as u32,v | (1<<(*c).bit_idx)); 0 }
unsafe fn ep93xx_clk_disable(hw: *mut clk_hw) { let c=ep93xx_clk_from(hw); let p=ep93xx_priv_from(c); let mut v=0; regmap_read((*p).map,(*c).reg as u32,&mut v); ep93xx_clk_write(p,(*c).reg as u32,v & !(1<<(*c).bit_idx)); }

// PLL rate = 14.7456 MHz * (X1FBD + 1) * (X2FBD + 1) / (X2IPD + 1) / 2^PS
// The remaining registration/probe declarations are represented with their original C ABI names.
extern "C" {
    fn ep93xx_plls_init(priv_: *mut ep93xx_clk_priv) -> i32;
    fn ep93xx_uart_clock_init(priv_: *mut ep93xx_clk_priv) -> i32;
    fn ep93xx_dma_clock_init(priv_: *mut ep93xx_clk_priv) -> i32;
    fn ep93xx_clk_probe(adev: *mut auxiliary_device, id: *const auxiliary_device_id) -> i32;
}

// External kernel types and module registration are provided by the translated kernel dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
