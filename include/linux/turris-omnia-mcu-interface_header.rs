/* SPDX-License-Identifier: GPL-2.0 */
/*
 * CZ.NIC's Turris Omnia MCU I2C interface commands definitions
 *
 * 2024 by Marek Behún <kabel@kernel.org>
 */

// Translated from turris-omnia-mcu-interface.h. Linux header dependencies are
// represented by equivalent local Rust operations and opaque declarations.

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
pub type __le16 = u16;
#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
pub type __le32 = u32;

macro_rules! BIT { ($n:expr) => { 1u32 << ($n) }; }
macro_rules! GENMASK { ($h:expr, $l:expr) => { (((1u32 << (($h) - ($l) + 1)) - 1) << ($l)) }; }
macro_rules! FIELD_PREP_CONST { ($mask:expr, $val:expr) => { (($val) << ($mask).trailing_zeros()) }; }

pub const OMNIA_CMD_GET_STATUS_WORD: u32 = 0x01;
pub const OMNIA_CMD_GENERAL_CONTROL: u32 = 0x02;
pub const OMNIA_CMD_LED_MODE: u32 = 0x03;
pub const OMNIA_CMD_LED_STATE: u32 = 0x04;
pub const OMNIA_CMD_LED_COLOR: u32 = 0x05;
pub const OMNIA_CMD_USER_VOLTAGE: u32 = 0x06;
pub const OMNIA_CMD_SET_BRIGHTNESS: u32 = 0x07;
pub const OMNIA_CMD_GET_BRIGHTNESS: u32 = 0x08;
pub const OMNIA_CMD_GET_RESET: u32 = 0x09;
pub const OMNIA_CMD_GET_FW_VERSION_APP: u32 = 0x0A;
pub const OMNIA_CMD_SET_WATCHDOG_STATE: u32 = 0x0B;
pub const OMNIA_CMD_GET_WATCHDOG_STATE: u32 = 0x0D;
pub const OMNIA_CMD_GET_FW_VERSION_BOOT: u32 = 0x0E;
pub const OMNIA_CMD_GET_FW_CHECKSUM: u32 = 0x0F;
pub const OMNIA_CMD_GET_FEATURES: u32 = 0x10;
pub const OMNIA_CMD_GET_EXT_STATUS_DWORD: u32 = 0x11;
pub const OMNIA_CMD_EXT_CONTROL: u32 = 0x12;
pub const OMNIA_CMD_GET_EXT_CONTROL_STATUS: u32 = 0x13;
pub const OMNIA_CMD_GET_INT_AND_CLEAR: u32 = 0x14;
pub const OMNIA_CMD_GET_INT_MASK: u32 = 0x15;
pub const OMNIA_CMD_SET_INT_MASK: u32 = 0x16;
pub const OMNIA_CMD_FLASH: u32 = 0x19;
pub const OMNIA_CMD_SET_WDT_TIMEOUT: u32 = 0x20;
pub const OMNIA_CMD_GET_WDT_TIMELEFT: u32 = 0x21;
pub const OMNIA_CMD_SET_WAKEUP: u32 = 0x22;
pub const OMNIA_CMD_GET_UPTIME_AND_WAKEUP: u32 = 0x23;
pub const OMNIA_CMD_POWER_OFF: u32 = 0x24;
pub const OMNIA_CMD_SET_USB_OVC_PROT: u32 = 0x25;
pub const OMNIA_CMD_GET_USB_OVC_PROT: u32 = 0x26;
pub const OMNIA_CMD_TRNG_COLLECT_ENTROPY: u32 = 0x28;
pub const OMNIA_CMD_CRYPTO_GET_PUBLIC_KEY: u32 = 0x29;
pub const OMNIA_CMD_CRYPTO_SIGN_MESSAGE: u32 = 0x2A;
pub const OMNIA_CMD_CRYPTO_COLLECT_SIGNATURE: u32 = 0x2B;
pub const OMNIA_CMD_BOARD_INFO_GET: u32 = 0x2C;
pub const OMNIA_CMD_BOARD_INFO_BURN: u32 = 0x2D;
pub const OMNIA_CMD_SET_GAMMA_CORRECTION: u32 = 0x30;
pub const OMNIA_CMD_GET_GAMMA_CORRECTION: u32 = 0x31;
pub const OMNIA_CMD_SET_LED_CORRECTIONS: u32 = 0x32;
pub const OMNIA_CMD_GET_LED_CORRECTIONS: u32 = 0x33;

