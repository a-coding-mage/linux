// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file implements an irqchip for OPAL events. Whenever there is
 * an interrupt that is handled by OPAL we get passed a list of events
 * that Linux needs to do something about. These basically look like
 * interrupts to Linux so we implement an irqchip to handle them.
 *
 * Copyright Alistair Popple, IBM Corporation 2014.
 */

// Linux kernel dependencies supplied by other translation units.

/* Maximum number of events supported by OPAL firmware */
const MAX_NUM_EVENTS: usize = 64;

#[repr(C)]
struct OpalEventIrqchip {
    irqchip: irq_chip,
    domain: *mut irq_domain,
    mask: c_ulong,
}

static mut OPAL_EVENT_IRQCHIP: OpalEventIrqchip = OpalEventIrqchip {
    irqchip: irq_chip {
        name: core::ptr::null(),
        irq_mask: Some(opal_event_mask),
        irq_unmask: Some(opal_event_unmask),
        irq_set_type: Some(opal_event_set_type),
    },
    domain: core::ptr::null_mut(),
    mask: 0,
};
static mut LAST_OUTSTANDING_EVENTS: u64 = 0;
static mut OPAL_IRQ_COUNT: c_int = 0;
static mut OPAL_IRQS: *mut resource = core::ptr::null_mut();

pub unsafe fn opal_handle_events() {
    let mut events: __be64 = 0;
    let mut e: u64 = core::ptr::read_volatile(&LAST_OUTSTANDING_EVENTS)
        & OPAL_EVENT_IRQCHIP.mask as u64;

    'again: loop {
        while e != 0 {
            let hwirq: c_int = 63 - e.leading_zeros() as c_int;
            e &= !(1u64 << hwirq);

            local_irq_disable();
            irq_enter();
            generic_handle_domain_irq(OPAL_EVENT_IRQCHIP.domain, hwirq as irq_hw_number_t);
            irq_exit();
            local_irq_enable();

            cond_resched();
        }
        core::ptr::write_volatile(&mut LAST_OUTSTANDING_EVENTS, 0);
        if opal_poll_events(&mut events) != OPAL_SUCCESS {
            return;
        }
        e = be64_to_cpu(events) & OPAL_EVENT_IRQCHIP.mask as u64;
        if e == 0 {
            break 'again;
        }
    }
}

pub unsafe fn opal_have_pending_events() -> bool {
    if core::ptr::read_volatile(&LAST_OUTSTANDING_EVENTS)
        & OPAL_EVENT_IRQCHIP.mask as u64 != 0
    {
        return true;
    }
    false
}

unsafe extern "C" fn opal_event_mask(d: *mut irq_data) {
    clear_bit((*d).hwirq, &mut OPAL_EVENT_IRQCHIP.mask);
}

unsafe extern "C" fn opal_event_unmask(d: *mut irq_data) {
    set_bit((*d).hwirq, &mut OPAL_EVENT_IRQCHIP.mask);
    if opal_have_pending_events() {
        opal_wake_poller();
    }
}

