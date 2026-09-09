// SPDX-License-Identifier: GPL-2.0

// C dependencies: <linux/array_size.h>, <linux/printk.h>, and <asm/tdx.h>.
// The original pr_fmt definition prefixes messages with "tdx: ".

// C macro translation:
// #define DEF_TDX_TD_ATTR_NAME(_name) [TDX_TD_ATTR_##_name##_BIT] = __stringify(_name)
// #define DEF_TD_CTLS_NAME(_name) [TD_CTLS_##_name##_BIT] = __stringify(_name)

// These arrays correspond to the C designated-initializer arrays.  The
// constants referenced below are supplied by the TDX dependency.
static TDX_ATTRIBUTES: [&str; 12] = [
    "DEBUG",
    "HGS_PLUS_PROF",
    "PERF_PROF",
    "PMT_PROF",
    "ICSSD",
    "LASS",
    "SEPT_VE_DISABLE",
    "MIGRATABLE",
    "PKS",
    "KL",
    "TPA",
    "PERFMON",
];

static TDCS_TD_CTLS: [&str; 5] = [
    "PENDING_VE_DISABLE",
    "ENUM_TOPOLOGY",
    "VIRT_CPUID2",
    "REDUCE_VE",
    "LOCK",
];

extern "C" {
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn pr_cont(fmt: *const core::ffi::c_char, ...);
}

pub unsafe fn tdx_dump_attributes(mut td_attr: u64) {
    pr_info(b"tdx: Attributes:\0".as_ptr() as *const core::ffi::c_char);

    for i in 0..TDX_ATTRIBUTES.len() {
        if (td_attr & (1u64 << i)) != 0 {
            pr_cont(
                b" %s\0".as_ptr() as *const core::ffi::c_char,
                TDX_ATTRIBUTES[i].as_ptr(),
            );
        }
        td_attr &= !(1u64 << i);
    }

    if td_attr != 0 {
        pr_cont(
            b" unknown:%#llx\0".as_ptr() as *const core::ffi::c_char,
            td_attr,
        );
    }
    pr_cont(b"\n\0".as_ptr() as *const core::ffi::c_char);
}

pub unsafe fn tdx_dump_td_ctls(mut td_ctls: u64) {
    pr_info(b"tdx: TD_CTLS:\0".as_ptr() as *const core::ffi::c_char);

    for i in 0..TDCS_TD_CTLS.len() {
        if (td_ctls & (1u64 << i)) != 0 {
            pr_cont(
                b" %s\0".as_ptr() as *const core::ffi::c_char,
                TDCS_TD_CTLS[i].as_ptr(),
            );
        }
        td_ctls &= !(1u64 << i);
    }
    if td_ctls != 0 {
        pr_cont(
            b" unknown:%#llx\0".as_ptr() as *const core::ffi::c_char,
            td_ctls,
        );
    }
    pr_cont(b"\n\0".as_ptr() as *const core::ffi::c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
