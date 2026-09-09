/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 1999-2002 Vojtech Pavlik
 */

/* Dependencies supplied by the surrounding kernel translation. */

extern "C" {
    pub static serio_bus: bus_type;
}

#[repr(C)]
pub struct serio {
    pub port_data: *mut core::ffi::c_void,

    pub name: [core::ffi::c_char; 32],
    pub phys: [core::ffi::c_char; 32],
    pub firmware_id: [core::ffi::c_char; 128],

    pub manual_bind: bool,

    pub id: serio_device_id,

    /* Protects critical sections from port's interrupt handler */
    pub lock: spinlock_t,

    pub write: Option<unsafe extern "C" fn(*mut serio, u8) -> core::ffi::c_int>,
    pub open: Option<unsafe extern "C" fn(*mut serio) -> core::ffi::c_int>,
    pub close: Option<unsafe extern "C" fn(*mut serio)>,
    pub start: Option<unsafe extern "C" fn(*mut serio) -> core::ffi::c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut serio)>,

    pub parent: *mut serio,
    /* Entry in parent->children list */
    pub child_node: list_head,
    pub children: list_head,
    /* Level of nesting in serio hierarchy */
    pub depth: u32,

    /*
     * serio->drv is accessed from interrupt handlers; when modifying
     * caller should acquire serio->drv_mutex and serio->lock.
     */
    pub drv: *mut serio_driver,
    /* Protects serio->drv so attributes can pin current driver */
    pub drv_mutex: mutex,

    pub dev: device,

    pub node: list_head,

    /*
     * For use by PS/2 layer when several ports share hardware and
     * may get indigestion when exposed to concurrent access (i8042).
     */
    pub ps2_cmd_mutex: *mut mutex,
}

/* C: container_of(d, struct serio, dev) */

#[repr(C)]
pub struct serio_driver {
    pub description: *const core::ffi::c_char,

    pub id_table: *const serio_device_id,
    pub manual_bind: bool,

    pub write_wakeup: Option<unsafe extern "C" fn(*mut serio)>,
    pub interrupt: Option<unsafe extern "C" fn(*mut serio, u8, u32) -> irqreturn_t>,
    pub connect: Option<unsafe extern "C" fn(*mut serio, *mut serio_driver) -> core::ffi::c_int>,
    pub reconnect: Option<unsafe extern "C" fn(*mut serio) -> core::ffi::c_int>,
    pub fast_reconnect: Option<unsafe extern "C" fn(*mut serio) -> core::ffi::c_int>,
    pub disconnect: Option<unsafe extern "C" fn(*mut serio)>,
    pub cleanup: Option<unsafe extern "C" fn(*mut serio)>,

    pub driver: device_driver,
}

/* C: container_of_const(d, struct serio_driver, driver) */

extern "C" {
    pub fn serio_open(serio: *mut serio, drv: *mut serio_driver) -> core::ffi::c_int;
    pub fn serio_close(serio: *mut serio);
    pub fn serio_rescan(serio: *mut serio);
    pub fn serio_reconnect(serio: *mut serio);
    pub fn serio_interrupt(serio: *mut serio, data: u8, flags: u32) -> irqreturn_t;

    pub fn __serio_register_port(serio: *mut serio, owner: *mut module);
    pub fn serio_unregister_port(serio: *mut serio);
    pub fn serio_unregister_child_port(serio: *mut serio);

    pub fn __serio_register_driver(
        drv: *mut serio_driver,
        owner: *mut module,
        mod_name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn serio_unregister_driver(drv: *mut serio_driver);
}

/*
 * C macros serio_register_port, serio_register_driver, and module_serio_driver
 * forward to the declarations above with build/module-specific arguments
 * (THIS_MODULE, KBUILD_MODNAME, and module_driver).
 */

#[inline]
pub unsafe fn serio_write(serio: *mut serio, data: u8) -> core::ffi::c_int {
    match (*serio).write {
        Some(write) => write(serio, data),
        None => -1,
    }
}

#[inline]
pub unsafe fn serio_drv_write_wakeup(serio: *mut serio) {
    if !(*serio).drv.is_null() {
        if let Some(write_wakeup) = (*(*serio).drv).write_wakeup {
            write_wakeup(serio);
        }
    }
}

/*
 * Use the following functions to manipulate serio's per-port
 * driver-specific data.
 */
#[inline]
pub unsafe fn serio_get_drvdata(serio: *mut serio) -> *mut core::ffi::c_void {
    dev_get_drvdata(&mut (*serio).dev)
}

#[inline]
pub unsafe fn serio_set_drvdata(serio: *mut serio, data: *mut core::ffi::c_void) {
    dev_set_drvdata(&mut (*serio).dev, data);
}

/*
 * Use the following functions to protect critical sections in
 * driver code from port's interrupt handler
 */
#[inline]
pub unsafe fn serio_pause_rx(serio: *mut serio) {
    spin_lock_irq(&mut (*serio).lock);
}

#[inline]
pub unsafe fn serio_continue_rx(serio: *mut serio) {
    spin_unlock_irq(&mut (*serio).lock);
}

/* C DEFINE_GUARD(serio_pause_rx, struct serio *, ...): cleanup guard intent. */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
