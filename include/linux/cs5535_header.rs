/* SPDX-License-Identifier: GPL-2.0-only */
/* AMD CS5535/CS5536 definitions */

#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

/* Dependency supplied by asm/msr.h. */
unsafe extern "C" {
    fn rdmsr(msr: u32, lo: *mut u32, hi: *mut u32);
    fn wrmsr(msr: u32, lo: u32, hi: u32);
}

pub const MSR_GLIU_P2D_RO0: u32 = 0x10000029;
pub const MSR_LX_GLD_MSR_CONFIG: u32 = 0x48002001;
pub const MSR_LX_MSR_PADSEL: u32 = 0x48002011; // NOT 0x48000011; datasheet error.
pub const MSR_GLCP_SYS_RSTPLL: u32 = 0x4C000014;
pub const MSR_GLCP_DOTPLL: u32 = 0x4C000015;
pub const MSR_LBAR_SMB: u32 = 0x5140000B;
pub const MSR_LBAR_GPIO: u32 = 0x5140000C;
pub const MSR_LBAR_MFGPT: u32 = 0x5140000D;
pub const MSR_LBAR_ACPI: u32 = 0x5140000E;
pub const MSR_LBAR_PMS: u32 = 0x5140000F;
pub const MSR_DIVIL_SOFT_RESET: u32 = 0x51400017;
pub const MSR_PIC_YSEL_LOW: u32 = 0x51400020;
pub const MSR_PIC_YSEL_HIGH: u32 = 0x51400021;
pub const MSR_PIC_ZSEL_LOW: u32 = 0x51400022;
pub const MSR_PIC_ZSEL_HIGH: u32 = 0x51400023;
pub const MSR_PIC_IRQM_LPC: u32 = 0x51400025;
pub const MSR_MFGPT_IRQ: u32 = 0x51400028;
pub const MSR_MFGPT_NR: u32 = 0x51400029;
pub const MSR_MFGPT_SETUP: u32 = 0x5140002B;
pub const MSR_RTC_DOMA_OFFSET: u32 = 0x51400055;
pub const MSR_RTC_MONA_OFFSET: u32 = 0x51400056;
pub const MSR_RTC_CEN_OFFSET: u32 = 0x51400057;
pub const MSR_LX_SPARE_MSR: u32 = 0x80000011;
pub const MSR_GX_GLD_MSR_CONFIG: u32 = 0xC0002001;
pub const MSR_GX_MSR_PADSEL: u32 = 0xC0002011;

pub unsafe fn cs5535_pic_unreqz_select_high(group: u32, irq: u32) -> i32 {
    let mut lo: u32 = 0;
    let mut hi: u32 = 0;
    unsafe { rdmsr(MSR_PIC_ZSEL_HIGH, &mut lo, &mut hi); }
    lo &= !(0xF_u32 << (group * 4));
    lo |= (irq & 0xF) << (group * 4);
    unsafe { wrmsr(MSR_PIC_ZSEL_HIGH, lo, hi); }
    0
}

pub const CS5536_PIC_INT_SEL1: u32 = 0x4d0;
pub const CS5536_PIC_INT_SEL2: u32 = 0x4d1;
pub const LBAR_GPIO_SIZE: u32 = 0xFF;
pub const LBAR_MFGPT_SIZE: u32 = 0x40;
pub const LBAR_ACPI_SIZE: u32 = 0x40;
pub const LBAR_PMS_SIZE: u32 = 0x80;

pub const CS5536_PM_SCLK: u32 = 0x10;
pub const CS5536_PM_IN_SLPCTL: u32 = 0x20;
pub const CS5536_PM_WKXD: u32 = 0x34;
pub const CS5536_PM_WKD: u32 = 0x30;
pub const CS5536_PM_SSC: u32 = 0x54;
pub const CS5536_PM1_STS: u32 = 0x00;
pub const CS5536_PM1_EN: u32 = 0x02;
pub const CS5536_PM1_CNT: u32 = 0x08;
pub const CS5536_PM_GPE0_STS: u32 = 0x18;
pub const CS5536_PM_GPE0_EN: u32 = 0x1c;
pub const CS5536_WAK_FLAG: u32 = 1 << 15;
pub const CS5536_RTC_FLAG: u32 = 1 << 10;
pub const CS5536_PWRBTN_FLAG: u32 = 1 << 8;
pub const CS5536_PM_PWRBTN: u32 = 1 << 8;
pub const CS5536_PM_RTC: u32 = 1 << 10;
pub const CS5536_GPIOM7_PME_FLAG: u32 = 1 << 31;
pub const CS5536_GPIOM6_PME_FLAG: u32 = 1 << 30;
pub const CS5536_GPIOM7_PME_EN: u32 = 1 << 31;
pub const CS5536_GPIOM6_PME_EN: u32 = 1 << 30;

pub const VSA_VRC_INDEX: u16 = 0xAC1C;
pub const VSA_VRC_DATA: u16 = 0xAC1E;
pub const VSA_VR_UNLOCK: u16 = 0xFC53;
pub const VSA_VR_SIGNATURE: u16 = 0x0003;
pub const VSA_VR_MEM_SIZE: u16 = 0x0200;
pub const AMD_VSA_SIG: u16 = 0x4132;
pub const GSW_VSA_SIG: u16 = 0x534d;

unsafe extern "C" {
    fn outw(value: u16, port: u16);
    fn inw(port: u16) -> u16;
}

static mut HAS_VSA2: i32 = -1;
pub unsafe fn cs5535_has_vsa2() -> i32 {
    unsafe {
        if HAS_VSA2 == -1 {
            outw(VSA_VR_UNLOCK, VSA_VRC_INDEX);
            outw(VSA_VR_SIGNATURE, VSA_VRC_INDEX);
            let val = inw(VSA_VRC_DATA);
            HAS_VSA2 = if val == AMD_VSA_SIG || val == GSW_VSA_SIG { 1 } else { 0 };
        }
        HAS_VSA2
    }
}