pub const OMNIA_FLASH_CMD_UNLOCK: u32 = 0x01;
pub const OMNIA_FLASH_CMD_SIZE_AND_CSUM: u32 = 0x02;
pub const OMNIA_FLASH_CMD_PROGRAM: u32 = 0x03;
pub const OMNIA_FLASH_CMD_RESET: u32 = 0x04;

pub const OMNIA_STS_MCU_TYPE_MASK: u32 = GENMASK!(1, 0);
pub const OMNIA_STS_MCU_TYPE_STM32: u32 = FIELD_PREP_CONST!(OMNIA_STS_MCU_TYPE_MASK, 0);
pub const OMNIA_STS_MCU_TYPE_GD32: u32 = FIELD_PREP_CONST!(OMNIA_STS_MCU_TYPE_MASK, 1);
pub const OMNIA_STS_MCU_TYPE_MKL: u32 = FIELD_PREP_CONST!(OMNIA_STS_MCU_TYPE_MASK, 2);
pub const OMNIA_STS_FEATURES_SUPPORTED: u32 = BIT!(2);
pub const OMNIA_STS_USER_REGULATOR_NOT_SUPPORTED: u32 = BIT!(3);
pub const OMNIA_STS_CARD_DET: u32 = BIT!(4);
pub const OMNIA_STS_MSATA_IND: u32 = BIT!(5);
pub const OMNIA_STS_USB30_OVC: u32 = BIT!(6);
pub const OMNIA_STS_USB31_OVC: u32 = BIT!(7);
pub const OMNIA_STS_USB30_PWRON: u32 = BIT!(8);
pub const OMNIA_STS_USB31_PWRON: u32 = BIT!(9);
pub const OMNIA_STS_ENABLE_4V5: u32 = BIT!(10);
pub const OMNIA_STS_BUTTON_MODE: u32 = BIT!(11);
pub const OMNIA_STS_BUTTON_PRESSED: u32 = BIT!(12);
pub const OMNIA_STS_BUTTON_COUNTER_MASK: u32 = GENMASK!(15, 13);

pub const OMNIA_CTL_LIGHT_RST: u32 = BIT!(0);
pub const OMNIA_CTL_HARD_RST: u32 = BIT!(1);
pub const OMNIA_CTL_USB30_PWRON: u32 = BIT!(3);
pub const OMNIA_CTL_USB31_PWRON: u32 = BIT!(4);
pub const OMNIA_CTL_ENABLE_4V5: u32 = BIT!(5);
pub const OMNIA_CTL_BUTTON_MODE: u32 = BIT!(6);
pub const OMNIA_CTL_BOOTLOADER: u32 = BIT!(7);

