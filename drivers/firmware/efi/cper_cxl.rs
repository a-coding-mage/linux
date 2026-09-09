// SPDX-License-Identifier: GPL-2.0-only
/*
 * UEFI Common Platform Error Record (CPER) support for CXL Section.
 *
 * Copyright (C) 2022 Advanced Micro Devices, Inc.
 *
 * Author: Smita Koralahalli <Smita.KoralahalliChannabasappa@amd.com>
 */

// Dependencies supplied by the Linux CPER and CXL event interfaces.

static PROT_ERR_AGENT_TYPE_STRS: [&'static [u8]; 8] = [
    b"Restricted CXL Device\0",
    b"Restricted CXL Host Downstream Port\0",
    b"CXL Device\0",
    b"CXL Logical Device\0",
    b"CXL Fabric Manager managed Logical Device\0",
    b"CXL Root Port\0",
    b"CXL Downstream Switch Port\0",
    b"CXL Upstream Switch Port\0",
];

extern "C" {
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn print_hex_dump(
        prefix_str: *const core::ffi::c_char,
        prefix_type: *const core::ffi::c_char,
        flags: i32,
        rowsize: i32,
        groupsize: i32,
        buf: *const core::ffi::c_void,
        len: usize,
        ascii: i32,
        ...
    );
}

