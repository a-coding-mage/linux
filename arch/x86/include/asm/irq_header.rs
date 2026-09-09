/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	(C) 1992, 1993 Linus Torvalds, (C) 1997 Ingo Molnar
 *
 *	IRQ/IPI changes taken from work by Thomas Radke
 *	<tomsoft@informatik.tu-chemnitz.de>
 */

/* Dependencies supplied by asm/apicdef.h and asm/irq_vectors.h. */

/*
 * The irq entry code is in the noinstr section and the start/end of
 * __irqentry_text is emitted via labels. Make the build fail if
 * something moves a C function into the __irq_entry section.
 *
 * C: #define __irq_entry __invalid_section
 */

#[inline]
pub fn irq_canonicalize(irq: i32) -> i32 {
	if irq == 2 { 9 } else { irq }
}

unsafe extern "C" {
	pub fn irq_init_percpu_irqstack(cpu: u32) -> i32;
}

pub enum irq_desc {}

unsafe extern "C" {
	pub fn fixup_irqs();

    #[cfg(feature = "CONFIG_KVM")]
    pub fn kvm_set_posted_intr_wakeup_handler(handler: Option<unsafe extern "C" fn()>);

	pub static mut x86_platform_ipi_callback: Option<unsafe extern "C" fn()>;
	pub fn native_init_IRQ();

	pub fn __handle_irq(desc: *mut irq_desc, regs: *mut pt_regs);

	pub fn init_ISA_irqs();

	#[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
	pub fn arch_trigger_cpumask_backtrace(mask: *const cpumask, exclude_cpu: i32);
}

/* C build-time alias: #define arch_trigger_cpumask_backtrace arch_trigger_cpumask_backtrace */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