pub const OMNIA_FEAT_PERIPH_MCU: u32 = BIT!(0);
pub const OMNIA_FEAT_EXT_CMDS: u32 = BIT!(1);
pub const OMNIA_FEAT_WDT_PING: u32 = BIT!(2);
pub const OMNIA_FEAT_LED_STATE_EXT_MASK: u32 = GENMASK!(4, 3);
pub const OMNIA_FEAT_LED_STATE_EXT: u32 = FIELD_PREP_CONST!(OMNIA_FEAT_LED_STATE_EXT_MASK, 1);
pub const OMNIA_FEAT_LED_STATE_EXT_V32: u32 = FIELD_PREP_CONST!(OMNIA_FEAT_LED_STATE_EXT_MASK, 2);
pub const OMNIA_FEAT_LED_GAMMA_CORRECTION: u32 = BIT!(5);
pub const OMNIA_FEAT_NEW_INT_API: u32 = BIT!(6);
pub const OMNIA_FEAT_BOOTLOADER: u32 = BIT!(7);
pub const OMNIA_FEAT_FLASHING: u32 = BIT!(8);
pub const OMNIA_FEAT_NEW_MESSAGE_API: u32 = BIT!(9);
pub const OMNIA_FEAT_BRIGHTNESS_INT: u32 = BIT!(10);
pub const OMNIA_FEAT_POWEROFF_WAKEUP: u32 = BIT!(11);
pub const OMNIA_FEAT_CAN_OLD_MESSAGE_API: u32 = BIT!(12);
pub const OMNIA_FEAT_TRNG: u32 = BIT!(13);
pub const OMNIA_FEAT_CRYPTO: u32 = BIT!(14);
pub const OMNIA_FEAT_BOARD_INFO: u32 = BIT!(15);
pub const OMNIA_FEAT_MCU_TYPE_MASK: u32 = GENMASK!(17, 16);
pub const OMNIA_FEAT_MCU_TYPE_STM32: u32 = FIELD_PREP_CONST!(OMNIA_FEAT_MCU_TYPE_MASK, 0);
pub const OMNIA_FEAT_MCU_TYPE_GD32: u32 = FIELD_PREP_CONST!(OMNIA_FEAT_MCU_TYPE_MASK, 1);
pub const OMNIA_FEAT_MCU_TYPE_MKL: u32 = FIELD_PREP_CONST!(OMNIA_FEAT_MCU_TYPE_MASK, 2);
pub const OMNIA_FEAT_FEATURES_SUPPORTED: u32 = BIT!(18);
pub const OMNIA_FEAT_USER_REGULATOR_NOT_SUPPORTED: u32 = BIT!(19);
pub const OMNIA_FEAT_FROM_BIT_16_INVALID: u32 = BIT!(20);
pub const OMNIA_FEAT_PER_LED_CORRECTION: u32 = BIT!(21);
pub const OMNIA_FEAT_USB_OVC_PROT_SETTING: u32 = BIT!(22);

pub const OMNIA_EXT_STS_SFP_nDET: u32 = BIT!(0);
pub const OMNIA_EXT_STS_LED_STATES_MASK: u32 = GENMASK!(31, 12);
pub const OMNIA_EXT_STS_WLAN0_MSATA_LED: u32 = BIT!(12);
pub const OMNIA_EXT_STS_WLAN1_LED: u32 = BIT!(13);
pub const OMNIA_EXT_STS_WLAN2_LED: u32 = BIT!(14);
pub const OMNIA_EXT_STS_WPAN0_LED: u32 = BIT!(15);
pub const OMNIA_EXT_STS_WPAN1_LED: u32 = BIT!(16);
pub const OMNIA_EXT_STS_WPAN2_LED: u32 = BIT!(17);
pub const OMNIA_EXT_STS_WAN_LED0: u32 = BIT!(18);
pub const OMNIA_EXT_STS_WAN_LED1: u32 = BIT!(19);
pub const OMNIA_EXT_STS_LAN0_LED0: u32 = BIT!(20);
pub const OMNIA_EXT_STS_LAN0_LED1: u32 = BIT!(21);
pub const OMNIA_EXT_STS_LAN1_LED0: u32 = BIT!(22);
pub const OMNIA_EXT_STS_LAN1_LED1: u32 = BIT!(23);
pub const OMNIA_EXT_STS_LAN2_LED0: u32 = BIT!(24);
pub const OMNIA_EXT_STS_LAN2_LED1: u32 = BIT!(25);
pub const OMNIA_EXT_STS_LAN3_LED0: u32 = BIT!(26);
pub const OMNIA_EXT_STS_LAN3_LED1: u32 = BIT!(27);
pub const OMNIA_EXT_STS_LAN4_LED0: u32 = BIT!(28);
pub const OMNIA_EXT_STS_LAN4_LED1: u32 = BIT!(29);
pub const OMNIA_EXT_STS_LAN5_LED0: u32 = BIT!(30);
pub const OMNIA_EXT_STS_LAN5_LED1: u32 = BIT!(31);
pub const OMNIA_EXT_CTL_nRES_MMC: u32 = BIT!(0);
pub const OMNIA_EXT_CTL_nRES_LAN: u32 = BIT!(1);
pub const OMNIA_EXT_CTL_nRES_PHY: u32 = BIT!(2);
pub const OMNIA_EXT_CTL_nPERST0: u32 = BIT!(3);
pub const OMNIA_EXT_CTL_nPERST1: u32 = BIT!(4);
pub const OMNIA_EXT_CTL_nPERST2: u32 = BIT!(5);
pub const OMNIA_EXT_CTL_PHY_SFP: u32 = BIT!(6);
pub const OMNIA_EXT_CTL_PHY_SFP_AUTO: u32 = BIT!(7);
pub const OMNIA_EXT_CTL_nVHV_CTRL: u32 = BIT!(8);

