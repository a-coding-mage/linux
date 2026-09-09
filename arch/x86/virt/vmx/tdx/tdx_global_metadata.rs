// SPDX-License-Identifier: GPL-2.0
/*
 * Automatically generated functions to read TDX global metadata.
 *
 * This file doesn't compile on its own as it lacks inclusion
 * of the SEAMCALL wrapper primitive which reads global metadata.
 * Include this file in another Rust file instead.
 */

unsafe extern "C" {
    fn read_sys_metadata_field(field_id: u64, value: *mut u64) -> i32;
}

unsafe fn get_tdx_sys_info_version(sysinfo_version: *mut tdx_sys_info_version) -> i32 {
    let mut ret: i32 = 0;
    let mut val: u64;

    if ret == 0 {
        ret = read_sys_metadata_field(0x0800000100000003, &mut val);
        if ret == 0 { (*sysinfo_version).minor_version = val; }
    }
    if ret == 0 {
        ret = read_sys_metadata_field(0x0800000100000004, &mut val);
        if ret == 0 { (*sysinfo_version).major_version = val; }
    }
    if ret == 0 {
        ret = read_sys_metadata_field(0x0800000100000005, &mut val);
        if ret == 0 { (*sysinfo_version).update_version = val; }
    }

    ret
}

unsafe fn get_tdx_sys_info_features(sysinfo_features: *mut tdx_sys_info_features) -> i32 {
    let mut ret: i32 = 0;
    let mut val: u64;

    if ret == 0 {
        ret = read_sys_metadata_field(0x0A00000300000008, &mut val);
        if ret == 0 { (*sysinfo_features).tdx_features0 = val; }
    }

    ret
}

unsafe fn get_tdx_sys_info_tdmr(sysinfo_tdmr: *mut tdx_sys_info_tdmr) -> i32 {
    let mut ret: i32 = 0;
    let mut val: u64;

    if ret == 0 { ret = read_sys_metadata_field(0x9100000100000008, &mut val); if ret == 0 { (*sysinfo_tdmr).max_tdmrs = val; } }
    if ret == 0 { ret = read_sys_metadata_field(0x9100000100000009, &mut val); if ret == 0 { (*sysinfo_tdmr).max_reserved_per_tdmr = val; } }
    if ret == 0 { ret = read_sys_metadata_field(0x9100000100000010, &mut val); if ret == 0 { (*sysinfo_tdmr).pamt_4k_entry_size = val; } }
    if ret == 0 { ret = read_sys_metadata_field(0x9100000100000011, &mut val); if ret == 0 { (*sysinfo_tdmr).pamt_2m_entry_size = val; } }
    if ret == 0 { ret = read_sys_metadata_field(0x9100000100000012, &mut val); if ret == 0 { (*sysinfo_tdmr).pamt_1g_entry_size = val; } }

    ret
}

unsafe fn get_tdx_sys_info_td_ctrl(sysinfo_td_ctrl: *mut tdx_sys_info_td_ctrl) -> i32 {
    let mut ret: i32 = 0;
    let mut val: u64;

    if ret == 0 { ret = read_sys_metadata_field(0x9800000100000000, &mut val); if ret == 0 { (*sysinfo_td_ctrl).tdr_base_size = val; } }
    if ret == 0 { ret = read_sys_metadata_field(0x9800000100000100, &mut val); if ret == 0 { (*sysinfo_td_ctrl).tdcs_base_size = val; } }
    if ret == 0 { ret = read_sys_metadata_field(0x9800000100000200, &mut val); if ret == 0 { (*sysinfo_td_ctrl).tdvps_base_size = val; } }

    ret
}

unsafe fn get_tdx_sys_info_td_conf(sysinfo_td_conf: *mut tdx_sys_info_td_conf) -> i32 {
    let mut ret: i32 = 0;
    let mut val: u64;

    if ret == 0 { ret = read_sys_metadata_field(0x1900000300000000, &mut val); if ret == 0 { (*sysinfo_td_conf).attributes_fixed0 = val; } }
    if ret == 0 { ret = read_sys_metadata_field(0x1900000300000001, &mut val); if ret == 0 { (*sysinfo_td_conf).attributes_fixed1 = val; } }
    if ret == 0 { ret = read_sys_metadata_field(0x1900000300000002, &mut val); if ret == 0 { (*sysinfo_td_conf).xfam_fixed0 = val; } }
    if ret == 0 { ret = read_sys_metadata_field(0x1900000300000003, &mut val); if ret == 0 { (*sysinfo_td_conf).xfam_fixed1 = val; } }
    if ret == 0 { ret = read_sys_metadata_field(0x9900000100000004, &mut val); if ret == 0 { (*sysinfo_td_conf).num_cpuid_config = val; } }
    if ret == 0 { ret = read_sys_metadata_field(0x9900000100000008, &mut val); if ret == 0 { (*sysinfo_td_conf).max_vcpus_per_td = val; } }
    if (*sysinfo_td_conf).num_cpuid_config as usize > (*sysinfo_td_conf).cpuid_config_leaves.len() { return -22; }
    for i in 0..((*sysinfo_td_conf).num_cpuid_config as usize) {
        if ret == 0 { ret = read_sys_metadata_field(0x9900000300000400u64.wrapping_add(i as u64), &mut val); if ret == 0 { (*sysinfo_td_conf).cpuid_config_leaves[i] = val; } }
    }
    if (*sysinfo_td_conf).num_cpuid_config as usize > (*sysinfo_td_conf).cpuid_config_values.len() { return -22; }
    for i in 0..((*sysinfo_td_conf).num_cpuid_config as usize) {
        for j in 0..2usize {
            if ret == 0 { ret = read_sys_metadata_field(0x9900000300000500u64.wrapping_add((i * 2 + j) as u64), &mut val); if ret == 0 { (*sysinfo_td_conf).cpuid_config_values[i][j] = val; } }
        }
    }
    ret
}

unsafe fn get_tdx_sys_info_handoff(sysinfo_handoff: *mut tdx_sys_info_handoff) -> i32 {
    let mut val: u64 = 0;
    let ret = read_sys_metadata_field(0x8900000100000000, &mut val);
    if ret != 0 { return ret; }
    (*sysinfo_handoff).module_hv = val;
    0
}

unsafe fn get_tdx_sys_info(sysinfo: *mut tdx_sys_info) -> i32 {
    let mut ret: i32 = 0;
    ret = if ret != 0 { ret } else { get_tdx_sys_info_version(&mut (*sysinfo).version) };

    // Equivalent to: pr_info!("Module version: " TDX_VERSION_FMT "\n", ...)
    // The logging macro is supplied by the including translation unit.
    ret = if ret != 0 { ret } else { get_tdx_sys_info_features(&mut (*sysinfo).features) };
    ret = if ret != 0 { ret } else { get_tdx_sys_info_tdmr(&mut (*sysinfo).tdmr) };
    ret = if ret != 0 { ret } else { get_tdx_sys_info_td_ctrl(&mut (*sysinfo).td_ctrl) };
    ret = if ret != 0 { ret } else { get_tdx_sys_info_td_conf(&mut (*sysinfo).td_conf) };
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
