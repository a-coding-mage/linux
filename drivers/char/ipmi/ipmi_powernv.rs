// SPDX-License-Identifier: GPL-2.0+
/*
 * PowerNV OPAL IPMI driver
 *
 * Copyright 2014 IBM Corp.
 */

// Linux kernel headers and asm/opal.h are supplied by the surrounding translation.

#[repr(C)]
struct IpmiSmiPowernv {
    interface_id: u64,
    intf: *mut ipmi_smi,
    irq: c_uint,

    /**
     * We assume that there can only be one outstanding request, so
     * keep the pending message in cur_msg. We protect this from concurrent
     * updates through send & recv calls, (and consequently opal_msg, which
     * is in-use when cur_msg is set) with msg_lock
     */
    msg_lock: spinlock_t,
    cur_msg: *mut ipmi_smi_msg,
    opal_msg: *mut opal_ipmi_msg,
}

unsafe fn ipmi_powernv_start_processing(send_info: *mut c_void, intf: *mut ipmi_smi) -> c_int {
    let smi = send_info as *mut IpmiSmiPowernv;
    (*smi).intf = intf;
    0
}

unsafe fn send_error_reply(smi: *mut IpmiSmiPowernv, msg: *mut ipmi_smi_msg, completion_code: u8) {
    (*msg).rsp[0] = (*msg).data[0] | 0x4;
    (*msg).rsp[1] = (*msg).data[1];
    (*msg).rsp[2] = completion_code;
    (*msg).rsp_size = 3;
    ipmi_smi_msg_received((*smi).intf, msg);
}

unsafe fn ipmi_powernv_send(send_info: *mut c_void, msg: *mut ipmi_smi_msg) -> c_int {
    let smi = send_info as *mut IpmiSmiPowernv;
    let opal_msg: *mut opal_ipmi_msg;
    let mut flags: c_ulong = 0;
    let mut comp: c_int;
    let rc: c_int;
    let size: usize;

    if (*msg).data_size > IPMI_MAX_MSG_LENGTH {
        comp = IPMI_REQ_LEN_EXCEEDED_ERR;
        return comp;
    }
    if (*msg).data_size < 2 {
        comp = IPMI_REQ_LEN_INVALID_ERR;
        return comp;
    }

    spin_lock_irqsave(&mut (*smi).msg_lock, &mut flags);
    if !(*smi).cur_msg.is_null() {
        comp = IPMI_NODE_BUSY_ERR;
        spin_unlock_irqrestore(&mut (*smi).msg_lock, flags);
        return comp;
    }

    opal_msg = (*smi).opal_msg;
    (*opal_msg).version = OPAL_IPMI_MSG_FORMAT_VERSION_1;
    (*opal_msg).netfn = (*msg).data[0];
    (*opal_msg).cmd = (*msg).data[1];
    if (*msg).data_size > 2 {
        memcpy((*opal_msg).data.as_mut_ptr() as *mut c_void,
               (*msg).data.as_ptr().add(2) as *const c_void,
               (*msg).data_size - 2);
    }

    size = core::mem::size_of::<opal_ipmi_msg>() + (*msg).data_size - 2;
    rc = opal_ipmi_send((*smi).interface_id, opal_msg, size);
    if rc != 0 {
        comp = IPMI_ERR_UNSPECIFIED;
        spin_unlock_irqrestore(&mut (*smi).msg_lock, flags);
        return comp;
    }

    (*smi).cur_msg = msg;
    spin_unlock_irqrestore(&mut (*smi).msg_lock, flags);
    IPMI_CC_NO_ERROR
}

unsafe fn ipmi_powernv_recv(smi: *mut IpmiSmiPowernv) -> c_int {
    let opal_msg: *mut opal_ipmi_msg;
    let msg: *mut ipmi_smi_msg;
    let mut flags: c_ulong = 0;
    let mut size: u64;
    let rc: c_int;

    spin_lock_irqsave(&mut (*smi).msg_lock, &mut flags);
    if (*smi).cur_msg.is_null() {
        spin_unlock_irqrestore(&mut (*smi).msg_lock, flags);
        return 0;
    }
    msg = (*smi).cur_msg;
    opal_msg = (*smi).opal_msg;
    size = cpu_to_be64((core::mem::size_of::<opal_ipmi_msg>() + IPMI_MAX_MSG_LENGTH) as u64);
    rc = opal_ipmi_recv((*smi).interface_id, opal_msg, &mut size);
    size = be64_to_cpu(size);
    if rc != 0 {
        if rc == OPAL_EMPTY {
            spin_unlock_irqrestore(&mut (*smi).msg_lock, flags);
            return 0;
        }
        (*smi).cur_msg = core::ptr::null_mut();
        spin_unlock_irqrestore(&mut (*smi).msg_lock, flags);
        send_error_reply(smi, msg, IPMI_ERR_UNSPECIFIED);
        return 0;
    }
    if size < core::mem::size_of::<opal_ipmi_msg>() as u64 {
        spin_unlock_irqrestore(&mut (*smi).msg_lock, flags);
        return 0;
    }
    if (*opal_msg).version != OPAL_IPMI_MSG_FORMAT_VERSION_1 {
        spin_unlock_irqrestore(&mut (*smi).msg_lock, flags);
        return 0;
    }
    (*msg).rsp[0] = (*opal_msg).netfn;
    (*msg).rsp[1] = (*opal_msg).cmd;
    if size > core::mem::size_of::<opal_ipmi_msg>() as u64 {
        memcpy((*msg).rsp.as_mut_ptr().add(2) as *mut c_void,
               (*opal_msg).data.as_ptr() as *const c_void,
               (size as usize) - core::mem::size_of::<opal_ipmi_msg>());
    }
    (*msg).rsp_size = 2 + (size as usize) - core::mem::size_of::<opal_ipmi_msg>();
    (*smi).cur_msg = core::ptr::null_mut();
    spin_unlock_irqrestore(&mut (*smi).msg_lock, flags);
    ipmi_smi_msg_received((*smi).intf, msg);
    0
}

