/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. The Linux timer, interrupt, workqueue, and
// platform-device types are supplied by external dependencies.

#[repr(C)]
pub struct push_switch {
    /* switch state */
    // C bit-field: unsigned int state:1;
    pub state: u32,
    /* debounce timer */
    pub debounce: timer_list,
    /* workqueue */
    pub work: work_struct,
    /* platform device, for workqueue handler */
    pub pdev: *mut platform_device,
}

#[repr(C)]
pub struct push_switch_platform_info {
    /* IRQ handler */
    pub irq_handler: Option<unsafe extern "C" fn(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t>,
    /* Special IRQ flags */
    pub irq_flags: u32,
    /* Bit location of switch */
    pub bit: u32,
    /* Symbolic switch name */
    pub name: *const core::ffi::c_char,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
