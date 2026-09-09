/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the corresponding kernel and driver modules are
// intentionally referenced here rather than reimplemented.

/**
 * struct zl3073x_dpll - ZL3073x DPLL sub-device structure
 * @list: this DPLL list entry
 * @dev: pointer to multi-function parent device
 * @id: DPLL index
 * @check_count: periodic check counter
 * @phase_monitor: is phase offset monitor enabled
 * @ops: DPLL device operations for this instance
 * @dpll_dev: pointer to registered DPLL device
 * @tracker: tracking object for the acquired reference
 * @lock: per-DPLL mutex serializing all operations
 * @type: DPLL type (PPS or EEC)
 * @lock_status: last saved DPLL lock status
 * @pins: list of pins
 * @ptp_info: PTP clock info
 * @ptp_clock: registered PTP clock (or NULL)
 */
#[repr(C)]
pub struct zl3073x_dpll {
    pub list: list_head,
    pub dev: *mut zl3073x_dev,
    pub id: u8,
    pub check_count: u8,
    pub phase_monitor: bool,
    pub ops: dpll_device_ops,
    pub dpll_dev: *mut dpll_device,
    pub tracker: dpll_tracker,
    pub lock: mutex,
    pub r#type: dpll_type,
    pub lock_status: dpll_lock_status,
    pub pins: list_head,
    pub ptp_info: ptp_clock_info,
    pub ptp_clock: *mut ptp_clock,
}

extern "C" {
    pub fn zl3073x_dpll_alloc(zldev: *mut zl3073x_dev, ch: u8) -> *mut zl3073x_dpll;
    pub fn zl3073x_dpll_free(zldpll: *mut zl3073x_dpll);

    pub fn zl3073x_dpll_register(zldpll: *mut zl3073x_dpll) -> i32;
    pub fn zl3073x_dpll_unregister(zldpll: *mut zl3073x_dpll);

    pub fn zl3073x_dpll_init_fine_phase_adjust(zldev: *mut zl3073x_dev) -> i32;
    pub fn zl3073x_dpll_changes_check(zldpll: *mut zl3073x_dpll);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
