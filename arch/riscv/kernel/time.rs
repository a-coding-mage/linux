// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Regents of the University of California
 * Copyright (C) 2017 SiFive
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_table_rhct {
    pub time_base_freq: u64,
}

#[repr(C)]
pub struct acpi_table_header {
    _private: [u8; 0],
}

type acpi_status = u32;
type u32_t = u32;

extern "C" {
    static mut acpi_disabled: bool;
    static mut lpj_fine: usize;

    fn of_find_node_by_path(path: *const u8) -> *mut device_node;
    fn of_property_read_u32(
        node: *const device_node,
        property: *const u8,
        value: *mut u32_t,
    ) -> i32;
    fn panic(format: *const u8) -> !;
    fn of_node_put(node: *mut device_node);
    fn of_clk_init(table: *const core::ffi::c_void);
    fn acpi_get_table(
        signature: u32,
        instance: u32,
        table: *mut *mut acpi_table_header,
    ) -> acpi_status;
    fn acpi_put_table(table: *mut acpi_table_header);
    fn timer_probe();
    fn tick_setup_hrtimer_broadcast();
    fn pv_time_init();
}

// ACPI_SIG_RHCT and ACPI_FAILURE are supplied by the ACPI headers.

#[no_mangle]
pub static mut riscv_timebase: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn time_init() {
    let mut cpu: *mut device_node;
    let mut rhct: *mut acpi_table_rhct;
    let mut status: acpi_status;
    let mut prop: u32_t = 0;

    if acpi_disabled {
        cpu = of_find_node_by_path(b"/cpus\0".as_ptr());
        if cpu.is_null()
            || of_property_read_u32(cpu, b"timebase-frequency\0".as_ptr(), &mut prop) != 0
        {
            panic(b"RISC-V system with no 'timebase-frequency' in DTS\n\0".as_ptr());
        }

        of_node_put(cpu);
        riscv_timebase = prop as usize;
        of_clk_init(core::ptr::null());
    } else {
        status = acpi_get_table(
            ACPI_SIG_RHCT,
            0,
            &mut rhct as *mut *mut acpi_table_rhct as *mut *mut acpi_table_header,
        );
        if ACPI_FAILURE(status) {
            panic(b"RISC-V ACPI system with no RHCT table\n\0".as_ptr());
        }

        riscv_timebase = (*rhct).time_base_freq as usize;
        acpi_put_table(rhct as *mut acpi_table_header);
    }

    lpj_fine = riscv_timebase / HZ;

    timer_probe();

    tick_setup_hrtimer_broadcast();

    pv_time_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
