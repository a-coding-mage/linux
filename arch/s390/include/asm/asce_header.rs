/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux/Rust translation units:
// linux::thread_info, linux::irqflags, asm::lowcore, and asm::ctlreg.

pub(crate) unsafe fn enable_sacf_uaccess() -> bool {
    let mut flags: usize;

    if test_thread_flag(TIF_ASCE_PRIMARY) {
        return true;
    }
    local_irq_save(&mut flags);
    local_ctl_load(1, &mut (*get_lowcore()).kernel_asce);
    set_thread_flag(TIF_ASCE_PRIMARY);
    local_irq_restore(flags);
    false
}

pub(crate) unsafe fn disable_sacf_uaccess(previous: bool) {
    let mut flags: usize;

    if previous {
        return;
    }
    local_irq_save(&mut flags);
    local_ctl_load(1, &mut (*get_lowcore()).user_asce);
    clear_thread_flag(TIF_ASCE_PRIMARY);
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
