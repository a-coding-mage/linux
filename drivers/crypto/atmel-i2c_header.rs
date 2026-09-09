/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2017, Microchip Technology Inc.
 * Author: Tudor Ambarus
 */

pub const ATMEL_ECC_PRIORITY: i32 = 300;

pub const COMMAND: u8 = 0x03; /* packet function */
pub const SLEEP_TOKEN: u8 = 0x01;
pub const WAKE_TOKEN_MAX_SIZE: usize = 8;

/* Definitions of Data and Command sizes */
pub const WORD_ADDR_SIZE: usize = 1;
pub const COUNT_SIZE: usize = 1;
pub const CRC_SIZE: usize = 2;
pub const CMD_OVERHEAD_SIZE: usize = COUNT_SIZE + CRC_SIZE;

/* size in bytes of the n prime */
pub const ATMEL_ECC_NIST_P256_N_SIZE: usize = 32;
pub const ATMEL_ECC_PUBKEY_SIZE: usize = 2 * ATMEL_ECC_NIST_P256_N_SIZE;

pub const STATUS_RSP_SIZE: usize = 4;
pub const ECDH_RSP_SIZE: usize = 32 + CMD_OVERHEAD_SIZE;
pub const GENKEY_RSP_SIZE: usize = ATMEL_ECC_PUBKEY_SIZE + CMD_OVERHEAD_SIZE;
pub const READ_RSP_SIZE: usize = 4 + CMD_OVERHEAD_SIZE;
pub const RANDOM_RSP_SIZE: usize = 32 + CMD_OVERHEAD_SIZE;
pub const MAX_RSP_SIZE: usize = GENKEY_RSP_SIZE;

/**
 * atmel_i2c_cmd - structure used for communicating with the device.
 * @word_addr: indicates the function of the packet sent to the device. This
 *             byte should have a value of COMMAND for normal operation.
 * @count    : number of bytes to be transferred to (or from) the device.
 * @opcode   : the command code.
 * @param1   : the first parameter; always present.
 * @param2   : the second parameter; always present.
 * @data     : optional remaining input data. Includes a 2-byte CRC.
 * @rxsize   : size of the data received from i2c client.
 * @msecs    : command execution time in milliseconds
 */
#[repr(C, packed)]
pub struct atmel_i2c_cmd {
    pub word_addr: u8,
    pub count: u8,
    pub opcode: u8,
    pub param1: u8,
    pub param2: u16,
    pub data: [u8; MAX_RSP_SIZE],
    pub msecs: u8,
    pub rxsize: u16,
}

/* Status/Error codes */
pub const STATUS_SIZE: u8 = 0x04;
pub const STATUS_NOERR: u8 = 0x00;
pub const STATUS_WAKE_SUCCESSFUL: u8 = 0x11;

/* Definitions for eeprom organization */
pub const CONFIGURATION_ZONE: u8 = 0;
pub const OTP_ZONE: u8 = 1;

/* Definitions for eeprom zone sizes */
pub const OTP_ZONE_SIZE: u8 = 64;

/* Definitions for Indexes common to all commands */
pub const RSP_DATA_IDX: u8 = 1; /* buffer index of data in response */
pub const DATA_SLOT_2: u8 = 2; /* used for ECDH private key */

/* Definitions for the device lock state */
pub const DEVICE_LOCK_ADDR: u8 = 0x15;
pub const LOCK_VALUE_IDX: u8 = RSP_DATA_IDX + 2;
pub const LOCK_CONFIG_IDX: u8 = RSP_DATA_IDX + 3;

/*
 * Wake High delay to data communication (microseconds). SDA should be stable
 * high for this entire duration.
 */
pub const TWHI_MIN: u32 = 1500;
pub const TWHI_MAX: u32 = 1550;

/* Wake Low duration */
pub const TWLO_USEC: u32 = 60;

/* Command execution time (milliseconds) */
pub const MAX_EXEC_TIME_ECDH: u32 = 58;
pub const MAX_EXEC_TIME_GENKEY: u32 = 115;
pub const MAX_EXEC_TIME_READ: u32 = 1;
pub const MAX_EXEC_TIME_RANDOM: u32 = 50;

/* Command opcode */
pub const OPCODE_ECDH: u8 = 0x43;
pub const OPCODE_GENKEY: u8 = 0x40;
pub const OPCODE_READ: u8 = 0x02;
pub const OPCODE_RANDOM: u8 = 0x1b;

