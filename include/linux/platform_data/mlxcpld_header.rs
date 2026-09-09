/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/*
 * Mellanox I2C multiplexer support in CPLD
 *
 * Copyright (C) 2016-2020 Mellanox Technologies
 */

/* Platform data for the CPLD I2C multiplexers */

/* mlxcpld_mux_plat_data - per mux data, used with i2c_register_board_info
 * @chan_ids - channels array
 * @num_adaps - number of adapters
 * @sel_reg_addr - mux select register offset in CPLD space
 * @reg_size: register size in bytes
 * @handle: handle to be passed by callback
 * @completion_notify: callback to notify when all the adapters are created
 */
#[repr(C)]
pub struct mlxcpld_mux_plat_data {
    pub chan_ids: *mut core::ffi::c_int,
    pub num_adaps: core::ffi::c_int,
    pub sel_reg_addr: core::ffi::c_int,
    pub reg_size: u8,
    pub handle: *mut core::ffi::c_void,
    pub completion_notify: Option<unsafe extern "C" fn(
        handle: *mut core::ffi::c_void,
        parent: *mut i2c_adapter,
        adapters: *mut *mut i2c_adapter,
    ) -> core::ffi::c_int>,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
