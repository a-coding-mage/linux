// SPDX-License-Identifier: GPL-2.0-only
/*
 * IRQ offload/bypass manager
 *
 * Copyright (C) 2015 Red Hat, Inc.
 * Copyright (c) 2015 Linaro Ltd.
 *
 * Various virtualization hardware acceleration techniques allow bypassing or
 * offloading interrupts received from devices around the host kernel.  Posted
 * Interrupts on Intel VT-d systems can allow interrupts to be received
 * directly by a virtual machine.  ARM IRQ Forwarding allows forwarded physical
 * interrupts to be directly deactivated by the guest.  This manager allows
 * interrupt producers and consumers to find each other to enable this sort of
 * bypass.
 */

// C dependencies: <linux/irqbypass.h>, <linux/list.h>, <linux/module.h>,
// <linux/mutex.h>
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("IRQ bypass manager utility module");

use core::ffi::{c_int, c_ulong, c_void};

const EINVAL: c_int = 22;
const GFP_KERNEL: c_int = 0;

#[repr(C)]
pub struct eventfd_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_bypass_producer {
    pub eventfd: *mut eventfd_ctx,
    pub irq: c_int,
    pub consumer: *mut irq_bypass_consumer,
    pub stop: Option<unsafe extern "C" fn(*mut irq_bypass_producer)>,
    pub start: Option<unsafe extern "C" fn(*mut irq_bypass_producer)>,
    pub add_consumer:
        Option<unsafe extern "C" fn(*mut irq_bypass_producer, *mut irq_bypass_consumer) -> c_int>,
    pub del_consumer:
        Option<unsafe extern "C" fn(*mut irq_bypass_producer, *mut irq_bypass_consumer)>,
}

#[repr(C)]
pub struct irq_bypass_consumer {
    pub eventfd: *mut eventfd_ctx,
    pub producer: *mut irq_bypass_producer,
    pub stop: Option<unsafe extern "C" fn(*mut irq_bypass_consumer)>,
    pub start: Option<unsafe extern "C" fn(*mut irq_bypass_consumer)>,
    pub add_producer:
        Option<unsafe extern "C" fn(*mut irq_bypass_consumer, *mut irq_bypass_producer) -> c_int>,
    pub del_producer:
        Option<unsafe extern "C" fn(*mut irq_bypass_consumer, *mut irq_bypass_producer)>,
}

extern "C" {
    static mut producers: xarray;
    static mut consumers: xarray;
    static mut lock: mutex;

    fn xa_insert(xa: *mut xarray, index: c_ulong, entry: *mut c_void, gfp: c_int) -> c_int;
    fn xa_load(xa: *mut xarray, index: c_ulong) -> *mut c_void;
    fn xa_erase(xa: *mut xarray, index: c_ulong) -> *mut c_void;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn WARN_ON_ONCE(condition: bool) -> bool;
}

/* @lock must be held when calling connect */
unsafe fn __connect(
    prod: *mut irq_bypass_producer,
    cons: *mut irq_bypass_consumer,
) -> c_int {
    let mut ret: c_int = 0;

    if let Some(stop) = (*prod).stop {
        stop(prod);
    }
    if let Some(stop) = (*cons).stop {
        stop(cons);
    }

    if let Some(add_consumer) = (*prod).add_consumer {
        ret = add_consumer(prod, cons);
    }

    if ret == 0 {
        ret = (*cons).add_producer.unwrap()(cons, prod);
        if ret != 0 {
            if let Some(del_consumer) = (*prod).del_consumer {
                del_consumer(prod, cons);
            }
        }
    }

    if let Some(start) = (*cons).start {
        start(cons);
    }
    if let Some(start) = (*prod).start {
        start(prod);
    }

    if ret == 0 {
        (*prod).consumer = cons;
        (*cons).producer = prod;
    }
    ret
}

/* @lock must be held when calling disconnect */
unsafe fn __disconnect(
    prod: *mut irq_bypass_producer,
    cons: *mut irq_bypass_consumer,
) {
    if let Some(stop) = (*prod).stop {
        stop(prod);
    }
    if let Some(stop) = (*cons).stop {
        stop(cons);
    }

    (*cons).del_producer.unwrap()(cons, prod);

    if let Some(del_consumer) = (*prod).del_consumer {
        del_consumer(prod, cons);
    }

    if let Some(start) = (*cons).start {
        start(cons);
    }
    if let Some(start) = (*prod).start {
        start(prod);
    }

    (*prod).consumer = core::ptr::null_mut();
    (*cons).producer = core::ptr::null_mut();
}

/**
 * irq_bypass_register_producer - register IRQ bypass producer
 * @producer: pointer to producer structure
 * @eventfd: pointer to the eventfd context associated with the producer
 * @irq: Linux IRQ number of the underlying producer device
 *
 * Add the provided IRQ producer to the set of producers and connect with the
 * consumer with a matching eventfd, if one exists.
 */
