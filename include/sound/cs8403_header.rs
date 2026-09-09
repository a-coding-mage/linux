/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Routines for Cirrus Logic CS8403/CS8404A IEC958 (S/PDIF) Transmitter
 *
 * The original header conditionally emits these declarations when SND_CS8403
 * or SND_CS8404 is defined. The C macro hooks SND_CS840*_DECL, DECODE, and
 * ENCODE may also rename or change linkage; the Rust functions below retain
 * the default names and external dependency symbols.
 */

#[cfg(feature = "SND_CS8403")]
pub unsafe fn snd_cs8403_decode_spdif_bits(diga: *mut snd_aes_iec958, bits: u8) {
    if bits & 0x01 != 0 {
        if bits & 0x02 == 0 {
            (*diga).status[0] |= IEC958_AES0_NONAUDIO;
        }
        if bits & 0x08 == 0 {
            (*diga).status[0] |= IEC958_AES0_CON_NOT_COPYRIGHT;
        }
        match bits & 0x10 {
            0x10 => (*diga).status[0] |= IEC958_AES0_CON_EMPHASIS_NONE,
            0x00 => (*diga).status[0] |= IEC958_AES0_CON_EMPHASIS_5015,
            _ => {}
        }
        if bits & 0x80 == 0 {
            (*diga).status[1] |= IEC958_AES1_CON_ORIGINAL;
        }
        match bits & 0x60 {
            0x00 => (*diga).status[1] |= IEC958_AES1_CON_MAGNETIC_ID,
            0x20 => (*diga).status[1] |= IEC958_AES1_CON_DIGDIGCONV_ID,
            0x40 => (*diga).status[1] |= IEC958_AES1_CON_LASEROPT_ID,
            0x60 => (*diga).status[1] |= IEC958_AES1_CON_GENERAL,
            _ => {}
        }
        match bits & 0x06 {
            0x00 => (*diga).status[3] |= IEC958_AES3_CON_FS_44100,
            0x02 => (*diga).status[3] |= IEC958_AES3_CON_FS_48000,
            0x04 => (*diga).status[3] |= IEC958_AES3_CON_FS_32000,
            _ => {}
        }
    } else {
        (*diga).status[0] = IEC958_AES0_PROFESSIONAL;
        match bits & 0x18 {
            0x00 => (*diga).status[0] |= IEC958_AES0_PRO_FS_32000,
            0x10 => (*diga).status[0] |= IEC958_AES0_PRO_FS_44100,
            0x08 => (*diga).status[0] |= IEC958_AES0_PRO_FS_48000,
            0x18 => (*diga).status[0] |= IEC958_AES0_PRO_FS_NOTID,
            _ => {}
        }
        match bits & 0x60 {
            0x20 => (*diga).status[0] |= IEC958_AES0_PRO_EMPHASIS_NONE,
            0x40 => (*diga).status[0] |= IEC958_AES0_PRO_EMPHASIS_5015,
            0x00 => (*diga).status[0] |= IEC958_AES0_PRO_EMPHASIS_CCITT,
            0x60 => (*diga).status[0] |= IEC958_AES0_PRO_EMPHASIS_NOTID,
            _ => {}
        }
        if bits & 0x80 != 0 {
            (*diga).status[1] |= IEC958_AES1_PRO_MODE_STEREOPHONIC;
        }
    }
}

#[cfg(feature = "SND_CS8403")]
pub unsafe fn snd_cs8403_encode_spdif_bits(diga: *mut snd_aes_iec958) -> u8 {
    let mut bits: u8;
    if (*diga).status[0] & IEC958_AES0_PROFESSIONAL == 0 {
        bits = 0x01;
        if (*diga).status[0] & IEC958_AES0_NONAUDIO != 0 { bits &= !0x02; } else { bits |= 0x02; }
        if (*diga).status[0] & IEC958_AES0_CON_NOT_COPYRIGHT != 0 { bits &= !0x08; } else { bits |= 0x08; }
        match (*diga).status[0] & IEC958_AES0_CON_EMPHASIS {
            IEC958_AES0_CON_EMPHASIS_5015 => {},
            _ => bits |= 0x10,
        }
        if (*diga).status[1] & IEC958_AES1_CON_ORIGINAL != 0 { bits &= !0x80; } else { bits |= 0x80; }
        if (*diga).status[1] & IEC958_AES1_CON_CATEGORY == IEC958_AES1_CON_GENERAL { bits |= 0x60; }
        else { match (*diga).status[1] & IEC958_AES1_CON_MAGNETIC_MASK { IEC958_AES1_CON_DIGDIGCONV_ID => bits |= 0x20, IEC958_AES1_CON_LASEROPT_ID => bits |= 0x40, _ => {} } }
        match (*diga).status[3] & IEC958_AES3_CON_FS { IEC958_AES3_CON_FS_48000 => bits |= 0x02, IEC958_AES3_CON_FS_32000 => bits |= 0x04, _ => {} }
    } else {
        bits = 0;
        if (*diga).status[0] & IEC958_AES0_NONAUDIO != 0 { bits &= !0x02; } else { bits |= 0x02; }
        match (*diga).status[0] & IEC958_AES0_PRO_FS { IEC958_AES0_PRO_FS_44100 => bits |= 0x10, IEC958_AES0_PRO_FS_48000 => bits |= 0x08, IEC958_AES0_PRO_FS_NOTID => bits |= 0x18, _ => {} }
        match (*diga).status[0] & IEC958_AES0_PRO_EMPHASIS { IEC958_AES0_PRO_EMPHASIS_NONE => bits |= 0x20, IEC958_AES0_PRO_EMPHASIS_5015 => bits |= 0x40, IEC958_AES0_PRO_EMPHASIS_NOTID => bits |= 0x60, _ => {} }
        match (*diga).status[1] & IEC958_AES1_PRO_MODE { IEC958_AES1_PRO_MODE_TWO | IEC958_AES1_PRO_MODE_STEREOPHONIC => {}, _ => bits |= 0x80 }
    }
    bits
}

