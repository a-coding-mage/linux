/* SPDX-License-Identifier: GPL-2.0 */

pub const ACI_REG_COMMAND: i32 = 0; /* write register offset */
pub const ACI_REG_STATUS: i32 = 1; /* read register offset */
pub const ACI_REG_BUSY: i32 = 2; /* busy register offset */
pub const ACI_REG_RDS: i32 = 2; /* PCM20: RDS register offset */
pub const ACI_MINTIME: i32 = 500; /* ACI time out limit */

pub const ACI_SET_MUTE: u8 = 0x0d;
pub const ACI_SET_POWERAMP: u8 = 0x0f;
pub const ACI_SET_TUNERMUTE: u8 = 0xa3;
pub const ACI_SET_TUNERMONO: u8 = 0xa4;
pub const ACI_SET_IDE: u8 = 0xd0;
pub const ACI_SET_WSS: u8 = 0xd1;
pub const ACI_SET_SOLOMODE: u8 = 0xd2;
pub const ACI_SET_PREAMP: u8 = 0x03;
pub const ACI_GET_PREAMP: u8 = 0x21;
pub const ACI_WRITE_TUNE: u8 = 0xa7;
pub const ACI_READ_TUNERSTEREO: u8 = 0xa8;
pub const ACI_READ_TUNERSTATION: u8 = 0xa9;
pub const ACI_READ_VERSION: u8 = 0xf1;
pub const ACI_READ_IDCODE: u8 = 0xf2;
pub const ACI_INIT: u8 = 0xff;
pub const ACI_STATUS: u8 = 0xf0;
pub const ACI_S_GENERAL: u8 = 0x00;
pub const ACI_ERROR_OP: u8 = 0xdf;

/* ACI Mixer */

/* These are the values for the right channel GET registers.
   Add an offset of 0x01 for the left channel register.
   (left=right+0x01) */

pub const ACI_GET_MASTER: u8 = 0x03;
pub const ACI_GET_MIC: u8 = 0x05;
pub const ACI_GET_LINE: u8 = 0x07;
pub const ACI_GET_CD: u8 = 0x09;
pub const ACI_GET_SYNTH: u8 = 0x0b;
pub const ACI_GET_PCM: u8 = 0x0d;
pub const ACI_GET_LINE1: u8 = 0x10; /* Radio on PCM20 */
pub const ACI_GET_LINE2: u8 = 0x12;

pub const ACI_GET_EQ1: u8 = 0x22; /* from Bass ... */
pub const ACI_GET_EQ2: u8 = 0x24;
pub const ACI_GET_EQ3: u8 = 0x26;
pub const ACI_GET_EQ4: u8 = 0x28;
pub const ACI_GET_EQ5: u8 = 0x2a;
pub const ACI_GET_EQ6: u8 = 0x2c;
pub const ACI_GET_EQ7: u8 = 0x2e; /* ... to Treble */

/* And these are the values for the right channel SET registers.
   For left channel access you have to add an offset of 0x08.
   MASTER is an exception, which needs an offset of 0x01 */

pub const ACI_SET_MASTER: u8 = 0x00;
pub const ACI_SET_MIC: u8 = 0x30;
pub const ACI_SET_LINE: u8 = 0x31;
pub const ACI_SET_CD: u8 = 0x34;
pub const ACI_SET_SYNTH: u8 = 0x33;
pub const ACI_SET_PCM: u8 = 0x32;
pub const ACI_SET_LINE1: u8 = 0x35; /* Radio on PCM20 */
pub const ACI_SET_LINE2: u8 = 0x36;

pub const ACI_SET_EQ1: u8 = 0x40; /* from Bass ... */
pub const ACI_SET_EQ2: u8 = 0x41;
pub const ACI_SET_EQ3: u8 = 0x42;
pub const ACI_SET_EQ4: u8 = 0x43;
pub const ACI_SET_EQ5: u8 = 0x44;
pub const ACI_SET_EQ6: u8 = 0x45;
pub const ACI_SET_EQ7: u8 = 0x46; /* ... to Treble */

/* Opaque types supplied by the surrounding kernel dependencies. */
pub enum snd_card {}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_miro_aci {
    pub card: *mut snd_card,
    pub aci_port: usize,
    pub aci_vendor: i32,
    pub aci_product: i32,
    pub aci_version: i32,
    pub aci_amp: i32,
    pub aci_preamp: i32,
    pub aci_solomode: i32,

    pub aci_mutex: mutex,
}

unsafe extern "C" {
    pub fn snd_aci_cmd(
        aci: *mut snd_miro_aci,
        write1: i32,
        write2: i32,
        write3: i32,
    ) -> i32;

    pub fn snd_aci_get_aci() -> *mut snd_miro_aci;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
