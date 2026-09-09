/* SPDX-License-Identifier: GPL-2.0 */
/* Internal GPIO functions. */

// Dependencies supplied by the surrounding kernel translation unit are intentionally
// not implemented here.

pub const GPIOCHIP_NAME: &str = "gpiochip";

#[repr(C)]
pub struct fwnode_handle;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct cdev;
#[repr(C)]
pub struct module;
#[repr(C)]
pub struct gpio_chip;
#[repr(C)]
pub struct gpio_desc;
#[repr(C)]
pub struct srcu_struct;
#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct raw_notifier_head;
#[repr(C)]
pub struct rwlock_t;
#[repr(C)]
pub struct workqueue_struct;
#[repr(C)]
pub struct blocking_notifier_head;
#[repr(C)]
pub struct rcu_head;
#[repr(C)]
pub struct device_node;

#[repr(C)]
pub struct gpio_device {
    pub dev: device,
    pub chrdev: cdev,
    pub id: core::ffi::c_int,
    pub owner: *mut module,
    pub chip: *mut gpio_chip,
    pub descs: *mut gpio_desc,
    pub valid_mask: *mut libc::c_ulong,
    pub desc_srcu: srcu_struct,
    pub base: libc::c_uint,
    pub ngpio: u16,
    pub can_sleep: bool,
    pub label: *const core::ffi::c_char,
    pub data: *mut core::ffi::c_void,
    pub list: list_head,
    pub line_state_notifier: raw_notifier_head,
    pub line_state_lock: rwlock_t,
    pub line_state_wq: *mut workqueue_struct,
    pub device_notifier: blocking_notifier_head,
    pub srcu: srcu_struct,
    // Present only when CONFIG_PINCTRL is enabled.
    #[cfg(CONFIG_PINCTRL)]
    pub pin_ranges: list_head,
}

// Equivalent to container_of(dev, struct gpio_device, dev).
#[inline]
pub unsafe fn to_gpio_device(dev: *mut device) -> *mut gpio_device {
    (dev as *mut u8).sub(core::mem::offset_of!(gpio_device, dev)) as *mut gpio_device
}

// GPIO suffixes used for ACPI and device-tree lookup.
pub static mut gpio_suffixes: *const *const core::ffi::c_char = core::ptr::null();

// C macro for_each_gpio_property_name(propname, con_id): iterate gpio_suffixes,
// formatting each suffix as either "<con_id>-<suffix>" or the suffix itself.

#[repr(C)]
pub struct gpio_array {
    pub desc: *mut *mut gpio_desc,
    pub size: libc::c_uint,
    pub gdev: *mut gpio_device,
    pub get_mask: *mut libc::c_ulong,
    pub set_mask: *mut libc::c_ulong,
    pub invert_mask: [libc::c_ulong; 0],
}

// C macros for_each_gpio_desc and for_each_gpio_desc_with_flag retain their
// iteration semantics through gpiochip_get_desc and test_bit.

extern "C" {
    pub fn gpiod_get_array_value_complex(
        raw: bool, can_sleep: bool, array_size: libc::c_uint,
        desc_array: *mut *mut gpio_desc, array_info: *mut gpio_array,
        value_bitmap: *mut libc::c_ulong,
    ) -> core::ffi::c_int;
    pub fn gpiod_set_array_value_complex(
        raw: bool, can_sleep: bool, array_size: libc::c_uint,
        desc_array: *mut *mut gpio_desc, array_info: *mut gpio_array,
        value_bitmap: *mut libc::c_ulong,
    ) -> core::ffi::c_int;
    pub fn gpiod_set_transitory(desc: *mut gpio_desc, transitory: bool) -> core::ffi::c_int;
    pub fn gpiod_line_state_notify(desc: *mut gpio_desc, action: libc::c_ulong);
    pub fn gpiod_direction_output_nonotify(desc: *mut gpio_desc, value: core::ffi::c_int) -> core::ffi::c_int;
    pub fn gpiod_direction_input_nonotify(desc: *mut gpio_desc) -> core::ffi::c_int;
}

#[repr(C)]
pub struct gpio_desc_label {
    pub rh: rcu_head,
    pub str_: [core::ffi::c_char; 0],
}

#[repr(C)]
pub struct gpio_desc {
    pub gdev: *mut gpio_device,
    pub flags: libc::c_ulong,
    pub label: *mut gpio_desc_label,
    pub name: *const core::ffi::c_char,
    #[cfg(CONFIG_OF_DYNAMIC)]
    pub hog: *mut device_node,
    #[cfg(CONFIG_GPIO_CDEV)]
    pub debounce_period_us: libc::c_uint,
}

