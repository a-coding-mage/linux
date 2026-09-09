/* SPDX-License-Identifier: GPL-2.0 */
/*
 * I2C Address Translator
 *
 * Copyright (c) 2019,2022 Luca Ceresoli <luca@lucaceresoli.net>
 * Copyright (c) 2022,2023 Tomi Valkeinen <tomi.valkeinen@ideasonboard.com>
 *
 * Based on i2c-mux.h
 */

// Declarations corresponding to <linux/i2c.h> and <linux/types.h>.

use core::ffi::c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_adapter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_atr {
    _private: [u8; 0],
}

/**
 * enum i2c_atr_flags - Flags for an I2C ATR driver
 *
 * @I2C_ATR_F_STATIC: ATR does not support dynamic mapping, use static mapping.
 *                    Mappings will only be added or removed as a result of
 *                    devices being added or removed from a child bus.
 *                    The ATR pool will have to be big enough to accomodate all
 *                    devices expected to be added to the child buses.
 * @I2C_ATR_F_PASSTHROUGH: Allow unmapped incoming addresses to pass through
 */
pub const I2C_ATR_F_STATIC: u32 = 1u32 << 0;
pub const I2C_ATR_F_PASSTHROUGH: u32 = 1u32 << 1;

/**
 * struct i2c_atr_ops - Callbacks from ATR to the device driver.
 * @attach_addr: Notify the driver of a new device connected on a child
 *               bus, with the alias assigned to it. The driver must
 *               configure the hardware to use the alias.
 * @detach_addr: Notify the driver of a device getting disconnected. The
 *               driver must configure the hardware to stop using the
 *               alias.
 *
 * All these functions return 0 on success, a negative error code otherwise.
 */
#[repr(C)]
pub struct i2c_atr_ops {
    pub attach_addr: Option<unsafe extern "C" fn(atr: *mut i2c_atr, chan_id: u32, addr: u16, alias: u16) -> i32>,
    pub detach_addr: Option<unsafe extern "C" fn(atr: *mut i2c_atr, chan_id: u32, addr: u16)>,
}

/**
 * struct i2c_atr_adap_desc - An ATR downstream bus descriptor
 * @chan_id:        Index of the new adapter (0 .. max_adapters-1).  This value is
 *                  passed to the callbacks in `struct i2c_atr_ops`.
 * @parent:         The device used as the parent of the new i2c adapter, or NULL
 *                  to use the i2c-atr device as the parent.
 * @bus_handle:     The fwnode handle that points to the adapter's i2c
 *                  peripherals, or NULL.
 * @num_aliases:    The number of aliases in this adapter's private alias pool. Set
 *                  to zero if this adapter uses the ATR's global alias pool.
 * @aliases:        An optional array of private aliases used by the adapter
 *                  instead of the ATR's global pool of aliases. Must contain
 *                  exactly num_aliases entries if num_aliases > 0, is ignored
 *                  otherwise.
 */
#[repr(C)]
pub struct i2c_atr_adap_desc {
    pub chan_id: u32,
    pub parent: *mut device,
    pub bus_handle: *mut fwnode_handle,
    pub num_aliases: usize,
    pub aliases: *mut u16,
}

extern "C" {
    pub fn i2c_atr_new(
        parent: *mut i2c_adapter,
        dev: *mut device,
        ops: *const i2c_atr_ops,
        max_adapters: i32,
        flags: u32,
    ) -> *mut i2c_atr;

    pub fn i2c_atr_delete(atr: *mut i2c_atr);

    pub fn i2c_atr_add_adapter(atr: *mut i2c_atr, desc: *mut i2c_atr_adap_desc) -> i32;

    pub fn i2c_atr_del_adapter(atr: *mut i2c_atr, chan_id: u32);

    pub fn i2c_atr_set_driver_data(atr: *mut i2c_atr, data: *mut c_void);

    pub fn i2c_atr_get_driver_data(atr: *mut i2c_atr) -> *mut c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