unsafe fn ipmi_powernv_request_events(_send_info: *mut c_void) {}
unsafe fn ipmi_powernv_set_run_to_completion(_send_info: *mut c_void, _run_to_completion: bool) {}
unsafe fn ipmi_powernv_poll(send_info: *mut c_void) {
    ipmi_powernv_recv(send_info as *mut IpmiSmiPowernv);
}

unsafe fn ipmi_opal_event(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    ipmi_powernv_recv(data as *mut IpmiSmiPowernv);
    IRQ_HANDLED
}

unsafe fn ipmi_powernv_probe(pdev: *mut platform_device) -> c_int {
    if pdev.is_null() || (*pdev).dev.of_node.is_null() { return -ENODEV; }
    let dev = &mut (*pdev).dev;
    let ipmi = devm_kzalloc(dev, core::mem::size_of::<IpmiSmiPowernv>(), GFP_KERNEL)
        as *mut IpmiSmiPowernv;
    if ipmi.is_null() { return -ENOMEM; }
    spin_lock_init(&mut (*ipmi).msg_lock);
    let mut prop: u32 = 0;
    let mut rc = of_property_read_u32((*dev).of_node, b"ibm,ipmi-interface-id\0".as_ptr(), &mut prop);
    if rc != 0 { dev_warn(dev, b"No interface ID property\0".as_ptr()); return rc; }
    (*ipmi).interface_id = prop as u64;
    rc = of_property_read_u32((*dev).of_node, b"interrupts\0".as_ptr(), &mut prop);
    if rc != 0 { dev_warn(dev, b"No interrupts property\0".as_ptr()); return rc; }
    (*ipmi).irq = irq_of_parse_and_map((*dev).of_node, 0);
    if (*ipmi).irq == 0 { (*ipmi).irq = opal_event_request(prop); }
    rc = request_irq((*ipmi).irq, Some(ipmi_opal_event), IRQ_TYPE_LEVEL_HIGH,
                     b"opal-ipmi\0".as_ptr(), ipmi as *mut c_void);
    if rc != 0 { irq_dispose_mapping((*ipmi).irq); return rc; }
    (*ipmi).opal_msg = devm_kmalloc(dev,
        core::mem::size_of::<opal_ipmi_msg>() + IPMI_MAX_MSG_LENGTH, GFP_KERNEL)
        as *mut opal_ipmi_msg;
    if (*ipmi).opal_msg.is_null() { free_irq((*ipmi).irq, ipmi as *mut c_void); return -ENOMEM; }
    rc = ipmi_register_smi(&ipmi_powernv_smi_handlers, ipmi as *mut c_void, dev, 0);
    if rc != 0 {
        devm_kfree(dev, (*ipmi).opal_msg as *mut c_void);
        free_irq((*ipmi).irq, ipmi as *mut c_void);
        irq_dispose_mapping((*ipmi).irq);
        return rc;
    }
    dev_set_drvdata(dev, ipmi as *mut c_void);
    0
}

unsafe fn ipmi_powernv_remove(pdev: *mut platform_device) {
    let smi = dev_get_drvdata(&mut (*pdev).dev) as *mut IpmiSmiPowernv;
    ipmi_unregister_smi((*smi).intf);
    free_irq((*smi).irq, smi as *mut c_void);
    irq_dispose_mapping((*smi).irq);
}

#[repr(C)]
struct OfDeviceId { compatible: *const u8 }
static IPMI_POWERNV_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"ibm,opal-ipmi\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

static mut POWERNV_IPMI_DRIVER: platform_driver = platform_driver { };

// module_platform_driver(powernv_ipmi_driver);
// MODULE_DEVICE_TABLE(of, ipmi_powernv_match);
// MODULE_DESCRIPTION("powernv IPMI driver");
// MODULE_AUTHOR("Jeremy Kerr <jk@ozlabs.org>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