pub const OMNIA_INT_CARD_DET: u32 = BIT!(0);
pub const OMNIA_INT_MSATA_IND: u32 = BIT!(1);
pub const OMNIA_INT_USB30_OVC: u32 = BIT!(2);
pub const OMNIA_INT_USB31_OVC: u32 = BIT!(3);
pub const OMNIA_INT_BUTTON_PRESSED: u32 = BIT!(4);
pub const OMNIA_INT_SFP_nDET: u32 = BIT!(5);
pub const OMNIA_INT_BRIGHTNESS_CHANGED: u32 = BIT!(6);
pub const OMNIA_INT_TRNG: u32 = BIT!(7);
pub const OMNIA_INT_MESSAGE_SIGNED: u32 = BIT!(8);
pub const OMNIA_INT_LED_STATES_MASK: u32 = GENMASK!(31, 12);
pub const OMNIA_INT_WLAN0_MSATA_LED: u32 = BIT!(12);
pub const OMNIA_INT_WLAN1_LED: u32 = BIT!(13);
pub const OMNIA_INT_WLAN2_LED: u32 = BIT!(14);
pub const OMNIA_INT_WPAN0_LED: u32 = BIT!(15);
pub const OMNIA_INT_WPAN1_LED: u32 = BIT!(16);
pub const OMNIA_INT_WPAN2_LED: u32 = BIT!(17);
pub const OMNIA_INT_WAN_LED0: u32 = BIT!(18);
pub const OMNIA_INT_WAN_LED1: u32 = BIT!(19);
pub const OMNIA_INT_LAN0_LED0: u32 = BIT!(20);
pub const OMNIA_INT_LAN0_LED1: u32 = BIT!(21);
pub const OMNIA_INT_LAN1_LED0: u32 = BIT!(22);
pub const OMNIA_INT_LAN1_LED1: u32 = BIT!(23);
pub const OMNIA_INT_LAN2_LED0: u32 = BIT!(24);
pub const OMNIA_INT_LAN2_LED1: u32 = BIT!(25);
pub const OMNIA_INT_LAN3_LED0: u32 = BIT!(26);
pub const OMNIA_INT_LAN3_LED1: u32 = BIT!(27);
pub const OMNIA_INT_LAN4_LED0: u32 = BIT!(28);
pub const OMNIA_INT_LAN4_LED1: u32 = BIT!(29);
pub const OMNIA_INT_LAN5_LED0: u32 = BIT!(30);
pub const OMNIA_INT_LAN5_LED1: u32 = BIT!(31);

pub const OMNIA_CMD_LED_MODE_LED_MASK: u32 = GENMASK!(3, 0);
pub const OMNIA_CMD_LED_MODE_USER: u32 = BIT!(4);
#[inline] pub const fn OMNIA_CMD_LED_MODE_LED(l: u32) -> u32 { (l) & OMNIA_CMD_LED_MODE_LED_MASK }
pub const OMNIA_CMD_LED_STATE_LED_MASK: u32 = GENMASK!(3, 0);
pub const OMNIA_CMD_LED_STATE_ON: u32 = BIT!(4);
#[inline] pub const fn OMNIA_CMD_LED_STATE_LED(l: u32) -> u32 { (l) & OMNIA_CMD_LED_STATE_LED_MASK }
pub const OMNIA_CMD_POWER_OFF_POWERON_BUTTON: u32 = BIT!(0);
pub const OMNIA_CMD_POWER_OFF_MAGIC: u32 = 0xdead;
pub const OMNIA_CMD_xET_USB_OVC_PROT_PORT_MASK: u32 = GENMASK!(3, 0);
pub const OMNIA_CMD_xET_USB_OVC_PROT_ENABLE: u32 = BIT!(4);

