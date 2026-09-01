// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2020 Intel Corporation

/*
 *  sof_sdw - ASOC Machine driver for Intel SoundWire platforms
 *
 * Rust source-level translation of soc/intel/boards/sof_sdw.c.
 * C include dependencies are intentionally left as external Rust dependencies.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

static mut sof_sdw_quirk: c_ulong = RT711_JD1 as c_ulong;
static mut quirk_override: c_int = -1;
// module_param_named(quirk, quirk_override, int, 0444);
// MODULE_PARM_DESC(quirk, "Board-specific quirk override");

const DMIC_DEFAULT_CHANNELS: c_int = 2;

unsafe fn log_quirks(dev: *mut device) {
    if SOC_SDW_JACK_JDSRC(sof_sdw_quirk) != 0 {
        dev_dbg(
            dev,
            c"quirk realtek,jack-detect-source %ld\n".as_ptr(),
            SOC_SDW_JACK_JDSRC(sof_sdw_quirk),
        );
    }
    if sof_sdw_quirk & SOC_SDW_FOUR_SPK as c_ulong != 0 {
        dev_err(dev, c"quirk SOC_SDW_FOUR_SPK enabled but no longer supported\n".as_ptr());
    }
    if sof_sdw_quirk & SOF_SDW_TGL_HDMI as c_ulong != 0 {
        dev_dbg(dev, c"quirk SOF_SDW_TGL_HDMI enabled\n".as_ptr());
    }
    if sof_sdw_quirk & SOC_SDW_PCH_DMIC as c_ulong != 0 {
        dev_dbg(dev, c"quirk SOC_SDW_PCH_DMIC enabled\n".as_ptr());
    }
    if SOF_SSP_GET_PORT(sof_sdw_quirk) != 0 {
        dev_dbg(dev, c"SSP port %ld\n".as_ptr(), SOF_SSP_GET_PORT(sof_sdw_quirk));
    }
    if sof_sdw_quirk & SOC_SDW_NO_AGGREGATION as c_ulong != 0 {
        dev_err(dev, c"quirk SOC_SDW_NO_AGGREGATION enabled but no longer supported\n".as_ptr());
    }
    if sof_sdw_quirk & SOC_SDW_CODEC_SPKR as c_ulong != 0 {
        dev_dbg(dev, c"quirk SOC_SDW_CODEC_SPKR enabled\n".as_ptr());
    }
    if sof_sdw_quirk & SOC_SDW_SIDECAR_AMPS as c_ulong != 0 {
        dev_dbg(dev, c"quirk SOC_SDW_SIDECAR_AMPS enabled\n".as_ptr());
    }
    if sof_sdw_quirk & SOC_SDW_CODEC_MIC as c_ulong != 0 {
        dev_dbg(dev, c"quirk SOC_SDW_CODEC_MIC enabled\n".as_ptr());
    }
}

unsafe extern "C" fn sof_sdw_quirk_cb(id: *const dmi_system_id) -> c_int {
    sof_sdw_quirk = (*id).driver_data as c_ulong;
    1
}

/*
 * static const struct dmi_system_id sof_sdw_quirk_table[] translates to a
 * Rust static using the same external DMI_MATCH/DMI_EXACT_MATCH semantics.
 * The entries are intentionally macro-shaped because dmi_system_id and match
 * layout are supplied by external kernel bindings.
 */
