/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Generic Hypervisor support
 *     Juergen Gross <jgross@suse.com>
 *
 * The C header guard and include directives are omitted; their declarations
 * are supplied by the corresponding Rust dependencies.
 */

#[cfg(feature = "CONFIG_X86")]
#[inline]
pub unsafe fn hypervisor_pin_vcpu(cpu: i32) {
    x86_platform.hyper.pin_vcpu(cpu);
}

/* !CONFIG_X86 */
#[cfg(not(feature = "CONFIG_X86"))]
#[inline]
pub fn hypervisor_pin_vcpu(_cpu: i32) {
}

#[cfg(not(feature = "CONFIG_X86"))]
#[inline]
pub unsafe fn jailhouse_paravirt() -> bool {
    of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "jailhouse,cell")
}

#[inline]
pub unsafe fn hypervisor_isolated_pci_functions() -> bool {
    if cfg!(feature = "CONFIG_S390") {
        return true;
    }

    if cfg!(feature = "CONFIG_LOONGARCH") {
        return true;
    }

    jailhouse_paravirt()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
