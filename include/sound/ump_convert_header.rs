// SPDX-License-Identifier: GPL-2.0-or-later

// Dependency supplied by the translated UMP message definitions.

/* context for converting from legacy control messages to UMP packet */
#[repr(C)]
pub struct ump_cvt_to_ump_bank {
    pub rpn_set: bool,
    pub nrpn_set: bool,
    pub bank_set: bool,
    pub cc_rpn_msb: u8,
    pub cc_rpn_lsb: u8,
    pub cc_nrpn_msb: u8,
    pub cc_nrpn_lsb: u8,
    pub cc_data_msb: u8,
    pub cc_data_lsb: u8,
    pub cc_bank_msb: u8,
    pub cc_bank_lsb: u8,
    pub cc_data_msb_set: bool,
    pub cc_data_lsb_set: bool,
}

/* context for converting from MIDI1 byte stream to UMP packet */
#[repr(C)]
pub struct ump_cvt_to_ump {
    /* MIDI1 intermediate buffer */
    pub buf: [u8; 6], /* up to 6 bytes for SysEx */
    pub len: i32,
    pub cmd_bytes: i32,

    /* UMP output packet */
    pub ump: [u32; 4],
    pub ump_bytes: i32,

    /* various status */
    pub in_sysex: u32,
    pub bank: [ump_cvt_to_ump_bank; 16], /* per channel */
}

pub unsafe extern "C" fn snd_ump_convert_from_ump(
    data: *const u32,
    dst: *mut u8,
    group_ret: *mut u8,
) -> i32;

pub unsafe extern "C" fn snd_ump_convert_to_ump(
    cvt: *mut ump_cvt_to_ump,
    group: u8,
    protocol: u32,
    c: u8,
);

/* reset the converter context, called at each open to ump */
#[inline]
pub unsafe fn snd_ump_convert_reset(ctx: *mut ump_cvt_to_ump) {
    core::ptr::write_bytes(
        ctx.cast::<u8>(),
        0,
        core::mem::size_of::<ump_cvt_to_ump>(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