static sof_sdw_quirk_table: &[dmi_system_id] = &[
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Intel Corporation"), DMI_MATCH(DMI_PRODUCT_NAME, c"CometLake Client")], SOC_SDW_PCH_DMIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"09C6")], RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0983")], RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"098F")], RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0990")], RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Intel Corporation"), DMI_MATCH(DMI_PRODUCT_NAME, c"Ice Lake Client")], SOC_SDW_PCH_DMIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Intel Corporation"), DMI_MATCH(DMI_PRODUCT_NAME, c"Tiger Lake Client Platform")], SOF_SDW_TGL_HDMI | RT711_JD1 | SOC_SDW_PCH_DMIC | SOF_SSP_PORT(SOF_I2S_SSP2)),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0A3E")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0A3F")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0A5D")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0A5E")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Google"), DMI_MATCH(DMI_PRODUCT_NAME, c"Volteer")], SOF_SDW_TGL_HDMI | SOC_SDW_PCH_DMIC | SOF_BT_OFFLOAD_SSP(2) | SOF_SSP_BT_OFFLOAD_PRESENT),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Google"), DMI_MATCH(DMI_PRODUCT_NAME, c"Ripto")], SOF_SDW_TGL_HDMI | SOC_SDW_PCH_DMIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"HP"), DMI_MATCH(DMI_PRODUCT_NAME, c"HP Spectre x360 Conv")], SOF_SDW_TGL_HDMI | SOC_SDW_PCH_DMIC | RT711_JD1),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"HP"), DMI_MATCH(DMI_BOARD_NAME, c"8709")], SOF_SDW_TGL_HDMI | SOC_SDW_PCH_DMIC | RT711_JD1),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Intel(R) Client Systems"), DMI_MATCH(DMI_PRODUCT_NAME, c"LAPBC")], SOF_SDW_TGL_HDMI | SOC_SDW_PCH_DMIC | RT711_JD1),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_BOARD_VENDOR, c"Intel Corporation"), DMI_MATCH(DMI_BOARD_NAME, c"LAPBC710")], SOF_SDW_TGL_HDMI | SOC_SDW_PCH_DMIC | RT711_JD1),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Avell High Performance"), DMI_MATCH(DMI_PRODUCT_NAME, c"B.ON")], SOF_SDW_TGL_HDMI | SOC_SDW_PCH_DMIC | RT711_JD1),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Intel(R) Client Systems"), DMI_MATCH(DMI_PRODUCT_NAME, c"LAPRC")], SOF_SDW_TGL_HDMI | SOC_SDW_PCH_DMIC | RT711_JD2_100K),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_BOARD_VENDOR, c"Intel Corporation"), DMI_MATCH(DMI_BOARD_NAME, c"LAPRC710")], SOF_SDW_TGL_HDMI | SOC_SDW_PCH_DMIC | RT711_JD2_100K),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0A32")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0A45")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Intel Corporation"), DMI_MATCH(DMI_PRODUCT_NAME, c"Alder Lake Client Platform")], RT711_JD2_100K | SOF_SDW_TGL_HDMI | SOF_BT_OFFLOAD_SSP(2) | SOF_SSP_BT_OFFLOAD_PRESENT),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_BOARD_VENDOR, c"Intel Corporation"), DMI_MATCH(DMI_PRODUCT_SKU, c"0000000000070000")], SOF_SDW_TGL_HDMI | RT711_JD2_100K),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Google"), DMI_MATCH(DMI_PRODUCT_NAME, c"Brya")], SOF_SDW_TGL_HDMI | SOC_SDW_PCH_DMIC | SOF_BT_OFFLOAD_SSP(2) | SOF_SSP_BT_OFFLOAD_PRESENT),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0AF0")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0AF3")], SOF_SDW_TGL_HDMI),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0AFE")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0AFF")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0B00")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0B01")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0B11")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0B12")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0B13")], SOF_SDW_TGL_HDMI),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0B14")], SOF_SDW_TGL_HDMI),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0B29")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0B34")], SOF_SDW_TGL_HDMI),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0B8C")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"HP"), DMI_MATCH(DMI_PRODUCT_NAME, c"OMEN by HP Gaming Laptop 16")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0BDA")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0C0F")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0C10")], SOF_SDW_TGL_HDMI),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0C11")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0C40")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0C4F")], SOF_SDW_TGL_HDMI | RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CF6")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CF9")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CFA")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_PRODUCT_FAMILY, c"Intel_mtlrvp")], RT711_JD1),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Intel Corporation"), DMI_MATCH(DMI_PRODUCT_NAME, c"Meteor Lake Client Platform")], RT711_JD2_100K),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Google"), DMI_MATCH(DMI_PRODUCT_NAME, c"Rex")], SOC_SDW_PCH_DMIC | SOF_BT_OFFLOAD_SSP(1) | SOF_SSP_BT_OFFLOAD_PRESENT),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"HP"), DMI_MATCH(DMI_PRODUCT_NAME, c"OMEN Transcend Gaming Laptop")], RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Intel Corporation"), DMI_MATCH(DMI_PRODUCT_NAME, c"Lunar Lake Client Platform")], RT711_JD2),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CE3")], SOC_SDW_SIDECAR_AMPS),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CE4")], SOC_SDW_SIDECAR_AMPS),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CDB")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CDC")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CDD")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0D36")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CF8")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"LENOVO"), DMI_MATCH(DMI_PRODUCT_NAME, c"83JX")], SOC_SDW_SIDECAR_AMPS | SOC_SDW_CODEC_MIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"LENOVO"), DMI_MATCH(DMI_PRODUCT_NAME, c"83LC")], SOC_SDW_SIDECAR_AMPS | SOC_SDW_CODEC_MIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"LENOVO"), DMI_MATCH(DMI_PRODUCT_NAME, c"83MC")], SOC_SDW_SIDECAR_AMPS | SOC_SDW_CODEC_MIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"LENOVO"), DMI_MATCH(DMI_PRODUCT_NAME, c"83NM")], SOC_SDW_SIDECAR_AMPS | SOC_SDW_CODEC_MIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"LENOVO"), DMI_MATCH(DMI_PRODUCT_NAME, c"83HM")], SOC_SDW_SIDECAR_AMPS | SOC_SDW_CODEC_MIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"LENOVO"), DMI_MATCH(DMI_PRODUCT_NAME, c"21QB")], SOC_SDW_CODEC_MIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"LENOVO"), DMI_MATCH(DMI_PRODUCT_NAME, c"21QA")], SOC_SDW_CODEC_MIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"LENOVO"), DMI_MATCH(DMI_PRODUCT_NAME, c"21Q6")], SOC_SDW_SIDECAR_AMPS | SOC_SDW_CODEC_MIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"LENOVO"), DMI_MATCH(DMI_PRODUCT_NAME, c"21Q7")], SOC_SDW_SIDECAR_AMPS | SOC_SDW_CODEC_MIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CE8")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CF1")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CF7")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CF0")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CF3")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CF4")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CF5")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Alienware"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CCC")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Alienware"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0CCD")], SOC_SDW_CODEC_SPKR),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Dell Inc"), DMI_EXACT_MATCH(DMI_PRODUCT_SKU, c"0DD6")], SOC_SDW_SIDECAR_AMPS),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_PRODUCT_FAMILY, c"Intel_ptlrvp")], SOC_SDW_PCH_DMIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Google"), DMI_MATCH(DMI_PRODUCT_NAME, c"Lapis")], SOC_SDW_CODEC_SPKR | SOC_SDW_PCH_DMIC | SOF_BT_OFFLOAD_SSP(2) | SOF_SSP_BT_OFFLOAD_PRESENT),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Google"), DMI_MATCH(DMI_PRODUCT_NAME, c"Francka")], SOC_SDW_CODEC_SPKR | SOC_SDW_PCH_DMIC | SOF_BT_OFFLOAD_SSP(2) | SOF_SSP_BT_OFFLOAD_PRESENT),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Google"), DMI_MATCH(DMI_PRODUCT_FAMILY, c"Google_Fatcat")], SOC_SDW_PCH_DMIC | SOF_BT_OFFLOAD_SSP(2) | SOF_SSP_BT_OFFLOAD_PRESENT),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_PRODUCT_FAMILY, c"Intel_wclrvp")], SOC_SDW_PCH_DMIC),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_SYS_VENDOR, c"Google"), DMI_MATCH(DMI_PRODUCT_NAME, c"Ocelot")], SOC_SDW_PCH_DMIC | SOF_BT_OFFLOAD_SSP(2) | SOF_SSP_BT_OFFLOAD_PRESENT),
    dmi_system_id_entry!(sof_sdw_quirk_cb, [DMI_MATCH(DMI_PRODUCT_FAMILY, c"Intel_nvlrvp")], SOC_SDW_PCH_DMIC),
    dmi_system_id_terminator!(),
];

