// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2012 (C), Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 *
 * arch/arm/mach-orion5x/board-dt.c
 *
 * Flattened Device Tree board initialization
 */

// C dependencies supplied by the surrounding kernel translation unit.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct OfDevAuxdata {
    pub compatible: *const c_char,
    pub phys_addr: u32,
    pub name: *const c_char,
    pub platform_data: *mut c_void,
}

extern "C" {
    static mut orion5x_tclk: u32;
    fn orion5x_id(dev: *mut u32, rev: *mut u32, dev_name: *mut *mut c_char);
    fn printk(fmt: *const c_char, ...);
    fn mvebu_mbus_dt_init(sync_only: bool) -> i32;
    fn orion5x_setup_wins();
    fn cpu_idle_poll_ctrl(enabled: bool);
    fn of_machine_is_compatible(compatible: *const c_char) -> bool;
    fn mss2_init();
    fn d2net_init();
    fn of_platform_default_populate(
        root: *const c_void,
        lookup: *const OfDevAuxdata,
        parent: *const c_void,
    ) -> i32;
    fn orion5x_map_io();
    fn orion5x_restart(mode: u32, cmd: *const c_char);
}

const MV88F5281_DEV_ID: u32 = 0x5281;
const MV88F5281_REV_D0: u32 = 0;

static ORION5X_AUXDATA_LOOKUP: [OfDevAuxdata; 6] = [
    OfDevAuxdata {
        compatible: b"marvell,orion-spi\0".as_ptr() as *const c_char,
        phys_addr: 0xf1010600,
        name: b"orion_spi.0\0".as_ptr() as *const c_char,
        platform_data: core::ptr::null_mut(),
    },
    OfDevAuxdata {
        compatible: b"marvell,mv64xxx-i2c\0".as_ptr() as *const c_char,
        phys_addr: 0xf1011000,
        name: b"mv64xxx_i2c.0\0".as_ptr() as *const c_char,
        platform_data: core::ptr::null_mut(),
    },
    OfDevAuxdata {
        compatible: b"marvell,orion-wdt\0".as_ptr() as *const c_char,
        phys_addr: 0xf1020300,
        name: b"orion_wdt\0".as_ptr() as *const c_char,
        platform_data: core::ptr::null_mut(),
    },
    OfDevAuxdata {
        compatible: b"marvell,orion-sata\0".as_ptr() as *const c_char,
        phys_addr: 0xf1080000,
        name: b"sata_mv.0\0".as_ptr() as *const c_char,
        platform_data: core::ptr::null_mut(),
    },
    OfDevAuxdata {
        compatible: b"marvell,orion-crypto\0".as_ptr() as *const c_char,
        phys_addr: 0xf1090000,
        name: b"mv_crypto\0".as_ptr() as *const c_char,
        platform_data: core::ptr::null_mut(),
    },
    OfDevAuxdata {
        compatible: core::ptr::null(),
        phys_addr: 0,
        name: core::ptr::null(),
        platform_data: core::ptr::null_mut(),
    },
];

unsafe fn orion5x_dt_init() {
    let mut dev: u32 = 0;
    let mut rev: u32 = 0;
    let mut dev_name: *mut c_char = core::ptr::null_mut();

    orion5x_id(&mut dev, &mut rev, &mut dev_name);
    printk(
        b"Orion ID: %s. TCLK=%d.\n\0".as_ptr() as *const c_char,
        dev_name,
        orion5x_tclk,
    );

    if mvebu_mbus_dt_init(false) != 0 {
        panic!("BUG_ON(mvebu_mbus_dt_init(false))");
    }

    /*
     * Setup Orion address map
     */
    orion5x_setup_wins();

    /*
     * Don't issue "Wait for Interrupt" instruction if we are
     * running on D0 5281 silicon.
     */
    if dev == MV88F5281_DEV_ID && rev == MV88F5281_REV_D0 {
        printk(
            b"Orion: Applying 5281 D0 WFI workaround.\n\0".as_ptr() as *const c_char,
        );
        cpu_idle_poll_ctrl(true);
    }

    if of_machine_is_compatible(b"maxtor,shared-storage-2\0".as_ptr() as *const c_char) {
        mss2_init();
    }

    if of_machine_is_compatible(b"lacie,d2-network\0".as_ptr() as *const c_char) {
        d2net_init();
    }

    of_platform_default_populate(
        core::ptr::null(),
        ORION5X_AUXDATA_LOOKUP.as_ptr(),
        core::ptr::null(),
    );
}

static ORION5X_DT_COMPAT: [*const c_char; 2] = [
    b"marvell,orion5x\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(ORION5X_DT, "Marvell Orion5x (Flattened Device Tree)")
// Maintainer: Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
#[repr(C)]
pub struct MachineDesc {
    pub map_io: unsafe extern "C" fn(),
    pub init_machine: unsafe fn(),
    pub restart: unsafe extern "C" fn(u32, *const c_char),
    pub dt_compat: *const *const c_char,
}

#[no_mangle]
pub static ORION5X_DT: MachineDesc = MachineDesc {
    map_io: orion5x_map_io,
    init_machine: orion5x_dt_init,
    restart: orion5x_restart,
    dt_compat: ORION5X_DT_COMPAT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
