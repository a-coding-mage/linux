// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Jaroslav Kysela <perex@perex.cz>

// Rust translation of hda/core/intel-dsp-config.c.
// C include dependencies are intentionally left as external Rust dependencies.

static mut dsp_driver: core::ffi::c_int = 0;

// module_param(dsp_driver, int, 0444);
// MODULE_PARM_DESC(dsp_driver, "Force the DSP driver for Intel DSP (0=auto, 1=legacy, 2=SST, 3=SOF, 4=AVS)");

const FLAG_SST: u32 = BIT(0);
const FLAG_SOF: u32 = BIT(1);
const FLAG_SST_ONLY_IF_DMIC: u32 = BIT(15);
const FLAG_SOF_ONLY_IF_DMIC: u32 = BIT(16);
const FLAG_SOF_ONLY_IF_SOUNDWIRE: u32 = BIT(17);

const FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE: u32 =
    FLAG_SOF_ONLY_IF_DMIC | FLAG_SOF_ONLY_IF_SOUNDWIRE;

#[repr(C)]
struct config_entry {
    flags: u32,
    device: u16,
    acpi_hid: [u8; ACPI_ID_LEN],
    dmi_table: *const dmi_system_id,
    codec_hid: *const snd_soc_acpi_codecs,
}

/* DMI compound literals from config_table entries.  The dmi_system_id field
 * layout and DMI_MATCH/DMI_EXACT_MATCH constructors are provided externally.
 */
static dmi_table_google_chromebooks: [dmi_system_id; 2] = [
    dmi_system_id {
        ident: b"Google Chromebooks\0".as_ptr(),
        matches: [DMI_MATCH(DMI_SYS_VENDOR, b"Google\0".as_ptr())],
    },
    dmi_system_id {},
];

static dmi_table_up_squared: [dmi_system_id; 2] = [
    dmi_system_id {
        ident: b"Up Squared\0".as_ptr(),
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"AAEON\0".as_ptr()),
            DMI_MATCH(DMI_BOARD_NAME, b"UP-APL01\0".as_ptr()),
        ],
    },
    dmi_system_id {},
];

static dmi_table_cnl_lp: [dmi_system_id; 3] = [
    dmi_system_id {
        ident: b"Google Chromebooks\0".as_ptr(),
        matches: [DMI_MATCH(DMI_SYS_VENDOR, b"Google\0".as_ptr())],
    },
    dmi_system_id {
        ident: b"UP-WHL\0".as_ptr(),
        matches: [DMI_MATCH(DMI_SYS_VENDOR, b"AAEON\0".as_ptr())],
    },
    dmi_system_id {},
];

static dmi_table_cml_lp: [dmi_system_id; 4] = [
    dmi_system_id {
        ident: core::ptr::null(),
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"Dell Inc\0".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_SKU, b"09C6\0".as_ptr()),
        ],
    },
    dmi_system_id {
        /* early version of SKU 09C6 */
        ident: core::ptr::null(),
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"Dell Inc\0".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_SKU, b"0983\0".as_ptr()),
        ],
    },
    dmi_system_id {},
];

static dmi_table_cml_h: [dmi_system_id; 3] = [
    dmi_system_id {
        ident: core::ptr::null(),
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"Dell Inc\0".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_SKU, b"098F\0".as_ptr()),
        ],
    },
    dmi_system_id {
        ident: core::ptr::null(),
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"Dell Inc\0".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_SKU, b"0990\0".as_ptr()),
        ],
    },
    dmi_system_id {},
];

static dmi_table_jsl_n: [dmi_system_id; 3] = [
    dmi_system_id {
        ident: b"Google Chromebooks\0".as_ptr(),
        matches: [DMI_MATCH(DMI_SYS_VENDOR, b"Google\0".as_ptr())],
    },
    dmi_system_id {
        ident: b"Google firmware\0".as_ptr(),
        matches: [DMI_MATCH(DMI_BIOS_VERSION, b"Google\0".as_ptr())],
    },
    dmi_system_id {},
];

static dmi_table_tgl_lp: [dmi_system_id; 3] = [
    dmi_system_id {
        ident: b"Google Chromebooks\0".as_ptr(),
        matches: [DMI_MATCH(DMI_SYS_VENDOR, b"Google\0".as_ptr())],
    },
    dmi_system_id {
        ident: b"UPX-TGL\0".as_ptr(),
        matches: [DMI_MATCH(DMI_SYS_VENDOR, b"AAEON\0".as_ptr())],
    },
    dmi_system_id {},
];

