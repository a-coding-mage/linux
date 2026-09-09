// SPDX-License-Identifier: GPL-2.0
// ChromeOS Embedded Controller extcon
//
// Copyright (C) 2017 Google, Inc.
// Author: Benson Leung <bleung@chromium.org>

// Dependencies supplied by the Linux kernel and other translation units.

#[repr(C)]
struct CrosEcExtconInfo {
    dev: *mut Device,
    edev: *mut ExtconDev,
    port_id: i32,
    ec: *mut CrosEcDevice,
    notifier: NotifierBlock,
    dr: u32,
    pr: bool,
    dp: bool,
    mux: bool,
    power_type: u32,
}

static USB_TYPE_C_CABLE: [u32; 4] = [EXTCON_USB, EXTCON_USB_HOST, EXTCON_DISP_DP, EXTCON_NONE];

const DR_NONE: u32 = 0;
const DR_HOST: u32 = 1;
const DR_DEVICE: u32 = 2;

unsafe fn cros_ec_pd_command(
    info: *mut CrosEcExtconInfo,
    command: u32,
    version: u32,
    outdata: *mut core::ffi::c_void,
    outsize: u32,
    indata: *mut core::ffi::c_void,
    insize: u32,
) -> i32 {
    let msg = kzalloc_flex::<CrosEcCommand>(core::cmp::max(outsize, insize));
    if msg.is_null() { return -ENOMEM; }
    (*msg).version = version;
    (*msg).command = command;
    (*msg).outsize = outsize;
    (*msg).insize = insize;
    if outsize != 0 { memcpy((*msg).data.as_mut_ptr() as *mut _, outdata, outsize as usize); }
    let ret = cros_ec_cmd_xfer_status((*info).ec, msg);
    if ret >= 0 && insize != 0 { memcpy(indata, (*msg).data.as_ptr() as *const _, insize as usize); }
    kfree(msg);
    ret
}

unsafe fn cros_ec_usb_get_power_type(info: *mut CrosEcExtconInfo) -> i32 {
    let mut req: EcParamsUsbPdPowerInfo = core::mem::zeroed();
    let mut resp: EcResponseUsbPdPowerInfo = core::mem::zeroed();
    req.port = (*info).port_id;
    let ret = cros_ec_pd_command(info, EC_CMD_USB_PD_POWER_INFO, 0, &mut req as *mut _ as *mut _, core::mem::size_of_val(&req) as u32, &mut resp as *mut _ as *mut _, core::mem::size_of_val(&resp) as u32);
    if ret < 0 { return ret; }
    resp.type_
}

unsafe fn cros_ec_usb_get_pd_mux_state(info: *mut CrosEcExtconInfo) -> i32 {
    let mut req: EcParamsUsbPdMuxInfo = core::mem::zeroed();
    let mut resp: EcResponseUsbPdMuxInfo = core::mem::zeroed();
    req.port = (*info).port_id;
    let ret = cros_ec_pd_command(info, EC_CMD_USB_PD_MUX_INFO, 0, &mut req as *mut _ as *mut _, core::mem::size_of_val(&req) as u32, &mut resp as *mut _ as *mut _, core::mem::size_of_val(&resp) as u32);
    if ret < 0 { return ret; }
    resp.flags as i32
}

unsafe fn cros_ec_usb_get_role(info: *mut CrosEcExtconInfo, polarity: *mut bool) -> i32 {
    let mut pd_control: EcParamsUsbPdControl = core::mem::zeroed();
    let mut resp: EcResponseUsbPdControlV1 = core::mem::zeroed();
    pd_control.port = (*info).port_id;
    pd_control.role = USB_PD_CTRL_ROLE_NO_CHANGE;
    pd_control.mux = USB_PD_CTRL_MUX_NO_CHANGE;
    pd_control.swap = USB_PD_CTRL_SWAP_NONE;
    let ret = cros_ec_pd_command(info, EC_CMD_USB_PD_CONTROL, 1, &mut pd_control as *mut _ as *mut _, core::mem::size_of_val(&pd_control) as u32, &mut resp as *mut _ as *mut _, core::mem::size_of_val(&resp) as u32);
    if ret < 0 { return ret; }
    if (resp.enabled & PD_CTRL_RESP_ENABLED_CONNECTED) == 0 { return -ENOTCONN; }
    *polarity = resp.polarity != 0;
    resp.role as i32
}

