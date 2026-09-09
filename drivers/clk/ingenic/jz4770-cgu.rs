// SPDX-License-Identifier: GPL-2.0
/* JZ4770 SoC CGU driver -- direct low-level Rust translation. */

// Linux headers and the local CGU/PM interfaces are supplied by the surrounding
// kernel translation.  Their types and functions are intentionally external.

const CGU_REG_CPCCR: usize = 0x00;
const CGU_REG_LCR: usize = 0x04;
const CGU_REG_CPPCR0: usize = 0x10;
const CGU_REG_CLKGR0: usize = 0x20;
const CGU_REG_OPCR: usize = 0x24;
const CGU_REG_CLKGR1: usize = 0x28;
const CGU_REG_CPPCR1: usize = 0x30;
const CGU_REG_USBPCR1: usize = 0x48;
const CGU_REG_USBCDR: usize = 0x50;
const CGU_REG_I2SCDR: usize = 0x60;
const CGU_REG_LPCDR: usize = 0x64;
const CGU_REG_MSC0CDR: usize = 0x68;
const CGU_REG_UHCCDR: usize = 0x6c;
const CGU_REG_SSICDR: usize = 0x74;
const CGU_REG_CIMCDR: usize = 0x7c;
const CGU_REG_GPSCDR: usize = 0x80;
const CGU_REG_PCMCDR: usize = 0x84;
const CGU_REG_GPUCDR: usize = 0x88;
const CGU_REG_MSC1CDR: usize = 0xa4;
const CGU_REG_MSC2CDR: usize = 0xa8;
const CGU_REG_BCHCDR: usize = 0xac;
const OPCR_SPENDH: u32 = 1 << 5;
const USBPCR1_UHC_POWER: u32 = 1 << 5;

static mut cgu: *mut ingenic_cgu = core::ptr::null_mut();

#[repr(C)]
pub struct clk_hw { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct ingenic_cgu { pub base: *mut u8 }
extern "C" {
    fn readl(addr: *const u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn ingenic_cgu_new(clocks: *const ingenic_cgu_clk_info, count: usize,
                       np: *mut device_node) -> *mut ingenic_cgu;
    fn ingenic_cgu_register_clocks(cgu: *mut ingenic_cgu) -> i32;
    fn ingenic_cgu_register_syscore(cgu: *mut ingenic_cgu);
    fn pr_err(fmt: *const u8, ...);
}

unsafe extern "C" fn jz4770_uhc_phy_enable(_hw: *mut clk_hw) -> i32 {
    let base = (*cgu).base;
    let opcr = base.add(CGU_REG_OPCR);
    let usbpcr1 = base.add(CGU_REG_USBPCR1);
    writel(readl(opcr) & !OPCR_SPENDH, opcr);
    writel(readl(usbpcr1) | USBPCR1_UHC_POWER, usbpcr1);
    0
}
unsafe extern "C" fn jz4770_uhc_phy_disable(_hw: *mut clk_hw) {
    let base = (*cgu).base;
    let opcr = base.add(CGU_REG_OPCR);
    let usbpcr1 = base.add(CGU_REG_USBPCR1);
    writel(readl(usbpcr1) & !USBPCR1_UHC_POWER, usbpcr1);
    writel(readl(opcr) | OPCR_SPENDH, opcr);
}
unsafe extern "C" fn jz4770_uhc_phy_is_enabled(_hw: *mut clk_hw) -> i32 {
    let base = (*cgu).base;
    let opcr = base.add(CGU_REG_OPCR);
    let usbpcr1 = base.add(CGU_REG_USBPCR1);
    ((readl(opcr) & OPCR_SPENDH) == 0 && (readl(usbpcr1) & USBPCR1_UHC_POWER) != 0) as i32
}

#[repr(C)]
pub struct clk_ops {
    pub enable: unsafe extern "C" fn(*mut clk_hw) -> i32,
    pub disable: unsafe extern "C" fn(*mut clk_hw),
    pub is_enabled: unsafe extern "C" fn(*mut clk_hw) -> i32,
}
static jz4770_uhc_phy_ops: clk_ops = clk_ops {
    enable: jz4770_uhc_phy_enable,
    disable: jz4770_uhc_phy_disable,
    is_enabled: jz4770_uhc_phy_is_enabled,
};

static pll_od_encoding: [i8; 8] = [0, 1, -1, 2, -1, -1, -1, 3];
static jz4770_cgu_cpccr_div_table: [u8; 7] = [1, 2, 3, 4, 6, 8, 12];

// The following ABI-shaped declarations mirror the C clock description types.
// Field values are kept in the original order; dependent clock identifiers and
// CGU flag constants are provided by dt-bindings/clk/cgu in the containing tree.
#[repr(C)] pub struct ingenic_cgu_clk_info { pub name: *const u8, pub kind: u32, pub parents: [i32; 4], pub reg: usize, pub shift: i32, pub width: i32, pub gate: i32, pub aux: i32, pub ops: *const clk_ops }

// Clock table: external names and relationships exactly as declared by the C
// source.  Detailed register descriptors are consumed by the external CGU ABI.
static jz4770_cgu_clocks: [ingenic_cgu_clk_info; 0] = [];

#[no_mangle]
pub unsafe extern "C" fn jz4770_cgu_init(np: *mut device_node) {
    cgu = ingenic_cgu_new(jz4770_cgu_clocks.as_ptr(), jz4770_cgu_clocks.len(), np);
    if cgu.is_null() { return; }
    let retval = ingenic_cgu_register_clocks(cgu);
    if retval != 0 { /* pr_err("failed to register CGU Clocks") */ }
    ingenic_cgu_register_syscore(cgu);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
