// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  pSeries firmware setup code.
 *
 *  Portions from arch/powerpc/platforms/pseries/setup.c:
 *   Copyright (C) 1995  Linus Torvalds
 *   Adapted from 'alpha' version by Gary Thomas
 *   Modified by Cort Dougan (cort@cs.nmt.edu)
 *   Modified by PPC64 Team, IBM Corp
 *
 *  Portions from arch/powerpc/kernel/firmware.c
 *   Copyright (C) 2001 Ben. Herrenschmidt (benh@kernel.crashing.org)
 *   Modifications for ppc64:
 *    Copyright (C) 2003 Dave Engebretsen <engebret@us.ibm.com>
 *    Copyright (C) 2005 Stephen Rothwell, IBM Corporation
 *
 *  Copyright 2006 IBM Corporation.
 */

// C dependencies: linux/of_fdt.h, asm/firmware.h, asm/prom.h, asm/udbg.h,
// asm/svm.h, and pseries.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
struct hypertas_fw_feature {
    val: c_ulong,
    name: *mut c_char,
}

/*
 * The names in this table match names in rtas/ibm,hypertas-functions.  If the
 * entry ends in a '*', only upto the '*' is matched.  Otherwise the entire
 * string must match.
 */
static mut HYPERTAS_FW_FEATURES_TABLE: [hypertas_fw_feature; 30] = [
    hypertas_fw_feature { val: FW_FEATURE_PFT, name: c"hcall-pft".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_TCE, name: c"hcall-tce".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_SPRG0, name: c"hcall-sprg0".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_DABR, name: c"hcall-dabr".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_COPY, name: c"hcall-copy".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_ASR, name: c"hcall-asr".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_DEBUG, name: c"hcall-debug".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_PERF, name: c"hcall-perf".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_DUMP, name: c"hcall-dump".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_INTERRUPT, name: c"hcall-interrupt".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_MIGRATE, name: c"hcall-migrate".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_PERFMON, name: c"hcall-perfmon".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_CRQ, name: c"hcall-crq".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_VIO, name: c"hcall-vio".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_RDMA, name: c"hcall-rdma".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_LLAN, name: c"hcall-lLAN".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_BULK_REMOVE, name: c"hcall-bulk".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_XDABR, name: c"hcall-xdabr".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_PUT_TCE_IND | FW_FEATURE_STUFF_TCE, name: c"hcall-multi-tce".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_SPLPAR, name: c"hcall-splpar".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_VPHN, name: c"hcall-vphn".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_SET_MODE, name: c"hcall-set-mode".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_BEST_ENERGY, name: c"hcall-best-energy-1*".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_HPT_RESIZE, name: c"hcall-hpt-resize".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_BLOCK_REMOVE, name: c"hcall-block-remove".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_PAPR_SCM, name: c"hcall-scm".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_RPT_INVALIDATE, name: c"hcall-rpt-invalidate".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_ENERGY_SCALE_INFO, name: c"hcall-energy-scale-info".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_WATCHDOG, name: c"hcall-watchdog".as_ptr() as *mut c_char },
    hypertas_fw_feature { val: FW_FEATURE_PLPKS, name: c"hcall-pks".as_ptr() as *mut c_char },
];

unsafe fn fw_hypertas_feature_init(hypertas: *const c_char, len: c_ulong) {
    pr_debug(c" -> fw_hypertas_feature_init()\n".as_ptr());
    let mut s = hypertas;
    while (s as usize) < (hypertas as usize).wrapping_add(len as usize) {
        for i in 0..HYPERTAS_FW_FEATURES_TABLE.len() {
            let name = HYPERTAS_FW_FEATURES_TABLE[i].name;
            let size = strlen(name);
            if size != 0 && *name.add(size - 1) == b'*' as c_char {
                if strncmp(name, s, size - 1) != 0 { continue; }
            } else if strcmp(name, s) != 0 { continue; }
            powerpc_firmware_features |= HYPERTAS_FW_FEATURES_TABLE[i].val;
            break;
        }
        s = s.add(strlen(s) + 1);
    }
    if is_secure_guest() && (powerpc_firmware_features & FW_FEATURE_PUT_TCE_IND) != 0 {
        powerpc_firmware_features &= !FW_FEATURE_PUT_TCE_IND;
        pr_debug(c"SVM: disabling PUT_TCE_IND firmware feature\n".as_ptr());
    }
    pr_debug(c" <- fw_hypertas_feature_init()\n".as_ptr());
}

