/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Userspace ABI for Counter character devices
 * Copyright (C) 2020 William Breathitt Gray
 */

/* Types supplied by linux/types.h. */
pub type __u8 = u8;
pub type __aligned_u64 = u64;

/* Component type definitions */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum counter_component_type {
    COUNTER_COMPONENT_NONE,
    COUNTER_COMPONENT_SIGNAL,
    COUNTER_COMPONENT_COUNT,
    COUNTER_COMPONENT_FUNCTION,
    COUNTER_COMPONENT_SYNAPSE_ACTION,
    COUNTER_COMPONENT_EXTENSION,
}

/* Component scope definitions */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum counter_scope {
    COUNTER_SCOPE_DEVICE,
    COUNTER_SCOPE_SIGNAL,
    COUNTER_SCOPE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct counter_component {
    pub type_: __u8,
    pub scope: __u8,
    pub parent: __u8,
    pub id: __u8,
}

/* Event type definitions */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum counter_event_type {
    /* Count value increased past ceiling */
    COUNTER_EVENT_OVERFLOW,
    /* Count value decreased past floor */
    COUNTER_EVENT_UNDERFLOW,
    /* Count value increased past ceiling, or decreased past floor */
    COUNTER_EVENT_OVERFLOW_UNDERFLOW,
    /* Count value reached threshold */
    COUNTER_EVENT_THRESHOLD,
    /* Index signal detected */
    COUNTER_EVENT_INDEX,
    /* State of counter is changed */
    COUNTER_EVENT_CHANGE_OF_STATE,
    /* Count value captured */
    COUNTER_EVENT_CAPTURE,
    /* Direction change detected */
    COUNTER_EVENT_DIRECTION_CHANGE,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct counter_watch {
    pub component: counter_component,
    pub event: __u8,
    pub channel: __u8,
}

/* Queues a Counter watch for the specified event. */
/* _IOC encoding follows linux/ioctl.h; the size is sizeof(struct counter_watch). */
pub const COUNTER_ADD_WATCH_IOCTL: u32 = (1u32 << 30) | (6u32 << 16) | (0x3Eu32 << 8);
/* Enables monitoring the events specified by the Counter watches. */
pub const COUNTER_ENABLE_EVENTS_IOCTL: u32 = (0x3Eu32 << 8) | 0x01u32;
/* Stops monitoring the previously enabled events. */
pub const COUNTER_DISABLE_EVENTS_IOCTL: u32 = (0x3Eu32 << 8) | 0x02u32;

#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Default)]
pub struct counter_event {
    pub timestamp: __aligned_u64,
    pub value: __aligned_u64,
    pub watch: counter_watch,
    pub status: __u8,
}

/* Count direction values */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum counter_count_direction {
    COUNTER_COUNT_DIRECTION_FORWARD,
    COUNTER_COUNT_DIRECTION_BACKWARD,
}

/* Count mode values */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum counter_count_mode {
    COUNTER_COUNT_MODE_NORMAL,
    COUNTER_COUNT_MODE_RANGE_LIMIT,
    COUNTER_COUNT_MODE_NON_RECYCLE,
    COUNTER_COUNT_MODE_MODULO_N,
    COUNTER_COUNT_MODE_INTERRUPT_ON_TERMINAL_COUNT,
    COUNTER_COUNT_MODE_HARDWARE_RETRIGGERABLE_ONESHOT,
    COUNTER_COUNT_MODE_RATE_GENERATOR,
    COUNTER_COUNT_MODE_SQUARE_WAVE_MODE,
    COUNTER_COUNT_MODE_SOFTWARE_TRIGGERED_STROBE,
    COUNTER_COUNT_MODE_HARDWARE_TRIGGERED_STROBE,
}

/* Count function values */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum counter_function {
    COUNTER_FUNCTION_INCREASE,
    COUNTER_FUNCTION_DECREASE,
    COUNTER_FUNCTION_PULSE_DIRECTION,
    COUNTER_FUNCTION_QUADRATURE_X1_A,
    COUNTER_FUNCTION_QUADRATURE_X1_B,
    COUNTER_FUNCTION_QUADRATURE_X2_A,
    COUNTER_FUNCTION_QUADRATURE_X2_B,
    COUNTER_FUNCTION_QUADRATURE_X4,
}

/* Signal values */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum counter_signal_level {
    COUNTER_SIGNAL_LEVEL_LOW,
    COUNTER_SIGNAL_LEVEL_HIGH,
}

/* Action mode values */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum counter_synapse_action {
    COUNTER_SYNAPSE_ACTION_NONE,
    COUNTER_SYNAPSE_ACTION_RISING_EDGE,
    COUNTER_SYNAPSE_ACTION_FALLING_EDGE,
    COUNTER_SYNAPSE_ACTION_BOTH_EDGES,
}

/* Signal polarity values */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum counter_signal_polarity {
    COUNTER_SIGNAL_POLARITY_POSITIVE,
    COUNTER_SIGNAL_POLARITY_NEGATIVE,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
