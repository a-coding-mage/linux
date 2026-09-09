/* SPDX-License-Identifier: GPL-2.0 */
/* HD-audio regmap helpers */

/* Dependencies supplied by the surrounding translation unit. */

pub const AC_AMP_FAKE_MUTE: u32 = 0x10;

extern "C" {
    pub fn snd_hdac_regmap_init(codec: *mut hdac_device) -> ::core::ffi::c_int;
    pub fn snd_hdac_regmap_exit(codec: *mut hdac_device);
    pub fn snd_hdac_regmap_add_vendor_verb(codec: *mut hdac_device, verb: u32) -> ::core::ffi::c_int;
    pub fn snd_hdac_regmap_read_raw(codec: *mut hdac_device, reg: u32, val: *mut u32) -> ::core::ffi::c_int;
    pub fn snd_hdac_regmap_read_raw_uncached(codec: *mut hdac_device, reg: u32, val: *mut u32) -> ::core::ffi::c_int;
    pub fn snd_hdac_regmap_write_raw(codec: *mut hdac_device, reg: u32, val: u32) -> ::core::ffi::c_int;
    pub fn snd_hdac_regmap_update_raw(codec: *mut hdac_device, reg: u32, mask: u32, val: u32) -> ::core::ffi::c_int;
    pub fn snd_hdac_regmap_update_raw_once(codec: *mut hdac_device, reg: u32, mask: u32, val: u32) -> ::core::ffi::c_int;
    pub fn snd_hdac_regmap_sync(codec: *mut hdac_device);
}

#[inline]
pub const fn snd_hdac_regmap_encode_verb(nid: u32, verb: u32) -> u32 {
    (verb << 8) | 0x80000 | (nid << 20)
}

#[inline]
pub fn snd_hdac_regmap_encode_amp(nid: u32, ch: i32, dir: i32, idx: u32) -> u32 {
    snd_hdac_regmap_encode_verb(nid, AC_VERB_GET_AMP_GAIN_MUTE)
        | if ch != 0 { AC_AMP_GET_RIGHT } else { AC_AMP_GET_LEFT }
        | if dir == HDA_OUTPUT { AC_AMP_GET_OUTPUT } else { AC_AMP_GET_INPUT }
        | idx
}

#[inline]
pub fn snd_hdac_regmap_encode_amp_stereo(nid: u32, dir: i32, idx: u32) -> u32 {
    snd_hdac_regmap_encode_verb(nid, AC_VERB_GET_AMP_GAIN_MUTE)
        | AC_AMP_SET_LEFT | AC_AMP_SET_RIGHT
        | if dir == HDA_OUTPUT { AC_AMP_GET_OUTPUT } else { AC_AMP_GET_INPUT }
        | idx
}

#[inline]
pub unsafe fn snd_hdac_regmap_write(codec: *mut hdac_device, nid: u32, verb: u32, val: u32) -> i32 {
    let cmd = snd_hdac_regmap_encode_verb(nid, verb);
    snd_hdac_regmap_write_raw(codec, cmd, val)
}

#[inline]
pub unsafe fn snd_hdac_regmap_update(codec: *mut hdac_device, nid: u32, verb: u32, mask: u32, val: u32) -> i32 {
    let cmd = snd_hdac_regmap_encode_verb(nid, verb);
    snd_hdac_regmap_update_raw(codec, cmd, mask, val)
}

#[inline]
pub unsafe fn snd_hdac_regmap_read(codec: *mut hdac_device, nid: u32, verb: u32, val: *mut u32) -> i32 {
    let cmd = snd_hdac_regmap_encode_verb(nid, verb);
    snd_hdac_regmap_read_raw(codec, cmd, val)
}

#[inline]
pub unsafe fn snd_hdac_regmap_get_amp(codec: *mut hdac_device, nid: u32, ch: i32, dir: i32, idx: i32) -> i32 {
    let cmd = snd_hdac_regmap_encode_amp(nid, ch, dir, idx as u32);
    let mut val: u32 = 0;
    let err = snd_hdac_regmap_read_raw(codec, cmd, &mut val);
    if err < 0 { err } else { val as i32 }
}

#[inline]
pub unsafe fn snd_hdac_regmap_update_amp(codec: *mut hdac_device, nid: u32, ch: i32, dir: i32, idx: i32, mask: i32, val: i32) -> i32 {
    let cmd = snd_hdac_regmap_encode_amp(nid, ch, dir, idx as u32);
    snd_hdac_regmap_update_raw(codec, cmd, mask as u32, val as u32)
}

#[inline]
pub unsafe fn snd_hdac_regmap_get_amp_stereo(codec: *mut hdac_device, nid: u32, dir: i32, idx: i32) -> i32 {
    let cmd = snd_hdac_regmap_encode_amp_stereo(nid, dir, idx as u32);
    let mut val: u32 = 0;
    let err = snd_hdac_regmap_read_raw(codec, cmd, &mut val);
    if err < 0 { err } else { val as i32 }
}

#[inline]
pub unsafe fn snd_hdac_regmap_update_amp_stereo(codec: *mut hdac_device, nid: u32, dir: i32, idx: i32, mask: i32, val: i32) -> i32 {
    let cmd = snd_hdac_regmap_encode_amp_stereo(nid, dir, idx as u32);
    snd_hdac_regmap_update_raw(codec, cmd, mask as u32, val as u32)
}

#[inline]
pub unsafe fn snd_hdac_regmap_sync_node(codec: *mut hdac_device, nid: u32) {
    regcache_mark_dirty((*codec).regmap);
    regcache_sync_region((*codec).regmap, nid << 20, ((nid + 1) << 20) - 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
