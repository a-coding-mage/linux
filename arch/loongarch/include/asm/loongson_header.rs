/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/init.h, linux/io.h, linux/irq.h, linux/pci.h,
// asm/addrspace.h, and asm/bootinfo.h.

pub const LOONGSON_LIO_BASE: usize = 0x18000000;
pub const LOONGSON_LIO_SIZE: usize = 0x00100000; // 1M
pub const LOONGSON_LIO_TOP: usize = LOONGSON_LIO_BASE + LOONGSON_LIO_SIZE - 1;

pub const LOONGSON_BOOT_BASE: usize = 0x1c000000;
pub const LOONGSON_BOOT_SIZE: usize = 0x02000000; // 32M
pub const LOONGSON_BOOT_TOP: usize = LOONGSON_BOOT_BASE + LOONGSON_BOOT_SIZE - 1;

pub const LOONGSON_REG_BASE: usize = 0x1fe00000;
pub const LOONGSON_REG_SIZE: usize = 0x00100000; // 1M
pub const LOONGSON_REG_TOP: usize = LOONGSON_REG_BASE + LOONGSON_REG_SIZE - 1;

// GPIO Regs - r/w
pub const LOONGSON_REG_GPIO_BASE: usize = LOONGSON_REG_BASE + 0x11c;

// Equivalent of LOONGSON_REG(x): volatile u32 access at the uncached register address.
#[inline]
pub unsafe fn loongson_reg(x: usize) -> *mut u32 {
    TO_UNCACHE(LOONGSON_REG_BASE + x) as *mut u32
}

pub const LOONGSON_GPIODATA: *mut u32 = unsafe { loongson_reg(0x11c) };
pub const LOONGSON_GPIOIE: *mut u32 = unsafe { loongson_reg(0x120) };

pub const MAX_PACKAGES: usize = 16;

// Direct equivalents of the original read access macros.
macro_rules! xconf_readl {
    ($addr:expr) => { readl($addr) };
}
macro_rules! xconf_readq {
    ($addr:expr) => { readq($addr) };
}

#[inline]
pub unsafe fn xconf_writel(val: u32, addr: *mut core::ffi::c_void) {
    core::arch::asm!(
        "st.w {v}, {hw}, 0",
        "ld.b $zero, {hw}, 0",
        hw = in(reg) addr,
        v = in(reg) val,
    );
}

#[inline]
pub unsafe fn xconf_writeq(val64: u64, addr: *mut core::ffi::c_void) {
    core::arch::asm!(
        "st.d {v}, {hw}, 0",
        "ld.b $zero, {hw}, 0",
        hw = in(reg) addr,
        v = in(reg) val64,
    );
}

/* ============== LS7A registers =============== */
pub const LS7A_PCH_REG_BASE: usize = 0x10000000;
/* LPC regs */
pub const LS7A_LPC_REG_BASE: usize = LS7A_PCH_REG_BASE + 0x00002000;
/* CHIPCFG regs */
pub const LS7A_CHIPCFG_REG_BASE: usize = LS7A_PCH_REG_BASE + 0x00010000;
/* MISC reg base */
pub const LS7A_MISC_REG_BASE: usize = LS7A_PCH_REG_BASE + 0x00080000;
/* ACPI regs */
pub const LS7A_ACPI_REG_BASE: usize = LS7A_MISC_REG_BASE + 0x00050000;
/* RTC regs */
pub const LS7A_RTC_REG_BASE: usize = LS7A_MISC_REG_BASE + 0x00050100;

pub const LS7A_DMA_CFG: *mut core::ffi::c_void = (TO_UNCACHE(LS7A_CHIPCFG_REG_BASE + 0x041c)) as *mut _;
pub const LS7A_DMA_NODE_SHF: usize = 8;
pub const LS7A_DMA_NODE_MASK: usize = 0x1F00;

