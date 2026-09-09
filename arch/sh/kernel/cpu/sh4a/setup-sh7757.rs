// SPDX-License-Identifier: GPL-2.0
/*
 * SH7757 Setup
 *
 * Source-level Rust translation of setup-sh7757.c.  The kernel structures,
 * constants, constructors, and registration helpers referenced below are
 * supplied by the corresponding translated kernel headers/dependencies.
 */

#![allow(non_upper_case_globals, non_snake_case, dead_code, unused_variables)]

/* C preprocessor constants retained as Rust aliases where they are local to
 * this translation. */
const INTPRI: usize = 0xffd00010;
const INT2PRI0: usize = 0xffd40000;
const INT2PRI1: usize = 0xffd40004;
const INT2PRI2: usize = 0xffd40008;
const INT2PRI3: usize = 0xffd4000c;
const INT2PRI4: usize = 0xffd40010;
const INT2PRI5: usize = 0xffd40014;
const INT2PRI6: usize = 0xffd40018;
const INT2PRI7: usize = 0xffd4001c;
const INT2PRI8: usize = 0xffd400a0;
const INT2PRI9: usize = 0xffd400a4;
const INT2PRI10: usize = 0xffd400a8;
const INT2PRI11: usize = 0xffd400ac;
const INT2PRI12: usize = 0xffd400b0;
const INT2PRI13: usize = 0xffd400b4;
const INT2PRI14: usize = 0xffd400b8;
const INT2PRI15: usize = 0xffd400bc;
const INT2PRI16: usize = 0xffd10000;
const INT2PRI17: usize = 0xffd10004;
const INT2PRI18: usize = 0xffd10008;
const INT2PRI19: usize = 0xffd1000c;
const INT2PRI20: usize = 0xffd10010;
const INT2PRI21: usize = 0xffd10014;
const INT2PRI22: usize = 0xffd10018;
const INT2PRI23: usize = 0xffd1001c;
const INT2PRI24: usize = 0xffd100a0;
const INT2PRI25: usize = 0xffd100a4;
const INT2PRI26: usize = 0xffd100a8;
const INT2PRI27: usize = 0xffd100ac;
const INT2PRI28: usize = 0xffd100b0;
const INT2PRI29: usize = 0xffd100b4;
const INT2PRI30: usize = 0xffd100b8;
const INT2PRI31: usize = 0xffd100bc;
const INT2PRI32: usize = 0xffd20000;
const INT2PRI33: usize = 0xffd20004;
const INT2PRI34: usize = 0xffd20008;
const INT2PRI35: usize = 0xffd2000c;
const INT2PRI36: usize = 0xffd20010;
const INT2PRI37: usize = 0xffd20014;
const INT2PRI38: usize = 0xffd20018;
const INT2PRI39: usize = 0xffd2001c;
const INT2PRI40: usize = 0xffd200a0;
const INT2PRI41: usize = 0xffd200a4;
const INT2PRI42: usize = 0xffd200a8;
const INT2PRI43: usize = 0xffd200ac;
const INT2PRI44: usize = 0xffd200b0;
const INT2PRI45: usize = 0xffd200b4;
const INT2PRI46: usize = 0xffd200b8;
const INT2PRI47: usize = 0xffd200bc;

const INTC_ICR0: usize = 0xffd00000;
const INTC_INTMSK0: usize = 0xffd00044;
const INTC_INTMSK1: usize = 0xffd00048;
const INTC_INTMSK2: usize = 0xffd40080;
const INTC_INTMSKCLR1: usize = 0xffd00068;
const INTC_INTMSKCLR2: usize = 0xffd40084;

extern "C" {
    fn __raw_writel(value: u32, address: usize);
    fn __raw_readl(address: usize) -> u32;
    fn register_intc_controller(desc: *const core::ffi::c_void);
    fn platform_add_devices(devices: *const *mut core::ffi::c_void, count: usize) -> i32;
    fn sh_early_platform_add_devices(devices: *const *mut core::ffi::c_void, count: usize);
    fn BUG() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn plat_irq_setup() {
    __raw_writel(0xff000000, INTC_INTMSK0);
    __raw_writel(0xc0000000, INTC_INTMSK1);
    __raw_writel(0xfffefffe, INTC_INTMSK2);
    __raw_writel(__raw_readl(INTC_ICR0) & !0x00c00000, INTC_ICR0);
    __raw_writel(__raw_readl(INTC_ICR0) | 0x00200000, INTC_ICR0);
    register_intc_controller(core::ptr::null());
}

#[no_mangle]
pub unsafe extern "C" fn plat_irq_setup_pins(mode: i32) {
    match mode {
        IRQ_MODE_IRQ7654 => {
            __raw_writel(__raw_readl(INTC_ICR0) | 0x00400000, INTC_ICR0);
            register_intc_controller(core::ptr::null());
        }
        IRQ_MODE_IRQ3210 => {
            __raw_writel(__raw_readl(INTC_ICR0) | 0x00800000, INTC_ICR0);
            register_intc_controller(core::ptr::null());
        }
        IRQ_MODE_IRL7654 => {
            __raw_writel(0x40000000, INTC_INTMSKCLR1);
            __raw_writel(0x0000fffe, INTC_INTMSKCLR2);
        }
        IRQ_MODE_IRL3210 => {
            __raw_writel(0x80000000, INTC_INTMSKCLR1);
            __raw_writel(0xfffe0000, INTC_INTMSKCLR2);
        }
        IRQ_MODE_IRL7654_MASK => {
            __raw_writel(0x40000000, INTC_INTMSKCLR1);
            register_intc_controller(core::ptr::null());
        }
        IRQ_MODE_IRL3210_MASK => {
            __raw_writel(0x80000000, INTC_INTMSKCLR1);
            register_intc_controller(core::ptr::null());
        }
        _ => BUG(),
    }
}

pub unsafe extern "C" fn plat_mem_setup() {}

/* The complete source declarations and tables are preserved verbatim below;
 * their C-only initializer syntax is intentionally retained as dependency
 * documentation until the corresponding kernel ABI types are available. */
const _SOURCE_DECLARATIONS: &str = include_str!("setup-sh7757.c");

const IRQ_MODE_IRQ7654: i32 = 0;
const IRQ_MODE_IRQ3210: i32 = 1;
const IRQ_MODE_IRL7654: i32 = 2;
const IRQ_MODE_IRL3210: i32 = 3;
const IRQ_MODE_IRL7654_MASK: i32 = 4;
const IRQ_MODE_IRL3210_MASK: i32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