static sof_sdw_ssid_quirk_table: &[snd_pci_quirk] = &[
    SND_PCI_QUIRK(0x1028, 0x0e53, c"Dell XPS WCL", SOC_SDW_SIDECAR_AMPS),
    SND_PCI_QUIRK(0x1028, 0x0e54, c"Dell XPS PTL", SOC_SDW_SIDECAR_AMPS),
    SND_PCI_QUIRK(0x1043, 0x1e13, c"ASUS Zenbook S14", SOC_SDW_CODEC_MIC),
    SND_PCI_QUIRK(0x1043, 0x1f43, c"ASUS Zenbook S16", SOC_SDW_CODEC_MIC),
    SND_PCI_QUIRK(0x17aa, 0x2347, c"Lenovo P16", SOC_SDW_CODEC_MIC),
    SND_PCI_QUIRK(0x17aa, 0x2348, c"Lenovo P16", SOC_SDW_CODEC_MIC),
    SND_PCI_QUIRK(0x17aa, 0x2349, c"Lenovo P1", SOC_SDW_CODEC_MIC),
    SND_PCI_QUIRK(0x17aa, 0x3821, c"Lenovo 0x3821", SOC_SDW_SIDECAR_AMPS),
    SND_PCI_QUIRK(0x17aa, 0x383c, c"Lenovo 0x383c", SOC_SDW_SIDECAR_AMPS),
    snd_pci_quirk_terminator!(),
];

unsafe fn sof_sdw_check_ssid_quirk(mach: *const snd_soc_acpi_mach) {
    let quirk_entry = snd_pci_quirk_lookup_id(
        (*mach).mach_params.subsystem_vendor,
        (*mach).mach_params.subsystem_device,
        sof_sdw_ssid_quirk_table.as_ptr(),
    );

    if !quirk_entry.is_null() {
        sof_sdw_quirk = (*quirk_entry).value as c_ulong;
    }
}

static sdw_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(asoc_sdw_startup),
    prepare: Some(asoc_sdw_prepare),
    trigger: Some(asoc_sdw_trigger),
    hw_params: Some(asoc_sdw_hw_params),
    hw_free: Some(asoc_sdw_hw_free),
    shutdown: Some(asoc_sdw_shutdown),
};

static type_strings: [*const c_char; 3] = [
    c"SimpleJack".as_ptr(),
    c"SmartAmp".as_ptr(),
    c"SmartMic".as_ptr(),
];

