// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011-12 Synopsys, Inc. (www.synopsys.com)
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct plat_smp_ops_type {
    pub init_per_cpu: Option<unsafe extern "C" fn(unsigned int)>,
}

#[repr(C)]
pub struct machine_desc_type {
    pub init_per_cpu: Option<unsafe extern "C" fn(unsigned int)>,
}

unsafe extern "C" {
    pub fn irqchip_init();
    pub static mut plat_smp_ops: plat_smp_ops_type;
    pub fn smp_processor_id() -> unsigned_int;
    pub static mut machine_desc: *mut machine_desc_type;

    pub fn irq_enter();
    pub fn set_irq_regs(regs: *mut pt_regs) -> *mut pt_regs;
    pub fn generic_handle_domain_irq(domain: *mut irq_domain, hwirq: unsigned_int);
    pub fn irq_exit();
}

pub type unsigned_int = u32;

/*
 * Late Interrupt system init called from start_kernel for Boot CPU only
 *
 * Since slab must already be initialized, platforms can start doing any
 * needed request_irq( )s
 */
pub unsafe extern "C" fn init_IRQ() {
    /*
     * process the entire interrupt tree in one go
     * Any external intc will be setup provided DT chains them
     * properly
     */
    irqchip_init();

    #[cfg(CONFIG_SMP)]
    {
        /* a SMP H/w block could do IPI IRQ request here */
        if let Some(init_per_cpu) = plat_smp_ops.init_per_cpu {
            init_per_cpu(smp_processor_id());
        }
    }

    if let Some(init_per_cpu) = (*machine_desc).init_per_cpu {
        init_per_cpu(smp_processor_id());
    }
}

/*
 * "C" Entry point for any ARC ISR, called from low level vector handler
 * @irq is the vector number read from ICAUSE reg of on-chip intc
 */
pub unsafe extern "C" fn arch_do_IRQ(hwirq: unsigned_int, regs: *mut pt_regs) {
    let old_regs: *mut pt_regs;

    irq_enter();
    old_regs = set_irq_regs(regs);
    generic_handle_domain_irq(core::ptr::null_mut(), hwirq);
    set_irq_regs(old_regs);
    irq_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