pub unsafe fn cxl_cper_print_prot_err(
    pfx: *const core::ffi::c_char,
    prot_err: *const cxl_cper_sec_prot_err,
) {
    if (*prot_err).valid_bits & PROT_ERR_VALID_AGENT_TYPE != 0 {
        let agent_type = (*prot_err).agent_type;
        let agent_type_str = if (agent_type as usize) < PROT_ERR_AGENT_TYPE_STRS.len() {
            PROT_ERR_AGENT_TYPE_STRS[agent_type as usize].as_ptr()
        } else {
            b"unknown\0".as_ptr()
        };
        pr_info(
            b"%s agent_type: %d, %s\n\0".as_ptr() as *const core::ffi::c_char,
            pfx,
            agent_type,
            agent_type_str,
        );
    }

    if (*prot_err).valid_bits & PROT_ERR_VALID_AGENT_ADDRESS != 0 {
        match (*prot_err).agent_type {
            RCD | DEVICE | LD | FMLD | RP | DSP | USP => {
                pr_info(
                    b"%s agent_address: %04x:%02x:%02x.%x\n\0".as_ptr()
                        as *const core::ffi::c_char,
                    pfx,
                    (*prot_err).agent_addr.segment,
                    (*prot_err).agent_addr.bus,
                    (*prot_err).agent_addr.device,
                    (*prot_err).agent_addr.function,
                );
            }
            RCH_DP => {
                pr_info(
                    b"%s rcrb_base_address: 0x%016llx\n\0".as_ptr()
                        as *const core::ffi::c_char,
                    pfx,
                    (*prot_err).agent_addr.rcrb_base_addr,
                );
            }
            _ => {}
        }
    }

    if (*prot_err).valid_bits & PROT_ERR_VALID_DEVICE_ID != 0 {
        match (*prot_err).agent_type {
            RCD | DEVICE | LD | FMLD | RP | DSP | USP => {
                pr_info(
                    b"%s slot: %d\n\0".as_ptr() as *const core::ffi::c_char,
                    pfx,
                    (*prot_err).device_id.slot >> CPER_PCIE_SLOT_SHIFT,
                );
                pr_info(
                    b"%s vendor_id: 0x%04x, device_id: 0x%04x\n\0".as_ptr()
                        as *const core::ffi::c_char,
                    pfx,
                    (*prot_err).device_id.vendor_id,
                    (*prot_err).device_id.device_id,
                );
                pr_info(
                    b"%s sub_vendor_id: 0x%04x, sub_device_id: 0x%04x\n\0".as_ptr()
                        as *const core::ffi::c_char,
                    pfx,
                    (*prot_err).device_id.subsystem_vendor_id,
                    (*prot_err).device_id.subsystem_id,
                );
                let class_code = (*prot_err).device_id.class_code;
                pr_info(
                    b"%s class_code: %02x%02x\n\0".as_ptr() as *const core::ffi::c_char,
                    pfx,
                    class_code[1],
                    class_code[0],
                );
            }
            _ => {}
        }
    }

    if (*prot_err).valid_bits & PROT_ERR_VALID_SERIAL_NUMBER != 0 {
        match (*prot_err).agent_type {
            RCD | DEVICE | LD | FMLD => pr_info(
                b"%s lower_dw: 0x%08x, upper_dw: 0x%08x\n\0".as_ptr()
                    as *const core::ffi::c_char,
                pfx,
                (*prot_err).dev_serial_num.lower_dw,
                (*prot_err).dev_serial_num.upper_dw,
            ),
            _ => {}
        }
    }

    if (*prot_err).valid_bits & PROT_ERR_VALID_CAPABILITY != 0 {
        match (*prot_err).agent_type {
            RCD | DEVICE | LD | FMLD | RP | DSP | USP => print_hex_dump(
                pfx,
                b"\0".as_ptr() as *const core::ffi::c_char,
                DUMP_PREFIX_OFFSET,
                16,
                4,
                (*prot_err).capability.as_ptr() as *const core::ffi::c_void,
                core::mem::size_of_val(&(*prot_err).capability),
                0,
            ),
            _ => {}
        }
    }

    if (*prot_err).valid_bits & PROT_ERR_VALID_DVSEC != 0 {
        pr_info(
            b"%s DVSEC length: 0x%04x\n\0".as_ptr() as *const core::ffi::c_char,
            pfx,
            (*prot_err).dvsec_len,
        );
        pr_info(b"%s CXL DVSEC:\n\0".as_ptr() as *const core::ffi::c_char, pfx);
        print_hex_dump(
            pfx,
            b"\0".as_ptr() as *const core::ffi::c_char,
            DUMP_PREFIX_OFFSET,
            16,
            4,
            prot_err.add(1) as *const core::ffi::c_void,
            (*prot_err).dvsec_len as usize,
            0,
        );
    }

    if (*prot_err).valid_bits & PROT_ERR_VALID_ERROR_LOG != 0 {
        let size = core::mem::size_of::<cxl_cper_sec_prot_err>() + (*prot_err).dvsec_len as usize;
        let cxl_ras = ((prot_err as *const u8).add(size)) as *const cxl_ras_capability_regs;
        pr_info(
            b"%s Error log length: 0x%04x\n\0".as_ptr() as *const core::ffi::c_char,
            pfx,
            (*prot_err).err_len,
        );
        pr_info(b"%s CXL Error Log:\n\0".as_ptr() as *const core::ffi::c_char, pfx);
        pr_info(b"%s cxl_ras_uncor_status: 0x%08x\0".as_ptr() as *const core::ffi::c_char, pfx, (*cxl_ras).uncor_status);
        pr_info(b"%s cxl_ras_uncor_mask: 0x%08x\n\0".as_ptr() as *const core::ffi::c_char, pfx, (*cxl_ras).uncor_mask);
        pr_info(b"%s cxl_ras_uncor_severity: 0x%08x\n\0".as_ptr() as *const core::ffi::c_char, pfx, (*cxl_ras).uncor_severity);
        pr_info(b"%s cxl_ras_cor_status: 0x%08x\0".as_ptr() as *const core::ffi::c_char, pfx, (*cxl_ras).cor_status);
        pr_info(b"%s cxl_ras_cor_mask: 0x%08x\n\0".as_ptr() as *const core::ffi::c_char, pfx, (*cxl_ras).cor_mask);
        pr_info(b"%s cap_control: 0x%08x\n\0".as_ptr() as *const core::ffi::c_char, pfx, (*cxl_ras).cap_control);
        pr_info(b"%s Header Log Registers:\n\0".as_ptr() as *const core::ffi::c_char, pfx);
        print_hex_dump(pfx, b"\0".as_ptr() as *const core::ffi::c_char, DUMP_PREFIX_OFFSET, 16, 4, (*cxl_ras).header_log.as_ptr() as *const core::ffi::c_void, core::mem::size_of_val(&(*cxl_ras).header_log), 0);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