static essx_83x6_codecs: [*const u8; 3] = [
    b"ESSX8316\0".as_ptr(),
    b"ESSX8326\0".as_ptr(),
    b"ESSX8336\0".as_ptr(),
];

static essx_83x6: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
    num_codecs: 3,
    codecs: essx_83x6_codecs.as_ptr(),
};

/*
 * configuration table
 * - the order of similar PCI ID entries is important!
 * - the first successful match will win
 */
static config_table: &[config_entry] = &[
    /* Merrifield */
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_MERRIFIELD) */
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_SST_TNG, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /*
     * Skylake, Kabylake, Apollolake
     * the legacy HDAudio driver is used except on Up Squared (SOF) and
     * Chromebooks (SST), as well as devices based on the ES8336 codec
     */
    /* if IS_ENABLED(CONFIG_SND_SOC_INTEL_AVS) */
    config_entry { flags: FLAG_SST, device: PCI_DEVICE_ID_INTEL_HDA_SKL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_google_chromebooks.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SST | FLAG_SST_ONLY_IF_DMIC, device: PCI_DEVICE_ID_INTEL_HDA_SKL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SST, device: PCI_DEVICE_ID_INTEL_HDA_KBL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_google_chromebooks.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SST | FLAG_SST_ONLY_IF_DMIC, device: PCI_DEVICE_ID_INTEL_HDA_KBL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SST, device: PCI_DEVICE_ID_INTEL_HDA_APL, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_google_chromebooks.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SST, device: PCI_DEVICE_ID_INTEL_HDA_RPL_M, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SST, device: PCI_DEVICE_ID_INTEL_HDA_FCL, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    /* else AVS disabled; force to legacy as SOF doesn't work for SKL or KBL */
    config_entry { flags: 0, device: PCI_DEVICE_ID_INTEL_HDA_SKL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: 0, device: PCI_DEVICE_ID_INTEL_HDA_KBL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_APOLLOLAKE) */
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_APL, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_up_squared.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_APL, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: &essx_83x6 },

    /*
     * Geminilake uses legacy HDAudio driver except for Google
     * Chromebooks and devices based on the ES8336 codec
     */
    /* Geminilake */
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_GEMINILAKE) */
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_GLK, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_google_chromebooks.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_GLK, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: &essx_83x6 },

    /*
     * CoffeeLake, CannonLake, CometLake, IceLake, TigerLake, AlderLake,
     * RaptorLake, MeteorLake use legacy HDAudio driver except for Google
     * Chromebooks and when DMICs are present. Two cases are required since
     * Coreboot does not expose NHLT tables.
     *
     * When the Chromebook quirk is not present, it's based on information
     * that no such device exists. When the quirk is present, it could be
     * either based on product information or a placeholder.
     */

    /* Cannonlake */
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_CANNONLAKE) */
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_CNL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_cnl_lp.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_CNL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: &essx_83x6 },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_CNL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /* Coffelake */
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_COFFEELAKE) */
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_CNL_H, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_google_chromebooks.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_CNL_H, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_COMETLAKE) */
    /* Cometlake-LP */
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_CML_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_cml_lp.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_CML_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: &essx_83x6 },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_CML_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    /* Cometlake-H */
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_CML_H, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_cml_h.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_CML_H, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: &essx_83x6 },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_CML_H, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /* Icelake */
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_ICELAKE) */
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_ICL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_google_chromebooks.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_ICL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: &essx_83x6 },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_ICL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /* Jasper Lake */
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_JASPERLAKE) */
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_JSL_N, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_jsl_n.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_JSL_N, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: &essx_83x6 },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC, device: PCI_DEVICE_ID_INTEL_HDA_JSL_N, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /* Tigerlake */
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_TIGERLAKE) */
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_TGL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_tgl_lp.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_TGL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: &essx_83x6 },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_TGL_LP, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_TGL_H, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /* Elkhart Lake */
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_ELKHARTLAKE) */
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC, device: PCI_DEVICE_ID_INTEL_HDA_EHL_0, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC, device: PCI_DEVICE_ID_INTEL_HDA_EHL_3, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /* Alder Lake / Raptor Lake */
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_ALDERLAKE) */
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_ADL_S, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_RPL_S, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_ADL_P, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_google_chromebooks.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_ADL_P, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: &essx_83x6 },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_ADL_P, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_ADL_PX, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_ADL_PS, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: &essx_83x6 },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_ADL_PS, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_ADL_M, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_ADL_N, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_google_chromebooks.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_ADL_N, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_RPL_P_0, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_google_chromebooks.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_RPL_P_0, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_RPL_P_1, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_google_chromebooks.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_RPL_P_1, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_RPL_M, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_RPL_PX, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /* Meteor Lake */
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_METEORLAKE) */
    /* Meteorlake-P */
    config_entry { flags: FLAG_SOF, device: PCI_DEVICE_ID_INTEL_HDA_MTL, acpi_hid: [0; ACPI_ID_LEN], dmi_table: dmi_table_google_chromebooks.as_ptr(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_MTL, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    /* ArrowLake-S */
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_ARL_S, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    /* ArrowLake */
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_ARL, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /* Lunar Lake */
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_LUNARLAKE) */
    /* Lunarlake-P */
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_LNL_P, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /* Panther Lake, Wildcat Lake */
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_PANTHERLAKE) */
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_PTL, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_PTL_H, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_WCL, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /* Nova Lake */
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_NOVALAKE) */
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_NVL, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SOF | FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE, device: PCI_DEVICE_ID_INTEL_HDA_NVL_S, acpi_hid: [0; ACPI_ID_LEN], dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
];

