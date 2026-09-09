// SPDX-License-Identifier: GPL-2.0-only
/*
 * System controller support for Armada 370, 375 and XP platforms.
 *
 * Copyright (C) 2012 Marvell
 *
 * Lior Amsalem <alior@marvell.com>
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 *
 * The Armada 370, 375 and Armada XP SoCs have a range of
 * miscellaneous registers, that do not belong to a particular device,
 * but rather provide system-level features. This basic
 * system-controller driver provides a device tree binding for those
 * registers, and implements utility functions offering various
 * features related to those registers.
 *
 * For now, the feature set is limited to restarting the platform by a
 * soft-reset, but it might be extended in the future.
 */

// Linux kernel, device-tree, I/O, reboot, common, SoC-ID, and PMSU
// declarations are supplied by the surrounding translation unit.

use core::ffi::c_void;

const ARMADA_375_CRYPT0_ENG_TARGET: u32 = 41;
const ARMADA_375_CRYPT0_ENG_ATTR: u32 = 1;

static mut system_controller_base: *mut u8 = core::ptr::null_mut();
static mut system_controller_phys_base: usize = 0;

#[repr(C)]
struct mvebu_system_controller {
    rstoutn_mask_offset: u32,
    system_soft_reset_offset: u32,
    rstoutn_mask_reset_out_en: u32,
    system_soft_reset: u32,
    resume_boot_addr: u32,
    dev_id: u32,
    rev_id: u32,
}

static mut mvebu_sc: *mut mvebu_system_controller = core::ptr::null_mut();

static armada_370_xp_system_controller: mvebu_system_controller = mvebu_system_controller {
    rstoutn_mask_offset: 0x60,
    system_soft_reset_offset: 0x64,
    rstoutn_mask_reset_out_en: 0x1,
    system_soft_reset: 0x1,
    resume_boot_addr: 0,
    dev_id: 0x38,
    rev_id: 0x3c,
};

static armada_375_system_controller: mvebu_system_controller = mvebu_system_controller {
    rstoutn_mask_offset: 0x54,
    system_soft_reset_offset: 0x58,
    rstoutn_mask_reset_out_en: 0x1,
    system_soft_reset: 0x1,
    resume_boot_addr: 0xd4,
    dev_id: 0x38,
    rev_id: 0x3c,
};

static orion_system_controller: mvebu_system_controller = mvebu_system_controller {
    rstoutn_mask_offset: 0x108,
    system_soft_reset_offset: 0x10c,
    rstoutn_mask_reset_out_en: 0x4,
    system_soft_reset: 0x1,
    resume_boot_addr: 0,
    dev_id: 0,
    rev_id: 0,
};

#[repr(C)]
struct of_device_id {
    compatible: &'static str,
    data: *const c_void,
}

#[repr(C)]
struct device_node;
#[repr(C)]
struct resource { start: usize }

extern "C" {
    fn of_machine_is_compatible(compatible: *const i8) -> bool;
    fn of_find_matching_node_and_match(np: *mut device_node, table: *const of_device_id,
                                       match_out: *mut *const of_device_id) -> *mut device_node;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut u8;
    fn of_address_to_resource(np: *mut device_node, index: i32, res: *mut resource) -> i32;
    fn of_node_put(np: *mut device_node);
}

static of_system_controller_table: [of_device_id; 4] = [
    of_device_id {
        compatible: "marvell,orion-system-controller",
        data: &orion_system_controller as *const _ as *const c_void,
    },
    of_device_id {
        compatible: "marvell,armada-370-xp-system-controller",
        data: &armada_370_xp_system_controller as *const _ as *const c_void,
    },
    of_device_id {
        compatible: "marvell,armada-375-system-controller",
        data: &armada_375_system_controller as *const _ as *const c_void,
    },
    of_device_id { compatible: "", data: core::ptr::null() },
];

pub unsafe fn mvebu_restart(_mode: *mut c_void, _cmd: *const i8) {
    if system_controller_base.is_null() {
        // pr_err("Cannot restart, system-controller not available: check the device tree\n");
    } else {
        core::ptr::write_volatile(
            system_controller_base.add((*mvebu_sc).rstoutn_mask_offset as usize) as *mut u32,
            (*mvebu_sc).rstoutn_mask_reset_out_en,
        );
        core::ptr::write_volatile(
            system_controller_base.add((*mvebu_sc).system_soft_reset_offset as usize) as *mut u32,
            (*mvebu_sc).system_soft_reset,
        );
    }

    loop {}
}

pub unsafe fn mvebu_system_controller_get_soc_id(dev: *mut u32, rev: *mut u32) -> i32 {
    if of_machine_is_compatible(b"marvell,armada380\0".as_ptr() as *const i8)
        && !system_controller_base.is_null() {
        *dev = core::ptr::read_volatile(
            system_controller_base.add((*mvebu_sc).dev_id as usize) as *const u32,
        ) >> 16;
        *rev = (core::ptr::read_volatile(
            system_controller_base.add((*mvebu_sc).rev_id as usize) as *const u32,
        ) >> 8) & 0xF;
        0
    } else {
        -19
    }
}

// Preserved from CONFIG_SMP && CONFIG_MACH_MVEBU_V7.
unsafe fn mvebu_armada375_smp_wa_init() {
    let mut dev: u32 = 0;
    let mut rev: u32 = 0;
    let resume_addr_reg: usize;

    if mvebu_system_controller_get_soc_id(&mut dev, &mut rev) != 0 {
        return;
    }
    resume_addr_reg = system_controller_phys_base + (*mvebu_sc).resume_boot_addr as usize;
    // mvebu_setup_boot_addr_wa(ARMADA_375_CRYPT0_ENG_TARGET,
    //                          ARMADA_375_CRYPT0_ENG_ATTR, resume_addr_reg);
    let _ = (ARMADA_375_CRYPT0_ENG_TARGET, ARMADA_375_CRYPT0_ENG_ATTR, resume_addr_reg);
}

pub unsafe fn mvebu_system_controller_set_cpu_boot_addr(boot_addr: *mut c_void) {
    // BUG_ON(system_controller_base == NULL);
    // BUG_ON(mvebu_sc->resume_boot_addr == 0);
    if core::ptr::eq(system_controller_base, core::ptr::null_mut()) {
        panic!("BUG_ON");
    }
    if (*mvebu_sc).resume_boot_addr == 0 {
        panic!("BUG_ON");
    }

    if of_machine_is_compatible(b"marvell,armada375\0".as_ptr() as *const i8) {
        mvebu_armada375_smp_wa_init();
    }
    core::ptr::write_volatile(
        system_controller_base.add((*mvebu_sc).resume_boot_addr as usize) as *mut u32,
        boot_addr as usize as u32,
    );
}

// __init mvebu_system_controller_init and early_initcall are registered by
// the surrounding kernel translation; device-tree helper calls remain external.
unsafe fn mvebu_system_controller_init() -> i32 {
    let mut match_ptr: *const of_device_id = core::ptr::null();
    let np = of_find_matching_node_and_match(
        core::ptr::null_mut(), of_system_controller_table.as_ptr(), &mut match_ptr);
    if !np.is_null() {
        let mut res = resource { start: 0 };
        system_controller_base = of_iomap(np, 0);
        of_address_to_resource(np, 0, &mut res);
        system_controller_phys_base = res.start;
        mvebu_sc = (*match_ptr).data as *mut mvebu_system_controller;
        of_node_put(np);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
