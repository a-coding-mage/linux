/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2011-2016 Synaptics Incorporated
 * Copyright (c) 2011 Unixphere
 */

// Linux kernel dependencies from rmi.h are supplied by other translation units.

pub const NAME_BUFFER_SIZE: usize = 256;

#[repr(C)]
pub struct rmi_2d_axis_alignment {
    pub swap_axes: bool,
    pub flip_x: bool,
    pub flip_y: bool,
    pub clip_x_low: u16,
    pub clip_y_low: u16,
    pub clip_x_high: u16,
    pub clip_y_high: u16,
    pub offset_x: u16,
    pub offset_y: u16,
    pub delta_x_threshold: u8,
    pub delta_y_threshold: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rmi_sensor_type {
    rmi_sensor_default = 0,
    rmi_sensor_touchscreen,
    rmi_sensor_touchpad,
}

pub const RMI_F11_DISABLE_ABS_REPORT: u32 = 1 << 0;

#[repr(C)]
pub struct rmi_2d_sensor_platform_data {
    pub axis_align: rmi_2d_axis_alignment,
    pub sensor_type: rmi_sensor_type,
    pub x_mm: core::ffi::c_int,
    pub y_mm: core::ffi::c_int,
    pub disable_report_mask: core::ffi::c_int,
    pub rezero_wait: u16,
    pub topbuttonpad: bool,
    pub kernel_tracking: bool,
    pub dmax: core::ffi::c_int,
    pub dribble: core::ffi::c_int,
    pub palm_detect: core::ffi::c_int,
}

#[repr(C)]
pub struct rmi_gpio_data {
    pub buttonpad: bool,
    pub trackstick_buttons: bool,
    pub disable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rmi_reg_state {
    RMI_REG_STATE_DEFAULT = 0,
    RMI_REG_STATE_OFF = 1,
    RMI_REG_STATE_ON = 2,
}

#[repr(C)]
pub struct rmi_f01_power_management {
    pub nosleep: rmi_reg_state,
    pub wakeup_threshold: u8,
    pub doze_holdoff: u8,
    pub doze_interval: u8,
}

#[repr(C)]
pub struct rmi_device_platform_data_spi {
    pub block_delay_us: u32,
    pub split_read_block_delay_us: u32,
    pub read_delay_us: u32,
    pub write_delay_us: u32,
    pub split_read_byte_delay_us: u32,
    pub pre_delay_us: u32,
    pub post_delay_us: u32,
    pub bits_per_word: u8,
    pub mode: u16,
    pub cs_assert_data: *mut core::ffi::c_void,
    pub cs_assert: Option<unsafe extern "C" fn(*const core::ffi::c_void, bool) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct rmi_device_platform_data {
    pub reset_delay_ms: core::ffi::c_int,
    pub irq: core::ffi::c_int,
    pub spi_data: rmi_device_platform_data_spi,
    pub sensor_pdata: rmi_2d_sensor_platform_data,
    pub power_management: rmi_f01_power_management,
    pub gpio_data: rmi_gpio_data,
}

#[repr(C)]
pub struct rmi_function_descriptor {
    pub query_base_addr: u16,
    pub command_base_addr: u16,
    pub control_base_addr: u16,
    pub data_base_addr: u16,
    pub interrupt_source_count: u8,
    pub function_number: u8,
    pub function_version: u8,
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct device_driver;
#[repr(C)]
pub struct input_dev;
#[repr(C)]
pub struct irq_domain;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct rmi_function;

#[repr(C)]
pub struct rmi_transport_ops {
    pub write_block: Option<unsafe extern "C" fn(*mut rmi_transport_dev, u16, *const core::ffi::c_void, usize) -> core::ffi::c_int>,
    pub read_block: Option<unsafe extern "C" fn(*mut rmi_transport_dev, u16, *mut core::ffi::c_void, usize) -> core::ffi::c_int>,
    pub reset: Option<unsafe extern "C" fn(*mut rmi_transport_dev, u16) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct rmi_transport_dev {
    pub dev: *mut device,
    pub rmi_dev: *mut rmi_device,
    pub proto_name: *const core::ffi::c_char,
    pub ops: *const rmi_transport_ops,
    pub pdata: rmi_device_platform_data,
    pub input: *mut input_dev,
}

#[repr(C)]
pub struct rmi_driver {
    pub driver: device_driver,
    pub reset_handler: Option<unsafe extern "C" fn(*mut rmi_device) -> core::ffi::c_int>,
    pub clear_irq_bits: Option<unsafe extern "C" fn(*mut rmi_device, *mut core::ffi::c_ulong) -> core::ffi::c_int>,
    pub set_irq_bits: Option<unsafe extern "C" fn(*mut rmi_device, *mut core::ffi::c_ulong) -> core::ffi::c_int>,
    pub store_productid: Option<unsafe extern "C" fn(*mut rmi_device) -> core::ffi::c_int>,
    pub set_input_params: Option<unsafe extern "C" fn(*mut rmi_device, *mut input_dev) -> core::ffi::c_int>,
    pub data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct rmi_device {
    pub dev: device,
    pub number: core::ffi::c_int,
    pub driver: *mut rmi_driver,
    pub xport: *mut rmi_transport_dev,
}

#[repr(C)]
pub struct rmi4_attn_data {
    pub irq_status: core::ffi::c_ulong,
    pub size: usize,
    pub data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct rmi_driver_data {
    pub function_list: list_head,
    pub rmi_dev: *mut rmi_device,
    pub f01_container: *mut rmi_function,
    pub f34_container: *mut rmi_function,
    pub bootloader_mode: bool,
    pub num_of_irq_regs: core::ffi::c_int,
    pub irq_count: core::ffi::c_int,
    pub irq_memory: *mut core::ffi::c_void,
    pub irq_status: *mut core::ffi::c_ulong,
    pub fn_irq_bits: *mut core::ffi::c_ulong,
    pub current_irq_mask: *mut core::ffi::c_ulong,
    pub new_irq_mask: *mut core::ffi::c_ulong,
    pub irq_mutex: mutex,
    pub input: *mut input_dev,
    pub irqdomain: *mut irq_domain,
    pub pdt_props: u8,
    pub num_rx_electrodes: u8,
    pub num_tx_electrodes: u8,
    pub enabled: bool,
    pub enabled_mutex: mutex,
    pub attn_data: rmi4_attn_data,
    // DECLARE_KFIFO(attn_fifo, struct rmi4_attn_data, 16)
    pub attn_fifo: [rmi4_attn_data; 16],
}

unsafe extern "C" {
    pub fn rmi_register_transport_device(xport: *mut rmi_transport_dev) -> core::ffi::c_int;
    pub fn rmi_unregister_transport_device(xport: *mut rmi_transport_dev);
    pub fn rmi_set_attn_data(rmi_dev: *mut rmi_device, irq_status: core::ffi::c_ulong, data: *mut core::ffi::c_void, size: usize);
    pub fn rmi_driver_suspend(rmi_dev: *mut rmi_device, enable_wake: bool) -> core::ffi::c_int;
    pub fn rmi_driver_resume(rmi_dev: *mut rmi_device, clear_wake: bool) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
