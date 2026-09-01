/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright(c) 2021-2022 Intel Corporation
 *
 * Author: Cezary Rojewski <cezary.rojewski@intel.com>
 */

// C header guard SND_SOC_CODECS_HDA_H omitted in Rust.

pub unsafe fn hda_codec_is_display(codec: *const hda_codec) -> bool {
    ((((*codec).core.vendor_id >> 16) & 0xFFFF) == 0x8086)
}

extern "C" {
    pub static snd_soc_hda_codec_dai_ops: snd_soc_dai_ops;

    pub static soc_hda_ext_bus_ops: hdac_ext_bus_ops;
    pub fn hda_codec_probe_complete(codec: *mut hda_codec) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
