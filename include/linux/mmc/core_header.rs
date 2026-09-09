/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/include/linux/mmc/core.h
 *
 * Rust translation of the C header. External types and functions are supplied
 * by other translated dependencies.
 */

pub const UHS2_MAX_PAYLOAD_LEN: usize = 2;
pub const UHS2_MAX_RESP_LEN: usize = 20;

#[repr(C)]
pub struct uhs2_command {
    pub header: u16,
    pub arg: u16,
    pub payload: [u32; UHS2_MAX_PAYLOAD_LEN], // __be32
    pub payload_len: u8,
    pub packet_len: u8,
    pub tmode_half_duplex: u8,
    pub uhs2_resp: [u8; UHS2_MAX_RESP_LEN], // UHS2 native cmd resp
    pub uhs2_resp_len: u8, // UHS2 native cmd resp len
}

pub const MMC_CMD23_ARG_REL_WR: u32 = 1u32 << 31;
pub const MMC_CMD23_ARG_TAG_REQ: u32 = 1u32 << 29;

pub const MMC_RSP_PRESENT: u32 = 1u32 << 0;
pub const MMC_RSP_136: u32 = 1u32 << 1;
pub const MMC_RSP_CRC: u32 = 1u32 << 2;
pub const MMC_RSP_BUSY: u32 = 1u32 << 3;
pub const MMC_RSP_OPCODE: u32 = 1u32 << 4;

pub const MMC_CMD_MASK: u32 = 3u32 << 5;
pub const MMC_CMD_AC: u32 = 0u32 << 5;
pub const MMC_CMD_ADTC: u32 = 1u32 << 5;
pub const MMC_CMD_BC: u32 = 2u32 << 5;
pub const MMC_CMD_BCR: u32 = 3u32 << 5;

pub const MMC_RSP_SPI_S1: u32 = 1u32 << 7;
pub const MMC_RSP_SPI_S2: u32 = 1u32 << 8;
pub const MMC_RSP_SPI_B4: u32 = 1u32 << 9;
pub const MMC_RSP_SPI_BUSY: u32 = 1u32 << 10;

pub const MMC_RSP_NONE: u32 = 0;
pub const MMC_RSP_R1: u32 = MMC_RSP_PRESENT | MMC_RSP_CRC | MMC_RSP_OPCODE;
pub const MMC_RSP_R1B: u32 = MMC_RSP_PRESENT | MMC_RSP_CRC | MMC_RSP_OPCODE | MMC_RSP_BUSY;
pub const MMC_RSP_R1B_NO_CRC: u32 = MMC_RSP_PRESENT | MMC_RSP_OPCODE | MMC_RSP_BUSY;
pub const MMC_RSP_R2: u32 = MMC_RSP_PRESENT | MMC_RSP_136 | MMC_RSP_CRC;
pub const MMC_RSP_R3: u32 = MMC_RSP_PRESENT;
pub const MMC_RSP_R4: u32 = MMC_RSP_PRESENT;
pub const MMC_RSP_R5: u32 = MMC_RSP_PRESENT | MMC_RSP_CRC | MMC_RSP_OPCODE;
pub const MMC_RSP_R6: u32 = MMC_RSP_PRESENT | MMC_RSP_CRC | MMC_RSP_OPCODE;
pub const MMC_RSP_R7: u32 = MMC_RSP_PRESENT | MMC_RSP_CRC | MMC_RSP_OPCODE;

#[inline]
pub unsafe fn mmc_resp_type(cmd: *const mmc_command) -> u32 {
    (*cmd).flags & (MMC_RSP_PRESENT | MMC_RSP_136 | MMC_RSP_CRC | MMC_RSP_BUSY | MMC_RSP_OPCODE)
}