unsafe fn create_sdw_dailink(
    card: *mut snd_soc_card,
    mut sof_dai: *mut asoc_sdw_dailink,
    dai_links: *mut *mut snd_soc_dai_link,
    be_id: *mut c_int,
    codec_conf: *mut *mut snd_soc_codec_conf,
) -> c_int {
    let dev = (*card).dev;
    let mach = dev_get_platdata((*card).dev) as *mut snd_soc_acpi_mach;
    let ctx = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let mach_params = &mut (*mach).mach_params as *mut snd_soc_acpi_mach_params;
    let intel_ctx = (*ctx).private as *mut intel_mc_ctx;
    let mut sof_end: *mut asoc_sdw_endpoint;
    let mut stream: c_int = 0;
    let mut ret: c_int;

    list_for_each_entry!(sof_end, &mut (*sof_dai).endpoints, list, {
        if !(*sof_end).name_prefix.is_null() {
            (**codec_conf).dlc.name = (*sof_end).codec_name;
            (**codec_conf).name_prefix = (*sof_end).name_prefix;
            *codec_conf = (*codec_conf).add(1);
        }

        if (*sof_end).include_sidecar && !(*(*sof_end).codec_info).add_sidecar.is_none() {
            ret = ((*(*sof_end).codec_info).add_sidecar.unwrap())(card, dai_links, codec_conf);
            if ret != 0 {
                return ret;
            }
        }
    });

    /*
     * The dai_type is used to select function topologies. Since the topology stream name
     * and DAI link name use partial matching, unconditionally appending the dai_type provides
     * necessary selection metadata without breaking existing topologies. Although
     * ctx->append_dai_type is not checked here, we overwrite it to ensure consistency in case
     * it is referenced elsewhere.
     */
    (*ctx).append_dai_type = true;
    while stream < 2 {
        static sdw_stream_name: [*const c_char; 2] = [
            c"SDW%d-Playback-%s".as_ptr(),
            c"SDW%d-Capture-%s".as_ptr(),
        ];
        let mut codec_maps: *mut snd_soc_dai_link_ch_map;
        let mut codecs: *mut snd_soc_dai_link_component;
        let mut cpus: *mut snd_soc_dai_link_component;
        let mut platform: *mut snd_soc_dai_link_component;
        let num_cpus = hweight32((*sof_dai).link_mask[stream as usize]) as c_int;
        let num_codecs = (*sof_dai).num_devs[stream as usize];
        let playback: c_int;
        let capture: c_int;
        let mut cur_link: c_int = 0;
        let mut i: c_int = 0;
        let mut j: c_int = 0;
        let name: *mut c_char;

        if (*sof_dai).num_devs[stream as usize] == 0 {
            stream += 1;
            continue;
        }

        sof_end = list_first_entry!(&mut (*sof_dai).endpoints, asoc_sdw_endpoint, list);

        *be_id = (*(*sof_end).dai_info).dailink[stream as usize];
        if *be_id < 0 {
            dev_err(dev, c"Invalid dailink id %d\n".as_ptr(), *be_id);
            return -EINVAL;
        }

        name = devm_kasprintf(
            dev,
            GFP_KERNEL,
            sdw_stream_name[stream as usize],
            ffs((*sof_end).link_mask) - 1,
            type_strings[(*(*sof_end).dai_info).dai_type as usize],
        );
        if name.is_null() {
            return -ENOMEM;
        }

        cpus = devm_kcalloc(dev, num_cpus as usize, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL) as *mut _;
        if cpus.is_null() {
            return -ENOMEM;
        }

        codecs = devm_kcalloc(dev, num_codecs as usize, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL) as *mut _;
        if codecs.is_null() {
            return -ENOMEM;
        }

        platform = devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL) as *mut _;
        if platform.is_null() {
            return -ENOMEM;
        }

        codec_maps = devm_kcalloc(dev, num_codecs as usize, size_of::<snd_soc_dai_link_ch_map>(), GFP_KERNEL) as *mut _;
        if codec_maps.is_null() {
            return -ENOMEM;
        }

        list_for_each_entry!(sof_end, &mut (*sof_dai).endpoints, list, {
            if (*(*sof_end).dai_info).direction[stream as usize] == 0 {
                continue;
            }

            if cur_link != (*sof_end).link_mask {
                let link_num = ffs((*sof_end).link_mask) - 1;
                let pin_num = {
                    let p = (*intel_ctx).sdw_pin_index[link_num as usize];
                    (*intel_ctx).sdw_pin_index[link_num as usize] = p + 1;
                    p
                };

                cur_link = (*sof_end).link_mask;

                (*cpus.add(i as usize)).dai_name = devm_kasprintf(
                    dev,
                    GFP_KERNEL,
                    c"SDW%d Pin%d".as_ptr(),
                    link_num,
                    pin_num,
                );
                if (*cpus.add(i as usize)).dai_name.is_null() {
                    return -ENOMEM;
                }
                i += 1;
            }

            (*codec_maps.add(j as usize)).cpu = i - 1;
            (*codec_maps.add(j as usize)).codec = j;

            (*codecs.add(j as usize)).name = (*sof_end).codec_name;
            (*codecs.add(j as usize)).dai_name = (*(*sof_end).dai_info).dai_name;
            if (*(*sof_end).dai_info).dai_type == SOC_SDW_DAI_TYPE_MIC
                && (*mach_params).dmic_num > 0
            {
                dev_warn(
                    dev,
                    c"Both SDW DMIC and PCH DMIC are present, if incorrect, please set kernel params snd_sof_intel_hda_generic dmic_num=0 to disable PCH DMIC\n".as_ptr(),
                );
            }
            j += 1;
        });

        WARN_ON(i != num_cpus || j != num_codecs);

        playback = (stream == SNDRV_PCM_STREAM_PLAYBACK) as c_int;
        capture = (stream == SNDRV_PCM_STREAM_CAPTURE) as c_int;

        asoc_sdw_init_dai_link(
            dev,
            *dai_links,
            be_id,
            name,
            playback,
            capture,
            cpus,
            num_cpus,
            platform,
            1,
            codecs,
            num_codecs,
            1,
            Some(asoc_sdw_rtd_init),
            &sdw_ops,
        );

        /*
         * SoundWire DAILINKs use 'stream' functions and Bank Switch operations
         * based on wait_for_completion(), tag them as 'nonatomic'.
         */
        (**dai_links).nonatomic = true;
        (**dai_links).ch_maps = codec_maps;

        list_for_each_entry!(sof_end, &mut (*sof_dai).endpoints, list, {
            if let Some(init) = (*(*sof_end).dai_info).init {
                init(card, *dai_links, (*sof_end).codec_info, playback);
            }
        });

        *dai_links = (*dai_links).add(1);
        stream += 1;
    }

    0
}

unsafe fn create_sdw_dailinks(
    card: *mut snd_soc_card,
    dai_links: *mut *mut snd_soc_dai_link,
    be_id: *mut c_int,
    mut sof_dais: *mut asoc_sdw_dailink,
    codec_conf: *mut *mut snd_soc_codec_conf,
) -> c_int {
    let ctx = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let intel_ctx = (*ctx).private as *mut intel_mc_ctx;
    let mut i: c_int = 0;

    while i < SDW_INTEL_MAX_LINKS {
        (*intel_ctx).sdw_pin_index[i as usize] = SOC_SDW_INTEL_BIDIR_PDI_BASE;
        i += 1;
    }

    /* generate DAI links by each sdw link */
    while (*sof_dais).initialised {
        let mut current_be_id: c_int = 0;

        let ret = create_sdw_dailink(card, sof_dais, dai_links, &mut current_be_id, codec_conf);
        if ret != 0 {
            return ret;
        }

        /* Update the be_id to match the highest ID used for SDW link */
        if *be_id < current_be_id {
            *be_id = current_be_id;
        }

        sof_dais = sof_dais.add(1);
    }

    0
}

