/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2006 Lennert Buytenhek <buytenh@wantstofly.org>
 */

/*
 * These machine IDs are no longer used by the kernel since EP93xx was converted
 * to DT booting, but they are still passed in by bootloaders, so we use our own
 * local definitions of the relevant macros.
 */

unsafe extern "C" {
    static __machine_arch_type: u32;
}

#[inline]
fn machine_is_bk3() -> bool {
    unsafe { __machine_arch_type == 1880 }
}

#[inline]
fn machine_is_edb9301() -> bool {
    unsafe { __machine_arch_type == 462 }
}

#[inline]
fn machine_is_edb9302a() -> bool {
    unsafe { __machine_arch_type == 1127 }
}

#[inline]
fn machine_is_edb9302() -> bool {
    unsafe { __machine_arch_type == 538 }
}

#[inline]
fn machine_is_edb9307a() -> bool {
    unsafe { __machine_arch_type == 1128 }
}

#[inline]
fn machine_is_edb9307() -> bool {
    unsafe { __machine_arch_type == 607 }
}

#[inline]
fn machine_is_edb9312() -> bool {
    unsafe { __machine_arch_type == 451 }
}

#[inline]
fn machine_is_edb9315a() -> bool {
    unsafe { __machine_arch_type == 772 }
}

#[inline]
fn machine_is_edb9315() -> bool {
    unsafe { __machine_arch_type == 463 }
}

#[inline]
fn machine_is_ts72xx() -> bool {
    unsafe { __machine_arch_type == 673 }
}

#[inline]
fn machine_is_vision_ep9307() -> bool {
    unsafe { __machine_arch_type == 1578 }
}

#[inline]
unsafe fn __raw_readl(ptr: u32) -> u32 {
    unsafe { core::ptr::read_volatile(ptr as *const u32) }
}

#[inline]
unsafe fn __raw_writeb(value: u8, ptr: u32) {
    unsafe { core::ptr::write_volatile(ptr as *mut u8, value) }
}

#[inline]
unsafe fn __raw_writel(value: u32, ptr: u32) {
    unsafe { core::ptr::write_volatile(ptr as *mut u32, value) }
}

/*
 * Some bootloaders don't turn off DMA from the ethernet MAC before
 * jumping to linux, which means that we might end up with bits of RX
 * status and packet data scribbled over the uncompressed kernel image.
 * Work around this by resetting the ethernet MAC before we uncompress.
 */
const PHYS_ETH_SELF_CTL: u32 = 0x80010020;
const ETH_SELF_CTL_RESET: u32 = 0x00000001;

#[inline]
unsafe fn ep93xx_ethernet_reset() {
    let mut v: u32;

    /* Reset the ethernet MAC.  */
    v = unsafe { __raw_readl(PHYS_ETH_SELF_CTL) };
    unsafe { __raw_writel(v | ETH_SELF_CTL_RESET, PHYS_ETH_SELF_CTL) };

    /* Wait for reset to finish.  */
    while unsafe { __raw_readl(PHYS_ETH_SELF_CTL) } & ETH_SELF_CTL_RESET != 0 {}
}

const TS72XX_WDT_CONTROL_PHYS_BASE: u32 = 0x23800000;
const TS72XX_WDT_FEED_PHYS_BASE: u32 = 0x23c00000;
const TS72XX_WDT_FEED_VAL: u8 = 0x05;

#[inline]
#[allow(dead_code)]
unsafe fn ts72xx_watchdog_disable() {
    unsafe { __raw_writeb(TS72XX_WDT_FEED_VAL, TS72XX_WDT_FEED_PHYS_BASE) };
    unsafe { __raw_writeb(0, TS72XX_WDT_CONTROL_PHYS_BASE) };
}

#[inline]
unsafe fn ep93xx_decomp_setup() {
    if machine_is_ts72xx() {
        unsafe { ts72xx_watchdog_disable() };
    }

    if machine_is_edb9301()
        || machine_is_edb9302()
        || machine_is_edb9302a()
        || machine_is_edb9307()
        || machine_is_edb9307a()
        || machine_is_edb9312()
        || machine_is_edb9315()
        || machine_is_edb9315a()
        || machine_is_ts72xx()
        || machine_is_bk3()
        || machine_is_vision_ep9307()
    {
        unsafe { ep93xx_ethernet_reset() };
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