#[repr(C)] pub struct i2c_client { _private: [u8; 0] }

extern "C" {
    pub fn omnia_cmd_write_read(client: *const i2c_client, cmd: *mut core::ffi::c_void,
        cmd_len: u32, reply: *mut core::ffi::c_void, reply_len: u32) -> i32;
}

#[inline] pub unsafe fn omnia_cmd_write(client: *const i2c_client, cmd: *mut core::ffi::c_void, len: u32) -> i32 {
    omnia_cmd_write_read(client, cmd, len, core::ptr::null_mut(), 0)
}
#[inline] pub unsafe fn omnia_cmd_write_u8(client: *const i2c_client, cmd: u8, val: u8) -> i32 {
    let mut buf = [cmd, val]; omnia_cmd_write(client, buf.as_mut_ptr().cast(), 2)
}
#[inline] pub unsafe fn omnia_cmd_write_u16(client: *const i2c_client, cmd: u8, val: u16) -> i32 {
    let mut buf = [0u8; 3]; buf[0] = cmd; buf[1..3].copy_from_slice(&val.to_le_bytes()); omnia_cmd_write(client, buf.as_mut_ptr().cast(), 3)
}
#[inline] pub unsafe fn omnia_cmd_write_u32(client: *const i2c_client, cmd: u8, val: u32) -> i32 {
    let mut buf = [0u8; 5]; buf[0] = cmd; buf[1..5].copy_from_slice(&val.to_le_bytes()); omnia_cmd_write(client, buf.as_mut_ptr().cast(), 5)
}
#[inline] pub unsafe fn omnia_cmd_read(client: *const i2c_client, cmd: u8, reply: *mut core::ffi::c_void, len: u32) -> i32 {
    omnia_cmd_write_read(client, (&cmd as *const u8).cast_mut().cast(), 1, reply, len)
}
#[inline] pub fn omnia_compute_reply_length(mask: u64, interleaved: bool, offset: u32) -> u32 {
    if mask == 0 { return 0; }
    ((63 - mask.leading_zeros() as u32) >> 3) + if interleaved { 1 } else { 0 } + 1 + offset
}
#[inline] pub unsafe fn omnia_cmd_read_bits(client: *const i2c_client, cmd: u8, bits: u64, dst: *mut u64) -> i32 {
    if bits == 0 { *dst = 0; return 0; }
    let mut reply = 0u32;
    let err = omnia_cmd_read(client, cmd, (&mut reply as *mut u32).cast(), omnia_compute_reply_length(bits, false, 0));
    if err != 0 { return err; } *dst = (u32::from_le(reply) as u64) & bits; 0
}
#[inline] pub unsafe fn omnia_cmd_read_bit(client: *const i2c_client, cmd: u8, bit: u64) -> i32 {
    let mut reply = 0u64; let err = omnia_cmd_read_bits(client, cmd, bit, &mut reply); if err != 0 { err } else { (reply != 0) as i32 }
}
#[inline] pub unsafe fn omnia_cmd_read_u32(client: *const i2c_client, cmd: u8, dst: *mut u32) -> i32 {
    let mut reply = 0u32; let err = omnia_cmd_read(client, cmd, (&mut reply as *mut u32).cast(), 4); if err != 0 { return err; } *dst = u32::from_le(reply); 0
}
#[inline] pub unsafe fn omnia_cmd_read_u16(client: *const i2c_client, cmd: u8, dst: *mut u16) -> i32 {
    let mut reply = 0u16; let err = omnia_cmd_read(client, cmd, (&mut reply as *mut u16).cast(), 2); if err != 0 { return err; } *dst = u16::from_le(reply); 0
}
#[inline] pub unsafe fn omnia_cmd_read_u8(client: *const i2c_client, cmd: u8, reply: *mut u8) -> i32 {
    omnia_cmd_read(client, cmd, reply.cast(), 1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
