/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Header file for NI general purpose counter support code (ni_tio.c)
 *
 * COMEDI - Linux Control and Measurement Device Interface
 */

// Dependency: <linux/comedi/comedidev.h>

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ni_gpct_register {
    NITIO_G0_AUTO_INC,
    NITIO_G1_AUTO_INC,
    NITIO_G2_AUTO_INC,
    NITIO_G3_AUTO_INC,
    NITIO_G0_CMD,
    NITIO_G1_CMD,
    NITIO_G2_CMD,
    NITIO_G3_CMD,
    NITIO_G0_HW_SAVE,
    NITIO_G1_HW_SAVE,
    NITIO_G2_HW_SAVE,
    NITIO_G3_HW_SAVE,
    NITIO_G0_SW_SAVE,
    NITIO_G1_SW_SAVE,
    NITIO_G2_SW_SAVE,
    NITIO_G3_SW_SAVE,
    NITIO_G0_MODE,
    NITIO_G1_MODE,
    NITIO_G2_MODE,
    NITIO_G3_MODE,
    NITIO_G0_LOADA,
    NITIO_G1_LOADA,
    NITIO_G2_LOADA,
    NITIO_G3_LOADA,
    NITIO_G0_LOADB,
    NITIO_G1_LOADB,
    NITIO_G2_LOADB,
    NITIO_G3_LOADB,
    NITIO_G0_INPUT_SEL,
    NITIO_G1_INPUT_SEL,
    NITIO_G2_INPUT_SEL,
    NITIO_G3_INPUT_SEL,
    NITIO_G0_CNT_MODE,
    NITIO_G1_CNT_MODE,
    NITIO_G2_CNT_MODE,
    NITIO_G3_CNT_MODE,
    NITIO_G0_GATE2,
    NITIO_G1_GATE2,
    NITIO_G2_GATE2,
    NITIO_G3_GATE2,
    NITIO_G01_STATUS,
    NITIO_G23_STATUS,
    NITIO_G01_RESET,
    NITIO_G23_RESET,
    NITIO_G01_STATUS1,
    NITIO_G23_STATUS1,
    NITIO_G01_STATUS2,
    NITIO_G23_STATUS2,
    NITIO_G0_DMA_CFG,
    NITIO_G1_DMA_CFG,
    NITIO_G2_DMA_CFG,
    NITIO_G3_DMA_CFG,
    NITIO_G0_DMA_STATUS,
    NITIO_G1_DMA_STATUS,
    NITIO_G2_DMA_STATUS,
    NITIO_G3_DMA_STATUS,
    NITIO_G0_ABZ,
    NITIO_G1_ABZ,
    NITIO_G0_INT_ACK,
    NITIO_G1_INT_ACK,
    NITIO_G2_INT_ACK,
    NITIO_G3_INT_ACK,
    NITIO_G0_STATUS,
    NITIO_G1_STATUS,
    NITIO_G2_STATUS,
    NITIO_G3_STATUS,
    NITIO_G0_INT_ENA,
    NITIO_G1_INT_ENA,
    NITIO_G2_INT_ENA,
    NITIO_G3_INT_ENA,
    NITIO_NUM_REGS,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ni_gpct_variant {
    ni_gpct_variant_e_series,
    ni_gpct_variant_m_series,
    ni_gpct_variant_660x,
}

#[repr(C)]
pub struct ni_gpct {
    pub counter_dev: *mut ni_gpct_device,
    pub counter_index: ::core::ffi::c_uint,
    pub chip_index: ::core::ffi::c_uint,
    pub clock_period_ps: u64, /* clock period in picoseconds */
    pub mite_chan: *mut mite_channel,
    pub lock: spinlock_t, /* protects 'mite_chan' */
}

#[repr(C)]
pub struct ni_gpct_device {
    pub dev: *mut comedi_device,
    pub write: Option<unsafe extern "C" fn(*mut ni_gpct, ::core::ffi::c_uint, ni_gpct_register)>,
    pub read: Option<unsafe extern "C" fn(*mut ni_gpct, ni_gpct_register) -> ::core::ffi::c_uint>,
    pub variant: ni_gpct_variant,
    pub counters: *mut ni_gpct,
    pub num_counters: ::core::ffi::c_uint,
    pub num_chips: ::core::ffi::c_uint,
    pub regs: *mut [::core::ffi::c_uint; NITIO_NUM_REGS as usize], /* [num_chips][NITIO_NUM_REGS] */
    pub regs_lock: spinlock_t, /* protects 'regs' */
    pub routing_tables: *const ni_route_tables, /* link to routes */
}

pub const NITIO_NUM_REGS: usize = ni_gpct_register::NITIO_NUM_REGS as usize;

extern "C" {
    pub fn ni_gpct_device_construct(
        dev: *mut comedi_device,
        write: Option<unsafe extern "C" fn(*mut ni_gpct, ::core::ffi::c_uint, ni_gpct_register)>,
        read: Option<unsafe extern "C" fn(*mut ni_gpct, ni_gpct_register) -> ::core::ffi::c_uint>,
        variant: ni_gpct_variant,
        num_counters: ::core::ffi::c_uint,
        counters_per_chip: ::core::ffi::c_uint,
        routing_tables: *const ni_route_tables,
    ) -> *mut ni_gpct_device;
    pub fn ni_gpct_device_destroy(counter_dev: *mut ni_gpct_device);
    pub fn ni_tio_init_counter(counter: *mut ni_gpct);
    pub fn ni_tio_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ni_tio_insn_config(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ni_tio_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ni_tio_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> ::core::ffi::c_int;
    pub fn ni_tio_cmdtest(dev: *mut comedi_device, s: *mut comedi_subdevice, cmd: *mut comedi_cmd) -> ::core::ffi::c_int;
    pub fn ni_tio_cancel(counter: *mut ni_gpct) -> ::core::ffi::c_int;
    pub fn ni_tio_handle_interrupt(counter: *mut ni_gpct, s: *mut comedi_subdevice);
    pub fn ni_tio_set_mite_channel(counter: *mut ni_gpct, mite_chan: *mut mite_channel);
    pub fn ni_tio_acknowledge(counter: *mut ni_gpct);
    pub fn ni_tio_get_routing(counter_dev: *mut ni_gpct_device, destination: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ni_tio_set_routing(counter_dev: *mut ni_gpct_device, destination: ::core::ffi::c_uint, register_value: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ni_tio_unset_routing(counter_dev: *mut ni_gpct_device, destination: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
