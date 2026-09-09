// SPDX-License-Identifier: GPL-2.0
// This file is intended to be built by the kernel offset-generation build,
// corresponding to the C-only Kbuild guard and included architecture headers.

// The following items are supplied by the surrounding kernel build:
// `offset!` emits an offset declaration and `blank!` emits a blank separator.

pub fn main() -> i32 {
    #[cfg(all(feature = "CONFIG_PARAVIRT", feature = "CONFIG_PARAVIRT_XXL", feature = "CONFIG_DEBUG_ENTRY"))]
    {
        unsafe {
            offset!("PV_IRQ_save_fl", paravirt_patch_template, irq.save_fl);
        }
    }

    #[cfg(feature = "CONFIG_PARAVIRT")]
    {
        blank!();
    }

    #[cfg(feature = "CONFIG_KVM_GUEST")]
    {
        unsafe {
            offset!("KVM_STEAL_TIME_preempted", kvm_steal_time, preempted);
        }
        blank!();
    }

    unsafe {
        offset!("pt_regs_bx", pt_regs, bx);
        offset!("pt_regs_cx", pt_regs, cx);
        offset!("pt_regs_dx", pt_regs, dx);
        offset!("pt_regs_sp", pt_regs, sp);
        offset!("pt_regs_bp", pt_regs, bp);
        offset!("pt_regs_si", pt_regs, si);
        offset!("pt_regs_di", pt_regs, di);
        offset!("pt_regs_r8", pt_regs, r8);
        offset!("pt_regs_r9", pt_regs, r9);
        offset!("pt_regs_r10", pt_regs, r10);
        offset!("pt_regs_r11", pt_regs, r11);
        offset!("pt_regs_r12", pt_regs, r12);
        offset!("pt_regs_r13", pt_regs, r13);
        offset!("pt_regs_r14", pt_regs, r14);
        offset!("pt_regs_r15", pt_regs, r15);
        offset!("pt_regs_flags", pt_regs, flags);
    }
    blank!();

    unsafe {
        offset!("saved_context_cr0", saved_context, cr0);
        offset!("saved_context_cr2", saved_context, cr2);
        offset!("saved_context_cr3", saved_context, cr3);
        offset!("saved_context_cr4", saved_context, cr4);
        offset!("saved_context_gdt_desc", saved_context, gdt_desc);
    }
    blank!();

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
