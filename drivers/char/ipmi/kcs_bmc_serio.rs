// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (c) 2021 IBM Corp. */

// Dependencies supplied by the surrounding kernel/Rust bindings:
// linux/delay.h, linux/device.h, linux/errno.h, linux/list.h,
// linux/module.h, linux/sched/signal.h, linux/serio.h, linux/slab.h,
// and kcs_bmc_client.h.

#[repr(C)]
struct KcsBmcSerio {
    entry: ListHead,
    client: KcsBmcClient,
    port: *mut Serio,
    lock: SpinlockT,
}

#[inline]
unsafe fn client_to_kcs_bmc_serio(client: *mut KcsBmcClient) -> *mut KcsBmcSerio {
    container_of!(client, KcsBmcSerio, client)
}

unsafe extern "C" fn kcs_bmc_serio_event(client: *mut KcsBmcClient) -> IrqreturnT {
    let priv_: *mut KcsBmcSerio;
    let mut handled: u8 = IRQ_NONE as u8;
    let status: u8;

    priv_ = client_to_kcs_bmc_serio(client);

    spin_lock(&mut (*priv_).lock);

    status = kcs_bmc_read_status((*client).dev);

    if status & KCS_BMC_STR_IBF != 0 {
        handled = serio_interrupt(
            (*priv_).port,
            kcs_bmc_read_data((*client).dev),
            0,
        );
    }

    spin_unlock(&mut (*priv_).lock);

    handled as IrqreturnT
}

static KCS_BMC_SERIO_CLIENT_OPS: KcsBmcClientOps = KcsBmcClientOps {
    event: Some(kcs_bmc_serio_event),
};

unsafe extern "C" fn kcs_bmc_serio_open(port: *mut Serio) -> i32 {
    let priv_: *mut KcsBmcSerio = (*port).port_data as *mut KcsBmcSerio;

    kcs_bmc_enable_device((*priv_).client.dev, &mut (*priv_).client)
}

unsafe extern "C" fn kcs_bmc_serio_close(port: *mut Serio) {
    let priv_: *mut KcsBmcSerio = (*port).port_data as *mut KcsBmcSerio;

    kcs_bmc_disable_device((*priv_).client.dev, &mut (*priv_).client);
}

static mut KCS_BMC_SERIO_INSTANCES_LOCK: SpinlockT = DEFINE_SPINLOCK!();
static mut KCS_BMC_SERIO_INSTANCES: ListHead = LIST_HEAD!();

unsafe extern "C" fn kcs_bmc_serio_add_device(kcs_bmc: *mut KcsBmcDevice) -> i32 {
    let priv_: *mut KcsBmcSerio;
    let port: *mut Serio;

    priv_ = devm_kzalloc((*kcs_bmc).dev, core::mem::size_of::<KcsBmcSerio>(), GFP_KERNEL);
    if priv_.is_null() {
        return -ENOMEM;
    }

    // Use kzalloc() as the allocation is cleaned up with kfree() via serio_unregister_port().
    port = kzalloc_obj::<Serio>();
    if port.is_null() {
        return -ENOMEM;
    }

    (*port).id.type_ = SERIO_8042;
    (*port).open = Some(kcs_bmc_serio_open);
    (*port).close = Some(kcs_bmc_serio_close);
    (*port).port_data = priv_ as *mut core::ffi::c_void;
    (*port).dev.parent = (*kcs_bmc).dev;

    spin_lock_init(&mut (*priv_).lock);
    (*priv_).port = port;
    (*priv_).client.dev = kcs_bmc;
    (*priv_).client.ops = &KCS_BMC_SERIO_CLIENT_OPS;

    spin_lock_irq(&mut KCS_BMC_SERIO_INSTANCES_LOCK);
    list_add(&mut (*priv_).entry, &mut KCS_BMC_SERIO_INSTANCES);
    spin_unlock_irq(&mut KCS_BMC_SERIO_INSTANCES_LOCK);

    serio_register_port(port);

    dev_info((*kcs_bmc).dev, "Initialised serio client for channel %d", (*kcs_bmc).channel);

    0
}

unsafe extern "C" fn kcs_bmc_serio_remove_device(kcs_bmc: *mut KcsBmcDevice) -> i32 {
    let mut priv_: *mut KcsBmcSerio = core::ptr::null_mut();
    let mut pos: *mut KcsBmcSerio;

    spin_lock_irq(&mut KCS_BMC_SERIO_INSTANCES_LOCK);
    list_for_each_entry!(pos, KCS_BMC_SERIO_INSTANCES, entry, {
        if (*pos).client.dev == kcs_bmc {
            priv_ = pos;
            list_del(&mut (*pos).entry);
            break;
        }
    });
    spin_unlock_irq(&mut KCS_BMC_SERIO_INSTANCES_LOCK);

    if priv_.is_null() {
        return -ENODEV;
    }

    // kfree()s priv->port via put_device().
    serio_unregister_port((*priv_).port);

    // Ensure the IBF IRQ is disabled if we were the active client.
    kcs_bmc_disable_device(kcs_bmc, &mut (*priv_).client);

    devm_kfree((*priv_).client.dev.dev, priv_ as *mut core::ffi::c_void);

    0
}

static KCS_BMC_SERIO_DRIVER_OPS: KcsBmcDriverOps = KcsBmcDriverOps {
    add_device: Some(kcs_bmc_serio_add_device),
    remove_device: Some(kcs_bmc_serio_remove_device),
};

static mut KCS_BMC_SERIO_DRIVER: KcsBmcDriver = KcsBmcDriver {
    ops: &KCS_BMC_SERIO_DRIVER_OPS,
};

unsafe extern "C" fn kcs_bmc_serio_init() -> i32 {
    kcs_bmc_register_driver(&mut KCS_BMC_SERIO_DRIVER);

    0
}
module_init!(kcs_bmc_serio_init);

unsafe extern "C" fn kcs_bmc_serio_exit() {
    kcs_bmc_unregister_driver(&mut KCS_BMC_SERIO_DRIVER);
}
module_exit!(kcs_bmc_serio_exit);

module_license!("GPL v2");
module_author!("Andrew Jeffery <andrew@aj.id.au>");
module_description!("Adapter driver for serio access to BMC KCS devices");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
