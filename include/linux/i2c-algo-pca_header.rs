/* SPDX-License-Identifier: GPL-2.0 */

/* Chips known to the pca algo */
pub const I2C_PCA_CHIP_9564: i32 = 0x00;
pub const I2C_PCA_CHIP_9665: i32 = 0x01;

/* Internal period for PCA9665 oscilator */
pub const I2C_PCA_OSC_PER: i32 = 3; /* e10-8s */

/* Clock speeds for the bus for PCA9564*/
pub const I2C_PCA_CON_330kHz: i32 = 0x00;
pub const I2C_PCA_CON_288kHz: i32 = 0x01;
pub const I2C_PCA_CON_217kHz: i32 = 0x02;
pub const I2C_PCA_CON_146kHz: i32 = 0x03;
pub const I2C_PCA_CON_88kHz: i32 = 0x04;
pub const I2C_PCA_CON_59kHz: i32 = 0x05;
pub const I2C_PCA_CON_44kHz: i32 = 0x06;
pub const I2C_PCA_CON_36kHz: i32 = 0x07;

/* PCA9564 registers */
pub const I2C_PCA_STA: i32 = 0x00; /* STATUS  Read Only  */
pub const I2C_PCA_TO: i32 = 0x00; /* TIMEOUT Write Only */
pub const I2C_PCA_DAT: i32 = 0x01; /* DATA    Read/Write */
pub const I2C_PCA_ADR: i32 = 0x02; /* OWN ADR Read/Write */
pub const I2C_PCA_CON: i32 = 0x03; /* CONTROL Read/Write */

/* PCA9665 registers */
pub const I2C_PCA_INDPTR: i32 = 0x00; /* INDIRECT Pointer Write Only */
pub const I2C_PCA_IND: i32 = 0x02; /* INDIRECT Read/Write */

/* PCA9665 indirect registers */
pub const I2C_PCA_ICOUNT: i32 = 0x00; /* Byte Count for buffered mode */
pub const I2C_PCA_IADR: i32 = 0x01; /* OWN ADR */
pub const I2C_PCA_ISCLL: i32 = 0x02; /* SCL LOW period */
pub const I2C_PCA_ISCLH: i32 = 0x03; /* SCL HIGH period */
pub const I2C_PCA_ITO: i32 = 0x04; /* TIMEOUT */
pub const I2C_PCA_IPRESET: i32 = 0x05; /* Parallel bus reset */
pub const I2C_PCA_IMODE: i32 = 0x06; /* I2C Bus mode */

/* PCA9665 I2C bus mode */
pub const I2C_PCA_MODE_STD: i32 = 0x00; /* Standard mode */
pub const I2C_PCA_MODE_FAST: i32 = 0x01; /* Fast mode */
pub const I2C_PCA_MODE_FASTP: i32 = 0x02; /* Fast Plus mode */
pub const I2C_PCA_MODE_TURBO: i32 = 0x03; /* Turbo mode */

pub const I2C_PCA_CON_AA: i32 = 0x80; /* Assert Acknowledge */
pub const I2C_PCA_CON_ENSIO: i32 = 0x40; /* Enable */
pub const I2C_PCA_CON_STA: i32 = 0x20; /* Start */
pub const I2C_PCA_CON_STO: i32 = 0x10; /* Stop */
pub const I2C_PCA_CON_SI: i32 = 0x08; /* Serial Interrupt */
pub const I2C_PCA_CON_CR: i32 = 0x07; /* Clock Rate (MASK) */

/**
 * struct pca_i2c_bus_settings - The configured PCA i2c bus settings
 * @mode: Configured i2c bus mode
 * @tlow: Configured SCL LOW period
 * @thi: Configured SCL HIGH period
 * @clock_freq: The configured clock frequency
 */
#[repr(C)]
pub struct pca_i2c_bus_settings {
    pub mode: i32,
    pub tlow: i32,
    pub thi: i32,
    pub clock_freq: i32,
}

#[repr(C)]
pub struct i2c_algo_pca_data {
    pub data: *mut core::ffi::c_void, /* private low level data */
    pub write_byte: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void, reg: i32, val: i32)>,
    pub read_byte: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void, reg: i32) -> i32>,
    pub wait_for_completion_cb: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void) -> i32>,
    pub reset_chip: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void)>,
    /* For PCA9564, use one of the predefined frequencies:
     * 330000, 288000, 217000, 146000, 88000, 59000, 44000, 36000
     * For PCA9665, use the frequency you want here. */
    pub i2c_clock: u32,
    pub chip: u32,
    pub bus_settings: pca_i2c_bus_settings,
}

extern "C" {
    pub fn i2c_pca_add_bus(adapter: *mut i2c_adapter) -> i32;
    pub fn i2c_pca_add_numbered_bus(adapter: *mut i2c_adapter) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