pub const LS7A_INT_MASK_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_PCH_REG_BASE + 0x020) as *mut _;
pub const LS7A_INT_EDGE_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_PCH_REG_BASE + 0x060) as *mut _;
pub const LS7A_INT_CLEAR_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_PCH_REG_BASE + 0x080) as *mut _;
pub const LS7A_INT_HTMSI_EN_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_PCH_REG_BASE + 0x040) as *mut _;
pub const LS7A_INT_ROUTE_ENTRY_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_PCH_REG_BASE + 0x100) as *mut _;
pub const LS7A_INT_HTMSI_VEC_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_PCH_REG_BASE + 0x200) as *mut _;
pub const LS7A_INT_STATUS_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_PCH_REG_BASE + 0x3a0) as *mut _;
pub const LS7A_INT_POL_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_PCH_REG_BASE + 0x3e0) as *mut _;
pub const LS7A_LPC_INT_CTL: *mut core::ffi::c_void = TO_UNCACHE(LS7A_PCH_REG_BASE + 0x2000) as *mut _;
pub const LS7A_LPC_INT_ENA: *mut core::ffi::c_void = TO_UNCACHE(LS7A_PCH_REG_BASE + 0x2004) as *mut _;
pub const LS7A_LPC_INT_STS: *mut core::ffi::c_void = TO_UNCACHE(LS7A_PCH_REG_BASE + 0x2008) as *mut _;
pub const LS7A_LPC_INT_CLR: *mut core::ffi::c_void = TO_UNCACHE(LS7A_PCH_REG_BASE + 0x200c) as *mut _;
pub const LS7A_LPC_INT_POL: *mut core::ffi::c_void = TO_UNCACHE(LS7A_PCH_REG_BASE + 0x2010) as *mut _;

pub const LS7A_PMCON_SOC_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x000) as *mut _;
pub const LS7A_PMCON_RESUME_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x004) as *mut _;
pub const LS7A_PMCON_RTC_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x008) as *mut _;
pub const LS7A_PM1_EVT_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x00c) as *mut _;
pub const LS7A_PM1_ENA_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x010) as *mut _;
pub const LS7A_PM1_CNT_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x014) as *mut _;
pub const LS7A_PM1_TMR_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x018) as *mut _;
pub const LS7A_P_CNT_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x01c) as *mut _;
pub const LS7A_GPE0_STS_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x028) as *mut _;
pub const LS7A_GPE0_ENA_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x02c) as *mut _;
pub const LS7A_RST_CNT_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x030) as *mut _;
pub const LS7A_WD_SET_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x034) as *mut _;
pub const LS7A_WD_TIMER_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x038) as *mut _;
pub const LS7A_THSENS_CNT_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x04c) as *mut _;
pub const LS7A_GEN_RTC_1_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x050) as *mut _;
pub const LS7A_GEN_RTC_2_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x054) as *mut _;
pub const LS7A_DPM_CFG_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x400) as *mut _;
pub const LS7A_DPM_STS_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x404) as *mut _;
pub const LS7A_DPM_CNT_REG: *mut core::ffi::c_void = TO_UNCACHE(LS7A_ACPI_REG_BASE + 0x408) as *mut _;

macro_rules! ls7a_readb { ($addr:expr) => { core::ptr::read_volatile(TO_UNCACHE($addr) as *const u8) }; }
macro_rules! ls7a_readw { ($addr:expr) => { core::ptr::read_volatile(TO_UNCACHE($addr) as *const u16) }; }
macro_rules! ls7a_readl { ($addr:expr) => { core::ptr::read_volatile(TO_UNCACHE($addr) as *const u32) }; }
macro_rules! ls7a_readq { ($addr:expr) => { core::ptr::read_volatile(TO_UNCACHE($addr) as *const usize) }; }
macro_rules! ls7a_writeb { ($val:expr, $addr:expr) => { core::ptr::write_volatile(TO_UNCACHE($addr) as *mut u8, $val) }; }
macro_rules! ls7a_writew { ($val:expr, $addr:expr) => { core::ptr::write_volatile(TO_UNCACHE($addr) as *mut u16, $val) }; }
macro_rules! ls7a_writel { ($val:expr, $addr:expr) => { core::ptr::write_volatile(TO_UNCACHE($addr) as *mut u32, $val) }; }
macro_rules! ls7a_writeq { ($val:expr, $addr:expr) => { core::ptr::write_volatile(TO_UNCACHE($addr) as *mut usize, $val) }; }

pub const HT1LO_OFFSET: u64 = 0xe0000000000;
pub const MCFG_EXT_PCICFG_BASE: u64 = 0xefe00000000;

#[repr(u32)]
pub enum AcpiEventStatusBits {
    ACPI_PCI_HOTPLUG_STATUS = 1 << 1,
    ACPI_CPU_HOTPLUG_STATUS = 1 << 2,
    ACPI_MEM_HOTPLUG_STATUS = 1 << 3,
    ACPI_POWERBUTTON_STATUS = 1 << 8,
    ACPI_RTC_WAKE_STATUS = 1 << 10,
    ACPI_PCI_WAKE_STATUS = 1 << 14,
    ACPI_ANY_WAKE_STATUS = 1 << 15,
}

extern "C" {
    pub fn enable_gpe_wakeup();
    pub fn enable_pci_wakeup();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
