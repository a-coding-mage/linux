// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerNV OPAL power control for graceful shutdown handling
 *
 * Copyright 2015 IBM Corp.
 */

// C dependencies supplied by the surrounding kernel translation unit.

const SOFT_OFF: u64 = 0x00;
const SOFT_REBOOT: u64 = 0x01;

extern "C" {
    fn opal_get_epow_status(status: *mut u16, classes: *mut u16) -> i32;
    fn opal_get_dpo_status(timeout: *mut u64) -> i32;
    fn orderly_poweroff(force: bool) -> i32;
    fn orderly_reboot() -> i32;
    fn opal_message_notifier_register(msg: u64, nb: *mut notifier_block) -> i32;
    fn of_find_node_by_path(path: *const i8) -> *mut device_node;
    fn of_device_is_compatible(np: *const device_node, compatible: *const i8) -> i32;
    fn of_node_put(np: *mut device_node);
    fn pr_err(fmt: *const i8, ...);
    fn pr_info(fmt: *const i8, ...);
}

const OPAL_SUCCESS: i32 = 0;
const OPAL_SYSEPOW_MAX: usize = 0;
const OPAL_SYSEPOW_POWER: i32 = 0;
const OPAL_SYSPOWER_CHNG: u16 = 0;
const OPAL_SYSPOWER_FAIL: u16 = 0;
const OPAL_SYSPOWER_INCL: u16 = 0;
const OPAL_MSG_EPOW: u64 = 0;
const OPAL_MSG_DPO: u64 = 0;
const OPAL_MSG_SHUTDOWN: u64 = 0;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct opal_msg {
    pub params: [u64; 8],
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, u64, *mut core::ffi::c_void) -> i32>,
    pub next: *mut notifier_block,
    pub priority: i32,
}

/* Detect EPOW event */
unsafe fn detect_epow() -> bool {
    let mut epow: u16;
    let mut epow_classes: u16 = OPAL_SYSEPOW_MAX as u16;
    let mut opal_epow_status = [0u16; OPAL_SYSEPOW_MAX];

    /*
     * Check for EPOW event. Kernel sends supported EPOW classes info
     * to OPAL. OPAL returns EPOW info along with classes present.
     */
    let rc = opal_get_epow_status(opal_epow_status.as_mut_ptr(), &mut epow_classes);
    if rc != OPAL_SUCCESS {
        pr_err(b"Failed to get EPOW event information\0".as_ptr() as *const i8);
        return false;
    }

    /* Look for EPOW events present */
    for i in 0..(epow_classes as usize) {
        epow = opal_epow_status[i];

        /* Filter events which do not need shutdown. */
        if i as i32 == OPAL_SYSEPOW_POWER {
            epow &= !(OPAL_SYSPOWER_CHNG | OPAL_SYSPOWER_FAIL | OPAL_SYSPOWER_INCL);
        }
        if epow != 0 {
            return true;
        }
    }

    false
}

/* Check for existing EPOW, DPO events */
unsafe fn poweroff_pending() -> bool {
    let mut opal_dpo_timeout: u64 = 0;

    /* Check for DPO event */
    let rc = opal_get_dpo_status(&mut opal_dpo_timeout);
    if rc == OPAL_SUCCESS {
        pr_info(b"Existing DPO event detected.\n\0".as_ptr() as *const i8);
        return true;
    }

    /* Check for EPOW event */
    if detect_epow() {
        pr_info(b"Existing EPOW event detected.\n\0".as_ptr() as *const i8);
        return true;
    }

    false
}

/* OPAL power-control events notifier */
unsafe extern "C" fn opal_power_control_event(
    _nb: *mut notifier_block,
    msg_type: u64,
    msg: *mut core::ffi::c_void,
) -> i32 {
    match msg_type {
        OPAL_MSG_EPOW => {
            if detect_epow() {
                pr_info(b"EPOW msg received. Powering off system\n\0".as_ptr() as *const i8);
                orderly_poweroff(true);
            }
        }
        OPAL_MSG_DPO => {
            pr_info(b"DPO msg received. Powering off system\n\0".as_ptr() as *const i8);
            orderly_poweroff(true);
        }
        OPAL_MSG_SHUTDOWN => {
            let type_ = (*(msg as *mut opal_msg)).params[0];
            match type_ {
                SOFT_REBOOT => {
                    pr_info(b"Reboot requested\n\0".as_ptr() as *const i8);
                    orderly_reboot();
                }
                SOFT_OFF => {
                    pr_info(b"Poweroff requested\n\0".as_ptr() as *const i8);
                    orderly_poweroff(true);
                }
                _ => pr_err(b"Unknown power-control type %llu\n\0".as_ptr() as *const i8, type_),
            }
        }
        _ => pr_err(b"Unknown OPAL message type %lu\n\0".as_ptr() as *const i8, msg_type),
    }

    0
}

/* OPAL EPOW event notifier block */
static mut opal_epow_nb: notifier_block = notifier_block {
    notifier_call: Some(opal_power_control_event),
    next: core::ptr::null_mut(),
    priority: 0,
};

/* OPAL DPO event notifier block */
static mut opal_dpo_nb: notifier_block = notifier_block {
    notifier_call: Some(opal_power_control_event),
    next: core::ptr::null_mut(),
    priority: 0,
};

/* OPAL power-control event notifier block */
static mut opal_power_control_nb: notifier_block = notifier_block {
    notifier_call: Some(opal_power_control_event),
    next: core::ptr::null_mut(),
    priority: 0,
};

pub unsafe extern "C" fn opal_power_control_init() -> i32 {
    let mut supported = 0;
    let mut np: *mut device_node;

    /* Register OPAL power-control events notifier */
    let mut ret = opal_message_notifier_register(OPAL_MSG_SHUTDOWN, &raw mut opal_power_control_nb);
    if ret != 0 {
        pr_err(b"Failed to register SHUTDOWN notifier, ret = %d\n\0".as_ptr() as *const i8, ret);
    }

    /* Determine OPAL EPOW, DPO support */
    np = of_find_node_by_path(b"/ibm,opal/epow\0".as_ptr() as *const i8);
    if !np.is_null() {
        supported = of_device_is_compatible(np, b"ibm,opal-v3-epow\0".as_ptr() as *const i8);
        of_node_put(np);
    }

    if supported == 0 {
        return 0;
    }
    pr_info(b"OPAL EPOW, DPO support detected.\n\0".as_ptr() as *const i8);

    /* Register EPOW event notifier */
    ret = opal_message_notifier_register(OPAL_MSG_EPOW, &raw mut opal_epow_nb);
    if ret != 0 {
        pr_err(b"Failed to register EPOW notifier, ret = %d\n\0".as_ptr() as *const i8, ret);
    }

    /* Register DPO event notifier */
    ret = opal_message_notifier_register(OPAL_MSG_DPO, &raw mut opal_dpo_nb);
    if ret != 0 {
        pr_err(b"Failed to register DPO notifier, ret = %d\n\0".as_ptr() as *const i8, ret);
    }

    /* Check for any pending EPOW or DPO events. */
    if poweroff_pending() {
        orderly_poweroff(true);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