unsafe extern "C" fn opal_event_set_type(
    _d: *mut irq_data,
    flow_type: c_uint,
) -> c_int {
    /*
     * For now we only support level triggered events. The irq
     * handler will be called continuously until the event has
     * been cleared in OPAL.
     */
    if flow_type != IRQ_TYPE_LEVEL_HIGH {
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn opal_event_map(
    _d: *mut irq_domain,
    irq: c_uint,
    hwirq: irq_hw_number_t,
) -> c_int {
    irq_set_chip_data(irq, &raw mut OPAL_EVENT_IRQCHIP as *mut _);
    irq_set_chip_and_handler(
        irq,
        &raw mut OPAL_EVENT_IRQCHIP.irqchip as *mut _,
        handle_level_irq,
    );
    0
}

unsafe extern "C" fn opal_interrupt(_irq: c_int, _data: *mut c_void) -> irqreturn_t {
    let mut events: __be64 = 0;
    opal_handle_interrupt(virq_to_hw(_irq), &mut events);
    core::ptr::write_volatile(&mut LAST_OUTSTANDING_EVENTS, be64_to_cpu(events));
    if opal_have_pending_events() {
        opal_wake_poller();
    }
    IRQ_HANDLED
}

unsafe extern "C" fn opal_event_match(
    h: *mut irq_domain,
    node: *mut device_node,
    _bus_token: irq_domain_bus_token,
) -> c_int {
    (irq_domain_get_of_node(h) == node) as c_int
}

unsafe extern "C" fn opal_event_xlate(
    _h: *mut irq_domain,
    _np: *mut device_node,
    intspec: *const u32,
    _intsize: c_uint,
    out_hwirq: *mut irq_hw_number_t,
    out_flags: *mut c_uint,
) -> c_int {
    *out_hwirq = *intspec as irq_hw_number_t;
    *out_flags = IRQ_TYPE_LEVEL_HIGH;
    0
}

static OPAL_EVENT_DOMAIN_OPS: irq_domain_ops = irq_domain_ops {
    match_: Some(opal_event_match),
    map: Some(opal_event_map),
    xlate: Some(opal_event_xlate),
};

pub unsafe fn opal_event_shutdown() {
    let mut i: c_uint = 0;
    while i < OPAL_IRQ_COUNT as c_uint {
        let r = OPAL_IRQS.add(i as usize);
        if OPAL_IRQS.is_null() || (*r).start == 0 {
            i += 1;
            continue;
        }
        if in_interrupt() || irqs_disabled() {
            disable_irq_nosync((*r).start);
        } else {
            free_irq((*r).start, core::ptr::null_mut());
        }
        (*r).start = 0;
        i += 1;
    }
}

pub unsafe extern "C" fn opal_event_init() -> c_int {
    let mut dn: *mut device_node;
    let opal_node = of_find_node_by_path(c"/ibm,opal".as_ptr());
    let mut old_style = false;
    let mut i: c_int;
    let mut rc: c_int = 0;

    if opal_node.is_null() {
        pr_warn(c"opal: Node not found\n".as_ptr());
        return -ENODEV;
    }

    dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), c"ibm,opal-event".as_ptr());
    OPAL_EVENT_IRQCHIP.domain = irq_domain_create_linear(
        of_fwnode_handle(dn),
        MAX_NUM_EVENTS,
        &OPAL_EVENT_DOMAIN_OPS,
        &raw mut OPAL_EVENT_IRQCHIP as *mut _,
    );
    of_node_put(dn);
    if OPAL_EVENT_IRQCHIP.domain.is_null() {
        pr_warn(c"opal: Unable to create irq domain\n".as_ptr());
        rc = -ENOMEM;
        goto out;
    }

    OPAL_IRQ_COUNT = of_irq_count(opal_node);
    if OPAL_IRQ_COUNT < 1 {
        rc = of_property_count_u32_elems(opal_node, c"opal-interrupts".as_ptr());
        if rc > 0 { OPAL_IRQ_COUNT = rc; }
        old_style = true;
    }
    if OPAL_IRQ_COUNT == 0 { goto out; }

    OPAL_IRQS = kzalloc_objs::<resource>(OPAL_IRQ_COUNT as usize);
    if WARN_ON(OPAL_IRQS.is_null()) { rc = -ENOMEM; goto out; }

    if old_style {
        i = 0;
        while i < OPAL_IRQ_COUNT {
            let r = OPAL_IRQS.add(i as usize);
            let mut name: *const c_char = core::ptr::null();
            let mut hw_irq: u32 = 0;
            rc = of_property_read_u32_index(opal_node, c"opal-interrupts".as_ptr(), i as usize, &mut hw_irq);
            if WARN_ON(rc < 0) { OPAL_IRQ_COUNT = i; break; }
            of_property_read_string_index(opal_node, c"opal-interrupts-names".as_ptr(), i as usize, &mut name);
            let virq = irq_create_mapping(core::ptr::null_mut(), hw_irq as irq_hw_number_t);
            if virq == 0 { pr_warn(c"Failed to map OPAL irq\n".as_ptr()); i += 1; continue; }
            (*r).start = virq; (*r).end = virq; (*r).flags = IORESOURCE_IRQ | IRQ_TYPE_LEVEL_LOW; (*r).name = name;
            i += 1;
        }
    } else {
        rc = of_irq_to_resource_table(opal_node, OPAL_IRQS, OPAL_IRQ_COUNT);
        if WARN_ON(rc < 0) { OPAL_IRQ_COUNT = 0; kfree(OPAL_IRQS as *mut c_void); goto out; }
        if WARN_ON(rc < OPAL_IRQ_COUNT) { OPAL_IRQ_COUNT = rc; }
    }

    i = 0;
    while i < OPAL_IRQ_COUNT {
        let r = OPAL_IRQS.add(i as usize);
        let name = if !(*r).name.is_null() && strlen((*r).name) != 0 { kasprintf(GFP_KERNEL, c"opal-%s".as_ptr(), (*r).name) } else { kasprintf(GFP_KERNEL, c"opal".as_ptr()) };
        if !name.is_null() {
            rc = request_irq((*r).start, Some(opal_interrupt), (*r).flags & IRQD_TRIGGER_MASK, name, core::ptr::null_mut());
            if rc != 0 { pr_warn(c"Error requesting OPAL irq\n".as_ptr()); kfree(name as *mut c_void); }
        }
        i += 1;
    }
    rc = 0;
out:
    of_node_put(opal_node);
    rc
}

pub unsafe fn opal_event_request(opal_event_nr: c_uint) -> c_uint {
    if WARN_ON_ONCE(OPAL_EVENT_IRQCHIP.domain.is_null()) { return 0; }
    irq_create_mapping(OPAL_EVENT_IRQCHIP.domain, opal_event_nr as irq_hw_number_t)
}

// EXPORT_SYMBOL(opal_event_request);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
