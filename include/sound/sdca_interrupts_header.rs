/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 *
 * Copyright (C) 2025 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

// C dependencies supplied by other translation units:
// linux/interrupt.h, linux/mutex.h, and linux/regmap.h

// struct device;
// struct snd_soc_component;
// struct sdca_function_data;

pub const SDCA_MAX_INTERRUPTS: usize = 31; // the last bit is reserved for future extensions

/**
 * struct sdca_interrupt - contains information about a single SDCA interrupt
 * @name: The name of the interrupt.
 * @dev: Pointer to the Function device.
 * @device_regmap: Pointer to the IRQ regmap.
 * @function_regmap: Pointer to the SDCA Function regmap.
 * @component: Pointer to the ASoC component owns the interrupt.
 * @function: Pointer to the Function that the interrupt is associated with.
 * @entity: Pointer to the Entity that the interrupt is associated with.
 * @control: Pointer to the Control that the interrupt is associated with.
 * @handler: Handler function to be called for the IRQ.
 * @priv: Pointer to private data for use by the handler.
 * @free_priv: Pointer to a function that can be used to free the priv data.
 * @irq: IRQ number allocated to this interrupt, also used internally to track
 * the IRQ being assigned.
 * @early_request: Flag to indicate this IRQ was requested at bus probe time.
 */
#[repr(C)]
pub struct sdca_interrupt {
    pub name: *const ::core::ffi::c_char,

    pub dev: *mut device,
    pub device_regmap: *mut regmap,
    pub function_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub function: *mut sdca_function_data,
    pub entity: *mut sdca_entity,
    pub control: *mut sdca_control,
    pub handler: irq_handler_t,

    pub priv_: *mut ::core::ffi::c_void,
    pub free_priv: Option<unsafe extern "C" fn(interrupt: *mut sdca_interrupt)>,

    pub irq: ::core::ffi::c_int,
    pub early_request: bool,
}

/**
 * struct sdca_interrupt_info - contains top-level SDCA interrupt information
 * @irq_chip: regmap irq chip structure.
 * @irq_data: regmap irq chip data structure.
 * @irqs: Array of data for each individual IRQ.
 * @irq_lock: Protects access to the list of sdca_interrupt structures.
 */
#[repr(C)]
pub struct sdca_interrupt_info {
    pub irq_chip: regmap_irq_chip,
    pub irq_data: *mut regmap_irq_chip_data,

    pub irqs: [sdca_interrupt; SDCA_MAX_INTERRUPTS],

    pub irq_lock: mutex, // Protect irqs list across functions
}

extern "C" {
    pub fn sdca_irq_request(dev: *mut device, interrupt_info: *mut sdca_interrupt_info,
                            sdca_irq: ::core::ffi::c_int, name: *const ::core::ffi::c_char,
                            handler: irq_handler_t, data: *mut ::core::ffi::c_void)
        -> ::core::ffi::c_int;
    pub fn sdca_irq_free(dev: *mut device, interrupt_info: *mut sdca_interrupt_info,
                         sdca_irq: ::core::ffi::c_int, name: *const ::core::ffi::c_char,
                         data: *mut ::core::ffi::c_void);
    pub fn sdca_irq_data_populate(dev: *mut device, function_regmap: *mut regmap,
                                  component: *mut snd_soc_component,
                                  function: *mut sdca_function_data,
                                  entity: *mut sdca_entity, control: *mut sdca_control,
                                  interrupt: *mut sdca_interrupt) -> ::core::ffi::c_int;
    pub fn sdca_irq_populate_early(dev: *mut device, function_regmap: *mut regmap,
                                   function: *mut sdca_function_data,
                                   info: *mut sdca_interrupt_info) -> ::core::ffi::c_int;
    pub fn sdca_irq_populate(function: *mut sdca_function_data,
                             component: *mut snd_soc_component,
                             info: *mut sdca_interrupt_info) -> ::core::ffi::c_int;
    pub fn sdca_irq_cleanup(dev: *mut device, function: *mut sdca_function_data,
                            info: *mut sdca_interrupt_info);
    pub fn sdca_irq_cleanup_late(dev: *mut device, function: *mut sdca_function_data,
                                 info: *mut sdca_interrupt_info);

    pub fn devm_sdca_irq_allocate(dev: *mut device, regmap: *mut regmap,
                                  irq: ::core::ffi::c_int) -> *mut sdca_interrupt_info;

    pub fn sdca_irq_enable_early(function: *mut sdca_function_data,
                                 info: *mut sdca_interrupt_info);
    pub fn sdca_irq_enable(function: *mut sdca_function_data,
                           info: *mut sdca_interrupt_info);
    pub fn sdca_irq_disable(function: *mut sdca_function_data,
                            info: *mut sdca_interrupt_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
