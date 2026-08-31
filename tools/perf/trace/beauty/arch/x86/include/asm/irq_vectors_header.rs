/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent from C header: #include <linux/threads.h> */

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
 *  Vectors   0 ...  31 : system traps and exceptions - hardcoded events
 *  Vectors  32 ... 127 : device interrupts
 *  Vector  128         : legacy int80 syscall interface
 *  Vectors 129 ... FIRST_SYSTEM_VECTOR-1 : device interrupts
 *  Vectors FIRST_SYSTEM_VECTOR ... 255   : special interrupts
 *
 * 64-bit x86 has per CPU IDT tables, 32-bit has one shared IDT table.
 *
 * This file enumerates the exact layout of them:
 */

/* This is used as an interrupt vector when programming the APIC. */
pub const NMI_VECTOR: usize = 0x02;

/*
 * IDT vectors usable for external interrupt sources start at 0x20.
 * (0x80 is the syscall vector, 0x30-0x3f are for ISA)
 */
pub const FIRST_EXTERNAL_VECTOR: usize = 0x20;

pub const IA32_SYSCALL_VECTOR: usize = 0x80;

/*
 * Vectors 0x30-0x3f are used for ISA interrupts.
 *   round up to the next 16-vector boundary
 */
pub const fn ISA_IRQ_VECTOR(irq: usize) -> usize {
    (((FIRST_EXTERNAL_VECTOR + 16) & !15) + irq)
}

/*
 * Special IRQ vectors used by the SMP architecture, 0xf0-0xff
 *
 *  some of the following vectors are 'rare', they are merged
 *  into a single vector (CALL_FUNCTION_VECTOR) to save vector space.
 *  TLB, reschedule and local APIC vectors are performance-critical.
 */

pub const SPURIOUS_APIC_VECTOR: usize = 0xff;
/*
 * Sanity check
 *
 * Original C preprocessor check:
 * #if ((SPURIOUS_APIC_VECTOR & 0x0F) != 0x0F)
 * # error SPURIOUS_APIC_VECTOR definition error
 * #endif
 */
const _: [(); 0] = [(); ((SPURIOUS_APIC_VECTOR & 0x0F) != 0x0F) as usize];

pub const ERROR_APIC_VECTOR: usize = 0xfe;
pub const RESCHEDULE_VECTOR: usize = 0xfd;
pub const CALL_FUNCTION_VECTOR: usize = 0xfc;
pub const CALL_FUNCTION_SINGLE_VECTOR: usize = 0xfb;
pub const THERMAL_APIC_VECTOR: usize = 0xfa;
pub const THRESHOLD_APIC_VECTOR: usize = 0xf9;
pub const REBOOT_VECTOR: usize = 0xf8;

/*
 * Generic system vector for platform specific use
 */
pub const X86_PLATFORM_IPI_VECTOR: usize = 0xf7;

/*
 * IRQ work vector:
 */
pub const IRQ_WORK_VECTOR: usize = 0xf6;

/* IRQ vector for PMIs when running a guest with a mediated PMU. */
pub const PERF_GUEST_MEDIATED_PMI_VECTOR: usize = 0xf5;

pub const DEFERRED_ERROR_VECTOR: usize = 0xf4;

/* Vector on which hypervisor callbacks will be delivered */
pub const HYPERVISOR_CALLBACK_VECTOR: usize = 0xf3;

/* Vector for KVM to deliver posted interrupt IPI */
pub const POSTED_INTR_VECTOR: usize = 0xf2;
pub const POSTED_INTR_WAKEUP_VECTOR: usize = 0xf1;
pub const POSTED_INTR_NESTED_VECTOR: usize = 0xf0;

pub const MANAGED_IRQ_SHUTDOWN_VECTOR: usize = 0xef;

/* Original condition: #if IS_ENABLED(CONFIG_HYPERV) */
#[cfg(CONFIG_HYPERV)]
pub const HYPERV_REENLIGHTENMENT_VECTOR: usize = 0xee;
#[cfg(CONFIG_HYPERV)]
pub const HYPERV_STIMER0_VECTOR: usize = 0xed;

pub const LOCAL_TIMER_VECTOR: usize = 0xec;

/*
 * Posted interrupt notification vector for all device MSIs delivered to
 * the host kernel.
 */
pub const POSTED_MSI_NOTIFICATION_VECTOR: usize = 0xeb;

pub const NR_VECTORS: usize = 256;

/* Original condition: #ifdef CONFIG_X86_LOCAL_APIC */
#[cfg(CONFIG_X86_LOCAL_APIC)]
pub const FIRST_SYSTEM_VECTOR: usize = POSTED_MSI_NOTIFICATION_VECTOR;
#[cfg(not(CONFIG_X86_LOCAL_APIC))]
pub const FIRST_SYSTEM_VECTOR: usize = NR_VECTORS;

pub const NR_EXTERNAL_VECTORS: usize = FIRST_SYSTEM_VECTOR - FIRST_EXTERNAL_VECTOR;
pub const NR_SYSTEM_VECTORS: usize = NR_VECTORS - FIRST_SYSTEM_VECTOR;

/*
 * Size the maximum number of interrupts.
 *
 * If the irq_desc[] array has a sparse layout, we can size things
 * generously - it scales up linearly with the maximum number of CPUs,
 * and the maximum number of IO-APICs, whichever is higher.
 *
 * In other cases we size more conservatively, to not create too large
 * static arrays.
 */

pub const NR_IRQS_LEGACY: usize = 16;

pub const CPU_VECTOR_LIMIT: usize = 64 * NR_CPUS;
pub const IO_APIC_VECTOR_LIMIT: usize = 32 * MAX_IO_APICS;

/* Original condition: #if defined(CONFIG_X86_IO_APIC) && defined(CONFIG_PCI_MSI) */
#[cfg(all(CONFIG_X86_IO_APIC, CONFIG_PCI_MSI))]
pub const NR_IRQS: usize = if CPU_VECTOR_LIMIT > IO_APIC_VECTOR_LIMIT {
    NR_VECTORS + CPU_VECTOR_LIMIT
} else {
    NR_VECTORS + IO_APIC_VECTOR_LIMIT
};

/* Original condition: #elif defined(CONFIG_X86_IO_APIC) */
#[cfg(all(CONFIG_X86_IO_APIC, not(CONFIG_PCI_MSI)))]
pub const NR_IRQS: usize = NR_VECTORS + IO_APIC_VECTOR_LIMIT;

/* Original condition: #elif defined(CONFIG_PCI_MSI) */
#[cfg(all(not(CONFIG_X86_IO_APIC), CONFIG_PCI_MSI))]
pub const NR_IRQS: usize = NR_VECTORS + CPU_VECTOR_LIMIT;

/* Original condition: #else */
#[cfg(all(not(CONFIG_X86_IO_APIC), not(CONFIG_PCI_MSI)))]
pub const NR_IRQS: usize = NR_IRQS_LEGACY;
