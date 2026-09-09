/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header defines this alias only when __KERNEL__ is set.  Rust has no
 * direct preprocessor equivalent here; usize preserves the platform-sized
 * unsigned-long intent used by kernel_ulong_t.
 */
pub type kernel_ulong_t = usize;

/*
 * DFL (Device Feature List)
 *
 * DFL defines a linked list of feature headers within the device MMIO space to
 * provide an extensible way of adding features. Software can walk through these
 * predefined data structures to enumerate features. It is now used in the FPGA.
 * See Documentation/fpga/dfl.rst for more information.
 *
 * The dfl bus type is introduced to match the individual feature devices (dfl
 * devices) for specific dfl drivers.
 */

/**
 * struct dfl_device_id -  dfl device identifier
 * @type: DFL FIU type of the device. See enum dfl_id_type.
 * @feature_id: feature identifier local to its DFL FIU type.
 * @driver_data: driver specific data.
 */
#[repr(C)]
pub struct dfl_device_id {
    pub r#type: u16,
    pub feature_id: u16,
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
