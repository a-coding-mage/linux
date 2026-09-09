/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Intel Core SoC Power Management Controller Header File
 *
 * Copyright (c) 2025, Intel Corporation.
 * All Rights Reserved.
 *
 */

// Dependency intent: linux/acpi.h and linux/cleanup.h supply the ACPI types,
// constants, functions, and cleanup behavior referenced below.

pub const IPC_SOC_REGISTER_ACCESS: u32 = 0xAA;
pub const IPC_SOC_SUB_CMD_READ: u32 = 0x00;
pub const IPC_SOC_SUB_CMD_WRITE: u32 = 0x01;
pub const PMC_IPCS_PARAM_COUNT: usize = 7;
pub const VALID_IPC_RESPONSE: usize = 5;

#[repr(C)]
pub struct pmc_ipc_cmd {
    pub cmd: u32,
    pub sub_cmd: u32,
    pub size: u32,
    pub wbuf: [u32; 4],
}

#[repr(C)]
pub struct pmc_ipc_rbuf {
    pub buf: [u32; 4],
}

/**
 * intel_pmc_ipc() - PMC IPC Mailbox accessor
 * @ipc_cmd:  Prepared input command to send
 * @rbuf:     Allocated array for returned IPC data
 *
 * Return: 0 on success. Non-zero on mailbox error
 */
#[inline]
pub unsafe fn intel_pmc_ipc(
    ipc_cmd: *mut pmc_ipc_cmd,
    rbuf: *mut pmc_ipc_rbuf,
) -> i32 {
    #[cfg(feature = "CONFIG_ACPI")]
    {
        let mut buffer: acpi_buffer = acpi_buffer {
            length: ACPI_ALLOCATE_BUFFER,
            pointer: core::ptr::null_mut(),
        };
        let mut params: [acpi_object; PMC_IPCS_PARAM_COUNT] =
            core::mem::zeroed();

        for param in &mut params {
            param.type_ = ACPI_TYPE_INTEGER;
        }

        let arg_list = acpi_object_list {
            count: PMC_IPCS_PARAM_COUNT as u32,
            pointer: params.as_mut_ptr(),
        };

        if ipc_cmd.is_null() || rbuf.is_null() {
            return -EINVAL;
        }

        /*
         * 0: IPC Command
         * 1: IPC Sub Command
         * 2: Size
         * 3-6: Write Buffer for offset
         */
        (*params.as_mut_ptr()).integer.value = (*ipc_cmd).cmd as _;
        (*params.as_mut_ptr().add(1)).integer.value = (*ipc_cmd).sub_cmd as _;
        (*params.as_mut_ptr().add(2)).integer.value = (*ipc_cmd).size as _;
        (*params.as_mut_ptr().add(3)).integer.value = (*ipc_cmd).wbuf[0] as _;
        (*params.as_mut_ptr().add(4)).integer.value = (*ipc_cmd).wbuf[1] as _;
        (*params.as_mut_ptr().add(5)).integer.value = (*ipc_cmd).wbuf[2] as _;
        (*params.as_mut_ptr().add(6)).integer.value = (*ipc_cmd).wbuf[3] as _;

        let status = acpi_evaluate_object(
            core::ptr::null_mut(),
            "\\IPCS\0".as_ptr() as *mut _,
            &arg_list,
            &mut buffer,
        );
        if ACPI_FAILURE(status) {
            return -ENODEV;
        }

        // C: union acpi_object *obj __free(kfree) = buffer.pointer;
        let obj = buffer.pointer as *mut acpi_object;

        if !obj.is_null()
            && (*obj).type_ == ACPI_TYPE_PACKAGE
            && (*obj).package.count == VALID_IPC_RESPONSE as u32
        {
            let objs = (*obj).package.elements;

            if ( (*objs).integer.value as u8) != 0 {
                return -EINVAL;
            }

            (*rbuf).buf[0] = (*objs.add(1)).integer.value as u32;
            (*rbuf).buf[1] = (*objs.add(2)).integer.value as u32;
            (*rbuf).buf[2] = (*objs.add(3)).integer.value as u32;
            (*rbuf).buf[3] = (*objs.add(4)).integer.value as u32;
        } else {
            return -EINVAL;
        }

        0
    }

    #[cfg(not(feature = "CONFIG_ACPI"))]
    {
        -ENODEV
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