unsafe fn snd_intel_dsp_find_config(
    pci: *mut pci_dev,
    mut table: *const config_entry,
    mut len: u32,
) -> *const config_entry {
    let device: u16 = (*pci).device;

    while len > 0 {
        if (*table).device != device {
            len -= 1;
            table = table.add(1);
            continue;
        }
        if !(*table).dmi_table.is_null() && !dmi_check_system((*table).dmi_table) {
            len -= 1;
            table = table.add(1);
            continue;
        }
        if !(*table).codec_hid.is_null() {
            let mut i: core::ffi::c_int = 0;

            while i < (*(*table).codec_hid).num_codecs {
                let mut nhlt: *mut nhlt_acpi_table;
                let mut ssp_found: bool = false;
                let hid = *(*(*table).codec_hid).codecs.add(i as usize);

                if !acpi_dev_present(hid, core::ptr::null(), -1) {
                    i += 1;
                    continue;
                }

                nhlt = intel_nhlt_init(&mut (*pci).dev);
                if nhlt.is_null() {
                    dev_warn(
                        &mut (*pci).dev,
                        b"%s: NHLT table not found, skipped HID %s\n\0".as_ptr(),
                        b"snd_intel_dsp_find_config\0".as_ptr(),
                        hid,
                    );
                    i += 1;
                    continue;
                }

                if intel_nhlt_has_endpoint_type(nhlt, NHLT_LINK_SSP)
                    && intel_nhlt_ssp_endpoint_mask(nhlt, NHLT_DEVICE_I2S) != 0
                {
                    ssp_found = true;
                }

                intel_nhlt_free(nhlt);

                if ssp_found {
                    break;
                }

                dev_warn(
                    &mut (*pci).dev,
                    b"%s: no valid SSP found for HID %s, skipped\n\0".as_ptr(),
                    b"snd_intel_dsp_find_config\0".as_ptr(),
                    hid,
                );
                i += 1;
            }
            if i == (*(*table).codec_hid).num_codecs {
                len -= 1;
                table = table.add(1);
                continue;
            }
        }
        return table;
    }
    core::ptr::null()
}

unsafe fn snd_intel_dsp_check_dmic(_pci: *mut pci_dev) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;

    acpi_nhlt_get_gbl_table();

    if !acpi_nhlt_find_endpoint(ACPI_NHLT_LINKTYPE_PDM, -1, -1, -1).is_null() {
        ret = 1;
    }

    acpi_nhlt_put_gbl_table();

    ret
}

/* if IS_ENABLED(CONFIG_SND_SOC_SOF_INTEL_SOUNDWIRE) */
unsafe fn snd_intel_dsp_check_soundwire(pci: *mut pci_dev) -> core::ffi::c_int {
    let mut info: sdw_intel_acpi_info = core::mem::zeroed();
    let handle: acpi_handle;
    let ret: core::ffi::c_int;

    handle = ACPI_HANDLE(&mut (*pci).dev);
    if handle.is_null() {
        return -ENODEV;
    }

    ret = sdw_intel_acpi_scan(handle, &mut info);
    if ret < 0 {
        return ret;
    }

    info.link_mask
}

