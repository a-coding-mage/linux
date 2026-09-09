/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the extcon provider translation.

/**
 * struct extcon_dev - An extcon device represents one external connector.
 * @name:                The name of this extcon device. Parent device name is
 *                      used if NULL.
 * @supported_cable:    Array of supported cable names ending with EXTCON_NONE.
 *                      If supported_cable is NULL, cable name related APIs
 *                      are disabled.
 * @mutually_exclusive: Array of mutually exclusive set of cables that cannot
 *                      be attached simultaneously. The array should be
 *                      ending with 0 or be NULL (no mutually exclusive cables).
 *                      For example, if it is {0x7, 0x30, 0}, then,
 *                      {0, 1}, {0, 1, 2}, {0, 2}, {1, 2}, or {4, 5} cannot
 *                      be attached simulataneously. {0x7, 0} is equivalent to
 *                      {0x3, 0x6, 0x5, 0}. If it is {0xFFFFFFFF, 0}, there
 *                      can be no simultaneous connections.
 * @dev:                Device of this extcon.
 * @id:                 Unique device ID of this extcon.
 * @state:              Attach/detach state of this extcon. Do not provide at
 *                      register-time.
 * @nh_all:             Notifier for the state change events for all supported
 *                      external connectors from this extcon.
 * @nh:                 Notifier for the state change events from this extcon
 * @entry:              To support list of extcon devices so that users can
 *                      search for extcon devices based on the extcon name.
 * @lock:               Protects device state and serialises device registration
 * @max_supported:      Internal value to store the number of cables.
 * @extcon_dev_type:    Device_type struct to provide attribute_groups
 *                      customized for each extcon device.
 * @cables:             Sysfs subdirectories. Each represents one cable.
 *
 * In most cases, users only need to provide "User initializing data" of
 * this struct when registering an extcon. In some exceptional cases,
 * optional callbacks may be needed. However, the values in "internal data"
 * are overwritten by register function.
 */
#[repr(C)]
pub struct extcon_dev {
    /* Optional user initializing data */
    pub name: *const core::ffi::c_char,
    pub supported_cable: *const core::ffi::c_uint,
    pub mutually_exclusive: *const u32,

    /* Internal data. Please do not set. */
    pub dev: device,
    pub id: core::ffi::c_uint,
    pub nh_all: raw_notifier_head,
    pub nh: *mut raw_notifier_head,
    pub entry: list_head,
    pub max_supported: core::ffi::c_int,
    pub lock: spinlock_t, /* could be called by irq handler */
    pub state: u32,

    /* /sys/class/extcon/.../cable.n/... */
    pub extcon_dev_type: device_type,
    pub cables: *mut extcon_cable,

    /* /sys/class/extcon/.../mutually_exclusive/... */
    pub attr_g_muex: attribute_group,
    pub attrs_muex: *mut *mut attribute,
    pub d_attrs_muex: *mut device_attribute,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
