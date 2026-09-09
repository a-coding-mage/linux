/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of linux/gpio/driver.h. External kernel types and symbols
// are intentionally left as dependencies supplied by other translated files.

use core::ffi::c_void;

#[repr(C)]
pub union gpio_irq_fwspec {
    pub fwspec: irq_fwspec,
    #[cfg(feature = "CONFIG_GENERIC_MSI_IRQ")]
    pub msiinfo: msi_alloc_info_t,
}

#[repr(C)]
pub struct gpio_irq_chip {
    pub chip: *mut irq_chip,
    pub domain: *mut irq_domain,
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
    pub fwnode: *mut fwnode_handle,
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
    pub parent_domain: *mut irq_domain,
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
    pub child_to_parent_hwirq: Option<unsafe extern "C" fn(*mut gpio_chip, u32, u32, *mut u32, *mut u32) -> i32>,
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
    pub populate_parent_alloc_arg: Option<unsafe extern "C" fn(*mut gpio_chip, *mut gpio_irq_fwspec, u32, u32) -> i32>,
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
    pub child_offset_to_irq: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> u32>,
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
    pub child_irq_domain_ops: irq_domain_ops,
    pub handler: irq_flow_handler_t,
    pub default_type: u32,
    pub lock_key: *mut lock_class_key,
    pub request_key: *mut lock_class_key,
    pub parent_handler: irq_flow_handler_t,
    pub parent_handler_data: *mut c_void,
    pub num_parents: u32,
    pub parents: *mut u32,
    pub map: *mut u32,
    pub threaded: bool,
    pub per_parent_data: bool,
    pub initialized: bool,
    pub domain_is_allocated_externally: bool,
    pub init_hw: Option<unsafe extern "C" fn(*mut gpio_chip) -> i32>,
    pub init_valid_mask: Option<unsafe extern "C" fn(*mut gpio_chip, *mut c_ulong, u32)>,
    pub valid_mask: *mut c_ulong,
    pub first: u32,
    pub irq_enable: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_disable: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
}

#[repr(C)]
pub struct gpio_chip {
    pub label: *const i8,
    pub gpiodev: *mut gpio_device,
    pub parent: *mut device,
    pub fwnode: *mut fwnode_handle,
    pub owner: *mut module,
    pub request: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub free: Option<unsafe extern "C" fn(*mut gpio_chip, u32)>,
    pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub get_multiple: Option<unsafe extern "C" fn(*mut gpio_chip, *mut c_ulong, *mut c_ulong) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)>,
    pub set_multiple: Option<unsafe extern "C" fn(*mut gpio_chip, *mut c_ulong, *mut c_ulong)>,
    pub set_config: Option<unsafe extern "C" fn(*mut gpio_chip, u32, c_ulong) -> i32>,
    pub to_irq: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub dbg_show: Option<unsafe extern "C" fn(*mut seq_file, *mut gpio_chip)>,
    pub init_valid_mask: Option<unsafe extern "C" fn(*mut gpio_chip, *mut c_ulong, u32) -> i32>,
    pub add_pin_ranges: Option<unsafe extern "C" fn(*mut gpio_chip) -> i32>,
    pub en_hw_timestamp: Option<unsafe extern "C" fn(*mut gpio_chip, u32, c_ulong) -> i32>,
    pub dis_hw_timestamp: Option<unsafe extern "C" fn(*mut gpio_chip, u32, c_ulong) -> i32>,
    pub base: i32,
    pub ngpio: u16,
    pub offset: u16,
    pub names: *const *const i8,
    pub can_sleep: bool,
    #[cfg(feature = "CONFIG_GPIOLIB_IRQCHIP")]
    pub irq: gpio_irq_chip,
    #[cfg(feature = "CONFIG_OF_GPIO")]
    pub of_gpio_n_cells: u32,
    #[cfg(feature = "CONFIG_OF_GPIO")]
    pub of_node_instance_match: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> bool>,
    #[cfg(feature = "CONFIG_OF_GPIO")]
    pub of_xlate: Option<unsafe extern "C" fn(*mut gpio_chip, *const of_phandle_args, *mut u32) -> i32>,
}

#[repr(C)]
pub struct _gpiochip_for_each_data {
    pub label: *mut *const i8,
    pub i: *mut u32,
}

