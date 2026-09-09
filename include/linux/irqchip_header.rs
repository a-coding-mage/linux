/*
 * Copyright (C) 2012 Thomas Petazzoni
 *
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is", without any
 * warranty of any kind, whether express or implied.
 */

// Dependencies supplied by the corresponding Linux interfaces:
// linux/acpi.h, linux/module.h, linux/of.h, linux/of_irq.h,
// linux/platform_device.h

pub type PlatformIrqProbeT = unsafe extern "C" fn(
    pdev: *mut platform_device,
    node: *mut device_node,
) -> ::core::ffi::c_int;

/* Undefined on purpose. */
extern "C" {
    pub static mut typecheck_irq_init_cb: of_irq_init_cb_t;
    pub static mut typecheck_irq_probe: PlatformIrqProbeT;
}

// The __typecheck expressions are retained as macro invocations; their
// definitions are supplied by the Linux compatibility layer.
#[macro_export]
macro_rules! typecheck_irq_init_cb {
    ($fn:expr) => {{
        if unsafe { __typecheck(typecheck_irq_init_cb, &$fn) } {
            $fn
        } else {
            $fn
        }
    }};
}

#[macro_export]
macro_rules! typecheck_irq_probe {
    ($fn:expr) => {{
        if unsafe { __typecheck(typecheck_irq_probe, &$fn) } {
            $fn
        } else {
            $fn
        }
    }};
}

/*
 * Declare the association between a DT compatible string and its
 * initialization function.
 */
#[macro_export]
macro_rules! IRQCHIP_DECLARE {
    ($name:ident, $compat:expr, $fn:expr) => {
        OF_DECLARE_2!(irqchip, $name, $compat, typecheck_irq_init_cb!($fn));
    };
}

extern "C" {
    pub fn platform_irqchip_probe(pdev: *mut platform_device) -> ::core::ffi::c_int;
}

/*
 * The C token-pasting in these driver macros is represented by the
 * corresponding Rust macro arguments; generated identifiers are supplied by
 * the surrounding Linux compatibility layer.
 */
#[macro_export]
macro_rules! IRQCHIP_PLATFORM_DRIVER_BEGIN {
    ($drv_name:ident) => {
        static $drv_name##_irqchip_match_table: [of_device_id; 1] = [];
    };
}

#[macro_export]
macro_rules! IRQCHIP_MATCH {
    ($compat:expr, $fn:expr) => {
        of_device_id {
            compatible: $compat,
            data: typecheck_irq_probe!($fn),
        }
    };
}

#[macro_export]
macro_rules! IRQCHIP_PLATFORM_DRIVER_END {
    ($drv_name:ident $(, $arg:tt)*) => {
        MODULE_DEVICE_TABLE!(of, $drv_name##_irqchip_match_table);
        static mut $drv_name##_driver: platform_driver = platform_driver {
            probe: if IS_ENABLED!(CONFIG_IRQCHIP) {
                Some(platform_irqchip_probe)
            } else {
                None
            },
            driver: device_driver {
                name: stringify!($drv_name),
                owner: THIS_MODULE,
                of_match_table: $drv_name##_irqchip_match_table,
                suppress_bind_attrs: true,
                $($arg)*
            },
        };
        builtin_platform_driver!($drv_name##_driver);
    };
}

/*
 * Declare the association between an ACPI MADT subtable and its
 * initialization function.
 */
#[macro_export]
macro_rules! IRQCHIP_ACPI_DECLARE {
    ($name:ident, $subtable:expr, $validate:expr, $data:expr, $fn:expr) => {
        ACPI_DECLARE_SUBTABLE_PROBE_ENTRY!(
            irqchip, $name, ACPI_SIG_MADT, $subtable, $validate, $data, $fn
        );
    };
}

#[cfg(CONFIG_IRQCHIP)]
extern "C" {
    pub fn irqchip_init();
}

#[cfg(not(CONFIG_IRQCHIP))]
#[inline]
pub unsafe fn irqchip_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
