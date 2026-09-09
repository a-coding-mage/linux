// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2026, STMicroelectronics - All Rights Reserved
 */

// Linux kernel dependencies supplied by the surrounding translation.

#[repr(C)]
enum Stm32DbgProfile {
    PeripheralDbgProfile = 0,
    HdpDbgProfile = 1,
}

#[repr(C)]
enum Stm32DbgPtaCommand {
    PtaCmdGrantDbgAccess = 0,
}

#[repr(C)]
struct Stm32DbgBus {
    dev: *mut Device,
    ctx: *mut TeeContext,
}

// Expect at most 1 instance of this driver.
static mut STM32_DBG_BUS_PRIV: *mut Stm32DbgBus = core::ptr::null_mut();

unsafe fn stm32_dbg_pta_open_session(id: *mut u32) -> i32 {
    let dbg_bus_dev = to_tee_client_device((*STM32_DBG_BUS_PRIV).dev);
    let mut sess_arg: TeeIoctlOpenSessionArg = core::mem::zeroed();
    export_uuid(sess_arg.uuid.as_mut_ptr(), &(*dbg_bus_dev).id.uuid);
    sess_arg.clnt_login = TEE_IOCTL_LOGIN_REE_KERNEL;

    let ret = tee_client_open_session((*STM32_DBG_BUS_PRIV).ctx, &mut sess_arg, core::ptr::null_mut());
    if ret < 0 || sess_arg.ret != 0 {
        dev_err((*STM32_DBG_BUS_PRIV).dev, "Failed opening tee session, err: %#x\n", sess_arg.ret);
        return -EOPNOTSUPP;
    }

    *id = sess_arg.session;
    0
}

unsafe fn stm32_dbg_pta_close_session(id: u32) {
    tee_client_close_session((*STM32_DBG_BUS_PRIV).ctx, id);
}

unsafe fn stm32_dbg_bus_grant_access(_ctrl: *mut Stm32FirewallController, dbg_profile: u32) -> i32 {
    let mut inv_arg: TeeIoctlInvokeArg = core::mem::zeroed();
    let mut param: [TeeParam; 1] = core::mem::zeroed();
    let mut session_id = 0u32;

    if dbg_profile != Stm32DbgProfile::PeripheralDbgProfile as u32
        && dbg_profile != Stm32DbgProfile::HdpDbgProfile as u32
    {
        return -EOPNOTSUPP;
    }

    let mut ret = stm32_dbg_pta_open_session(&mut session_id);
    if ret != 0 {
        return ret;
    }

    inv_arg.func = Stm32DbgPtaCommand::PtaCmdGrantDbgAccess as u32;
    inv_arg.session = session_id;
    inv_arg.num_params = 1;
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT;
    param[0].u.value.a = dbg_profile;

    ret = tee_client_invoke_func((*STM32_DBG_BUS_PRIV).ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 {
        dev_dbg((*STM32_DBG_BUS_PRIV).dev, "When invoking function, err %x, TEE returns: %x\n", ret, inv_arg.ret);
        if ret == 0 {
            ret = -EACCES;
        }
    }

    stm32_dbg_pta_close_session(session_id);
    ret
}

/* Implement mandatory release_access ops even if it does nothing. */
unsafe fn stm32_dbg_bus_release_access(_ctrl: *mut Stm32FirewallController, _dbg_profile: u32) {}

unsafe fn stm32_dbg_bus_plat_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut dbg_controller: *mut Stm32FirewallController;
    let ret: i32;

    /* Defer if OP-TEE service is not yet available. */
    if STM32_DBG_BUS_PRIV.is_null() {
        return -EPROBE_DEFER;
    }

    dbg_controller = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Stm32FirewallController>(), GFP_KERNEL) as *mut Stm32FirewallController;
    if dbg_controller.is_null() {
        return dev_err_probe(&mut (*pdev).dev, -ENOMEM, "Couldn't allocate debug controller\n");
    }

    (*dbg_controller).dev = &mut (*pdev).dev;
    (*dbg_controller).mmio = core::ptr::null_mut();
    (*dbg_controller).name = dev_driver_string((*dbg_controller).dev);
    (*dbg_controller).type_ = STM32_PERIPHERAL_FIREWALL;
    (*dbg_controller).grant_access = Some(stm32_dbg_bus_grant_access);
    (*dbg_controller).release_access = Some(stm32_dbg_bus_release_access);

    ret = stm32_firewall_controller_register(dbg_controller);
    if ret != 0 {
        dev_err((*dbg_controller).dev, "Couldn't register as a firewall controller: %d", ret);
        return ret;
    }

    ret = stm32_firewall_populate_bus(dbg_controller);
    if ret != 0 {
        dev_err((*dbg_controller).dev, "Couldn't populate debug bus: %d", ret);
        stm32_firewall_controller_unregister(dbg_controller);
        return ret;
    }

    pm_runtime_enable(&mut (*pdev).dev);
    ret = of_platform_populate((*pdev).dev.of_node, core::ptr::null(), core::ptr::null(), &mut (*pdev).dev);
    if ret != 0 {
        dev_err((*dbg_controller).dev, "Couldn't populate the node: %d", ret);
        stm32_firewall_controller_unregister(dbg_controller);
        return ret;
    }
    0
}

