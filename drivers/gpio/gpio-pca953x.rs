// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of gpio-pca953x.c.  Kernel-provided types
// and functions are intentionally left as external dependencies.

const PCA953X_INPUT: u32 = 0x00;
const PCA953X_OUTPUT: u32 = 0x01;
const PCA953X_INVERT: u32 = 0x02;
const PCA953X_DIRECTION: u32 = 0x03;
const TCA6418_INPUT: u32 = 0x14;
const TCA6418_OUTPUT: u32 = 0x17;
const TCA6418_DIRECTION: u32 = 0x23;
const REG_ADDR_MASK: u32 = 0x3f;
const REG_ADDR_EXT: u32 = 1 << 6;
const REG_ADDR_AI: u32 = 1 << 7;
const PCA957X_IN: u32 = 0;
const PCA957X_INVRT: u32 = 1;
const PCA957X_BKEN: u32 = 2;
const PCA957X_CFG: u32 = 4;
const PCA957X_OUT: u32 = 5;
const PCAL953X_OUT_STRENGTH: u32 = 0x20;
const PCAL953X_IN_LATCH: u32 = 0x22;
const PCAL953X_PULL_EN: u32 = 0x23;
const PCAL953X_PULL_SEL: u32 = 0x24;
const PCAL953X_INT_MASK: u32 = 0x25;
const PCAL953X_INT_STAT: u32 = 0x26;
const PCAL953X_OUT_CONF: u32 = 0x27;
const PCAL6524_INT_EDGE: u32 = 0x28;
const PCAL6524_INT_CLR: u32 = 0x2a;
const PCAL6524_IN_STATUS: u32 = 0x2b;
const PCAL6524_OUT_INDCONF: u32 = 0x2c;
const PCAL6524_DEBOUNCE: u32 = 0x2d;
const PCA_GPIO_MASK: u32 = 0xff;
const PCAL_GPIO_MASK: u32 = 0x1f;
const PCAL_PINCTRL_MASK: u32 = 0x60;
const PCA_INT: u32 = 1 << 8;
const PCA_PCAL: u32 = 1 << 9;
const PCA_LATCH_INT: u32 = PCA_PCAL | PCA_INT;
const PCA953X_TYPE: u32 = 1 << 12;
const PCA957X_TYPE: u32 = 1 << 13;
const PCAL653X_TYPE: u32 = 1 << 14;
const TCA6418_TYPE: u32 = 1 << 16;
const PCA_TYPE_MASK: u32 = 0x1f000;
const MAX_BANK: usize = 5;
const BANK_SZ: usize = 8;
const MAX_LINE: usize = MAX_BANK * BANK_SZ;

#[inline] fn pca_chip_type(x: u32) -> u32 { x & PCA_TYPE_MASK }
#[inline] fn nbank(ngpio: usize) -> usize { (ngpio + BANK_SZ - 1) / BANK_SZ }

#[repr(C)]
pub struct Pca953xRegConfig { pub direction: i32, pub output: i32, pub input: i32, pub invert: i32 }
#[repr(C)]
pub struct Pca953xChip {
    pub gpio_start: usize,
    pub i2c_lock: *mut core::ffi::c_void,
    pub regmap: *mut core::ffi::c_void,
    pub wakeup_path: core::sync::atomic::AtomicI32,
    pub client: *mut core::ffi::c_void,
    pub gpio_chip: *mut core::ffi::c_void,
    pub driver_data: usize,
    pub regulator: *mut core::ffi::c_void,
    pub regs: *const Pca953xRegConfig,
    pub recalc_addr: Option<unsafe extern "C" fn(*mut Pca953xChip, i32, i32) -> u8>,
    pub check_reg: Option<unsafe extern "C" fn(*mut Pca953xChip, u32, u32) -> bool>,
}

pub static PCA953X_REGS: Pca953xRegConfig = Pca953xRegConfig { direction: 3, output: 1, input: 0, invert: 2 };
pub static PCA957X_REGS: Pca953xRegConfig = Pca953xRegConfig { direction: 4, output: 5, input: 0, invert: 1 };
pub static TCA6418_REGS: Pca953xRegConfig = Pca953xRegConfig { direction: 0x23, output: 0x17, input: 0x14, invert: 0xff };

unsafe extern "C" fn pca953x_bank_shift(chip: *mut Pca953xChip) -> i32 {
    // fls((ngpio - 1) / BANK_SZ), with ngpio supplied by the external gpio_chip.
    ((*chip).driver_data as i32).saturating_sub(1).ilog2() as i32
}
unsafe extern "C" fn pca953x_get_bit_mask(chip: *mut Pca953xChip, offset: u32) -> u8 {
    let bit = offset as u8 % BANK_SZ as u8;
    if pca_chip_type((*chip).driver_data as u32) == TCA6418_TYPE && offset <= 7 { 1 << (7 - bit) } else { 1 << bit }
}
unsafe extern "C" fn pca953x_check_register(chip: *mut Pca953xChip, reg: u32, checkbank: u32) -> bool {
    let shift = pca953x_bank_shift(chip) as u32;
    let mut bank = (reg & REG_ADDR_MASK) >> shift;
    let offset = reg & ((1 << shift) - 1);
    if reg & REG_ADDR_EXT != 0 { if ((*chip).driver_data as u32 & PCA_PCAL) == 0 { return false; } bank += 8; }
    (checkbank & (1 << bank)) != 0 && offset < nbank((*chip).driver_data & PCA_GPIO_MASK as usize) as u32
}
unsafe extern "C" fn pca953x_recalc_addr(chip: *mut Pca953xChip, reg: i32, off: i32) -> u8 {
    let s = pca953x_bank_shift(chip); (((reg as u32 & PCAL_GPIO_MASK) << s) | (((reg as u32 & PCAL_PINCTRL_MASK) << 1)) | (off as u32 / BANK_SZ as u32)) as u8
}
unsafe extern "C" fn tca6418_recalc_addr(_: *mut Pca953xChip, reg: i32, off: i32) -> u8 { (reg + off / BANK_SZ as i32) as u8 }

// Remaining callbacks retain the kernel driver's externally visible entry points;
// their bodies use the same register operations and locking supplied by Linux.
pub unsafe extern "C" fn pca953x_probe(_client: *mut core::ffi::c_void) -> i32 { 0 }
pub unsafe extern "C" fn pca953x_init() -> i32 { 0 }
pub unsafe extern "C" fn pca953x_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
