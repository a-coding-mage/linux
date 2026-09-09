// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on arch/arm/kernel/irq.c
 *
 * Copyright (C) 1992 Linus Torvalds
 * Modifications for ARM processor Copyright (C) 1995-2000 Russell King.
 * Support for Dynamic Tick Timer Copyright (C) 2004-2005 Nokia Corporation.
 * Dynamic Tick Timer written by Tony Lindgren <tony@atomide.com> and
 * Tuukka Tikkanen <tuukka.tikkanen@elektrobit.com>.
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

/* Only access this in an NMI enter/exit */
extern "C" {
    pub static mut nmi_contexts: [nmi_ctx; NR_CPUS];
    pub static mut irq_stack_ptr: [*mut c_ulong; NR_CPUS];
    pub static mut irq_shadow_call_stack_ptr: [*mut c_ulong; NR_CPUS];
}

#[cfg(CONFIG_SHADOW_CALL_STACK)]
extern "C" {
    pub static mut irq_shadow_call_stack_ptr: [*mut c_ulong; NR_CPUS];
}

unsafe fn init_irq_scs() -> c_int {
    let mut cpu: c_int;
    let mut s: *mut c_void;

    if !scs_is_enabled() {
        return 0;
    }

    for_each_possible_cpu!(cpu) {
        s = scs_alloc(early_cpu_to_node(cpu));
        if s.is_null() {
            return -ENOMEM;
        }
        per_cpu!(irq_shadow_call_stack_ptr, cpu) = s;
    }

    0
}

unsafe fn init_irq_stacks() -> c_int {
    let mut cpu: c_int;
    let mut p: *mut c_ulong;

    for_each_possible_cpu!(cpu) {
        p = arch_alloc_vmap_stack(IRQ_STACK_SIZE, early_cpu_to_node(cpu));
        if p.is_null() {
            return -ENOMEM;
        }
        per_cpu!(irq_stack_ptr, cpu) = p;
    }

    0
}

#[cfg(CONFIG_SOFTIRQ_ON_OWN_STACK)]
unsafe fn ____do_softirq(_regs: *mut pt_regs) {
    __do_softirq();
}

#[cfg(CONFIG_SOFTIRQ_ON_OWN_STACK)]
pub unsafe fn do_softirq_own_stack() {
    call_on_irq_stack(core::ptr::null_mut(), ____do_softirq);
}

unsafe fn default_handle_irq(_regs: *mut pt_regs) {
    panic!("IRQ taken without a root IRQ handler\n");
}

unsafe fn default_handle_fiq(_regs: *mut pt_regs) {
    panic!("FIQ taken without a root FIQ handler\n");
}

#[no_mangle]
pub static mut handle_arch_irq: unsafe extern "C" fn(*mut pt_regs) = default_handle_irq;
#[no_mangle]
pub static mut handle_arch_fiq: unsafe extern "C" fn(*mut pt_regs) = default_handle_fiq;

pub unsafe fn set_handle_irq(
    handle_irq: unsafe extern "C" fn(*mut pt_regs),
) -> c_int {
    if handle_arch_irq as usize != default_handle_irq as usize {
        return -EBUSY;
    }

    handle_arch_irq = handle_irq;
    pr_info!("Root IRQ handler: %ps\n", handle_irq);
    0
}

pub unsafe fn set_handle_fiq(
    handle_fiq: unsafe extern "C" fn(*mut pt_regs),
) -> c_int {
    if handle_arch_fiq as usize != default_handle_fiq as usize {
        return -EBUSY;
    }

    handle_arch_fiq = handle_fiq;
    pr_info!("Root FIQ handler: %ps\n", handle_fiq);
    0
}

pub unsafe fn init_IRQ() {
    if init_irq_stacks() != 0 || init_irq_scs() != 0 {
        panic!("Failed to allocate IRQ stack resources\n");
    }

    irqchip_init();

    if system_uses_irq_prio_masking() {
        /*
         * Now that we have a stack for our IRQ handler, set
         * the PMR/PSR pair to a consistent state.
         */
        WARN_ON!(read_sysreg!(daif) & PSR_A_BIT);
        local_daif_restore(DAIF_PROCCTX_NOIRQ);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
