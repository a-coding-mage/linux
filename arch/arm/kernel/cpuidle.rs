// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Linaro Ltd.
 */

// Dependencies supplied by the kernel headers and other translation units.

extern "C" {
    static mut __cpuidle_method_of_table: [of_cpuidle_method; 0];
}

#[repr(C)]
pub struct of_cpuidle_method {
    pub method: *const core::ffi::c_char,
    pub ops: *const cpuidle_ops,
}

#[repr(C)]
pub struct cpuidle_ops {
    pub init: Option<unsafe extern "C" fn(*mut device_node, i32) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(i32) -> i32>,
}

#[repr(C)]
pub struct cpuidle_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpuidle_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    fn cpu_do_idle();
    fn smp_processor_id() -> i32;
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
    fn of_get_property(
        node: *mut device_node,
        name: *const core::ffi::c_char,
        length: *mut i32,
    ) -> *const core::ffi::c_char;
    fn of_cpu_device_node_get(cpu: i32) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
}

extern "C" {
    static mut cpuidle_ops: [cpuidle_ops; NR_CPUS];
}

// Build-time kernel constants and logging facilities are supplied externally.
extern "C" {
    static NR_CPUS: usize;
}

/// arm_cpuidle_simple_enter() - a wrapper to cpu_do_idle()
/// @dev: not used
/// @drv: not used
/// @index: not used
///
/// A trivial wrapper to allow the cpu_do_idle function to be assigned as a
/// cpuidle callback by matching the function signature.
///
/// Returns the index passed as parameter
pub unsafe extern "C" fn arm_cpuidle_simple_enter(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    index: i32,
) -> i32 {
    cpu_do_idle();
    index
}

/// arm_cpuidle_suspend() - function to enter low power idle states
/// @index: an integer used as an identifier for the low level PM callbacks
///
/// This function calls the underlying arch specific low level PM code as
/// registered at the init time.
///
/// Returns the result of the suspend callback.
pub unsafe extern "C" fn arm_cpuidle_suspend(index: i32) -> i32 {
    let cpu = smp_processor_id() as usize;
    ((*cpuidle_ops.as_ptr().add(cpu)).suspend.unwrap())(index)
}

/// arm_cpuidle_get_ops() - find a registered cpuidle_ops by name
/// @method: the method name
///
/// Search in the __cpuidle_method_of_table array the cpuidle ops matching the
/// method name.
///
/// Returns a struct cpuidle_ops pointer, NULL if not found.
unsafe fn arm_cpuidle_get_ops(method: *const core::ffi::c_char) -> *const cpuidle_ops {
    let mut m = __cpuidle_method_of_table.as_mut_ptr();
    while !(*m).method.is_null() {
        if strcmp((*m).method, method) == 0 {
            return (*m).ops;
        }
        m = m.add(1);
    }
    core::ptr::null()
}

/// arm_cpuidle_read_ops() - Initialize the cpuidle ops with the device tree
/// @dn: a pointer to a struct device node corresponding to a cpu node
/// @cpu: the cpu identifier
///
/// Get the method name defined in the 'enable-method' property, retrieve the
/// associated cpuidle_ops and do a struct copy. This copy is needed because all
/// cpuidle_ops are tagged __initconst and will be unloaded after the init
/// process.
unsafe fn arm_cpuidle_read_ops(dn: *mut device_node, cpu: i32) -> i32 {
    let property = b"enable-method\0";
    let enable_method = of_get_property(dn, property.as_ptr() as *const _, core::ptr::null_mut());
    if enable_method.is_null() {
        return -2; // -ENOENT
    }

    let ops = arm_cpuidle_get_ops(enable_method);
    if ops.is_null() {
        return -95; // -EOPNOTSUPP
    }
    if (*ops).init.is_none() || (*ops).suspend.is_none() {
        return -95; // -EOPNOTSUPP
    }

    cpuidle_ops[cpu as usize] = *ops;
    0
}

/// arm_cpuidle_init() - Initialize cpuidle_ops for a specific cpu
/// @cpu: the cpu to be initialized
pub unsafe extern "C" fn arm_cpuidle_init(cpu: i32) -> i32 {
    let cpu_node = of_cpu_device_node_get(cpu);
    if cpu_node.is_null() {
        return -19; // -ENODEV
    }

    let mut ret = arm_cpuidle_read_ops(cpu_node, cpu);
    if ret == 0 {
        ret = ((*cpuidle_ops.as_ptr().add(cpu as usize)).init.unwrap())(cpu_node, cpu);
    }
    of_node_put(cpu_node);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
