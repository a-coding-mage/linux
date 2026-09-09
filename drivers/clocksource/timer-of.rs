// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017, Linaro Ltd.  All rights reserved.
 *
 * Author: Daniel Lezcano <daniel.lezcano@linaro.org>
 */

// Linux headers and "timer-of.h" supply the types, constants, macros, and
// external functions referenced below.

unsafe fn timer_of_irq_exit(of_irq: *mut crate::of_timer_irq) {
    let to = crate::container_of!(of_irq, crate::timer_of, of_irq);
    let clkevt: *mut crate::clock_event_device = &mut (*to).clkevt;

    crate::free_irq((*of_irq).irq, clkevt);
}

unsafe fn timer_of_irq_init(
    np: *mut crate::device_node,
    of_irq: *mut crate::of_timer_irq,
) -> i32 {
    let mut ret: i32;
    let to = crate::container_of!(of_irq, crate::timer_of, of_irq);
    let clkevt: *mut crate::clock_event_device = &mut (*to).clkevt;

    if !(*of_irq).name.is_null() {
        ret = crate::of_irq_get_byname(np, (*of_irq).name);
        (*of_irq).irq = ret;
        if ret < 0 {
            crate::pr_err!("Failed to get interrupt %s for %pOF\n", (*of_irq).name, np);
            return ret;
        }
    } else {
        (*of_irq).irq = crate::irq_of_parse_and_map(np, (*of_irq).index);
    }

    if (*of_irq).irq == 0 {
        crate::pr_err!("Failed to map interrupt for %pOF\n", np);
        return -crate::EINVAL;
    }

    ret = crate::request_irq(
        (*of_irq).irq,
        (*of_irq).handler,
        if (*of_irq).flags != 0 { (*of_irq).flags } else { crate::IRQF_TIMER },
        (*np).full_name,
        clkevt,
    );
    if ret != 0 {
        crate::pr_err!("Failed to request irq %d for %pOF\n", (*of_irq).irq, np);
        return ret;
    }

    (*clkevt).irq = (*of_irq).irq;
    0
}

unsafe fn timer_of_clk_exit(of_clk: *mut crate::of_timer_clk) {
    (*of_clk).rate = 0;
    crate::clk_disable_unprepare((*of_clk).clk);
    crate::clk_put((*of_clk).clk);
}

unsafe fn timer_of_clk_init(
    np: *mut crate::device_node,
    of_clk: *mut crate::of_timer_clk,
) -> i32 {
    let mut ret: i32;

    (*of_clk).clk = if !(*of_clk).name.is_null() {
        crate::of_clk_get_by_name(np, (*of_clk).name)
    } else {
        crate::of_clk_get(np, (*of_clk).index)
    };
    if crate::IS_ERR!((*of_clk).clk) {
        ret = crate::PTR_ERR!((*of_clk).clk);
        if ret != -crate::EPROBE_DEFER {
            crate::pr_err!("Failed to get clock for %pOF\n", np);
        }
        return ret;
    }

    ret = crate::clk_prepare_enable((*of_clk).clk);
    if ret != 0 {
        crate::pr_err!("Failed for enable clock for %pOF\n", np);
        crate::clk_put((*of_clk).clk);
        return ret;
    }

    (*of_clk).rate = crate::clk_get_rate((*of_clk).clk);
    if (*of_clk).rate == 0 {
        ret = -crate::EINVAL;
        crate::pr_err!("Failed to get clock rate for %pOF\n", np);
        crate::clk_disable_unprepare((*of_clk).clk);
        crate::clk_put((*of_clk).clk);
        return ret;
    }

    (*of_clk).period = crate::DIV_ROUND_UP!((*of_clk).rate, crate::HZ);
    ret
}

unsafe fn timer_of_base_exit(of_base: *mut crate::of_timer_base) {
    crate::iounmap((*of_base).base);
}

unsafe fn timer_of_base_init(
    np: *mut crate::device_node,
    of_base: *mut crate::of_timer_base,
) -> i32 {
    (*of_base).base = if !(*of_base).name.is_null() {
        crate::of_io_request_and_map(np, (*of_base).index, (*of_base).name)
    } else {
        crate::of_iomap(np, (*of_base).index)
    };
    if crate::IS_ERR_OR_NULL!((*of_base).base) {
        crate::pr_err!("Failed to iomap (%s:%s)\n", (*np).name, (*of_base).name);
        return if !(*of_base).base.is_null() {
            crate::PTR_ERR!((*of_base).base)
        } else {
            -crate::ENOMEM
        };
    }
    0
}

pub unsafe fn timer_of_init(np: *mut crate::device_node, to: *mut crate::timer_of) -> i32 {
    let mut ret: i32 = -crate::EINVAL;
    let mut flags: u32 = 0;

    if (*to).flags & crate::TIMER_OF_BASE != 0 {
        ret = timer_of_base_init(np, &mut (*to).of_base);
        if ret != 0 { return timer_of_init_fail(to, flags, ret); }
        flags |= crate::TIMER_OF_BASE;
    }
    if (*to).flags & crate::TIMER_OF_CLOCK != 0 {
        ret = timer_of_clk_init(np, &mut (*to).of_clk);
        if ret != 0 { return timer_of_init_fail(to, flags, ret); }
        flags |= crate::TIMER_OF_CLOCK;
    }
    if (*to).flags & crate::TIMER_OF_IRQ != 0 {
        ret = timer_of_irq_init(np, &mut (*to).of_irq);
        if ret != 0 { return timer_of_init_fail(to, flags, ret); }
        flags |= crate::TIMER_OF_IRQ;
    }
    if (*to).clkevt.name.is_null() { (*to).clkevt.name = (*np).full_name; }
    (*to).np = np;
    ret
}

unsafe fn timer_of_init_fail(to: *mut crate::timer_of, flags: u32, ret: i32) -> i32 {
    if flags & crate::TIMER_OF_IRQ != 0 { timer_of_irq_exit(&mut (*to).of_irq); }
    if flags & crate::TIMER_OF_CLOCK != 0 { timer_of_clk_exit(&mut (*to).of_clk); }
    if flags & crate::TIMER_OF_BASE != 0 { timer_of_base_exit(&mut (*to).of_base); }
    ret
}

pub unsafe fn timer_of_cleanup(to: *mut crate::timer_of) {
    if (*to).flags & crate::TIMER_OF_IRQ != 0 { timer_of_irq_exit(&mut (*to).of_irq); }
    if (*to).flags & crate::TIMER_OF_CLOCK != 0 { timer_of_clk_exit(&mut (*to).of_clk); }
    if (*to).flags & crate::TIMER_OF_BASE != 0 { timer_of_base_exit(&mut (*to).of_base); }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
