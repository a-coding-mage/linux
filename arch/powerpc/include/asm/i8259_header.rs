/* SPDX-License-Identifier: GPL-2.0 */

// Original header guard: _ASM_POWERPC_I8259_H
// These declarations are available only when building the kernel (__KERNEL__).

// Dependency supplied by the surrounding translation:
// struct device_node;
// struct irq_domain;

unsafe extern "C" {
    pub fn i8259_init(node: *mut device_node, intack_addr: ::core::ffi::c_ulong);
    pub fn i8259_irq() -> ::core::ffi::c_uint;
    // Original declaration carried the kernel __init attribute.
    pub fn i8259_get_host() -> *mut irq_domain;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
