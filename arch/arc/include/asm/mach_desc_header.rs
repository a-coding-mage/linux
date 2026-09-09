/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Synopsys, Inc. (www.synopsys.com)
 *
 * based on METAG mach/arch.h (which in turn was based on ARM)
 */

/**
 * struct machine_desc - Board specific callbacks, called from ARC common code
 *	Provided by each ARC board using MACHINE_START()/MACHINE_END(), so
 *	a multi-platform kernel builds with array of such descriptors.
 *	We extend the early DT scan to also match the DT's "compatible" string
 *	against the @dt_compat of all such descriptors, and one with highest
 *	"DT score" is selected as global @machine_desc.
 *
 * @name:              Board/SoC name
 * @dt_compat:         Array of device tree 'compatible' strings
 *                     (XXX: although only 1st entry is looked at)
 * @init_early:        Very early callback [called from setup_arch()]
 * @init_per_cpu:      for each CPU as it is coming up (SMP as well as UP)
 *                     [(M):init_IRQ(), (o):start_kernel_secondary()]
 * @init_machine:      arch initcall level callback (e.g. populate static
 *                     platform devices or parse Devicetree)
 * @init_late:         Late initcall level callback
 */
#[repr(C)]
pub struct machine_desc {
    pub name: *const core::ffi::c_char,
    pub dt_compat: *const *const core::ffi::c_char,
    pub init_early: Option<unsafe extern "C" fn()>,
    pub init_per_cpu: Option<unsafe extern "C" fn(core::ffi::c_uint)>,
    pub init_machine: Option<unsafe extern "C" fn()>,
    pub init_late: Option<unsafe extern "C" fn()>,
}

/* Current machine - only accessible during boot. */
unsafe extern "C" {
    pub static machine_desc: *const machine_desc;
}

/* Machine type table - also only accessible during boot. */
unsafe extern "C" {
    pub static __arch_info_begin: [machine_desc; 0];
    pub static __arch_info_end: [machine_desc; 0];
}

/*
 * Set of macros to define architecture features.
 * This is built into a table by the linker.
 *
 * The C form permits designated initializer lines between MACHINE_START and
 * MACHINE_END. Rust callers provide those fields as `field: value` pairs.
 */
#[macro_export]
macro_rules! MACHINE_START {
    ($type:ident, $name:expr $(, $field:ident : $value:expr)* $(,)?) => {
        #[used]
        #[link_section = ".arch.info.init"]
        static __mach_desc_$type: $crate::machine_desc = $crate::machine_desc {
            name: $name,
            dt_compat: core::ptr::null(),
            init_early: None,
            init_per_cpu: None,
            init_machine: None,
            init_late: None,
            $( $field: $value, )*
        };
    };
}

#[macro_export]
macro_rules! MACHINE_END {
    () => {};
}

unsafe extern "C" {
    pub fn setup_machine_fdt(dt: *mut core::ffi::c_void) -> *const machine_desc;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