#[no_mangle]
pub unsafe extern "C" fn irq_bypass_register_producer(
    producer: *mut irq_bypass_producer,
    eventfd: *mut eventfd_ctx,
    irq: c_int,
) -> c_int {
    let index: c_ulong = eventfd as c_ulong;
    let mut consumer: *mut irq_bypass_consumer;
    let mut ret: c_int;

    if WARN_ON_ONCE(!(*producer).eventfd.is_null()) {
        return -EINVAL;
    }

    (*producer).irq = irq;

    mutex_lock(core::ptr::addr_of_mut!(lock));

    ret = xa_insert(
        core::ptr::addr_of_mut!(producers),
        index,
        producer as *mut c_void,
        GFP_KERNEL,
    );
    if ret != 0 {
        mutex_unlock(core::ptr::addr_of_mut!(lock));
        return ret;
    }

    consumer = xa_load(core::ptr::addr_of_mut!(consumers), index) as *mut irq_bypass_consumer;
    if !consumer.is_null() {
        ret = __connect(producer, consumer);
        if ret != 0 {
            WARN_ON_ONCE(
                xa_erase(core::ptr::addr_of_mut!(producers), index)
                    != producer as *mut c_void,
            );
            mutex_unlock(core::ptr::addr_of_mut!(lock));
            return ret;
        }
    }

    (*producer).eventfd = eventfd;
    mutex_unlock(core::ptr::addr_of_mut!(lock));
    0
}
// EXPORT_SYMBOL_GPL(irq_bypass_register_producer);

/**
 * irq_bypass_unregister_producer - unregister IRQ bypass producer
 * @producer: pointer to producer structure
 *
 * Remove a previously registered IRQ producer (note, it's safe to call this
 * even if registration was unsuccessful).  Disconnect from the associated
 * consumer, if one exists.
 */
#[no_mangle]
pub unsafe extern "C" fn irq_bypass_unregister_producer(
    producer: *mut irq_bypass_producer,
) {
    let index: c_ulong = (*producer).eventfd as c_ulong;

    if (*producer).eventfd.is_null() {
        return;
    }

    mutex_lock(core::ptr::addr_of_mut!(lock));

    if !(*producer).consumer.is_null() {
        __disconnect(producer, (*producer).consumer);
    }

    WARN_ON_ONCE(
        xa_erase(core::ptr::addr_of_mut!(producers), index)
            != producer as *mut c_void,
    );
    (*producer).eventfd = core::ptr::null_mut();
    mutex_unlock(core::ptr::addr_of_mut!(lock));
}
// EXPORT_SYMBOL_GPL(irq_bypass_unregister_producer);

/**
 * irq_bypass_register_consumer - register IRQ bypass consumer
 * @consumer: pointer to consumer structure
 * @eventfd: pointer to the eventfd context associated with the consumer
 *
 * Add the provided IRQ consumer to the set of consumers and connect with the
 * producer with a matching eventfd, if one exists.
 */
#[no_mangle]
pub unsafe extern "C" fn irq_bypass_register_consumer(
    consumer: *mut irq_bypass_consumer,
    eventfd: *mut eventfd_ctx,
) -> c_int {
    let index: c_ulong = eventfd as c_ulong;
    let mut producer: *mut irq_bypass_producer;
    let mut ret: c_int;

    if WARN_ON_ONCE(!(*consumer).eventfd.is_null()) {
        return -EINVAL;
    }

    if (*consumer).add_producer.is_none() || (*consumer).del_producer.is_none() {
        return -EINVAL;
    }

    mutex_lock(core::ptr::addr_of_mut!(lock));

    ret = xa_insert(
        core::ptr::addr_of_mut!(consumers),
        index,
        consumer as *mut c_void,
        GFP_KERNEL,
    );
    if ret != 0 {
        mutex_unlock(core::ptr::addr_of_mut!(lock));
        return ret;
    }

    producer = xa_load(core::ptr::addr_of_mut!(producers), index) as *mut irq_bypass_producer;
    if !producer.is_null() {
        ret = __connect(producer, consumer);
        if ret != 0 {
            WARN_ON_ONCE(
                xa_erase(core::ptr::addr_of_mut!(consumers), index)
                    != consumer as *mut c_void,
            );
            mutex_unlock(core::ptr::addr_of_mut!(lock));
            return ret;
        }
    }

    (*consumer).eventfd = eventfd;
    mutex_unlock(core::ptr::addr_of_mut!(lock));
    0
}
// EXPORT_SYMBOL_GPL(irq_bypass_register_consumer);

/**
 * irq_bypass_unregister_consumer - unregister IRQ bypass consumer
 * @consumer: pointer to consumer structure
 *
 * Remove a previously registered IRQ consumer (note, it's safe to call this
 * even if registration was unsuccessful).  Disconnect from the associated
 * producer, if one exists.
 */
#[no_mangle]
pub unsafe extern "C" fn irq_bypass_unregister_consumer(
    consumer: *mut irq_bypass_consumer,
) {
    let index: c_ulong = (*consumer).eventfd as c_ulong;

    if (*consumer).eventfd.is_null() {
        return;
    }

    mutex_lock(core::ptr::addr_of_mut!(lock));

    if !(*consumer).producer.is_null() {
        __disconnect((*consumer).producer, consumer);
    }

    WARN_ON_ONCE(
        xa_erase(core::ptr::addr_of_mut!(consumers), index)
            != consumer as *mut c_void,
    );
    (*consumer).eventfd = core::ptr::null_mut();
    mutex_unlock(core::ptr::addr_of_mut!(lock));
}
// EXPORT_SYMBOL_GPL(irq_bypass_unregister_consumer);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