pub const MMC_RSP_SPI_R1: u32 = MMC_RSP_SPI_S1;
pub const MMC_RSP_SPI_R1B: u32 = MMC_RSP_SPI_S1 | MMC_RSP_SPI_BUSY;
pub const MMC_RSP_SPI_R2: u32 = MMC_RSP_SPI_S1 | MMC_RSP_SPI_S2;
pub const MMC_RSP_SPI_R3: u32 = MMC_RSP_SPI_S1 | MMC_RSP_SPI_B4;
pub const MMC_RSP_SPI_R4: u32 = MMC_RSP_SPI_S1 | MMC_RSP_SPI_B4;
pub const MMC_RSP_SPI_R5: u32 = MMC_RSP_SPI_S1 | MMC_RSP_SPI_S2;
pub const MMC_RSP_SPI_R7: u32 = MMC_RSP_SPI_S1 | MMC_RSP_SPI_B4;

#[inline]
pub unsafe fn mmc_spi_resp_type(cmd: *const mmc_command) -> u32 {
    (*cmd).flags & (MMC_RSP_SPI_S1 | MMC_RSP_SPI_BUSY | MMC_RSP_SPI_S2 | MMC_RSP_SPI_B4)
}

#[inline]
pub unsafe fn mmc_cmd_type(cmd: *const mmc_command) -> u32 {
    (*cmd).flags & MMC_CMD_MASK
}

#[repr(C)]
pub struct mmc_command {
    pub opcode: u32,
    pub arg: u32,
    pub resp: [u32; 4],
    pub flags: u32,
    pub retries: u32,
    pub error: i32,
    pub busy_timeout: u32,
    pub data: *mut mmc_data,
    pub mrq: *mut mmc_request,
    pub uhs2_cmd: *mut uhs2_command,
    pub has_ext_addr: bool,
    pub ext_addr: u8,
}

pub const MMC_DATA_WRITE: u32 = 1u32 << 8;
pub const MMC_DATA_READ: u32 = 1u32 << 9;
pub const MMC_DATA_QBR: u32 = 1u32 << 10;
pub const MMC_DATA_PRIO: u32 = 1u32 << 11;
pub const MMC_DATA_REL_WR: u32 = 1u32 << 12;
pub const MMC_DATA_DAT_TAG: u32 = 1u32 << 13;
pub const MMC_DATA_FORCED_PRG: u32 = 1u32 << 14;

#[repr(C)]
pub struct mmc_data {
    pub timeout_ns: u32,
    pub timeout_clks: u32,
    pub blksz: u32,
    pub blocks: u32,
    pub blk_addr: u32,
    pub error: i32,
    pub flags: u32,
    pub bytes_xfered: u32,
    pub stop: *mut mmc_command,
    pub mrq: *mut mmc_request,
    pub sg_len: u32,
    pub sg_count: i32,
    pub sg: *mut scatterlist,
    pub host_cookie: i32,
}

#[repr(C)]
pub struct mmc_request {
    pub sbc: *mut mmc_command,
    pub cmd: *mut mmc_command,
    pub data: *mut mmc_data,
    pub stop: *mut mmc_command,
    pub completion: completion,
    pub cmd_completion: completion,
    pub done: Option<unsafe extern "C" fn(*mut mmc_request)>,
    pub recovery_notifier: Option<unsafe extern "C" fn(*mut mmc_request)>,
    pub host: *mut mmc_host,
    pub cap_cmd_during_tfr: bool,
    pub tag: i32,
    #[cfg(CONFIG_MMC_CRYPTO)]
    pub crypto_ctx: *const bio_crypt_ctx,
    #[cfg(CONFIG_MMC_CRYPTO)]
    pub crypto_key_slot: i32,
    pub uhs2_cmd: uhs2_command,
}

extern "C" {
    pub fn mmc_wait_for_req(host: *mut mmc_host, mrq: *mut mmc_request);
    pub fn mmc_wait_for_cmd(host: *mut mmc_host, cmd: *mut mmc_command, retries: i32) -> i32;
    pub fn mmc_hw_reset(card: *mut mmc_card) -> i32;
    pub fn mmc_sw_reset(card: *mut mmc_card) -> i32;
    pub fn mmc_set_data_timeout(data: *mut mmc_data, card: *const mmc_card);
}

// External declarations supplied by other translated headers.
pub enum mmc_host {}
pub enum mmc_card {}
pub enum scatterlist {}
pub enum bio_crypt_ctx {}
#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
