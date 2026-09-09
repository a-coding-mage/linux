/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Linux IRQ vector layout.
 *
 * There are 256 IDT entries (per CPU - each entry is 8 bytes) which can
 * be defined by Linux. They are used as a jump table by the CPU when a
 * given vector is triggered - by a CPU-external, CPU-internal or
 * software-triggered event.
 *
 * Linux sets the kernel code address each entry jumps to early during
 * bootup, and never changes them. This is the general layout of the
 * IDT entries:
 *
 *  Vectors 0 ... 31 : system traps and exceptions - hardcoded events
 *  Vectors 32 ... 127 : device interrupts
 *  Vector 128 : legacy int80 syscall interface
 *  Vectors 129 ... FIRST_SYSTEM_VECTOR-1 : device interrupts
 *  Vectors FIRST_SYSTEM_VECTOR ... 255 : special interrupts
 *
 * 64-bit x86 has per CPU IDT tables, 32-bit has one shared IDT table.
 */

/* This is used as an interrupt vector when programming the APIC. */
pub const NMI_VECTOR: u32 = 0x02;

/* IDT vectors usable for external interrupt sources start at 0x20. */
pub const FIRST_EXTERNAL_VECTOR: u32 = 0x20;

pub const IA32_SYSCALL_VECTOR: u32 = 0x80;

/* Vectors 0x30-0x3f are used for ISA interrupts. */
#[inline]
pub const fn ISA_IRQ_VECTOR(irq: u32) -> u32 {
    ((FIRST_EXTERNAL_VECTOR + 16) & !15) + irq
}

/* Special IRQ vectors used by the SMP architecture, 0xf0-0xff. */
pub const SPURIOUS_APIC_VECTOR: u32 = 0xff;
pub const ERROR_APIC_VECTOR: u32 = 0xfe;
pub const RESCHEDULE_VECTOR: u32 = 0xfd;
pub const CALL_FUNCTION_VECTOR: u32 = 0xfc;
pub const CALL_FUNCTION_SINGLE_VECTOR: u32 = 0xfb;
pub const THERMAL_APIC_VECTOR: u32 = 0xfa;
pub const THRESHOLD_APIC_VECTOR: u32 = 0xf9;
pub const REBOOT_VECTOR: u32 = 0xf8;

/* Generic system vector for platform specific use. */
pub const X86_PLATFORM_IPI_VECTOR: u32 = 0xf7;
/* IRQ work vector. */
pub const IRQ_WORK_VECTOR: u32 = 0xf6;
/* IRQ vector for PMIs when running a guest with a mediated PMU. */
pub const PERF_GUEST_MEDIATED_PMI_VECTOR: u32 = 0xf5;
pub const DEFERRED_ERROR_VECTOR: u32 = 0xf4;
/* Vector on which hypervisor callbacks will be delivered. */
pub const HYPERVISOR_CALLBACK_VECTOR: u32 = 0xf3;
/* Vector for KVM to deliver posted interrupt IPI. */
pub const POSTED_INTR_VECTOR: u32 = 0xf2;
pub const POSTED_INTR_WAKEUP_VECTOR: u32 = 0xf1;
pub const POSTED_INTR_NESTED_VECTOR: u32 = 0xf0;
pub const MANAGED_IRQ_SHUTDOWN_VECTOR: u32 = 0xef;

/* Preserved from #if IS_ENABLED(CONFIG_HYPERV). */
#[cfg(feature = "CONFIG_HYPERV")]
pub const HYPERV_REENLIGHTENMENT_VECTOR: u32 = 0xee;
#[cfg(feature = "CONFIG_HYPERV")]
pub const HYPERV_STIMER0_VECTOR: u32 = 0xed;

pub const LOCAL_TIMER_VECTOR: u32 = 0xec;
/* Posted interrupt notification vector for all device MSIs delivered to the host kernel. */
pub const POSTED_MSI_NOTIFICATION_VECTOR: u32 = 0xeb;

pub const NR_VECTORS: u32 = 256;

/* Preserved from CONFIG_X86_LOCAL_APIC conditional compilation. */
#[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
pub const FIRST_SYSTEM_VECTOR: u32 = POSTED_MSI_NOTIFICATION_VECTOR;
#[cfg(not(feature = "CONFIG_X86_LOCAL_APIC"))]
pub const FIRST_SYSTEM_VECTOR: u32 = NR_VECTORS;

pub const NR_EXTERNAL_VECTORS: u32 = FIRST_SYSTEM_VECTOR - FIRST_EXTERNAL_VECTOR;
pub const NR_SYSTEM_VECTORS: u32 = NR_VECTORS - FIRST_SYSTEM_VECTOR;

pub const NR_IRQS_LEGACY: u32 = 16;

/* NR_CPUS and MAX_IO_APICS are supplied by the including environment. */
pub const CPU_VECTOR_LIMIT: u32 = 64 * NR_CPUS;
pub const IO_APIC_VECTOR_LIMIT: u32 = 32 * MAX_IO_APICS;

/* Preserved from the source CONFIG_X86_IO_APIC/CONFIG_PCI_MSI conditionals. */
#[cfg(all(feature = "CONFIG_X86_IO_APIC", feature = "CONFIG_PCI_MSI"))]
pub const NR_IRQS: u32 = if CPU_VECTOR_LIMIT > IO_APIC_VECTOR_LIMIT {
    NR_VECTORS + CPU_VECTOR_LIMIT
} else {
    NR_VECTORS + IO_APIC_VECTOR_LIMIT
};
#[cfg(all(feature = "CONFIG_X86_IO_APIC", not(feature = "CONFIG_PCI_MSI")))]
pub const NR_IRQS: u32 = NR_VECTORS + IO_APIC_VECTOR_LIMIT;
#[cfg(all(not(feature = "CONFIG_X86_IO_APIC"), feature = "CONFIG_PCI_MSI"))]
pub const NR_IRQS: u32 = NR_VECTORS + CPU_VECTOR_LIMIT;
#[cfg(all(not(feature = "CONFIG_X86_IO_APIC"), not(feature = "CONFIG_PCI_MSI")))]
pub const NR_IRQS: u32 = NR_IRQS_LEGACY;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
