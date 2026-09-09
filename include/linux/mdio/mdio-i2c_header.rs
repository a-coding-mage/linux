/* SPDX-License-Identifier: GPL-2.0 */
/*
 * MDIO I2C bridge
 *
 * Copyright (C) 2015 Russell King
 */

// Opaque declarations corresponding to the C forward declarations.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_adapter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mii_bus {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mdio_i2c_proto {
    MDIO_I2C_NONE = 0,
    MDIO_I2C_MARVELL_C22 = 1,
    MDIO_I2C_C45 = 2,
    MDIO_I2C_ROLLBALL = 3,
}

unsafe extern "C" {
    pub fn mdio_i2c_alloc(
        parent: *mut device,
        i2c: *mut i2c_adapter,
        protocol: mdio_i2c_proto,
    ) -> *mut mii_bus;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
