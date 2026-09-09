// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2025 Intel Corporation. All rights reserved

// Required kernel declarations and types are supplied by the surrounding crate.

pub unsafe fn cxl_cper_sec_prot_err_valid(
    prot_err: *mut cxl_cper_sec_prot_err,
) -> i32 {
    if ((*prot_err).valid_bits & PROT_ERR_VALID_AGENT_ADDRESS) == 0 {
        pr_err_ratelimited!("CXL CPER invalid agent type\n");
        return -EINVAL;
    }

    if ((*prot_err).valid_bits & PROT_ERR_VALID_ERROR_LOG) == 0 {
        pr_err_ratelimited!("CXL CPER invalid protocol error log\n");
        return -EINVAL;
    }

    if (*prot_err).err_len != core::mem::size_of::<cxl_ras_capability_regs>() {
        pr_err_ratelimited!(
            "CXL CPER invalid RAS Cap size (%u)\n",
            (*prot_err).err_len
        );
        return -EINVAL;
    }

    if (((*prot_err).agent_type == RCD
        || (*prot_err).agent_type == DEVICE
        || (*prot_err).agent_type == LD
        || (*prot_err).agent_type == FMLD)
        && ((*prot_err).valid_bits & PROT_ERR_VALID_SERIAL_NUMBER) == 0)
    {
        pr_warn_ratelimited!(FW_WARN "CXL CPER no device serial number\n");
    }

    0
}

pub unsafe fn cxl_cper_setup_prot_err_work_data(
    wd: *mut cxl_cper_prot_err_work_data,
    prot_err: *mut cxl_cper_sec_prot_err,
    severity: i32,
) -> i32 {
    let (dvsec_start, cap_start): (*const u8, *const u8);

    match (*prot_err).agent_type {
        RCD | DEVICE | LD | FMLD | RP | DSP | USP => {
            core::ptr::copy_nonoverlapping(
                prot_err,
                core::ptr::addr_of_mut!((*wd).prot_err),
                1,
            );

            dvsec_start = (prot_err.add(1)) as *const u8;
            cap_start = dvsec_start.add((*prot_err).dvsec_len as usize);

            core::ptr::copy_nonoverlapping(
                cap_start as *const cxl_ras_capability_regs,
                core::ptr::addr_of_mut!((*wd).ras_cap),
                1,
            );
            (*wd).severity = cper_severity_to_aer(severity);
        }
        _ => {
            pr_err_ratelimited!(
                "CXL CPER invalid agent type: %d\n",
                (*prot_err).agent_type
            );
            return -EINVAL;
        }
    }

    0
}

// EXPORT_SYMBOL_GPL(cxl_cper_sec_prot_err_valid);
// EXPORT_SYMBOL_GPL(cxl_cper_setup_prot_err_work_data);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