pub const GPIOD_FLAG_REQUESTED: u32 = 0;
pub const GPIOD_FLAG_IS_OUT: u32 = 1;
pub const GPIOD_FLAG_EXPORT: u32 = 2;
pub const GPIOD_FLAG_SYSFS: u32 = 3;
pub const GPIOD_FLAG_ACTIVE_LOW: u32 = 6;
pub const GPIOD_FLAG_OPEN_DRAIN: u32 = 7;
pub const GPIOD_FLAG_OPEN_SOURCE: u32 = 8;
pub const GPIOD_FLAG_USED_AS_IRQ: u32 = 9;
pub const GPIOD_FLAG_IRQ_IS_ENABLED: u32 = 10;
pub const GPIOD_FLAG_IS_HOGGED: u32 = 11;
pub const GPIOD_FLAG_TRANSITORY: u32 = 12;
pub const GPIOD_FLAG_PULL_UP: u32 = 13;
pub const GPIOD_FLAG_PULL_DOWN: u32 = 14;
pub const GPIOD_FLAG_BIAS_DISABLE: u32 = 15;
pub const GPIOD_FLAG_EDGE_RISING: u32 = 16;
pub const GPIOD_FLAG_EDGE_FALLING: u32 = 17;
pub const GPIOD_FLAG_EVENT_CLOCK_REALTIME: u32 = 18;
pub const GPIOD_FLAG_EVENT_CLOCK_HTE: u32 = 19;
pub const GPIOD_FLAG_SHARED: u32 = 20;
pub const GPIOD_FLAG_SHARED_PROXY: u32 = 21;

// gpiod_not_found(desc): IS_ERR(desc) && PTR_ERR(desc) == -ENOENT.

#[repr(C)]
pub struct gpio_chip_guard {
    pub gdev: *mut gpio_device,
    pub gc: *mut gpio_chip,
    pub idx: core::ffi::c_int,
}

// DEFINE_CLASS(gpio_chip_guard, ...): scoped SRCU read locking of gdev->chip;
// cleanup calls srcu_read_unlock and construction performs srcu_read_lock and
// srcu_dereference. The source declaration takes const struct gpio_desc *desc.

extern "C" {
    pub fn gpiod_request(desc: *mut gpio_desc, label: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn gpiod_request_commit(desc: *mut gpio_desc, label: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn gpiod_free(desc: *mut gpio_desc);
    pub fn gpiod_free_commit(desc: *mut gpio_desc);
}

#[inline]
pub unsafe fn gpiod_request_user(desc: *mut gpio_desc, label: *const core::ffi::c_char) -> core::ffi::c_int {
    let mut ret = gpiod_request(desc, label);
    // -EPROBE_DEFER is converted to -ENODEV.
    if ret == -517 { ret = -19; }
    ret
}

#[repr(C)]
pub enum gpiod_flags {}

extern "C" {
    pub fn gpiod_find_and_request(consumer: *mut device, fwnode: *mut fwnode_handle,
        con_id: *const core::ffi::c_char, idx: libc::c_uint, flags: gpiod_flags,
        label: *const core::ffi::c_char, platform_lookup_allowed: bool) -> *mut gpio_desc;
    pub fn gpio_do_set_config(desc: *mut gpio_desc, config: libc::c_ulong) -> core::ffi::c_int;
    pub fn gpiod_configure_flags(desc: *mut gpio_desc, con_id: *const core::ffi::c_char,
        lflags: libc::c_ulong, dflags: gpiod_flags) -> core::ffi::c_int;
    pub fn gpio_set_debounce_timeout(desc: *mut gpio_desc, debounce: libc::c_uint) -> core::ffi::c_int;
    pub fn gpiod_hog(desc: *mut gpio_desc, name: *const core::ffi::c_char,
        lflags: libc::c_ulong, dflags: gpiod_flags) -> core::ffi::c_int;
    pub fn gpiochip_add_hog(gc: *mut gpio_chip, fwnode: *mut fwnode_handle) -> core::ffi::c_int;
    pub fn gpiochip_get_ngpios(gc: *mut gpio_chip, dev: *mut device) -> core::ffi::c_int;
    pub fn gpiochip_get_desc(gc: *mut gpio_chip, hwnum: libc::c_uint) -> *mut gpio_desc;
    pub fn gpiod_get_label(desc: *mut gpio_desc) -> *const core::ffi::c_char;
}

// Descriptor/chip-prefixed logging macros (__gpiod_pr, gpiod_{err,warn,dbg},
// __gpiochip_pr, gpiochip_{err,warn,info,dbg}) are preserved as source-level
// macro intent; they use scoped SRCU protection and the kernel logging APIs.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