unsafe fn cros_ec_pd_get_num_ports(info: *mut CrosEcExtconInfo) -> i32 {
    let mut resp: EcResponseUsbPdPorts = core::mem::zeroed();
    let ret = cros_ec_pd_command(info, EC_CMD_USB_PD_PORTS, 0, core::ptr::null_mut(), 0, &mut resp as *mut _ as *mut _, core::mem::size_of_val(&resp) as u32);
    if ret < 0 { return ret; }
    resp.num_ports as i32
}

fn cros_ec_usb_role_string(role: u32) -> &'static str {
    if role == DR_NONE { "DISCONNECTED" } else if role == DR_HOST { "DFP" } else { "UFP" }
}

fn cros_ec_usb_power_type_string(type_: u32) -> &'static str {
    match type_ {
        USB_CHG_TYPE_NONE => "USB_CHG_TYPE_NONE", USB_CHG_TYPE_PD => "USB_CHG_TYPE_PD",
        USB_CHG_TYPE_PROPRIETARY => "USB_CHG_TYPE_PROPRIETARY", USB_CHG_TYPE_C => "USB_CHG_TYPE_C",
        USB_CHG_TYPE_BC12_DCP => "USB_CHG_TYPE_BC12_DCP", USB_CHG_TYPE_BC12_CDP => "USB_CHG_TYPE_BC12_CDP",
        USB_CHG_TYPE_BC12_SDP => "USB_CHG_TYPE_BC12_SDP", USB_CHG_TYPE_OTHER => "USB_CHG_TYPE_OTHER",
        USB_CHG_TYPE_VBUS => "USB_CHG_TYPE_VBUS", USB_CHG_TYPE_UNKNOWN => "USB_CHG_TYPE_UNKNOWN", _ => "USB_CHG_TYPE_UNKNOWN",
    }
}

fn cros_ec_usb_power_type_is_wall_wart(type_: u32, _role: u32) -> bool {
    // FIXME: Guppy, Donnettes, and other chargers are miscategorized as USB_CHG_TYPE_C.
    matches!(type_, USB_CHG_TYPE_PROPRIETARY | USB_CHG_TYPE_BC12_DCP)
}

