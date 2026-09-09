/******************************************************************************
 * arch-x86/cpuid.h
 *
 * CPUID interface to Xen.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 *
 * Copyright (c) 2007 Citrix Systems, Inc.
 *
 * Authors:
 *    Keir Fraser <keir@xen.org>
 */

// For compatibility with other hypervisor interfaces, the Xen cpuid leaves
// can be found at the first otherwise unused 0x100 aligned boundary starting
// from 0x40000000.
//
// e.g If viridian extensions are enabled for an HVM domain, the Xen cpuid
// leaves will start at 0x40000100

pub const XEN_CPUID_FIRST_LEAF: u32 = 0x40000000;
pub const fn XEN_CPUID_LEAF(i: u32) -> u32 {
    XEN_CPUID_FIRST_LEAF + i
}

// Leaf 1 (0x40000x00)
// EAX: Largest Xen-information leaf. All leaves up to an including @EAX
//      are supported by the Xen host.
// EBX-EDX: "XenVMMXenVMM" signature, allowing positive identification
//      of a Xen host.
pub const XEN_CPUID_SIGNATURE_EBX: u32 = 0x566e6558; // "XenV"
pub const XEN_CPUID_SIGNATURE_ECX: u32 = 0x65584d4d; // "MMXe"
pub const XEN_CPUID_SIGNATURE_EDX: u32 = 0x4d4d566e; // "nVMM"

// Leaf 2 (0x40000x01)
// EAX[31:16]: Xen major version.
// EAX[15: 0]: Xen minor version.
// EBX-EDX: Reserved (currently all zeroes).

// Leaf 3 (0x40000x02)
// EAX: Number of hypercall transfer pages. This register is always guaranteed
//      to specify one hypercall page.
// EBX: Base address of Xen-specific MSRs.
// ECX: Features 1. Unused bits are set to zero.
// EDX: Features 2. Unused bits are set to zero.

// Does the host support MMU_PT_UPDATE_PRESERVE_AD for this guest?
pub const _XEN_CPUID_FEAT1_MMU_PT_UPDATE_PRESERVE_AD: u32 = 0;
pub const XEN_CPUID_FEAT1_MMU_PT_UPDATE_PRESERVE_AD: u32 = 1u32 << 0;

// Leaf 4 (0x40000x03)
// Sub-leaf 0: EAX: bit 0: emulated tsc
//                  bit 1: host tsc is known to be reliable
//                  bit 2: RDTSCP instruction available
//             EBX: tsc_mode: 0=default (emulate if necessary), 1=emulate,
//                            2=no emulation, 3=no emulation + TSC_AUX support
//             ECX: guest tsc frequency in kHz
//             EDX: guest tsc incarnation (migration count)
// Sub-leaf 1: EAX: tsc offset low part
//             EBX: tsc offset high part
//             ECX: multiplicator for tsc->ns conversion
//             EDX: shift amount for tsc->ns conversion
// Sub-leaf 2: EAX: host tsc frequency in kHz

pub const XEN_CPUID_TSC_EMULATED: u32 = 1u32 << 0;
pub const XEN_CPUID_HOST_TSC_RELIABLE: u32 = 1u32 << 1;
pub const XEN_CPUID_RDTSCP_INSTR_AVAIL: u32 = 1u32 << 2;

pub const XEN_CPUID_TSC_MODE_DEFAULT: u32 = 0;
pub const XEN_CPUID_TSC_MODE_ALWAYS_EMULATE: u32 = 1u32;
pub const XEN_CPUID_TSC_MODE_NEVER_EMULATE: u32 = 2u32;
pub const XEN_CPUID_TSC_MODE_PVRDTSCP: u32 = 3u32;

// Leaf 5 (0x40000x04)
// HVM-specific features
// Sub-leaf 0: EAX: Features
// Sub-leaf 0: EBX: vcpu id (iff EAX has XEN_HVM_CPUID_VCPU_ID_PRESENT flag)
// Sub-leaf 0: ECX: domain id (iff EAX has XEN_HVM_CPUID_DOMID_PRESENT flag)
pub const XEN_HVM_CPUID_APIC_ACCESS_VIRT: u32 = 1u32 << 0; // Virtualized APIC registers
pub const XEN_HVM_CPUID_X2APIC_VIRT: u32 = 1u32 << 1; // Virtualized x2APIC accesses
// Memory mapped from other domains has valid IOMMU entries
pub const XEN_HVM_CPUID_IOMMU_MAPPINGS: u32 = 1u32 << 2;
pub const XEN_HVM_CPUID_VCPU_ID_PRESENT: u32 = 1u32 << 3; // vcpu id is present in EBX
pub const XEN_HVM_CPUID_DOMID_PRESENT: u32 = 1u32 << 4; // domid is present in ECX
// With interrupt format set to 0 (non-remappable) bits 55:49 from the
// IO-APIC RTE and bits 11:5 from the MSI address can be used to store
// high bits for the Destination ID. This expands the Destination ID
// field from 8 to 15 bits, allowing to target APIC IDs up 32768.
pub const XEN_HVM_CPUID_EXT_DEST_ID: u32 = 1u32 << 5;
// Per-vCPU event channel upcalls work correctly with physical IRQs
// bound to event channels.
pub const XEN_HVM_CPUID_UPCALL_VECTOR: u32 = 1u32 << 6;

// Leaf 6 (0x40000x05)
// PV-specific parameters
// Sub-leaf 0: EAX: max available sub-leaf
// Sub-leaf 0: EBX: bits 0-7: max machine address width

// Max. address width in bits taking memory hotplug into account.
pub const XEN_CPUID_MACHINE_ADDRESS_WIDTH_MASK: u32 = 0xffu32 << 0;

pub const XEN_CPUID_MAX_NUM_LEAVES: u32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