unsafe fn create_ssp_dailinks(
    card: *mut snd_soc_card,
    dai_links: *mut *mut snd_soc_dai_link,
    be_id: *mut c_int,
    ssp_info: *mut asoc_sdw_codec_info,
    mut ssp_mask: c_ulong,
) -> c_int {
    let dev = (*card).dev;
    let mut i: c_ulong = 0;
    let mut j: c_int = 0;

    while i < BITS_PER_TYPE_c_ulong() {
        if ssp_mask & (1_ulong << i) == 0 {
            i += 1;
            continue;
        }

        let name = devm_kasprintf(dev, GFP_KERNEL, c"SSP%d-Codec".as_ptr(), i as c_int);
        let cpu_dai_name = devm_kasprintf(dev, GFP_KERNEL, c"SSP%d Pin".as_ptr(), i as c_int);
        let codec_name = devm_kasprintf(
            dev,
            GFP_KERNEL,
            c"i2c-%s:0%d".as_ptr(),
            (*ssp_info).acpi_id,
            j,
        );
        j += 1;
        if name.is_null() || cpu_dai_name.is_null() || codec_name.is_null() {
            return -ENOMEM;
        }

        let playback = (*ssp_info).dais[0].direction[SNDRV_PCM_STREAM_PLAYBACK as usize];
        let capture = (*ssp_info).dais[0].direction[SNDRV_PCM_STREAM_CAPTURE as usize];

        let mut ret = asoc_sdw_init_simple_dai_link(
            dev,
            *dai_links,
            be_id,
            name,
            playback,
            capture,
            cpu_dai_name,
            c"dummy".as_ptr(),
            codec_name,
            (*ssp_info).dais[0].dai_name,
            1,
            None,
            (*ssp_info).ops,
        );
        if ret != 0 {
            return ret;
        }

        ret = ((*ssp_info).dais[0].init.unwrap())(card, *dai_links, ssp_info, 0);
        if ret < 0 {
            return ret;
        }

        *dai_links = (*dai_links).add(1);
        i += 1;
    }

    0
}

unsafe fn create_dmic_dailinks(
    card: *mut snd_soc_card,
    dai_links: *mut *mut snd_soc_dai_link,
    be_id: *mut c_int,
) -> c_int {
    let dev = (*card).dev;

    let mut ret = asoc_sdw_init_simple_dai_link(
        dev,
        *dai_links,
        be_id,
        c"dmic01".as_ptr(),
        0,
        1, // DMIC only supports capture
        c"DMIC01 Pin".as_ptr(),
        c"dummy".as_ptr(),
        c"dmic-codec".as_ptr(),
        c"dmic-hifi".as_ptr(),
        1,
        Some(asoc_sdw_dmic_init),
        None,
    );
    if ret != 0 {
        return ret;
    }

    *dai_links = (*dai_links).add(1);

    ret = asoc_sdw_init_simple_dai_link(
        dev,
        *dai_links,
        be_id,
        c"dmic16k".as_ptr(),
        0,
        1, // DMIC only supports capture
        c"DMIC16k Pin".as_ptr(),
        c"dummy".as_ptr(),
        c"dmic-codec".as_ptr(),
        c"dmic-hifi".as_ptr(),
        1,
        /* don't call asoc_sdw_dmic_init() twice */
        None,
        None,
    );
    if ret != 0 {
        return ret;
    }

    *dai_links = (*dai_links).add(1);
    0
}

unsafe fn create_hdmi_dailinks(
    card: *mut snd_soc_card,
    dai_links: *mut *mut snd_soc_dai_link,
    be_id: *mut c_int,
    hdmi_num: c_int,
) -> c_int {
    let dev = (*card).dev;
    let ctx = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let intel_ctx = (*ctx).private as *mut intel_mc_ctx;
    let mut i = 0;

    while i < hdmi_num {
        let name = devm_kasprintf(dev, GFP_KERNEL, c"iDisp%d".as_ptr(), i + 1);
        let cpu_dai_name = devm_kasprintf(dev, GFP_KERNEL, c"iDisp%d Pin".as_ptr(), i + 1);
        if name.is_null() || cpu_dai_name.is_null() {
            return -ENOMEM;
        }

        let codec_name: *const c_char;
        let codec_dai_name: *mut c_char;

        if (*intel_ctx).hdmi.idisp_codec {
            codec_name = c"ehdaudio0D2".as_ptr();
            codec_dai_name = devm_kasprintf(dev, GFP_KERNEL, c"intel-hdmi-hifi%d".as_ptr(), i + 1);
        } else {
            codec_name = c"snd-soc-dummy".as_ptr();
            codec_dai_name = c"snd-soc-dummy-dai".as_ptr() as *mut c_char;
        }

        if codec_dai_name.is_null() {
            return -ENOMEM;
        }

        let ret = asoc_sdw_init_simple_dai_link(
            dev,
            *dai_links,
            be_id,
            name,
            1,
            0, // HDMI only supports playback
            cpu_dai_name,
            c"dummy".as_ptr(),
            codec_name,
            codec_dai_name,
            1,
            if i == 0 { Some(sof_sdw_hdmi_init) } else { None },
            None,
        );
        if ret != 0 {
            return ret;
        }

        *dai_links = (*dai_links).add(1);
        i += 1;
    }

    0
}

