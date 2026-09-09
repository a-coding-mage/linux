// SPDX-License-Identifier: GPL-2.0
/*
 * Device Tree support for MStar/Sigmastar Armv7 SoCs
 *
 * Copyright (c) 2020 thingy.jp
 * Author: Daniel Palmer <daniel@thingy.jp>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const MSTARV7_L3BRIDGE_FLUSH: usize = 0x14;
const MSTARV7_L3BRIDGE_STATUS: usize = 0x40;
const MSTARV7_L3BRIDGE_FLUSH_TRIGGER: u32 = 1 << 0;
const MSTARV7_L3BRIDGE_STATUS_DONE: u32 = 1 << 12;

#[cfg(CONFIG_SMP)]
const MSTARV7_CPU1_BOOT_ADDR_HIGH: usize = 0x4c;
#[cfg(CONFIG_SMP)]
const MSTARV7_CPU1_BOOT_ADDR_LOW: usize = 0x50;
#[cfg(CONFIG_SMP)]
const MSTARV7_CPU1_UNLOCK: usize = 0x58;
#[cfg(CONFIG_SMP)]
const MSTARV7_CPU1_UNLOCK_MAGIC: u16 = 0xbabe;

static mut L3BRIDGE: *mut u8 = core::ptr::null_mut();

static MSTARV7_BOARD_DT_COMPAT: [*const core::ffi::c_char; 5] = [
    b"mstar,infinity\0".as_ptr() as *const core::ffi::c_char,
    b"mstar,infinity2m\0".as_ptr() as *const core::ffi::c_char,
    b"mstar,infinity3\0".as_ptr() as *const core::ffi::c_char,
    b"mstar,mercury5\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

extern "C" {
    fn writel_relaxed(value: u32, address: *mut u8);
    fn readl_relaxed(address: *mut u8) -> u32;
}

/*
 * In the u-boot code the area these registers are in is called "L3 bridge".
 * The vendor code uses this operation to flush pending CPU writes before DMA.
 */
unsafe fn mstarv7_mb() {
    /* toggle the flush miu pipe fire bit */
    writel_relaxed(0, L3BRIDGE.add(MSTARV7_L3BRIDGE_FLUSH));
    writel_relaxed(
        MSTARV7_L3BRIDGE_FLUSH_TRIGGER,
        L3BRIDGE.add(MSTARV7_L3BRIDGE_FLUSH),
    );
    while readl_relaxed(L3BRIDGE.add(MSTARV7_L3BRIDGE_STATUS))
        & MSTARV7_L3BRIDGE_STATUS_DONE == 0
    {
        /* wait for flush to complete */
    }
}

#[cfg(CONFIG_SMP)]
extern "C" {
    fn __pa_symbol(symbol: unsafe extern "C" fn());
    fn secondary_startup_arm();
    fn of_find_compatible_node(
        from: *mut core::ffi::c_void,
        type_: *const core::ffi::c_char,
        compatible: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_void;
    fn of_iomap(node: *mut core::ffi::c_void, index: i32) -> *mut u8;
    fn writew(value: u16, address: *mut u8);
    fn arch_send_wakeup_ipi_mask(mask: *mut core::ffi::c_void);
    fn cpumask_of(cpu: u32) -> *mut core::ffi::c_void;
    fn iounmap(address: *mut u8);
}

#[cfg(CONFIG_SMP)]
unsafe fn mstarv7_boot_secondary(cpu: u32, _idle: *mut core::ffi::c_void) -> i32 {
    /* right now we don't know how to boot anything except cpu 1. */
    if cpu != 1 {
        return -22; // -EINVAL
    }

    let np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"mstar,smpctrl\0".as_ptr() as *const core::ffi::c_char,
    );
    let smpctrl = of_iomap(np, 0);

    if smpctrl.is_null() {
        return -19; // -ENODEV
    }

    /* set the boot address for the second cpu */
    let bootaddr = __pa_symbol(secondary_startup_arm) as usize as u32;
    writew((bootaddr & 0xffff) as u16, smpctrl.add(MSTARV7_CPU1_BOOT_ADDR_LOW));
    writew(
        ((bootaddr >> 16) & 0xffff) as u16,
        smpctrl.add(MSTARV7_CPU1_BOOT_ADDR_HIGH),
    );

    /* unlock the second cpu */
    writew(MSTARV7_CPU1_UNLOCK_MAGIC, smpctrl.add(MSTARV7_CPU1_UNLOCK));

    /* and away we go...*/
    arch_send_wakeup_ipi_mask(cpumask_of(cpu));
    iounmap(smpctrl);
    0
}

#[cfg(CONFIG_SMP)]
// Equivalent to: static const struct smp_operations mstarv7_smp_ops = {
//     .smp_boot_secondary = mstarv7_boot_secondary,
// };
static MSTARV7_SMP_OPS: Option<unsafe fn(u32, *mut core::ffi::c_void) -> i32> =
    Some(mstarv7_boot_secondary);

unsafe fn mstarv7_init() {
    let np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"mstar,l3bridge\0".as_ptr() as *const core::ffi::c_char,
    );
    L3BRIDGE = of_iomap(np, 0);
    if !L3BRIDGE.is_null() {
        soc_mb = Some(mstarv7_mb);
    } else {
        pr_warn!("Failed to install memory barrier, DMA will be broken!\n");
    }
}

// DT_MACHINE_START(MSTARV7_DT, "MStar/Sigmastar Armv7 (Device Tree)")
//     .dt_compat = mstarv7_board_dt_compat,
//     .init_machine = mstarv7_init,
//     .smp = smp_ops(mstarv7_smp_ops),
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
