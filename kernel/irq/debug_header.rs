/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Debugging printout:
 */

macro_rules! ___P {
    ($f:expr) => {
        if unsafe { (*desc).status_use_accessors & $f } != 0 {
            unsafe { printk(concat!("%14s set\n", "\0").as_ptr() as *const i8, stringify!($f).as_ptr() as *const i8); }
        }
    };
}

macro_rules! ___PS {
    ($f:expr) => {
        if unsafe { (*desc).istate & $f } != 0 {
            unsafe { printk(concat!("%14s set\n", "\0").as_ptr() as *const i8, stringify!($f).as_ptr() as *const i8); }
        }
    };
}

/* FIXME */
macro_rules! ___PD {
    ($f:expr) => {{
        loop {
            break;
        }
    }};
}

#[allow(non_snake_case)]
unsafe fn print_irq_desc(irq: u32, desc: *mut irq_desc) {
    // static DEFINE_RATELIMIT_STATE(ratelimit, 5 * HZ, 5);
    static mut ratelimit: ratelimit_state = ratelimit_state {
        _opaque: [0; 0],
    };

    if !__ratelimit(&mut ratelimit) {
        return;
    }

    printk(
        "irq %d, desc: %p, depth: %d, count: %d, unhandled: %d\n\0".as_ptr()
            as *const i8,
        irq,
        desc,
        (*desc).depth,
        (*desc).irq_count,
        (*desc).irqs_unhandled,
    );
    printk(
        "->handle_irq():  %p, %pS\n\0".as_ptr() as *const i8,
        (*desc).handle_irq,
        (*desc).handle_irq,
    );
    printk(
        "->irq_data.chip(): %p, %pS\n\0".as_ptr() as *const i8,
        (*desc).irq_data.chip,
        (*desc).irq_data.chip,
    );
    printk(
        "->action(): %p\n\0".as_ptr() as *const i8,
        (*desc).action,
    );
    if !(*desc).action.is_null() {
        printk(
            "->action->handler(): %p, %pS\n\0".as_ptr() as *const i8,
            (*(*desc).action).handler,
            (*(*desc).action).handler,
        );
    }

    ___P!(IRQ_LEVEL);
    ___P!(IRQ_PER_CPU);
    ___P!(IRQ_NOPROBE);
    ___P!(IRQ_NOREQUEST);
    ___P!(IRQ_NOTHREAD);
    ___P!(IRQ_NOAUTOEN);

    ___PS!(IRQS_AUTODETECT);
    ___PS!(IRQS_REPLAY);
    ___PS!(IRQS_WAITING);
    ___PS!(IRQS_PENDING);

    ___PD!(IRQS_INPROGRESS);
    ___PD!(IRQS_DISABLED);
    ___PD!(IRQS_MASKED);
}

// C preprocessor macros ___P, ___PS, and ___PD are scoped to this header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