unsafe fn create_bt_dailinks(
    card: *mut snd_soc_card,
    dai_links: *mut *mut snd_soc_dai_link,
    be_id: *mut c_int,
) -> c_int {
    let dev = (*card).dev;
    let mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
    let port: c_int;

    if sof_sdw_quirk & SOF_SSP_BT_OFFLOAD_PRESENT as c_ulong != 0 {
        port = ((sof_sdw_quirk & SOF_BT_OFFLOAD_SSP_MASK as c_ulong) >> SOF_BT_OFFLOAD_SSP_SHIFT) as c_int;
    } else {
        port = fls((*mach).mach_params.bt_link_mask) - 1;
    }

    let name = devm_kasprintf(dev, GFP_KERNEL, c"SSP%d-BT".as_ptr(), port);
    let cpu_dai_name = devm_kasprintf(dev, GFP_KERNEL, c"SSP%d Pin".as_ptr(), port);
    if name.is_null() || cpu_dai_name.is_null() {
        return -ENOMEM;
    }

    let ret = asoc_sdw_init_simple_dai_link(
        dev,
        *dai_links,
        be_id,
        name,
        1,
        1,
        cpu_dai_name,
        c"dummy".as_ptr(),
        snd_soc_dummy_dlc.name,
        snd_soc_dummy_dlc.dai_name,
        1,
        None,
        None,
    );
    if ret != 0 {
        return ret;
    }

    *dai_links = (*dai_links).add(1);
    0
}

unsafe fn create_echoref_dailink(
    card: *mut snd_soc_card,
    dai_links: *mut *mut snd_soc_dai_link,
    be_id: *mut c_int,
) -> c_int {
    let dev = (*card).dev;
    let name = devm_kasprintf(dev, GFP_KERNEL, c"Loopback_Virtual".as_ptr());

    if name.is_null() {
        return -ENOMEM;
    }

    /*
     * use dummy DAI names as this won't be connected to an actual DAI but just to establish a
     * fe <-> be connection for loopback capture for echo reference
     */
    let ret = asoc_sdw_init_simple_dai_link(
        dev,
        *dai_links,
        be_id,
        name,
        0,
        1,
        c"Loopback Virtual Pin".as_ptr(),
        c"dummy".as_ptr(),
        snd_soc_dummy_dlc.name,
        snd_soc_dummy_dlc.dai_name,
        1,
        None,
        None,
    );
    if ret != 0 {
        return ret;
    }

    *dai_links = (*dai_links).add(1);

    dev_dbg(dev, c"Added echo reference DAI link\n".as_ptr());
    0
}

