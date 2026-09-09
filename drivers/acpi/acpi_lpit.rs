// SPDX-License-Identifier: GPL-2.0-only

/*
 * acpi_lpit.c - LPIT table processing functions
 *
 * Copyright (C) 2017 Intel Corporation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/cpu.h, linux/acpi.h, asm/msr.h, asm/tsc.h, and internal.h.

#[repr(C)]
struct lpit_residency_info {
    gaddr: acpi_generic_address,
    frequency: u64,
    iomem_addr: *mut core::ffi::c_void,
}

/* Storage for an memory mapped and FFH based entries */
static mut residency_info_mem: lpit_residency_info = lpit_residency_info {
    gaddr: acpi_generic_address::default(),
    frequency: 0,
    iomem_addr: core::ptr::null_mut(),
};
static mut residency_info_ffh: lpit_residency_info = lpit_residency_info {
    gaddr: acpi_generic_address::default(),
    frequency: 0,
    iomem_addr: core::ptr::null_mut(),
};

unsafe fn lpit_read_residency_counter_us(counter: *mut u64, io_mem: bool) -> i32 {
    if io_mem {
        let mut count: u64 = 0;
        let error = acpi_os_read_iomem(
            residency_info_mem.iomem_addr,
            &mut count,
            residency_info_mem.gaddr.bit_width,
        );
        if error != 0 {
            return error;
        }

        *counter = div64_u64(
            count.wrapping_mul(1_000_000u64),
            residency_info_mem.frequency,
        );
        return 0;
    }

    let err = rdmsrq_safe(residency_info_ffh.gaddr.address, counter);
    if err == 0 {
        let mask = genmask_ull(
            residency_info_ffh.gaddr.bit_offset
                + residency_info_ffh.gaddr.bit_width
                - 1,
            residency_info_ffh.gaddr.bit_offset,
        );

        *counter &= mask;
        *counter >>= residency_info_ffh.gaddr.bit_offset;
        *counter = div64_u64(
            (*counter).wrapping_mul(1_000_000u64),
            residency_info_ffh.frequency,
        );
        return 0;
    }

    -ENODATA
}

unsafe fn low_power_idle_system_residency_us_show(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let mut counter: u64 = 0;
    let ret = lpit_read_residency_counter_us(&mut counter, true);
    if ret != 0 {
        return ret as isize;
    }

    sprintf(buf, c"%llu\n".as_ptr(), counter) as isize
}

static DEVICE_ATTR_RO(low_power_idle_system_residency_us);

unsafe fn low_power_idle_cpu_residency_us_show(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let mut counter: u64 = 0;
    let ret = lpit_read_residency_counter_us(&mut counter, false);
    if ret != 0 {
        return ret as isize;
    }

    sprintf(buf, c"%llu\n".as_ptr(), counter) as isize
}

static DEVICE_ATTR_RO(low_power_idle_cpu_residency_us);

#[no_mangle]
pub unsafe extern "C" fn lpit_read_residency_count_address(address: *mut u64) -> i32 {
    if residency_info_mem.gaddr.address == 0 {
        return -EINVAL;
    }

    *address = residency_info_mem.gaddr.address;
    0
}

EXPORT_SYMBOL_GPL!(lpit_read_residency_count_address);

unsafe fn lpit_update_residency(
    info: *mut lpit_residency_info,
    lpit_native: *mut acpi_lpit_native,
) {
    let dev_root = bus_get_dev_root(&mut cpu_subsys);

    /* Silently fail, if cpuidle attribute group is not present */
    if dev_root.is_null() {
        return;
    }

    (*info).frequency = if (*lpit_native).counter_frequency != 0 {
        (*lpit_native).counter_frequency
    } else {
        mul_u32_u32(tsc_khz, 1000u32)
    };
    if (*info).frequency == 0 {
        (*info).frequency = 1;
    }

    (*info).gaddr = (*lpit_native).residency_counter;
    if (*info).gaddr.space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY {
        (*info).iomem_addr = ioremap(
            (*info).gaddr.address,
            (*info).gaddr.bit_width / 8,
        );
        if (*info).iomem_addr.is_null() {
            put_device(dev_root);
            return;
        }

        sysfs_add_file_to_group(
            &mut (*dev_root).kobj,
            &mut dev_attr_low_power_idle_system_residency_us.attr,
            c"cpuidle".as_ptr(),
        );
    } else if (*info).gaddr.space_id == ACPI_ADR_SPACE_FIXED_HARDWARE {
        sysfs_add_file_to_group(
            &mut (*dev_root).kobj,
            &mut dev_attr_low_power_idle_cpu_residency_us.attr,
            c"cpuidle".as_ptr(),
        );
    }
    put_device(dev_root);
}

unsafe fn lpit_process(mut begin: u64, end: u64) {
    while begin.wrapping_add(core::mem::size_of::<acpi_lpit_native>() as u64) <= end {
        let lpit_native = begin as *mut acpi_lpit_native;

        if (*lpit_native).header.type_ == 0 && (*lpit_native).header.flags == 0 {
            if (*lpit_native).residency_counter.space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY
                && residency_info_mem.gaddr.address == 0
            {
                lpit_update_residency(&mut residency_info_mem, lpit_native);
            } else if (*lpit_native).residency_counter.space_id
                == ACPI_ADR_SPACE_FIXED_HARDWARE
                && residency_info_ffh.gaddr.address == 0
            {
                lpit_update_residency(&mut residency_info_ffh, lpit_native);
            }
        }
        begin = begin.wrapping_add((*lpit_native).header.length as u64);
    }
}

#[no_mangle]
pub unsafe extern "C" fn acpi_init_lpit() {
    let mut status: acpi_status = 0;
    let mut lpit: *mut acpi_table_lpit = core::ptr::null_mut();

    status = acpi_get_table(ACPI_SIG_LPIT, 0, &mut lpit as *mut _ as *mut acpi_table_header);
    if ACPI_FAILURE(status) {
        return;
    }

    lpit_process(
        lpit as u64 + core::mem::size_of::<acpi_table_lpit>() as u64,
        lpit as u64 + (*lpit).header.length as u64,
    );

    acpi_put_table(lpit as *mut acpi_table_header);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