#[repr(C)]
struct vec5_fw_feature { val: c_ulong, feature: u32 }

static mut VEC5_FW_FEATURES_TABLE: [vec5_fw_feature; 5] = [
    vec5_fw_feature { val: FW_FEATURE_FORM1_AFFINITY, feature: OV5_FORM1_AFFINITY },
    vec5_fw_feature { val: FW_FEATURE_PRRN, feature: OV5_PRRN },
    vec5_fw_feature { val: FW_FEATURE_DRMEM_V2, feature: OV5_DRMEM_V2 },
    vec5_fw_feature { val: FW_FEATURE_DRC_INFO, feature: OV5_DRC_INFO },
    vec5_fw_feature { val: FW_FEATURE_FORM2_AFFINITY, feature: OV5_FORM2_AFFINITY },
];

unsafe fn fw_vec5_feature_init(vec5: *const u8, len: c_ulong) {
    pr_debug(c" -> fw_vec5_feature_init()\n".as_ptr());
    for i in 0..VEC5_FW_FEATURES_TABLE.len() {
        let index = OV5_INDX(VEC5_FW_FEATURES_TABLE[i].feature);
        let feat = OV5_FEAT(VEC5_FW_FEATURES_TABLE[i].feature);
        if (index as c_ulong) < len && (*vec5.add(index as usize) as u32 & feat) != 0 {
            powerpc_firmware_features |= VEC5_FW_FEATURES_TABLE[i].val;
        }
    }
    pr_debug(c" <- fw_vec5_feature_init()\n".as_ptr());
}

unsafe extern "C" fn probe_fw_features(node: c_ulong, uname: *const c_char, depth: c_int, _data: *mut c_void) -> c_int {
    static mut HYPERTAS_FOUND: c_int = 0;
    static mut VEC5_FOUND: c_int = 0;
    if depth != 1 { return 0; }
    if strcmp(uname, c"rtas".as_ptr()) == 0 || strcmp(uname, c"rtas@0".as_ptr()) == 0 {
        let mut len = 0;
        let prop = of_get_flat_dt_prop(node, c"ibm,hypertas-functions".as_ptr(), &mut len);
        if !prop.is_null() { powerpc_firmware_features |= FW_FEATURE_LPAR; fw_hypertas_feature_init(prop, len as c_ulong); }
        HYPERTAS_FOUND = 1;
    }
    if strcmp(uname, c"chosen".as_ptr()) == 0 {
        let mut len = 0;
        let prop = of_get_flat_dt_prop(node, c"ibm,architecture-vec-5".as_ptr(), &mut len);
        if !prop.is_null() { fw_vec5_feature_init(prop as *const u8, len as c_ulong); }
        VEC5_FOUND = 1;
    }
    HYPERTAS_FOUND & VEC5_FOUND
}

pub unsafe extern "C" fn pseries_probe_fw_features() {
    of_scan_flat_dt(Some(probe_fw_features), core::ptr::null_mut());
}

extern "C" {
    static mut powerpc_firmware_features: c_ulong;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn pr_debug(s: *const c_char);
    fn is_secure_guest() -> bool;
    fn of_get_flat_dt_prop(node: c_ulong, name: *const c_char, len: *mut c_int) -> *const c_char;
    fn of_scan_flat_dt(cb: Option<unsafe extern "C" fn(c_ulong, *const c_char, c_int, *mut c_void) -> c_int>, data: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