pub unsafe extern "C" {
    pub fn gpiochip_dup_line_label(gc: *mut gpio_chip, offset: u32) -> *mut i8;
    pub fn gpiochip_add_data_with_key(gc: *mut gpio_chip, data: *mut c_void, lock_key: *mut lock_class_key, request_key: *mut lock_class_key) -> i32;
    pub fn gpiochip_remove(gc: *mut gpio_chip);
    pub fn devm_gpiochip_add_data_with_key(dev: *mut device, gc: *mut gpio_chip, data: *mut c_void, lock_key: *mut lock_class_key, request_key: *mut lock_class_key) -> i32;
    pub fn gpio_device_find(data: *const c_void, r#match: Option<unsafe extern "C" fn(*mut gpio_chip, *const c_void) -> i32>) -> *mut gpio_device;
    pub fn gpio_device_get(gdev: *mut gpio_device) -> *mut gpio_device;
    pub fn gpio_device_put(gdev: *mut gpio_device);
    pub fn gpio_device_to_device(gdev: *mut gpio_device) -> *mut device;
    pub fn gpiochip_line_is_irq(gc: *mut gpio_chip, offset: u32) -> bool;
    pub fn gpiochip_reqres_irq(gc: *mut gpio_chip, offset: u32) -> i32;
    pub fn gpiochip_relres_irq(gc: *mut gpio_chip, offset: u32);
    pub fn gpiochip_disable_irq(gc: *mut gpio_chip, offset: u32);
    pub fn gpiochip_enable_irq(gc: *mut gpio_chip, offset: u32);
    pub fn gpiochip_irq_reqres(data: *mut irq_data) -> i32;
    pub fn gpiochip_irq_relres(data: *mut irq_data);
    pub fn gpiochip_line_is_open_drain(gc: *mut gpio_chip, offset: u32) -> bool;
    pub fn gpiochip_line_is_open_source(gc: *mut gpio_chip, offset: u32) -> bool;
    pub fn gpiochip_line_is_persistent(gc: *mut gpio_chip, offset: u32) -> bool;
    pub fn gpiochip_line_is_valid(gc: *const gpio_chip, offset: u32) -> bool;
    pub fn gpiochip_query_valid_mask(gc: *const gpio_chip) -> *const c_ulong;
    pub fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut c_void;
    pub fn gpiochip_generic_request(gc: *mut gpio_chip, offset: u32) -> i32;
    pub fn gpiochip_generic_free(gc: *mut gpio_chip, offset: u32);
    pub fn gpiochip_generic_config(gc: *mut gpio_chip, offset: u32, config: c_ulong) -> i32;
    pub fn gpiochip_request_own_desc(gc: *mut gpio_chip, hwnum: u32, label: *const i8, lflags: gpio_lookup_flags, dflags: gpiod_flags) -> *mut gpio_desc;
    pub fn gpiochip_free_own_desc(desc: *mut gpio_desc);
    pub fn gpio_device_get_desc(gdev: *mut gpio_device, hwnum: u32) -> *mut gpio_desc;
    pub fn gpio_device_get_chip(gdev: *mut gpio_device) -> *mut gpio_chip;
    pub fn gpiochip_add_pin_range_with_pins(gc: *mut gpio_chip, pinctl_name: *const i8, gpio_offset: u32, pin_offset: u32, pins: *const u32, npins: u32) -> i32;
    pub fn gpiochip_add_pingroup_range(gc: *mut gpio_chip, pctldev: *mut pinctrl_dev, gpio_offset: u32, pin_group: *const i8) -> i32;
    pub fn gpiochip_remove_pin_ranges(gc: *mut gpio_chip);
    pub fn gpiochip_irqchip_add_domain(gc: *mut gpio_chip, domain: *mut irq_domain) -> i32;
    pub fn gpiochip_lock_as_irq(gc: *mut gpio_chip, offset: u32) -> i32;
    pub fn gpiochip_unlock_as_irq(gc: *mut gpio_chip, offset: u32);
    pub fn gpiod_to_chip(desc: *const gpio_desc) -> *mut gpio_chip;
    pub fn gpiod_to_gpio_device(desc: *mut gpio_desc) -> *mut gpio_device;
    pub fn gpio_device_get_base(gdev: *mut gpio_device) -> i32;
    pub fn gpio_device_get_label(gdev: *mut gpio_device) -> *const i8;
    pub fn gpio_device_find_by_label(label: *const i8) -> *mut gpio_device;
    pub fn gpio_device_find_by_fwnode(fwnode: *const fwnode_handle) -> *mut gpio_device;
}

#[repr(C)]
pub struct gpio_pin_range {
    pub node: list_head,
    pub pctldev: *mut pinctrl_dev,
    pub range: pinctrl_gpio_range,
}

#[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
pub unsafe extern "C" fn gpiochip_populate_parent_fwspec_twocell(gc: *mut gpio_chip, gfwspec: *mut gpio_irq_fwspec, parent_hwirq: u32, parent_type: u32) -> i32;
#[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")]
pub unsafe extern "C" fn gpiochip_populate_parent_fwspec_fourcell(gc: *mut gpio_chip, gfwspec: *mut gpio_irq_fwspec, parent_hwirq: u32, parent_type: u32) -> i32;

#[cfg(feature = "CONFIG_GPIOLIB")]
pub unsafe extern "C" fn gpiochip_irqchip_add_domain(gc: *mut gpio_chip, domain: *mut irq_domain) -> i32;

// C macros preserved in intent; callers may provide equivalent Rust iteration:
// for_each_hwgpio_in_range, for_each_hwgpio, for_each_requested_gpio_in_range,
// for_each_requested_gpio, gpiochip_add_data, devm_gpiochip_add_data,
// GPIOCHIP_IRQ_RESOURCE_HELPERS, and for_each_gpiochip_node.

#[cfg(feature = "CONFIG_PINCTRL")]
pub unsafe fn gpiochip_add_pin_range(gc: *mut gpio_chip, pinctl_name: *const i8, gpio_offset: u32, pin_offset: u32, npins: u32) -> i32 {
    gpiochip_add_pin_range_with_pins(gc, pinctl_name, gpio_offset, pin_offset, core::ptr::null(), npins)
}

#[cfg(feature = "CONFIG_PINCTRL")]
pub unsafe fn gpiochip_add_sparse_pin_range(gc: *mut gpio_chip, pinctl_name: *const i8, gpio_offset: u32, pins: *const u32, npins: u32) -> i32 {
    gpiochip_add_pin_range_with_pins(gc, pinctl_name, gpio_offset, 0, pins, npins)
}

#[cfg(feature = "CONFIG_GPIOLIB")]
pub unsafe extern "C" fn gpiochip_node_count(_dev: *mut device) -> u32;
#[cfg(feature = "CONFIG_GPIOLIB")]
pub unsafe extern "C" fn gpiochip_node_get_first(_dev: *mut device) -> *mut fwnode_handle;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
