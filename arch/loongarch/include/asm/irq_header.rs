/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// C header dependencies: linux/irqdomain.h, linux/irqreturn.h, and
// asm-generic/irq.h. Their supplied Rust names are intentionally external.

pub const IRQ_STACK_SIZE: usize = THREAD_SIZE;
pub const IRQ_STACK_START: usize = IRQ_STACK_SIZE - 16;

// DECLARE_PER_CPU(unsigned long, irq_stack)
extern "C" {
    pub static mut irq_stack: usize;
}

/*
 * The highest address on the IRQ stack contains a dummy frame which is
 * structured as follows:
 *
 *   top ------------
 *       | task sp  | <- irq_stack[cpu] + IRQ_STACK_START
 *       ------------
 *       |          | <- First frame of IRQ context
 *       ------------
 *
 * task sp holds a copy of the task stack pointer where the struct pt_regs
 * from exception entry can be found.
 */
#[inline]
pub unsafe fn on_irq_stack(cpu: i32, sp: usize) -> bool {
    let low = per_cpu_irq_stack(cpu);
    let high = low.wrapping_add(IRQ_STACK_SIZE);

    low <= sp && sp <= high
}

// External equivalent of the kernel's per_cpu(irq_stack, cpu) accessor.
extern "C" {
    fn per_cpu_irq_stack(cpu: i32) -> usize;
}

extern "C" {
    pub fn spurious_interrupt();
}

pub const NR_IRQS_LEGACY: usize = 16;

/*
 * 256 Vectors Mapping for AVECINTC:
 *
 * 0 - 15: Mapping classic IPs, e.g. IP0-12.
 * 16 - 255: Mapping vectors for external IRQ.
 *
 */
pub const NR_VECTORS: usize = 256;
pub const NR_LEGACY_VECTORS: usize = 16;

pub const AVEC_IRQ_SHIFT: usize = 4;
pub const AVEC_IRQ_BIT: usize = 8;
pub const AVEC_IRQ_MASK: usize = (1usize << AVEC_IRQ_BIT) - 1;
pub const AVEC_CPU_SHIFT: usize = 12;
pub const AVEC_CPU_BIT: usize = 16;
pub const AVEC_CPU_MASK: usize = (1usize << AVEC_CPU_BIT) - 1;

// #define arch_trigger_cpumask_backtrace arch_trigger_cpumask_backtrace
extern "C" {
    pub fn arch_trigger_cpumask_backtrace(mask: *const cpumask, exclude_cpu: i32);
}

#[cfg(CONFIG_32BIT)]
pub const MAX_IO_PICS: usize = 1;
#[cfg(not(CONFIG_32BIT))]
pub const MAX_IO_PICS: usize = 8;

pub const NR_IRQS: usize = 64 + NR_VECTORS * (NR_CPUS + MAX_IO_PICS);

#[repr(C)]
pub struct acpi_vector_group {
    pub node: i32,
    pub pci_segment: i32,
    pub parent: *mut irq_domain,
}

extern "C" {
    pub static mut pch_group: [acpi_vector_group; MAX_IO_PICS];
    pub static mut msi_group: [acpi_vector_group; MAX_IO_PICS];
}

pub const CORES_PER_EIO_NODE: usize = 4;
pub const CORES_PER_VEIO_NODE: usize = 256;

pub const LOONGSON_CPU_UART0_VEC: usize = 10; // CPU UART0
pub const LOONGSON_CPU_THSENS_VEC: usize = 14; // CPU Thsens
pub const LOONGSON_CPU_HT0_VEC: usize = 16; // CPU HT0 irq vector base number
pub const LOONGSON_CPU_HT1_VEC: usize = 24; // CPU HT1 irq vector base number

/* IRQ number definitions */
pub const LOONGSON_LPC_IRQ_BASE: usize = 0;
pub const LOONGSON_LPC_LAST_IRQ: usize = LOONGSON_LPC_IRQ_BASE + 15;
pub const LOONGSON_CPU_IRQ_BASE: usize = 16;
pub const LOONGSON_CPU_LAST_IRQ: usize = LOONGSON_CPU_IRQ_BASE + 15;
pub const LOONGSON_PCH_IRQ_BASE: usize = 64;
pub const LOONGSON_PCH_ACPI_IRQ: usize = LOONGSON_PCH_IRQ_BASE + 47;
pub const LOONGSON_PCH_LAST_IRQ: usize = LOONGSON_PCH_IRQ_BASE + 64 - 1;
pub const LOONGSON_MSI_IRQ_BASE: usize = LOONGSON_PCH_IRQ_BASE + 64;
pub const LOONGSON_MSI_LAST_IRQ: usize = LOONGSON_PCH_IRQ_BASE + 256 - 1;
pub const GSI_MIN_LPC_IRQ: usize = LOONGSON_LPC_IRQ_BASE;
pub const GSI_MAX_LPC_IRQ: usize = LOONGSON_LPC_IRQ_BASE + 16 - 1;
pub const GSI_MIN_CPU_IRQ: usize = LOONGSON_CPU_IRQ_BASE;
pub const GSI_MAX_CPU_IRQ: usize = LOONGSON_CPU_IRQ_BASE + 48 - 1;
pub const GSI_MIN_PCH_IRQ: usize = LOONGSON_PCH_IRQ_BASE;
pub const GSI_MAX_PCH_IRQ: usize = LOONGSON_PCH_IRQ_BASE + 256 - 1;

pub enum acpi_madt_lio_pic {}
pub enum acpi_madt_eio_pic {}
pub enum acpi_madt_ht_pic {}
pub enum acpi_madt_bio_pic {}
pub enum acpi_madt_msi_pic {}
pub enum acpi_madt_lpc_pic {}

extern "C" {
    pub fn complete_irq_moving();
    pub fn get_pch_msi_handle(pci_segment: i32) -> *mut fwnode_handle;
    pub static mut acpi_liointc: *mut acpi_madt_lio_pic;
    pub static mut acpi_eiointc: [*mut acpi_madt_eio_pic; MAX_IO_PICS];
    pub static mut acpi_htintc: *mut acpi_madt_ht_pic;
    pub static mut acpi_pchlpc: *mut acpi_madt_lpc_pic;
    pub static mut acpi_pchmsi: [*mut acpi_madt_msi_pic; MAX_IO_PICS];
    pub static mut acpi_pchpic: [*mut acpi_madt_bio_pic; MAX_IO_PICS];
    pub static mut cpuintc_handle: *mut fwnode_handle;
    pub static mut liointc_handle: *mut fwnode_handle;
    pub static mut pch_lpc_handle: *mut fwnode_handle;
    pub static mut pch_pic_handle: [*mut fwnode_handle; MAX_IO_PICS];
}

#[inline]
pub unsafe fn get_percpu_irq(vector: i32) -> i32 {
    let d = irq_find_matching_fwnode(cpuintc_handle, DOMAIN_BUS_ANY);
    if !d.is_null() {
        return irq_create_mapping(d, vector);
    }
    -EINVAL
}

// External names supplied by the dependent kernel bindings.
extern "C" {
    static THREAD_SIZE: usize;
    static NR_CPUS: usize;
    static DOMAIN_BUS_ANY: i32;
    static EINVAL: i32;
    fn irq_find_matching_fwnode(handle: *mut fwnode_handle, bus_token: i32) -> *mut irq_domain;
    fn irq_create_mapping(domain: *mut irq_domain, vector: i32) -> i32;
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}
#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