#[cfg(feature = "SND_CS8404")]
pub unsafe fn snd_cs8404_decode_spdif_bits(diga: *mut snd_aes_iec958, bits: u8) {
    if bits & 0x10 != 0 {
        if bits & 0x20 == 0 { (*diga).status[0] |= IEC958_AES0_CON_NOT_COPYRIGHT; }
        if bits & 0x40 == 0 { (*diga).status[0] |= IEC958_AES0_CON_EMPHASIS_5015; }
        if bits & 0x80 == 0 { (*diga).status[1] |= IEC958_AES1_CON_ORIGINAL; }
        match bits & 0x03 { 0x00 => (*diga).status[1] |= IEC958_AES1_CON_DAT, 0x03 => (*diga).status[1] |= IEC958_AES1_CON_GENERAL, _ => {} }
        match bits & 0x06 { 0x02 => (*diga).status[3] |= IEC958_AES3_CON_FS_32000, 0x04 => (*diga).status[3] |= IEC958_AES3_CON_FS_48000, 0x06 => (*diga).status[3] |= IEC958_AES3_CON_FS_44100, _ => {} }
    } else {
        (*diga).status[0] = IEC958_AES0_PROFESSIONAL;
        if bits & 0x04 == 0 { (*diga).status[0] |= IEC958_AES0_NONAUDIO; }
        match bits & 0x60 { 0x00 => (*diga).status[0] |= IEC958_AES0_PRO_FS_32000, 0x40 => (*diga).status[0] |= IEC958_AES0_PRO_FS_44100, 0x20 => (*diga).status[0] |= IEC958_AES0_PRO_FS_48000, 0x60 => (*diga).status[0] |= IEC958_AES0_PRO_FS_NOTID, _ => {} }
        match bits & 0x03 { 0x02 => (*diga).status[0] |= IEC958_AES0_PRO_EMPHASIS_NONE, 0x01 => (*diga).status[0] |= IEC958_AES0_PRO_EMPHASIS_5015, 0x00 => (*diga).status[0] |= IEC958_AES0_PRO_EMPHASIS_CCITT, 0x03 => (*diga).status[0] |= IEC958_AES0_PRO_EMPHASIS_NOTID, _ => {} }
        if bits & 0x80 == 0 { (*diga).status[1] |= IEC958_AES1_PRO_MODE_STEREOPHONIC; }
    }
}

#[cfg(feature = "SND_CS8404")]
pub unsafe fn snd_cs8404_encode_spdif_bits(diga: *mut snd_aes_iec958) -> u8 {
    let mut bits: u8;
    if (*diga).status[0] & IEC958_AES0_PROFESSIONAL == 0 {
        bits = 0x10;
        if (*diga).status[0] & IEC958_AES0_CON_NOT_COPYRIGHT == 0 { bits |= 0x20; }
        if (*diga).status[0] & IEC958_AES0_CON_EMPHASIS == IEC958_AES0_CON_EMPHASIS_NONE { bits |= 0x40; }
        if (*diga).status[1] & IEC958_AES1_CON_ORIGINAL == 0 { bits |= 0x80; }
        if (*diga).status[1] & IEC958_AES1_CON_CATEGORY == IEC958_AES1_CON_GENERAL { bits |= 0x03; }
        match (*diga).status[3] & IEC958_AES3_CON_FS { IEC958_AES3_CON_FS_48000 => bits |= 0x04, IEC958_AES3_CON_FS_32000 => bits |= 0x02, _ => bits |= 0x06 }
    } else {
        bits = 0;
        if (*diga).status[0] & IEC958_AES0_NONAUDIO == 0 { bits |= 0x04; }
        match (*diga).status[0] & IEC958_AES0_PRO_FS { IEC958_AES0_PRO_FS_44100 => bits |= 0x40, IEC958_AES0_PRO_FS_48000 => bits |= 0x20, _ => {} }
        match (*diga).status[0] & IEC958_AES0_PRO_EMPHASIS { IEC958_AES0_PRO_EMPHASIS_NONE => bits |= 0x02, IEC958_AES0_PRO_EMPHASIS_5015 => bits |= 0x01, IEC958_AES0_PRO_EMPHASIS_NOTID => bits |= 0x03, _ => {} }
        match (*diga).status[1] & IEC958_AES1_PRO_MODE { IEC958_AES1_PRO_MODE_TWO | IEC958_AES1_PRO_MODE_STEREOPHONIC => {}, _ => bits |= 0x80 }
    }
    bits
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