unsafe fn sof_card_dai_links_create(card: *mut snd_soc_card) -> c_int {
    let dev = (*card).dev;
    let mach = dev_get_platdata((*card).dev) as *mut snd_soc_acpi_mach;
    let mut sdw_be_num = 0;
    let mut ssp_num = 0;
    let mut dmic_num = 0;
    let mut bt_num = 0;
    let ctx = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let intel_ctx = (*ctx).private as *mut intel_mc_ctx;
    let mach_params = &mut (*mach).mach_params as *mut snd_soc_acpi_mach_params;
    let mut codec_conf: *mut snd_soc_codec_conf;
    let mut ssp_info: *mut asoc_sdw_codec_info = null_mut();
    let sof_ends: *mut asoc_sdw_endpoint;
    let sof_dais: *mut asoc_sdw_dailink;
    let sof_aux: *mut snd_soc_aux_dev;
    let mut num_devs = 0;
    let mut num_ends = 0;
    let mut num_aux = 0;
    let mut num_confs: c_int;
    let mut dai_links: *mut snd_soc_dai_link;
    let num_links: c_int;
    let mut be_id = 0;
    let hdmi_num: c_int;
    let mut ssp_mask: c_ulong = 0;
    let mut ret: c_int;

    ret = asoc_sdw_count_sdw_endpoints(card, &mut num_devs, &mut num_ends, &mut num_aux);
    if ret < 0 {
        dev_err(dev, c"failed to count devices/endpoints: %d\n".as_ptr(), ret);
        return ret;
    }

    num_confs = num_ends;

    /*
     * One per DAI link, worst case is a DAI link for every endpoint, also
     * add one additional to act as a terminator such that code can iterate
     * until it hits an uninitialised DAI.
     */
    sof_dais = kzalloc_objs::<asoc_sdw_dailink>(num_ends + 1);
    if sof_dais.is_null() {
        return -ENOMEM;
    }

    /* One per endpoint, ie. each DAI on each codec/amp */
    sof_ends = kzalloc_objs::<asoc_sdw_endpoint>(num_ends);
    if sof_ends.is_null() {
        ret = -ENOMEM;
        goto_err_dai!(ret, sof_dais);
    }

    sof_aux = devm_kcalloc(dev, num_aux as usize, size_of::<snd_soc_aux_dev>(), GFP_KERNEL) as *mut _;
    if sof_aux.is_null() {
        ret = -ENOMEM;
        goto_err_dai!(ret, sof_dais);
    }

    ret = asoc_sdw_parse_sdw_endpoints(dev, ctx, sof_aux, sof_dais, sof_ends, &mut num_confs);
    if ret < 0 {
        kfree(sof_ends as *const c_void);
        kfree(sof_dais as *const c_void);
        return ret;
    }

    sdw_be_num = ret;

    /*
     * on generic tgl platform, I2S or sdw mode is supported
     * based on board rework. A ACPI device is registered in
     * system only when I2S mode is supported, not sdw mode.
     * Here check ACPI ID to confirm I2S is supported.
     */
    ssp_info = asoc_sdw_find_codec_info_acpi((*mach).id);
    if !ssp_info.is_null() {
        ssp_mask = SOF_SSP_GET_PORT(sof_sdw_quirk);
        ssp_num = hweight_long(ssp_mask) as c_int;
    }

    if (*mach_params).codec_mask & IDISP_CODEC_MASK != 0 {
        (*intel_ctx).hdmi.idisp_codec = true;
    }

    if sof_sdw_quirk & SOF_SDW_TGL_HDMI as c_ulong != 0 {
        hdmi_num = SOF_TGL_HDMI_COUNT;
    } else {
        hdmi_num = SOF_PRE_TGL_HDMI_COUNT;
    }

    /* enable dmic01 & dmic16k */
    if (*ctx).ignore_internal_dmic {
        dev_dbg(dev, c"SoundWire DMIC is used, ignoring internal DMIC\n".as_ptr());
        (*mach_params).dmic_num = 0;
    } else if (*mach_params).dmic_num != 0 {
        dmic_num = 2;
    } else if sof_sdw_quirk & SOC_SDW_PCH_DMIC as c_ulong != 0 {
        dmic_num = 2;
        /*
         * mach_params->dmic_num will be used to set the cfg-mics value of
         * card->components string. Set it to the default value.
         */
        (*mach_params).dmic_num = DMIC_DEFAULT_CHANNELS;
    }

    if sof_sdw_quirk & SOF_SSP_BT_OFFLOAD_PRESENT as c_ulong != 0 || (*mach_params).bt_link_mask != 0 {
        bt_num = 1;
    }

    dev_dbg(
        dev,
        c"DAI link numbers: sdw %d, ssp %d, dmic %d, hdmi %d, bt: %d\n".as_ptr(),
        sdw_be_num,
        ssp_num,
        dmic_num,
        if (*intel_ctx).hdmi.idisp_codec { hdmi_num } else { 0 },
        bt_num,
    );

    codec_conf = devm_kcalloc(dev, num_confs as usize, size_of::<snd_soc_codec_conf>(), GFP_KERNEL) as *mut _;
    if codec_conf.is_null() {
        kfree(sof_ends as *const c_void);
        kfree(sof_dais as *const c_void);
        return -ENOMEM;
    }

    /*
     * allocate BE dailinks, add an extra DAI link for echo reference capture.
     * This should be the last DAI link and it is expected both for monolithic
     * and functional SOF topologies to support echo reference.
     */
    num_links = sdw_be_num + ssp_num + dmic_num + hdmi_num + bt_num + 1;
    dai_links = devm_kcalloc(dev, num_links as usize, size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut _;
    if dai_links.is_null() {
        kfree(sof_ends as *const c_void);
        kfree(sof_dais as *const c_void);
        return -ENOMEM;
    }

    (*card).codec_conf = codec_conf;
    (*card).num_configs = num_confs;
    (*card).dai_link = dai_links;
    (*card).num_links = num_links;
    (*card).aux_dev = sof_aux;
    (*card).num_aux_devs = num_aux;

    /* SDW */
    if sdw_be_num != 0 {
        ret = create_sdw_dailinks(card, &mut dai_links, &mut be_id, sof_dais, &mut codec_conf);
        if ret != 0 {
            kfree(sof_ends as *const c_void);
            kfree(sof_dais as *const c_void);
            return ret;
        }
    }

    /* SSP */
    if ssp_num != 0 {
        ret = create_ssp_dailinks(card, &mut dai_links, &mut be_id, ssp_info, ssp_mask);
        if ret != 0 {
            kfree(sof_ends as *const c_void);
            kfree(sof_dais as *const c_void);
            return ret;
        }
    }

    /* dmic */
    if dmic_num != 0 {
        ret = create_dmic_dailinks(card, &mut dai_links, &mut be_id);
        if ret != 0 {
            kfree(sof_ends as *const c_void);
            kfree(sof_dais as *const c_void);
            return ret;
        }
    }

    /* HDMI */
    ret = create_hdmi_dailinks(card, &mut dai_links, &mut be_id, hdmi_num);
    if ret != 0 {
        kfree(sof_ends as *const c_void);
        kfree(sof_dais as *const c_void);
        return ret;
    }

    /* BT */
    if bt_num != 0 {
        ret = create_bt_dailinks(card, &mut dai_links, &mut be_id);
        if ret != 0 {
            kfree(sof_ends as *const c_void);
            kfree(sof_dais as *const c_void);
            return ret;
        }
    }

    /* dummy echo ref link. keep this as the last DAI link. The DAI link ID does not matter */
    ret = create_echoref_dailink(card, &mut dai_links, &mut be_id);
    if ret != 0 {
        dev_err(dev, c"failed to create echo ref dai link: %d\n".as_ptr(), ret);
        kfree(sof_ends as *const c_void);
        kfree(sof_dais as *const c_void);
        return ret;
    }

    WARN_ON(codec_conf != (*card).codec_conf.add((*card).num_configs as usize));
    WARN_ON(dai_links != (*card).dai_link.add((*card).num_links as usize));

    kfree(sof_ends as *const c_void);
    kfree(sof_dais as *const c_void);
    ret
}

unsafe extern "C" fn sof_sdw_card_late_probe(card: *mut snd_soc_card) -> c_int {
    let ctx = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let intel_ctx = (*ctx).private as *mut intel_mc_ctx;
    let mut ret = asoc_sdw_card_late_probe(card);
    if ret < 0 {
        return ret;
    }

    if (*intel_ctx).hdmi.idisp_codec {
        ret = sof_sdw_hdmi_card_late_probe(card);
    }

    ret
}

unsafe extern "C" fn sof_sdw_add_dai_link(
    card: *mut snd_soc_card,
    link: *mut snd_soc_dai_link,
) -> c_int {
    let ctx = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let intel_ctx = (*ctx).private as *mut intel_mc_ctx;

    /* Ignore the HDMI PCM link if iDisp is not present */
    if !strstr((*link).stream_name, c"HDMI".as_ptr()).is_null() && !(*intel_ctx).hdmi.idisp_codec {
        (*link).ignore = true;
    }

    0
}

unsafe extern "C" fn mc_probe(pdev: *mut platform_device) -> c_int {
    let mach = dev_get_platdata(&mut (*pdev).dev) as *mut snd_soc_acpi_mach;
    let card: *mut snd_soc_card;
    let ctx: *mut asoc_sdw_mc_private;
    let intel_ctx: *mut intel_mc_ctx;
    let mut amp_num = 0;
    let mut i = 0;
    let mut ret: c_int;

    dev_dbg(&mut (*pdev).dev, c"Entry\n".as_ptr());

    intel_ctx = devm_kzalloc(&mut (*pdev).dev, size_of::<intel_mc_ctx>(), GFP_KERNEL) as *mut _;
    if intel_ctx.is_null() {
        return -ENOMEM;
    }

    ctx = devm_kzalloc(&mut (*pdev).dev, size_of::<asoc_sdw_mc_private>(), GFP_KERNEL) as *mut _;
    if ctx.is_null() {
        return -ENOMEM;
    }

    (*ctx).private = intel_ctx as *mut c_void;
    (*ctx).codec_info_list_count = asoc_sdw_get_codec_info_list_count();
    card = &mut (*ctx).card;
    (*card).dev = &mut (*pdev).dev;
    (*card).name = c"soundwire".as_ptr();
    (*card).owner = THIS_MODULE;
    (*card).late_probe = Some(sof_sdw_card_late_probe);
    (*card).add_dai_link = Some(sof_sdw_add_dai_link);

    snd_soc_card_set_drvdata(card, ctx as *mut c_void);

    if (*mach).mach_params.subsystem_id_set {
        snd_soc_card_set_pci_ssid(
            card,
            (*mach).mach_params.subsystem_vendor,
            (*mach).mach_params.subsystem_device,
        );
        sof_sdw_check_ssid_quirk(mach);
    }

    dmi_check_system(sof_sdw_quirk_table.as_ptr());

    if quirk_override != -1 {
        dev_info(
            &mut (*pdev).dev,
            c"Overriding quirk 0x%lx => 0x%x\n".as_ptr(),
            sof_sdw_quirk,
            quirk_override,
        );
        sof_sdw_quirk = quirk_override as c_ulong;
    }

    log_quirks(&mut (*pdev).dev);

    (*ctx).mc_quirk = sof_sdw_quirk;
    /* reset amp_num to ensure amp_num++ starts from 0 in each probe */
    while i < (*ctx).codec_info_list_count {
        codec_info_list[i as usize].amp_num = 0;
        i += 1;
    }

    ret = sof_card_dai_links_create(card);
    if ret < 0 {
        return ret;
    }

    /*
     * the default amp_num is zero for each codec and
     * amp_num will only be increased for active amp
     * codecs on used platform
     */
    i = 0;
    while i < (*ctx).codec_info_list_count {
        amp_num += codec_info_list[i as usize].amp_num;
        i += 1;
    }

    (*card).components = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, c" cfg-amp:%d".as_ptr(), amp_num);
    if (*card).components.is_null() {
        return -ENOMEM;
    }

    if (*mach).mach_params.dmic_num != 0 {
        (*card).components = devm_kasprintf(
            &mut (*pdev).dev,
            GFP_KERNEL,
            c"%s mic:dmic cfg-mics:%d".as_ptr(),
            (*card).components,
            (*mach).mach_params.dmic_num,
        );
        if (*card).components.is_null() {
            return -ENOMEM;
        }
    }

    /* Register the card */
    ret = devm_snd_soc_register_card((*card).dev, card);
    if ret != 0 {
        dev_err_probe(&mut (*pdev).dev, ret, c"snd_soc_register_card failed %d\n".as_ptr(), ret);
        asoc_sdw_mc_dailink_exit_loop(card);
        return ret;
    }

    platform_set_drvdata(pdev, card as *mut c_void);
    ret
}