static STM32_DBG_BUS_OF_MATCH: [OfDeviceId; 3] = [
    OfDeviceId { compatible: "st,stm32mp131-dbg-bus" },
    OfDeviceId { compatible: "st,stm32mp151-dbg-bus" },
    OfDeviceId { compatible: core::ptr::null() },
];

static mut STM32_DBG_BUS_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(stm32_dbg_bus_plat_probe),
    driver: Driver { name: "stm32-dbg-bus", of_match_table: STM32_DBG_BUS_OF_MATCH.as_ptr() },
};

unsafe fn optee_ctx_match(ver: *mut TeeIoctlVersionData, _data: *const core::ffi::c_void) -> i32 {
    ((*ver).impl_id == TEE_IMPL_ID_OPTEE) as i32
}

unsafe fn stm32_dbg_bus_remove(tee_dev: *mut TeeClientDevice) {
    tee_client_close_context((*STM32_DBG_BUS_PRIV).ctx);
    STM32_DBG_BUS_PRIV = core::ptr::null_mut();
    of_platform_depopulate(&mut (*tee_dev).dev);
}

unsafe fn stm32_dbg_bus_probe(tee_dev: *mut TeeClientDevice) -> i32 {
    let dev = &mut (*tee_dev).dev;
    if !STM32_DBG_BUS_PRIV.is_null() {
        return dev_err_probe(dev, -EBUSY, "A STM32 debug bus device is already initialized\n");
    }
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<Stm32DbgBus>(), GFP_KERNEL) as *mut Stm32DbgBus;
    if priv_.is_null() { return -ENOMEM; }
    (*priv_).ctx = tee_client_open_context(core::ptr::null_mut(), Some(optee_ctx_match), core::ptr::null(), core::ptr::null_mut());
    if is_err_or_null((*priv_).ctx) { return dev_err_probe(dev, ptr_err_or_zero((*priv_).ctx), "Cannot open TEE context\n"); }
    STM32_DBG_BUS_PRIV = priv_;
    (*STM32_DBG_BUS_PRIV).dev = dev;
    0
}

static OPTEE_DBG_BUS_ID_TABLE: [TeeClientDeviceId; 2] = [
    TeeClientDeviceId { uuid: Uuid { b: [0xdd, 0x05, 0xbc, 0x8b, 0x9f, 0x3b, 0x49, 0xf0, 0xb6, 0x49, 0x01, 0xaa, 0x10, 0xc1, 0xc2, 0x10] } },
    TeeClientDeviceId { uuid: Uuid { b: [0; 16] } },
];

static mut STM32_OPTEE_DBG_BUS_DRIVER: TeeClientDriver = TeeClientDriver {
    id_table: OPTEE_DBG_BUS_ID_TABLE.as_ptr(), probe: Some(stm32_dbg_bus_probe), remove: Some(stm32_dbg_bus_remove),
    driver: Driver { name: "optee_dbg_bus", of_match_table: core::ptr::null() },
};

unsafe fn stm32_optee_dbg_bus_driver_exit() {
    platform_driver_unregister(&mut STM32_DBG_BUS_DRIVER as *mut _ as *mut PlatformDriver);
    tee_client_driver_unregister(&mut STM32_OPTEE_DBG_BUS_DRIVER);
}

unsafe fn stm32_optee_dbg_bus_driver_init() -> i32 {
    let err = tee_client_driver_register(&mut STM32_OPTEE_DBG_BUS_DRIVER);
    if err != 0 { return err; }
    let err = platform_driver_register(&mut STM32_DBG_BUS_DRIVER);
    if err != 0 { tee_client_driver_unregister(&mut STM32_OPTEE_DBG_BUS_DRIVER); }
    err
}

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Gatien Chevallier <gatien.chevallier@foss.st.com>");
// MODULE_DESCRIPTION("OP-TEE based STM32 debug access bus driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
