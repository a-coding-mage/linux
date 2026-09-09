// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OPAL asynchronus Memory error handling support in PowerNV.
 *
 * Copyright 2013 IBM Corporation
 * Author: Mahesh Salgaonkar <mahesh@linux.vnet.ibm.com>
 */

// Kernel dependencies and build-time declarations are supplied by other files.

static mut opal_mem_err_nb_init: i32 = 0;
static mut opal_memory_err_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut opal_mem_err_lock: spinlock_t = spinlock_t { _private: [] };

#[repr(C)]
struct OpalMsgNode {
    list: list_head,
    msg: opal_msg,
}

unsafe fn handle_memory_error_event(merr_evt: *mut OpalMemoryErrorData) {
    let (mut paddr_start, paddr_end): (u64, u64);

    pr_debug!("{}: Retrieved memory error event, type: 0x{:x}\n", "handle_memory_error_event", (*merr_evt).type_);
    match (*merr_evt).type_ {
        OPAL_MEM_ERR_TYPE_RESILIENCE => {
            paddr_start = be64_to_cpu((*merr_evt).u.resilience.physical_address_start);
            paddr_end = be64_to_cpu((*merr_evt).u.resilience.physical_address_end);
        }
        OPAL_MEM_ERR_TYPE_DYN_DALLOC => {
            paddr_start = be64_to_cpu((*merr_evt).u.dyn_dealloc.physical_address_start);
            paddr_end = be64_to_cpu((*merr_evt).u.dyn_dealloc.physical_address_end);
        }
        _ => return,
    }

    while paddr_start < paddr_end {
        memory_failure(paddr_start >> PAGE_SHIFT, 0);
        paddr_start = paddr_start.wrapping_add(PAGE_SIZE);
    }
}

unsafe fn handle_memory_error() {
    let mut flags: c_ulong = 0;
    let mut merr_evt: *mut OpalMemoryErrorData;
    let mut msg_node: *mut OpalMsgNode;

    spin_lock_irqsave(&raw mut opal_mem_err_lock, &mut flags);
    while !list_empty(&raw mut opal_memory_err_list) {
        msg_node = list_entry((*opal_memory_err_list).next, OpalMsgNode, list);
        list_del(&mut (*msg_node).list);
        spin_unlock_irqrestore(&raw mut opal_mem_err_lock, flags);

        merr_evt = (&mut (*msg_node).msg.params[0] as *mut _).cast::<OpalMemoryErrorData>();
        handle_memory_error_event(merr_evt);
        kfree(msg_node.cast());
        spin_lock_irqsave(&raw mut opal_mem_err_lock, &mut flags);
    }
    spin_unlock_irqrestore(&raw mut opal_mem_err_lock, flags);
}

unsafe extern "C" fn mem_error_handler(_work: *mut work_struct) {
    handle_memory_error();
}

static mut mem_error_work: work_struct = DECLARE_WORK!(mem_error_handler);

/*
 * opal_memory_err_event - notifier handler that queues up the opal message
 * to be processed later.
 */
unsafe extern "C" fn opal_mem_err_event(
    _nb: *mut notifier_block,
    msg_type: c_ulong,
    msg: *mut c_void,
) -> i32 {
    let mut flags: c_ulong = 0;
    let msg_node: *mut OpalMsgNode;

    if msg_type != OPAL_MSG_MEM_ERR {
        return 0;
    }

    msg_node = kzalloc_obj!(OpalMsgNode, GFP_ATOMIC);
    if msg_node.is_null() {
        pr_err!("MEMORY_ERROR: out of memory, Opal message event nothandled\n");
        return -ENOMEM;
    }
    memcpy(
        &mut (*msg_node).msg as *mut opal_msg as *mut c_void,
        msg,
        core::mem::size_of::<opal_msg>(),
    );

    spin_lock_irqsave(&raw mut opal_mem_err_lock, &mut flags);
    list_add(&mut (*msg_node).list, &raw mut opal_memory_err_list);
    spin_unlock_irqrestore(&raw mut opal_mem_err_lock, flags);

    schedule_work(&raw mut mem_error_work);
    0
}

static mut opal_mem_err_nb: notifier_block = notifier_block {
    notifier_call: Some(opal_mem_err_event),
    next: core::ptr::null_mut(),
    priority: 0,
};

unsafe extern "C" fn opal_mem_err_init() -> i32 {
    let ret: i32;

    if opal_mem_err_nb_init == 0 {
        ret = opal_message_notifier_register(OPAL_MSG_MEM_ERR, &raw mut opal_mem_err_nb);
        if ret != 0 {
            pr_err!("{}: Can't register OPAL event notifier ({})\n", "opal_mem_err_init", ret);
            return ret;
        }
        opal_mem_err_nb_init = 1;
    }
    0
}

machine_device_initcall!(powernv, opal_mem_err_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
