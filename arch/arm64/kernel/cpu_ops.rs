// SPDX-License-Identifier: GPL-2.0-only
/*
 * CPU kernel entry/exit control
 *
 * Copyright (C) 2013 ARM Ltd.
 */

// C dependencies: linux/acpi.h, linux/cache.h, linux/errno.h, linux/of.h,
// linux/string.h, asm/acpi.h, asm/cpu_ops.h, asm/smp_plat.h

extern "C" {
    static smp_spin_table_ops: cpu_operations;
    // Preserved from CONFIG_ARM64_ACPI_PARKING_PROTOCOL.
    #[cfg(CONFIG_ARM64_ACPI_PARKING_PROTOCOL)]
    static acpi_parking_protocol_ops: cpu_operations;
    static cpu_psci_ops: cpu_operations;
}

#[repr(C)]
pub struct cpu_operations {
    pub name: *const core::ffi::c_char,
}

// Supplied by the surrounding kernel dependencies.
extern "C" {
    static acpi_disabled: bool;
    static mut cpu_ops: [*const cpu_operations; NR_CPUS];

    fn of_get_cpu_node(cpu: core::ffi::c_int, thread: *mut core::ffi::c_int)
        -> *mut device_node;
    fn of_get_property(
        node: *const device_node,
        name: *const core::ffi::c_char,
        length: *mut core::ffi::c_int,
    ) -> *const core::ffi::c_char;
    fn of_node_put(node: *mut device_node);
    fn acpi_get_enable_method(cpu: core::ffi::c_int) -> *const core::ffi::c_char;
    fn strcmp(
        left: *const core::ffi::c_char,
        right: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn pr_err(format: *const core::ffi::c_char, ...);
    fn pr_warn(format: *const core::ffi::c_char, ...);
}

pub const NR_CPUS: usize = 0;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

static mut DT_SUPPORTED_CPU_OPS: [*const cpu_operations; 3] = [
    unsafe { &smp_spin_table_ops },
    unsafe { &cpu_psci_ops },
    core::ptr::null(),
];

#[cfg(CONFIG_ARM64_ACPI_PARKING_PROTOCOL)]
static mut ACPI_SUPPORTED_CPU_OPS: [*const cpu_operations; 3] = [
    unsafe { &acpi_parking_protocol_ops },
    unsafe { &cpu_psci_ops },
    core::ptr::null(),
];

#[cfg(not(CONFIG_ARM64_ACPI_PARKING_PROTOCOL))]
static mut ACPI_SUPPORTED_CPU_OPS: [*const cpu_operations; 2] = [
    unsafe { &cpu_psci_ops },
    core::ptr::null(),
];

unsafe fn cpu_get_ops(name: *const core::ffi::c_char) -> *const cpu_operations {
    let mut ops: *const *const cpu_operations = if acpi_disabled {
        DT_SUPPORTED_CPU_OPS.as_ptr()
    } else {
        ACPI_SUPPORTED_CPU_OPS.as_ptr()
    };

    while !(*ops).is_null() {
        if strcmp(name, (**ops).name) == 0 {
            return *ops;
        }
        ops = ops.add(1);
    }

    core::ptr::null()
}

unsafe fn cpu_read_enable_method(cpu: core::ffi::c_int) -> *const core::ffi::c_char {
    let enable_method: *const core::ffi::c_char;

    if acpi_disabled {
        let dn = of_get_cpu_node(cpu, core::ptr::null_mut());

        if dn.is_null() {
            if cpu == 0 {
                pr_err(b"Failed to find device node for boot cpu\n\0".as_ptr() as *const _);
            }
            return core::ptr::null();
        }

        enable_method = of_get_property(
            dn,
            b"enable-method\0".as_ptr() as *const _,
            core::ptr::null_mut(),
        );
        if enable_method.is_null() {
            /*
             * The boot CPU may not have an enable method (e.g.
             * when spin-table is used for secondaries).
             * Don't warn spuriously.
             */
            if cpu != 0 {
                pr_err(b"%pOF: missing enable-method property\n\0".as_ptr() as *const _, dn);
            }
        }
        of_node_put(dn);
    } else {
        enable_method = acpi_get_enable_method(cpu);
        if enable_method.is_null() {
            /*
             * In ACPI systems the boot CPU does not require
             * checking the enable method since for some
             * boot protocol (ie parking protocol) it need not
             * be initialized. Don't warn spuriously.
             */
            if cpu != 0 {
                pr_err(b"Unsupported ACPI enable-method\n\0".as_ptr() as *const _);
            }
        }
    }

    enable_method
}

/*
 * Read a cpu's enable method and record it in cpu_ops.
 */
pub unsafe fn init_cpu_ops(cpu: core::ffi::c_int) -> core::ffi::c_int {
    let enable_method = cpu_read_enable_method(cpu);

    if enable_method.is_null() {
        return -19; // -ENODEV
    }

    cpu_ops[cpu as usize] = cpu_get_ops(enable_method);
    if cpu_ops[cpu as usize].is_null() {
        pr_warn(
            b"Unsupported enable-method: %s\n\0".as_ptr() as *const _,
            enable_method,
        );
        return -95; // -EOPNOTSUPP
    }

    0
}

pub unsafe fn get_cpu_ops(cpu: core::ffi::c_int) -> *const cpu_operations {
    cpu_ops[cpu as usize]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