/* Definitions for the READ Command */
pub const READ_COUNT: u8 = 7;

/* Definitions for the RANDOM Command */
pub const RANDOM_COUNT: u8 = 7;

/* Definitions for the GenKey Command */
pub const GENKEY_COUNT: u8 = 7;
pub const GENKEY_MODE_PRIVATE: u8 = 0x04;

/* Definitions for the ECDH Command */
pub const ECDH_COUNT: u8 = 71;
pub const ECDH_PREFIX_MODE: u8 = 0x00;

/* Used for binding tfm objects to i2c clients. */
#[repr(C)]
pub struct atmel_ecc_driver_data {
    pub i2c_client_list: list_head,
    pub i2c_list_lock: spinlock_t,
}

/**
 * atmel_i2c_client_priv - i2c_client private data
 * @client              : pointer to i2c client device
 * @i2c_client_list_node: part of i2c_client_list
 * @lock                : lock for sending i2c commands
 * @wake_token          : wake token array of zeros
 * @wake_token_sz       : size in bytes of the wake_token
 * @tfm_count           : number of active crypto transformations on i2c client
 * @hwrng               : hold the hardware generated rng
 *
 * Reads and writes from/to the i2c client are sequential. The first byte
 * transmitted to the device is treated as the byte size. Any attempt to send
 * more than this number of bytes will cause the device to not ACK those bytes.
 * After the host writes a single command byte to the input buffer, reads are
 * prohibited until after the device completes command execution. Use a mutex
 * when sending i2c commands.
 */
#[repr(C)]
pub struct atmel_i2c_client_priv {
    pub client: *mut i2c_client,
    pub i2c_client_list_node: list_head,
    pub lock: mutex,
    pub wake_token: [u8; WAKE_TOKEN_MAX_SIZE],
    pub wake_token_sz: usize,
    pub tfm_count: atomic_t,
    pub hwrng: hwrng,
}

/**
 * atmel_i2c_work_data - data structure representing the work
 * @ctx : transformation context.
 * @cbk : pointer to a callback function to be invoked upon completion of this
 *        request. This has the form:
 *        callback(struct atmel_i2c_work_data *work_data, void *areq, u8 status)
 *        where:
 *        @work_data: data structure representing the work
 *        @areq     : optional pointer to an argument passed with the original
 *                    request.
 *        @status   : status returned from the i2c client device or i2c error.
 * @areq: optional pointer to a user argument for use at callback time.
 * @work: describes the task to be executed.
 * @cmd : structure used for communicating with the device.
 */
#[repr(C)]
pub struct atmel_i2c_work_data {
    pub ctx: *mut core::ffi::c_void,
    pub client: *mut i2c_client,
    pub cbk: Option<unsafe extern "C" fn(work_data: *mut atmel_i2c_work_data,
                                          areq: *mut core::ffi::c_void,
                                          status: i32)>,
    pub areq: *mut core::ffi::c_void,
    pub work: work_struct,
    pub cmd: atmel_i2c_cmd,
}

unsafe extern "C" {
    pub fn atmel_i2c_probe(client: *mut i2c_client) -> i32;

    pub fn atmel_i2c_enqueue(
        work_data: *mut atmel_i2c_work_data,
        cbk: Option<unsafe extern "C" fn(
            work_data: *mut atmel_i2c_work_data,
            areq: *mut core::ffi::c_void,
            status: i32,
        )>,
        areq: *mut core::ffi::c_void,
    );
    pub fn atmel_i2c_flush_queue();

    pub fn atmel_i2c_send_receive(client: *mut i2c_client, cmd: *mut atmel_i2c_cmd) -> i32;

    pub fn atmel_i2c_init_read_config_cmd(cmd: *mut atmel_i2c_cmd);
    pub fn atmel_i2c_init_read_otp_cmd(cmd: *mut atmel_i2c_cmd, addr: u16) -> i32;
    pub fn atmel_i2c_init_random_cmd(cmd: *mut atmel_i2c_cmd);
    pub fn atmel_i2c_init_genkey_cmd(cmd: *mut atmel_i2c_cmd, keyid: u16);
    pub fn atmel_i2c_init_ecdh_cmd(cmd: *mut atmel_i2c_cmd, pubkey: *mut scatterlist) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