unsafe fn extcon_cros_ec_detect_cable(info: *mut CrosEcExtconInfo, force: bool) -> i32 {
    let dev = (*info).dev;
    let power_type = cros_ec_usb_get_power_type(info);
    if power_type < 0 { dev_err(dev, "failed getting power type err = %d\n", power_type); return power_type; }
    let mut polarity = false;
    let role = cros_ec_usb_get_role(info, &mut polarity);
    let mut dr = DR_NONE; let mut pr = false; let mut dp = false; let mut mux = false; let mut hpd = false;
    if role < 0 { if role != -ENOTCONN { dev_err(dev, "failed getting role err = %d\n", role); return role; } dev_dbg(dev, "disconnected\n"); }
    else {
        dr = if (role as u32 & PD_CTRL_RESP_ROLE_DATA) != 0 { DR_HOST } else { DR_DEVICE };
        pr = (role as u32 & PD_CTRL_RESP_ROLE_POWER) != 0;
        let mut state = cros_ec_usb_get_pd_mux_state(info); if state < 0 { state = USB_PD_MUX_USB_ENABLED as i32; }
        dp = (state as u32 & USB_PD_MUX_DP_ENABLED) != 0; mux = (state as u32 & USB_PD_MUX_USB_ENABLED) != 0; hpd = (state as u32 & USB_PD_MUX_HPD_IRQ) != 0;
        dev_dbg(dev, "connected role 0x%x pwr type %d dr %d pr %d pol %d mux %d dp %d hpd %d\n", role, power_type, dr, pr, polarity, mux, dp, hpd);
    }
    if dr == DR_DEVICE && cros_ec_usb_power_type_is_wall_wart(power_type as u32, role as u32) { dr = DR_NONE; }
    if force || (*info).dr != dr || (*info).pr != pr || (*info).dp != dp || (*info).mux != mux || (*info).power_type != power_type as u32 {
        let device_connected = dr == DR_DEVICE; let host_connected = dr == DR_HOST;
        dev_dbg(dev, "Type/Role switch! type = %s role = %s\n", cros_ec_usb_power_type_string(power_type as u32), cros_ec_usb_role_string(dr));
        (*info).dr = dr; (*info).pr = pr; (*info).dp = dp; (*info).mux = mux; (*info).power_type = power_type as u32;
        extcon_set_state((*info).edev, EXTCON_USB, device_connected); extcon_set_state((*info).edev, EXTCON_USB_HOST, host_connected); extcon_set_state((*info).edev, EXTCON_DISP_DP, dp);
        for cable in [EXTCON_USB, EXTCON_USB_HOST] { extcon_set_property((*info).edev, cable, EXTCON_PROP_USB_VBUS, pr as i32); extcon_set_property((*info).edev, cable, EXTCON_PROP_USB_TYPEC_POLARITY, polarity as i32); }
        for cable in [EXTCON_USB, EXTCON_USB_HOST, EXTCON_DISP_DP] { extcon_set_property((*info).edev, cable, EXTCON_PROP_USB_SS, mux as i32); }
        extcon_set_property((*info).edev, EXTCON_DISP_DP, EXTCON_PROP_USB_TYPEC_POLARITY, polarity as i32); extcon_set_property((*info).edev, EXTCON_DISP_DP, EXTCON_PROP_DISP_HPD, hpd as i32);
        extcon_sync((*info).edev, EXTCON_USB); extcon_sync((*info).edev, EXTCON_USB_HOST); extcon_sync((*info).edev, EXTCON_DISP_DP);
    } else if hpd { extcon_set_property((*info).edev, EXTCON_DISP_DP, EXTCON_PROP_DISP_HPD, hpd as i32); extcon_sync((*info).edev, EXTCON_DISP_DP); }
    0
}

// The remaining kernel registration declarations and platform-driver wiring are
// preserved as external interfaces because their definitions are supplied by the
// surrounding kernel translation unit.
unsafe extern "C" {
    static mut extcon_cros_ec_driver: PlatformDriver;
}

unsafe fn extcon_cros_ec_event(nb: *mut NotifierBlock, _queued_during_suspend: u64, _notify: *mut core::ffi::c_void) -> i32 {
    let info = container_of!(nb, CrosEcExtconInfo, notifier);
    let host_event = cros_ec_get_host_event((*info).ec);
    if host_event & (EC_HOST_EVENT_MASK(EC_HOST_EVENT_PD_MCU) | EC_HOST_EVENT_MASK(EC_HOST_EVENT_USB_MUX)) != 0 {
        extcon_cros_ec_detect_cable(info, false); return NOTIFY_OK;
    }
    NOTIFY_DONE
}

