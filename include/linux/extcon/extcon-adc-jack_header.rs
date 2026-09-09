/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/linux/extcon/extcon-adc-jack.h
 *
 * Analog Jack extcon driver with ADC-based detection capability.
 *
 * Copyright (C) 2012 Samsung Electronics
 * MyungJoo Ham <myungjoo.ham@samsung.com>
 */

// Dependency intent from the original header:
// #include <linux/module.h>
// #include <linux/extcon.h>

/**
 * struct adc_jack_cond - condition to use an extcon state
 *			denotes the last adc_jack_cond element among the array)
 * @id:			the unique id of each external connector
 * @min_adc:		min adc value for this condition
 * @max_adc:		max adc value for this condition
 *
 * For example, if { .state = 0x3, .min_adc = 100, .max_adc = 200}, it means
 * that if ADC value is between (inclusive) 100 and 200, than the cable 0 and
 * 1 are attached (1<<0 | 1<<1 == 0x3)
 *
 * Note that you don't need to describe condition for "no cable attached"
 * because when no adc_jack_cond is met, state = 0 is automatically chosen.
 */
#[repr(C)]
pub struct adc_jack_cond {
    pub id: u32,
    pub min_adc: u32,
    pub max_adc: u32,
}

/**
 * struct adc_jack_pdata - platform data for adc jack device.
 * @name:		name of the extcon device. If null, "adc-jack" is used.
 * @consumer_channel:	Unique name to identify the channel on the consumer
 *			side. This typically describes the channels used within
 *			the consumer. E.g. 'battery_voltage'
 * @cable_names:	array of extcon id for supported cables.
 * @adc_contitions:	array of struct adc_jack_cond conditions ending
 *			with .state = 0 entry. This describes how to decode
 *			adc values into extcon state.
 * @irq_flags:		irq flags used for the @irq
 * @handling_delay_ms:	in some devices, we need to read ADC value some
 *			milli-seconds after the interrupt occurs. You may
 *			describe such delays with @handling_delay_ms, which is
 *			rounded-off by jiffies.
 * @wakeup_source:	flag to wake up the system for extcon events.
 */
#[repr(C)]
pub struct adc_jack_pdata {
    pub name: *const core::ffi::c_char,
    pub consumer_channel: *const core::ffi::c_char,
    pub cable_names: *const u32,
    /* The last entry's state should be 0 */
    pub adc_conditions: *mut adc_jack_cond,
    pub irq_flags: usize,
    pub handling_delay_ms: usize, /* in ms */
    pub wakeup_source: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