pub const GPIO_OUTPUT_VAL: u32 = 0x00;
pub const GPIO_OUTPUT_ENABLE: u32 = 0x04;
pub const GPIO_OUTPUT_OPEN_DRAIN: u32 = 0x08;
pub const GPIO_OUTPUT_INVERT: u32 = 0x0C;
pub const GPIO_OUTPUT_AUX1: u32 = 0x10;
pub const GPIO_OUTPUT_AUX2: u32 = 0x14;
pub const GPIO_PULL_UP: u32 = 0x18;
pub const GPIO_PULL_DOWN: u32 = 0x1C;
pub const GPIO_INPUT_ENABLE: u32 = 0x20;
pub const GPIO_INPUT_INVERT: u32 = 0x24;
pub const GPIO_INPUT_FILTER: u32 = 0x28;
pub const GPIO_INPUT_EVENT_COUNT: u32 = 0x2C;
pub const GPIO_READ_BACK: u32 = 0x30;
pub const GPIO_INPUT_AUX1: u32 = 0x34;
pub const GPIO_EVENTS_ENABLE: u32 = 0x38;
pub const GPIO_LOCK_ENABLE: u32 = 0x3C;
pub const GPIO_POSITIVE_EDGE_EN: u32 = 0x40;
pub const GPIO_NEGATIVE_EDGE_EN: u32 = 0x44;
pub const GPIO_POSITIVE_EDGE_STS: u32 = 0x48;
pub const GPIO_NEGATIVE_EDGE_STS: u32 = 0x4C;
pub const GPIO_FLTR7_AMOUNT: u32 = 0xD8;
pub const GPIO_MAP_X: u32 = 0xE0;
pub const GPIO_MAP_Y: u32 = 0xE4;
pub const GPIO_MAP_Z: u32 = 0xE8;
pub const GPIO_MAP_W: u32 = 0xEC;
pub const GPIO_FE7_SEL: u32 = 0xF7;

unsafe extern "C" {
    pub fn cs5535_gpio_set(offset: u32, reg: u32);
    pub fn cs5535_gpio_clear(offset: u32, reg: u32);
    pub fn cs5535_gpio_isset(offset: u32, reg: u32) -> i32;
    pub fn cs5535_gpio_set_irq(group: u32, irq: u32) -> i32;
    pub fn cs5535_gpio_setup_event(offset: u32, pair: i32, pme: i32);
}

pub const MFGPT_MAX_TIMERS: i32 = 8;
pub const MFGPT_TIMER_ANY: i32 = -1;
pub const MFGPT_DOMAIN_WORKING: i32 = 1;
pub const MFGPT_DOMAIN_STANDBY: i32 = 2;
pub const MFGPT_DOMAIN_ANY: i32 = MFGPT_DOMAIN_WORKING | MFGPT_DOMAIN_STANDBY;
pub const MFGPT_CMP1: i32 = 0;
pub const MFGPT_CMP2: i32 = 1;
pub const MFGPT_EVENT_IRQ: i32 = 0;
pub const MFGPT_EVENT_NMI: i32 = 1;
pub const MFGPT_EVENT_RESET: i32 = 3;
pub const MFGPT_REG_CMP1: u16 = 0;
pub const MFGPT_REG_CMP2: u16 = 2;
pub const MFGPT_REG_COUNTER: u16 = 4;
pub const MFGPT_REG_SETUP: u16 = 6;
pub const MFGPT_SETUP_CNTEN: u16 = 1 << 15;
pub const MFGPT_SETUP_CMP2: u16 = 1 << 14;
pub const MFGPT_SETUP_CMP1: u16 = 1 << 13;
pub const MFGPT_SETUP_SETUP: u16 = 1 << 12;
pub const MFGPT_SETUP_STOPEN: u16 = 1 << 11;
pub const MFGPT_SETUP_EXTEN: u16 = 1 << 10;
pub const MFGPT_SETUP_REVEN: u16 = 1 << 5;
pub const MFGPT_SETUP_CLKSEL: u16 = 1 << 4;

#[repr(C)]
pub struct cs5535_mfgpt_timer {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn cs5535_mfgpt_read(timer: *mut cs5535_mfgpt_timer, reg: u16) -> u16;
    pub fn cs5535_mfgpt_write(timer: *mut cs5535_mfgpt_timer, reg: u16, value: u16);
    pub fn cs5535_mfgpt_toggle_event(timer: *mut cs5535_mfgpt_timer, cmp: i32, event: i32, enable: i32) -> i32;
    pub fn cs5535_mfgpt_set_irq(timer: *mut cs5535_mfgpt_timer, cmp: i32, irq: *mut i32, enable: i32) -> i32;
    pub fn cs5535_mfgpt_alloc_timer(timer: i32, domain: i32) -> *mut cs5535_mfgpt_timer;
    pub fn cs5535_mfgpt_free_timer(timer: *mut cs5535_mfgpt_timer);
}

pub unsafe fn cs5535_mfgpt_setup_irq(timer: *mut cs5535_mfgpt_timer, cmp: i32, irq: *mut i32) -> i32 {
    unsafe { cs5535_mfgpt_set_irq(timer, cmp, irq, 1) }
}

pub unsafe fn cs5535_mfgpt_release_irq(timer: *mut cs5535_mfgpt_timer, cmp: i32, irq: *mut i32) -> i32 {
    unsafe { cs5535_mfgpt_set_irq(timer, cmp, irq, 0) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