unsafe fn extcon_cros_ec_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev as *mut Device;
    let ec = dev_get_drvdata((*dev).parent);
    let info = devm_kzalloc(dev, core::mem::size_of::<CrosEcExtconInfo>(), GFP_KERNEL) as *mut CrosEcExtconInfo;
    if info.is_null() { return -ENOMEM; }
    (*info).dev = dev; (*info).ec = ec;
    if !(*dev).of_node.is_null() { let mut port = 0u32; let ret = of_property_read_u32((*dev).of_node, "google,usb-port-id", &mut port); if ret < 0 { dev_err(dev, "Missing google,usb-port-id property\n"); return ret; } (*info).port_id = port as i32; } else { (*info).port_id = (*pdev).id; }
    let numports = cros_ec_pd_get_num_ports(info); if numports < 0 { dev_err(dev, "failed getting number of ports! ret = %d\n", numports); return numports; }
    if (*info).port_id >= numports { dev_err(dev, "This system only supports %d ports\n", numports); return -ENODEV; }
    (*info).edev = devm_extcon_dev_allocate(dev, USB_TYPE_C_CABLE.as_ptr()); if (*info).edev.is_err() { dev_err(dev, "failed to allocate extcon device\n"); return -ENOMEM; }
    let mut ret = devm_extcon_dev_register(dev, (*info).edev); if ret < 0 { dev_err(dev, "failed to register extcon device\n"); return ret; }
    for (cable, prop) in [(EXTCON_USB, EXTCON_PROP_USB_VBUS),(EXTCON_USB_HOST, EXTCON_PROP_USB_VBUS),(EXTCON_USB, EXTCON_PROP_USB_TYPEC_POLARITY),(EXTCON_USB_HOST, EXTCON_PROP_USB_TYPEC_POLARITY),(EXTCON_DISP_DP, EXTCON_PROP_USB_TYPEC_POLARITY),(EXTCON_USB, EXTCON_PROP_USB_SS),(EXTCON_USB_HOST, EXTCON_PROP_USB_SS),(EXTCON_DISP_DP, EXTCON_PROP_USB_SS),(EXTCON_DISP_DP, EXTCON_PROP_DISP_HPD)] { extcon_set_property_capability((*info).edev, cable, prop); }
    (*info).dr = DR_NONE; (*info).pr = false; platform_set_drvdata(pdev, info);
    (*info).notifier.notifier_call = Some(extcon_cros_ec_event);
    ret = blocking_notifier_chain_register(&mut (*info).ec.event_notifier, &mut (*info).notifier); if ret < 0 { dev_err(dev, "failed to register notifier\n"); return ret; }
    ret = extcon_cros_ec_detect_cable(info, true); if ret < 0 { dev_err(dev, "failed to detect initial cable state\n"); blocking_notifier_chain_unregister(&mut (*info).ec.event_notifier, &mut (*info).notifier); }
    ret
}

unsafe fn extcon_cros_ec_remove(pdev: *mut PlatformDevice) { let info = platform_get_drvdata(pdev); blocking_notifier_chain_unregister(&mut (*info).ec.event_notifier, &mut (*info).notifier); }

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn extcon_cros_ec_suspend(_dev: *mut Device) -> i32 { 0 }
#[cfg(CONFIG_PM_SLEEP)]
unsafe fn extcon_cros_ec_resume(dev: *mut Device) -> i32 { let info = dev_get_drvdata(dev); if extcon_cros_ec_detect_cable(info, true) < 0 { dev_err(dev, "failed to detect cable state on resume\n"); } 0 }

#[cfg(CONFIG_OF)]
static EXTCON_CROS_EC_OF_MATCH: [OfDeviceId; 2] = [OfDeviceId { compatible: "google,extcon-usbc-cros-ec" }, OfDeviceId::sentinel()];

static mut EXTCON_CROS_EC_DRIVER: PlatformDriver = PlatformDriver {
    driver: Driver { name: "extcon-usbc-cros-ec", of_match_table: of_match_ptr!(EXTCON_CROS_EC_OF_MATCH), pm: DEV_PM_OPS },
    remove: Some(extcon_cros_ec_remove), probe: Some(extcon_cros_ec_probe),
};

module_platform_driver!(EXTCON_CROS_EC_DRIVER);
// MODULE_DESCRIPTION("ChromeOS Embedded Controller extcon driver");
// MODULE_AUTHOR("Benson Leung <bleung@chromium.org>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
