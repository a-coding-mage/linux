// SPDX-License-Identifier: GPL-2.0
/*
 * The Vendor OEM extension for System Control and Power Interface (SCMI)
 * Protocol based clock driver
 *
 * Copyright 2025 NXP
 */

// Dependencies supplied by the surrounding kernel translation.

const SCMI_CLOCK_CFG_IMX_SSC: u32 = 0x80;
const SCMI_CLOCK_IMX_SS_PERCENTAGE_MASK: u32 = GENMASK(7, 0);
const SCMI_CLOCK_IMX_SS_MOD_FREQ_MASK: u32 = GENMASK(23, 8);
const SCMI_CLOCK_IMX_SS_ENABLE_MASK: u32 = BIT(24);

/*
 * Selection is based on SCMI vendor_id/sub_vendor_id and optional machine
 * compatible string, without involving impl_ver. impl_ver-specific behavior
 * should be considered a bug and handled via SCMI Quirk framework.
 */
#[repr(C)]
struct scmi_clk_oem_info {
    vendor_id: *mut i8,
    sub_vendor_id: *mut i8,
    compatible: *mut i8,
    data: *const core::ffi::c_void,
}

unsafe fn scmi_clk_imx_set_spread_spectrum(
    hw: *mut clk_hw,
    ss_conf: *const clk_spread_spectrum,
) -> i32 {
    let clk: *mut scmi_clk = to_scmi_clk(hw);
    let ret: i32;
    let mut val: u32;

    /*
     * extConfigValue[7:0]   - spread percentage (%)
     * extConfigValue[23:8]  - Modulation Frequency
     * extConfigValue[24]    - Enable/Disable
     * extConfigValue[31:25] - Reserved
     */
    val = FIELD_PREP(
        SCMI_CLOCK_IMX_SS_PERCENTAGE_MASK,
        (*ss_conf).spread_bp / 10000,
    );
    val |= FIELD_PREP(SCMI_CLOCK_IMX_SS_MOD_FREQ_MASK, (*ss_conf).modfreq_hz);
    if (*ss_conf).method != CLK_SPREAD_NO {
        val |= SCMI_CLOCK_IMX_SS_ENABLE_MASK;
    }
    ret = scmi_proto_clk_ops.config_oem_set(
        (*clk).ph,
        (*clk).id,
        SCMI_CLOCK_CFG_IMX_SSC,
        val,
        false,
    );
    if ret != 0 {
        dev_warn(
            (*clk).dev,
            "Failed to set spread spectrum(%u,%u,%u) for clock ID %d\n",
            (*ss_conf).modfreq_hz,
            (*ss_conf).spread_bp,
            (*ss_conf).method,
            (*clk).id,
        );
    }

    ret
}

unsafe fn scmi_clk_imx_query_oem_feats(
    ph: *const scmi_protocol_handle,
    id: u32,
    feats_key: *mut u32,
) -> i32 {
    let ret: i32;
    let mut val: u32 = 0;

    ret = scmi_proto_clk_ops.config_oem_get(
        ph,
        id,
        SCMI_CLOCK_CFG_IMX_SSC,
        &mut val,
        core::ptr::null_mut(),
        false,
    );
    if ret == 0 {
        *feats_key |= BIT(SCMI_CLK_EXT_OEM_SSC_SUPPORTED);
    }

    0
}

static scmi_clk_oem_imx: scmi_clk_oem = scmi_clk_oem {
    query_ext_oem_feats: Some(scmi_clk_imx_query_oem_feats),
    set_spread_spectrum: Some(scmi_clk_imx_set_spread_spectrum),
};

static info: [scmi_clk_oem_info; 1] = [scmi_clk_oem_info {
    vendor_id: SCMI_IMX_VENDOR,
    sub_vendor_id: SCMI_IMX_SUBVENDOR,
    compatible: core::ptr::null_mut(),
    data: &scmi_clk_oem_imx as *const scmi_clk_oem as *const core::ffi::c_void,
}];

unsafe fn scmi_clk_oem_init(sdev: *mut scmi_device) -> i32 {
    let handle: *const scmi_handle = (*sdev).handle;
    let mut i: usize = 0;
    let size: usize = info.len();

    while i < size {
        if strcmp((*(*handle).version).vendor_id, info[i].vendor_id) != 0
            || strcmp((*(*handle).version).sub_vendor_id, info[i].sub_vendor_id) != 0
        {
            i += 1;
            continue;
        }
        if !info[i].compatible.is_null()
            && !of_machine_is_compatible(info[i].compatible)
        {
            i += 1;
            continue;
        }

        break;
    }

    if i < size {
        dev_set_drvdata(&mut (*sdev).dev, info[i].data as *mut core::ffi::c_void);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
