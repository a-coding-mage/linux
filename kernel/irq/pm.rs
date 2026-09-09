// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2009 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.
 *
 * This file contains power management functions related to interrupts.
 */

pub unsafe fn irq_pm_handle_wakeup(desc: *mut irq_desc) {
    irqd_clear(&mut (*desc).irq_data, IRQD_WAKEUP_ARMED);
    (*desc).istate |= IRQS_SUSPENDED | IRQS_PENDING;
    (*desc).depth += 1;
    irq_disable(desc);
    pm_system_irq_wakeup(irq_desc_get_irq(desc));
}

/* Called from __setup_irq() with desc->lock held after action has been installed. */
pub unsafe fn irq_pm_install_action(desc: *mut irq_desc, action: *mut irqaction) {
    (*desc).nr_actions += 1;

    if (*action).flags & IRQF_FORCE_RESUME != 0 {
        (*desc).force_resume_depth += 1;
    }

    WARN_ON_ONCE((*desc).force_resume_depth != 0
        && (*desc).force_resume_depth != (*desc).nr_actions);

    if (*action).flags & IRQF_NO_SUSPEND != 0 {
        (*desc).no_suspend_depth += 1;
    } else if (*action).flags & IRQF_COND_SUSPEND != 0 {
        (*desc).cond_suspend_depth += 1;
    }

    WARN_ON_ONCE((*desc).no_suspend_depth != 0
        && ((*desc).no_suspend_depth + (*desc).cond_suspend_depth) != (*desc).nr_actions);
}

/* Called from __free_irq() with desc->lock held after action has been removed. */
pub unsafe fn irq_pm_remove_action(desc: *mut irq_desc, action: *mut irqaction) {
    (*desc).nr_actions -= 1;

    if (*action).flags & IRQF_FORCE_RESUME != 0 {
        (*desc).force_resume_depth -= 1;
    }

    if (*action).flags & IRQF_NO_SUSPEND != 0 {
        (*desc).no_suspend_depth -= 1;
    } else if (*action).flags & IRQF_COND_SUSPEND != 0 {
        (*desc).cond_suspend_depth -= 1;
    }
}

unsafe fn suspend_device_irq(desc: *mut irq_desc) -> bool {
    let chipflags = (*irq_desc_get_chip(desc)).flags;
    let irqd = &mut (*desc).irq_data as *mut irq_data;

    if (*desc).action.is_null() || irq_desc_is_chained(desc) || (*desc).no_suspend_depth != 0 {
        return false;
    }

    if irqd_is_wakeup_set(irqd) {
        irqd_set(irqd, IRQD_WAKEUP_ARMED);

        if chipflags & IRQCHIP_ENABLE_WAKEUP_ON_SUSPEND != 0 && irqd_irq_disabled(irqd) {
            __enable_irq(desc);
            irqd_set(irqd, IRQD_IRQ_ENABLED_ON_SUSPEND);
        }
        return true;
    }

    (*desc).istate |= IRQS_SUSPENDED;
    __disable_irq(desc);

    if chipflags & IRQCHIP_MASK_ON_SUSPEND != 0 {
        mask_irq(desc);
    }
    true
}

/// Disable all currently enabled interrupt lines during system suspend.
pub unsafe fn suspend_device_irqs() {
    let mut desc: *mut irq_desc;
    let mut irq: i32;

    for_each_irq_desc!(irq, desc, {
        let sync: bool;

        if irq_settings_is_nested_thread(desc) {
            continue;
        }
        // scoped_guard(raw_spinlock_irqsave, &desc->lock)
        sync = suspend_device_irq(desc);

        if sync {
            synchronize_irq(irq);
        }
    });
}

unsafe fn resume_irq(desc: *mut irq_desc) {
    let irqd = &mut (*desc).irq_data as *mut irq_data;

    irqd_clear(irqd, IRQD_WAKEUP_ARMED);

    if irqd_is_enabled_on_suspend(irqd) {
        __disable_irq(desc);
        irqd_clear(irqd, IRQD_IRQ_ENABLED_ON_SUSPEND);
    }

    if (*desc).istate & IRQS_SUSPENDED != 0 {
        (*desc).istate &= !IRQS_SUSPENDED;
        __enable_irq(desc);
        return;
    }

    if (*desc).force_resume_depth == 0 {
        return;
    }

    (*desc).depth += 1;
    irq_state_set_disabled(desc);
    irq_state_set_masked(desc);
    __enable_irq(desc);
}

unsafe fn resume_irqs(want_early: bool) {
    let mut desc: *mut irq_desc;
    let mut irq: i32;

    for_each_irq_desc!(irq, desc, {
        let is_early = !(*desc).action.is_null()
            && (*(*desc).action).flags & IRQF_EARLY_RESUME != 0;

        if !is_early && want_early {
            continue;
        }
        if irq_settings_is_nested_thread(desc) {
            continue;
        }

        // guard(raw_spinlock_irqsave)(&desc->lock)
        resume_irq(desc);
    });
}

/// Rearm a wakeup interrupt line after signaling wakeup.
pub unsafe fn rearm_wake_irq(irq: u32) {
    scoped_irqdesc_get_and_buslock!(irq, IRQ_GET_DESC_CHECK_GLOBAL, {
        let desc = scoped_irqdesc;

        if (*desc).istate & IRQS_SUSPENDED == 0 || !irqd_is_wakeup_set(&mut (*desc).irq_data) {
            return;
        }

        (*desc).istate &= !IRQS_SUSPENDED;
        irqd_set(&mut (*desc).irq_data, IRQD_WAKEUP_ARMED);
        __enable_irq(desc);
    });
}

/// Enable interrupt lines early.
unsafe fn irq_pm_syscore_resume(_data: *mut core::ffi::c_void) {
    resume_irqs(true);
}

static irq_pm_syscore_ops: syscore_ops = syscore_ops {
    resume: Some(irq_pm_syscore_resume),
};

static mut irq_pm_syscore: syscore = syscore {
    ops: &irq_pm_syscore_ops,
};

unsafe fn irq_pm_init_ops() -> i32 {
    register_syscore(&raw mut irq_pm_syscore);
    0
}

device_initcall!(irq_pm_init_ops);

/// Enable interrupt lines disabled by suspend_device_irqs().
pub unsafe fn resume_device_irqs() {
    resume_irqs(false);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
