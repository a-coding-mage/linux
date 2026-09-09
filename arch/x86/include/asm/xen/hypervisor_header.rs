/*
 * hypervisor.h
 *
 * Linux-specific hypervisor handling.
 *
 * Copyright (c) 2002-2004, K A Fraser
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License version 2
 * as published by the Free Software Foundation; or, when distributed
 * separately from the Linux kernel or incorporated into other
 * software packages, subject to the following license:
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this source file (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use, copy, modify,
 * merge, publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

// C header dependencies: <asm/bug.h>, <asm/cpuid/api.h>, and <asm/processor.h>.

pub enum shared_info {}
pub enum start_info {}
pub enum pci_dev {}
pub enum boot_params {}

extern "C" {
    pub static mut HYPERVISOR_shared_info: *mut shared_info;
    pub static mut xen_start_info: *mut start_info;

    fn cpuid_base_hypervisor(signature: *const u8, leaf: i32) -> u32;
    fn BUG() -> !;
}

pub const XEN_SIGNATURE: &[u8] = b"XenVMMXenVMM\0";

#[inline]
pub unsafe fn xen_cpuid_base() -> u32 {
    cpuid_base_hypervisor(XEN_SIGNATURE.as_ptr(), 2)
}

#[cfg(CONFIG_XEN_PV_DOM0)]
extern "C" {
    pub fn xen_initdom_restore_msi(dev: *mut pci_dev) -> bool;
}

#[cfg(not(CONFIG_XEN_PV_DOM0))]
#[inline]
pub unsafe fn xen_initdom_restore_msi(_dev: *mut pci_dev) -> bool {
    true
}

#[cfg(CONFIG_HOTPLUG_CPU)]
extern "C" {
    pub fn xen_arch_register_cpu(num: i32);
    pub fn xen_arch_unregister_cpu(num: i32);
}

#[cfg(CONFIG_PVH)]
extern "C" {
    pub fn xen_pvh_init(boot_params: *mut boot_params);
    pub fn mem_map_via_hcall(boot_params_p: *mut boot_params);
}

extern "C" {
    pub fn xen_is_cpu_lazy_mode() -> bool;
}

#[cfg(all(CONFIG_XEN_DOM0, CONFIG_ACPI))]
extern "C" {
    pub fn xen_sanitize_proc_cap_bits(buf: *mut u32);
}

#[cfg(not(all(CONFIG_XEN_DOM0, CONFIG_ACPI)))]
#[inline]
pub unsafe fn xen_sanitize_proc_cap_bits(_buf: *mut u32) {
    BUG();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