/* else */
unsafe fn snd_intel_dsp_check_soundwire_disabled(_pci: *mut pci_dev) -> core::ffi::c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_intel_dsp_driver_probe(pci: *mut pci_dev) -> core::ffi::c_int {
    let cfg: *const config_entry;

    /* Intel vendor only */
    if (*pci).vendor != PCI_VENDOR_ID_INTEL {
        return SND_INTEL_DSP_DRIVER_ANY;
    }

    /*
     * Legacy devices don't have a PCI-based DSP and use HDaudio
     * for HDMI/DP support, ignore kernel parameter
     */
    match (*pci).device {
        PCI_DEVICE_ID_INTEL_HDA_BDW
        | PCI_DEVICE_ID_INTEL_HDA_HSW_0
        | PCI_DEVICE_ID_INTEL_HDA_HSW_2
        | PCI_DEVICE_ID_INTEL_HDA_HSW_3
        | PCI_DEVICE_ID_INTEL_HDA_BYT
        | PCI_DEVICE_ID_INTEL_HDA_BSW => return SND_INTEL_DSP_DRIVER_ANY,
        _ => {}
    }

    if dsp_driver > 0 && dsp_driver <= SND_INTEL_DSP_DRIVER_LAST {
        return dsp_driver;
    }

    /*
     * detect DSP by checking class/subclass/prog-id information
     * class=04 subclass 03 prog-if 00: no DSP, use legacy driver
     * class=04 subclass 01 prog-if 00: DSP is present
     *  (and may be required e.g. for DMIC or SSP support)
     * class=04 subclass 03 prog-if 80: use DSP or legacy mode
     */
    if (*pci).class == 0x040300 {
        return SND_INTEL_DSP_DRIVER_LEGACY;
    }
    if (*pci).class != 0x040100 && (*pci).class != 0x040380 {
        dev_err(
            &mut (*pci).dev,
            b"Unknown PCI class/subclass/prog-if information (0x%06x) found, selecting HDAudio legacy driver\n\0".as_ptr(),
            (*pci).class,
        );
        return SND_INTEL_DSP_DRIVER_LEGACY;
    }

    dev_dbg(
        &mut (*pci).dev,
        b"DSP detected with PCI class/subclass/prog-if info 0x%06x\n\0".as_ptr(),
        (*pci).class,
    );

    /* find the configuration for the specific device */
    cfg = snd_intel_dsp_find_config(
        pci,
        config_table.as_ptr(),
        config_table.len() as u32,
    );
    if cfg.is_null() {
        return if IS_ENABLED_CONFIG_SND_HDA_INTEL {
            SND_INTEL_DSP_DRIVER_LEGACY
        } else {
            SND_INTEL_DSP_DRIVER_ANY
        };
    }

    if (*cfg).flags & FLAG_SOF != 0 {
        if (*cfg).flags & FLAG_SOF_ONLY_IF_SOUNDWIRE != 0
            && snd_intel_dsp_check_soundwire(pci) > 0
        {
            dev_info_once(
                &mut (*pci).dev,
                b"SoundWire enabled on CannonLake+ platform, using SOF driver\n\0".as_ptr(),
            );
            return SND_INTEL_DSP_DRIVER_SOF;
        }
        if (*cfg).flags & FLAG_SOF_ONLY_IF_DMIC != 0 && snd_intel_dsp_check_dmic(pci) != 0 {
            dev_info_once(
                &mut (*pci).dev,
                b"Digital mics found on Skylake+ platform, using SOF driver\n\0".as_ptr(),
            );
            return SND_INTEL_DSP_DRIVER_SOF;
        }
        if (*cfg).flags & FLAG_SOF_ONLY_IF_DMIC_OR_SOUNDWIRE == 0 {
            return SND_INTEL_DSP_DRIVER_SOF;
        }
    }

    if (*cfg).flags & FLAG_SST != 0 {
        if (*cfg).flags & FLAG_SST_ONLY_IF_DMIC != 0 {
            if snd_intel_dsp_check_dmic(pci) != 0 {
                dev_info_once(
                    &mut (*pci).dev,
                    b"Digital mics found on Skylake+ platform, using SST driver\n\0".as_ptr(),
                );
                return SND_INTEL_DSP_DRIVER_SST;
            }
        } else {
            return SND_INTEL_DSP_DRIVER_SST;
        }
    }

    SND_INTEL_DSP_DRIVER_LEGACY
}
// EXPORT_SYMBOL_GPL(snd_intel_dsp_driver_probe);

/* Should we default to SOF or SST for BYT/CHT ? */
/* if IS_ENABLED(CONFIG_SND_INTEL_BYT_PREFER_SOF) ||
 *    !IS_ENABLED(CONFIG_SND_SST_ATOM_HIFI2_PLATFORM_ACPI)
 */
