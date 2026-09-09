// SPDX-License-Identifier: GPL-2.0-only
/*
 * Implement CPPC FFH helper routines for RISC-V.
 *
 * Copyright (C) 2024 Ventana Micro Systems Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

const SBI_EXT_CPPC: u64 = 0x4350_5043;

/* CPPC interfaces defined in SBI spec */
const SBI_CPPC_PROBE: u64 = 0x0;
const SBI_CPPC_READ: u64 = 0x1;
const SBI_CPPC_READ_HI: u64 = 0x2;
const SBI_CPPC_WRITE: u64 = 0x3;

/* RISC-V FFH definitions from RISC-V FFH spec */
const FFH_CPPC_SBI: u64 = 0x1;
const FFH_CPPC_CSR: u64 = 0x2;

#[repr(C)]
struct SbiCppcData {
    val: u64,
    reg: u32,
    ret: Sbiret,
}

static mut CPPC_EXT_PRESENT: bool = false;

unsafe fn sbi_cppc_init() -> i32 {
    if sbi_spec_version >= sbi_mk_version(2, 0)
        && sbi_probe_extension(SBI_EXT_CPPC) > 0
    {
        CPPC_EXT_PRESENT = true;
    } else {
        CPPC_EXT_PRESENT = false;
    }

    0
}

// Corresponds to device_initcall(sbi_cppc_init).

unsafe fn sbi_cppc_read(read_data: *mut core::ffi::c_void) {
    let data = &mut *(read_data as *mut SbiCppcData);

    data.ret = sbi_ecall(SBI_EXT_CPPC, SBI_CPPC_READ, data.reg as u64, 0, 0, 0, 0, 0);
}

unsafe fn sbi_cppc_write(write_data: *mut core::ffi::c_void) {
    let data = &mut *(write_data as *mut SbiCppcData);

    data.ret = sbi_ecall(SBI_EXT_CPPC, SBI_CPPC_WRITE, data.reg as u64, data.val, 0, 0, 0, 0);
}

unsafe fn cppc_ffh_csr_read(read_data: *mut core::ffi::c_void) {
    let data = &mut *(read_data as *mut SbiCppcData);

    match data.reg {
        // Support only TIME CSR for now
        CSR_TIME => {
            data.ret.value = csr_read(CSR_TIME);
            data.ret.error = 0;
        }
        _ => {
            data.ret.error = -EINVAL;
        }
    }
}

unsafe fn cppc_ffh_csr_write(write_data: *mut core::ffi::c_void) {
    let data = &mut *(write_data as *mut SbiCppcData);

    data.ret.error = -EINVAL;
}

/*
 * Refer to drivers/acpi/cppc_acpi.c for the description of the functions
 * below.
 */
pub unsafe fn cpc_ffh_supported() -> bool {
    true
}

pub unsafe fn cpc_read_ffh(cpu: i32, reg: *const CpcReg, val: *mut u64) -> i32 {
    let mut data: SbiCppcData = core::mem::zeroed();
    let address = (*reg).address;

    if WARN_ON_ONCE(irqs_disabled()) {
        return -EPERM;
    }

    if ((address & 0xf000_0000_0000_0000) >> 60) == FFH_CPPC_SBI {
        if !CPPC_EXT_PRESENT {
            return -EINVAL;
        }

        data.reg = (address & 0xffff_ffff) as u32;

        smp_call_function_single(cpu, sbi_cppc_read, &mut data as *mut _ as *mut core::ffi::c_void, 1);

        *val = data.ret.value;

        if data.ret.error != 0 { sbi_err_map_linux_errno(data.ret.error) } else { 0 }
    } else if ((address & 0xf000_0000_0000_0000) >> 60) == FFH_CPPC_CSR {
        data.reg = (address & 0xfff) as u32;

        smp_call_function_single(cpu, cppc_ffh_csr_read, &mut data as *mut _ as *mut core::ffi::c_void, 1);

        *val = data.ret.value;

        data.ret.error
    } else {
        -EINVAL
    }
}

pub unsafe fn cpc_write_ffh(cpu: i32, reg: *const CpcReg, val: u64) -> i32 {
    let mut data: SbiCppcData = core::mem::zeroed();
    let address = (*reg).address;

    if WARN_ON_ONCE(irqs_disabled()) {
        return -EPERM;
    }

    if ((address & 0xf000_0000_0000_0000) >> 60) == FFH_CPPC_SBI {
        if !CPPC_EXT_PRESENT {
            return -EINVAL;
        }

        data.reg = (address & 0xffff_ffff) as u32;
        data.val = val;

        smp_call_function_single(cpu, sbi_cppc_write, &mut data as *mut _ as *mut core::ffi::c_void, 1);

        if data.ret.error != 0 { sbi_err_map_linux_errno(data.ret.error) } else { 0 }
    } else if ((address & 0xf000_0000_0000_0000) >> 60) == FFH_CPPC_CSR {
        data.reg = (address & 0xfff) as u32;
        data.val = val;

        smp_call_function_single(cpu, cppc_ffh_csr_write, &mut data as *mut _ as *mut core::ffi::c_void, 1);

        data.ret.error
    } else {
        -EINVAL
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