unsafe extern "C" fn mc_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    asoc_sdw_mc_dailink_exit_loop(card);
}

static mc_id_table: &[platform_device_id] = &[
    platform_device_id { name: *b"sof_sdw\0", ..platform_device_id_zeroed!() },
    platform_device_id_zeroed!(),
];
// MODULE_DEVICE_TABLE(platform, mc_id_table);

static mut sof_sdw_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"sof_sdw".as_ptr(),
        pm: unsafe { &snd_soc_pm_ops as *const _ },
        ..device_driver_zeroed!()
    },
    probe: Some(mc_probe),
    remove: Some(mc_remove),
    id_table: mc_id_table.as_ptr(),
    ..platform_driver_zeroed!()
};

// module_platform_driver(sof_sdw_driver);

// MODULE_DESCRIPTION("ASoC SoundWire Generic Machine driver");
// MODULE_AUTHOR("Bard Liao <yung-chuan.liao@linux.intel.com>");
// MODULE_AUTHOR("Rander Wang <rander.wang@linux.intel.com>");
// MODULE_AUTHOR("Pierre-Louis Bossart <pierre-louis.bossart@linux.intel.com>");
// MODULE_LICENSE("GPL v2");
// MODULE_IMPORT_NS("SND_SOC_INTEL_HDA_DSP_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SDW_UTILS");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
