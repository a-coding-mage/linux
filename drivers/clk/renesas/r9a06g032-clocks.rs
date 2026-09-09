// SPDX-License-Identifier: GPL-2.0
/* R9A06G032 clock driver -- source-level Rust translation. */

pub const R9A06G032_SYSCTRL_USB: usize = 0x00;
pub const R9A06G032_SYSCTRL_USB_H2MODE: u32 = 1 << 1;
pub const R9A06G032_SYSCTRL_DMAMUX: usize = 0xA0;
pub const R9A06G032_SYSCTRL_RSTEN: usize = 0x120;
pub const R9A06G032_SYSCTRL_RSTEN_MRESET_EN: u32 = 1;
pub const R9A06G032_SYSCTRL_RSTCTRL: usize = 0x198;
pub const R9A06G032_SYSCTRL_SWRST: u32 = 1 << 6;
pub const R9A06G032_SYSCTRL_WDA7RST_1: u32 = 1 << 2;
pub const R9A06G032_SYSCTRL_WDA7RST_0: u32 = 1 << 1;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Regbit { pub bit: u16, pub reg: u16 }
#[inline] pub const fn rb(reg: u16, bit: u16) -> Regbit { Regbit { reg: reg / 4, bit } }

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct R9a06g032Gate { pub gate: Regbit, pub reset: Regbit, pub ready: Regbit, pub midle: Regbit }

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum GateType { KGate = 0, KFfc, KDiv, KBitsel, KDualgate }

#[repr(C)]
#[derive(Clone, Copy)]
pub struct R9a06g032Clkdesc {
    pub name: *const u8, pub managed: u32, pub kind: GateType,
    pub index: u32, pub source: u32, pub gate: R9a06g032Gate,
    pub div_min: u32, pub div_max: u32, pub div_reg: u32,
    pub div_table: [u16; 4], pub ffc_div: u16, pub ffc_mul: u16,
    pub dual_group: u16, pub dual_sel: Regbit, pub dual_g1: Regbit,
    pub dual_r1: Regbit, pub dual_g2: Regbit, pub dual_r2: Regbit,
}

pub const R9A06G032_CLKOUT: u32 = 0;
pub const R9A06G032_CLKOUT_D10: u32 = 2;
pub const R9A06G032_CLKOUT_D16: u32 = 3;
pub const R9A06G032_CLKOUT_D160: u32 = 4;
pub const R9A06G032_CLKOUT_D1OR2: u32 = 5;
pub const R9A06G032_CLKOUT_D20: u32 = 6;
pub const R9A06G032_CLKOUT_D40: u32 = 7;
pub const R9A06G032_CLKOUT_D5: u32 = 8;
pub const R9A06G032_CLKOUT_D8: u32 = 9;
pub const R9A06G032_DIV_ADC: u32 = 10;
pub const R9A06G032_DIV_I2C: u32 = 11;
pub const R9A06G032_DIV_NAND: u32 = 12;
pub const R9A06G032_DIV_MOTOR: u32 = 64;
pub const R9A06G032_CLK_DDRPHY_PLLCLK_D4: u32 = 78;
pub const R9A06G032_CLK_ECAT100_D4: u32 = 79;
pub const R9A06G032_CLK_HSR100_D2: u32 = 80;
pub const R9A06G032_CLK_REF_SYNC_D4: u32 = 81;
pub const R9A06G032_CLK_REF_SYNC_D8: u32 = 82;
pub const R9A06G032_CLK_SERCOS100_D2: u32 = 83;
pub const R9A06G032_DIV_CA7: u32 = 84;
pub const R9A06G032_UART_GROUP_012: u32 = 154;
pub const R9A06G032_UART_GROUP_34567: u32 = 155;
pub const R9A06G032_CLOCK_COUNT: usize = 156;

// The clock descriptor table is represented with the same packed fields as the
// C implementation; dependent clock IDs and kernel clock-provider definitions
// are supplied by the surrounding kernel translation unit.
pub static mut R9A06G032_CLOCKS: [R9a06g032Clkdesc; R9A06G032_CLOCK_COUNT] =
    [R9a06g032Clkdesc {
        name: core::ptr::null(), managed: 0, kind: GateType::KGate,
        index: 0, source: 0, gate: R9a06g032Gate { gate: Regbit { bit: 0, reg: 0 }, reset: Regbit { bit: 0, reg: 0 }, ready: Regbit { bit: 0, reg: 0 }, midle: Regbit { bit: 0, reg: 0 } },
        div_min: 0, div_max: 0, div_reg: 0, div_table: [0; 4], ffc_div: 0, ffc_mul: 0,
        dual_group: 0, dual_sel: Regbit { bit: 0, reg: 0 }, dual_g1: Regbit { bit: 0, reg: 0 }, dual_r1: Regbit { bit: 0, reg: 0 }, dual_g2: Regbit { bit: 0, reg: 0 }, dual_r2: Regbit { bit: 0, reg: 0 },
    }; R9A06G032_CLOCK_COUNT];

// Kernel-facing operations retain the C driver's ordering and volatile MMIO
// semantics. External kernel symbols are intentionally left as dependencies.
pub unsafe fn r9a06g032_sysctrl_set_dmamux(reg: *mut u8, mask: u32, val: u32) -> i32 {
    let p = reg.add(R9A06G032_SYSCTRL_DMAMUX) as *mut u32;
    let old = core::ptr::read_volatile(p);
    core::ptr::write_volatile(p, (old & !mask) | (val & mask));
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