const FLAG_SST_OR_SOF_BYT: u32 = FLAG_SOF;
/* else: const FLAG_SST_OR_SOF_BYT: u32 = FLAG_SST; */

/*
 * configuration table
 * - the order of similar ACPI ID entries is important!
 * - the first successful match will win
 */
static acpi_config_table: &[config_entry] = &[
    /* if IS_ENABLED(CONFIG_SND_SST_ATOM_HIFI2_PLATFORM_ACPI) ||
     *    IS_ENABLED(CONFIG_SND_SOC_SOF_BAYTRAIL)
     */
    /* BayTrail */
    config_entry { flags: FLAG_SST_OR_SOF_BYT, device: 0, acpi_hid: acpi_id(*b"LPE0F28\0"), dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    config_entry { flags: FLAG_SST_OR_SOF_BYT, device: 0, acpi_hid: acpi_id(*b"80860F28"), dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    /* CherryTrail */
    config_entry { flags: FLAG_SST_OR_SOF_BYT, device: 0, acpi_hid: acpi_id(*b"808622A8"), dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },

    /* Broadwell */
    /* if IS_ENABLED(CONFIG_SND_SOC_INTEL_CATPT) */
    config_entry { flags: FLAG_SST, device: 0, acpi_hid: acpi_id(*b"INT3438\0"), dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    /* if IS_ENABLED(CONFIG_SND_SOC_SOF_BROADWELL) */
    config_entry { flags: FLAG_SOF, device: 0, acpi_hid: acpi_id(*b"INT3438\0"), dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
    /* Haswell - not supported by SOF but added for consistency */
    /* if IS_ENABLED(CONFIG_SND_SOC_INTEL_CATPT) */
    config_entry { flags: FLAG_SST, device: 0, acpi_hid: acpi_id(*b"INT33C8\0"), dmi_table: core::ptr::null(), codec_hid: core::ptr::null() },
];

const fn acpi_id<const N: usize>(src: [u8; N]) -> [u8; ACPI_ID_LEN] {
    let mut dst = [0u8; ACPI_ID_LEN];
    let mut i = 0;
    while i < N && i < ACPI_ID_LEN {
        dst[i] = src[i];
        i += 1;
    }
    dst
}

unsafe fn snd_intel_acpi_dsp_find_config(
    acpi_hid: *const u8,
    mut table: *const config_entry,
    mut len: u32,
) -> *const config_entry {
    while len > 0 {
        if memcmp((*table).acpi_hid.as_ptr(), acpi_hid, ACPI_ID_LEN) != 0 {
            len -= 1;
            table = table.add(1);
            continue;
        }
        if !(*table).dmi_table.is_null() && !dmi_check_system((*table).dmi_table) {
            len -= 1;
            table = table.add(1);
            continue;
        }
        return table;
    }
    core::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn snd_intel_acpi_dsp_driver_probe(
    dev: *mut device,
    acpi_hid: *const u8,
) -> core::ffi::c_int {
    let cfg: *const config_entry;

    if dsp_driver > SND_INTEL_DSP_DRIVER_LEGACY && dsp_driver <= SND_INTEL_DSP_DRIVER_LAST {
        return dsp_driver;
    }

    if dsp_driver == SND_INTEL_DSP_DRIVER_LEGACY {
        dev_warn(
            dev,
            b"dsp_driver parameter %d not supported, using automatic detection\n\0".as_ptr(),
            SND_INTEL_DSP_DRIVER_LEGACY,
        );
    }

    /* find the configuration for the specific device */
    cfg = snd_intel_acpi_dsp_find_config(
        acpi_hid,
        acpi_config_table.as_ptr(),
        acpi_config_table.len() as u32,
    );
    if cfg.is_null() {
        return SND_INTEL_DSP_DRIVER_ANY;
    }

    if (*cfg).flags & FLAG_SST != 0 {
        return SND_INTEL_DSP_DRIVER_SST;
    }

    if (*cfg).flags & FLAG_SOF != 0 {
        return SND_INTEL_DSP_DRIVER_SOF;
    }

    SND_INTEL_DSP_DRIVER_SST
}
// EXPORT_SYMBOL_GPL(snd_intel_acpi_dsp_driver_probe);

// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("Intel DSP config driver");
// MODULE_IMPORT_NS("SND_INTEL_SOUNDWIRE_ACPI");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
