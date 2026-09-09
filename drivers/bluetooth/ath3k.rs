// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2008-2009 Atheros Communications Inc.
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const VERSION: &str = "1.0";
const ATH3K_FIRMWARE: &str = "ath3k-1.fw";

const ATH3K_DNLOAD: u8 = 0x01;
const ATH3K_GETSTATE: u8 = 0x05;
const ATH3K_SET_NORMAL_MODE: u8 = 0x07;
const ATH3K_GETVERSION: u8 = 0x09;
const USB_REG_SWITCH_VID_PID: u8 = 0x0a;
const ATH3K_MODE_MASK: u8 = 0x3F;
const ATH3K_NORMAL_MODE: u8 = 0x0E;
const ATH3K_PATCH_UPDATE: u8 = 0x80;
const ATH3K_SYSCFG_UPDATE: u8 = 0x40;
const ATH3K_XTAL_FREQ_26M: u8 = 0x00;
const ATH3K_XTAL_FREQ_40M: u8 = 0x01;
const ATH3K_XTAL_FREQ_19P2: u8 = 0x02;
const ATH3K_NAME_LEN: usize = 0xFF;

#[repr(C, packed)]
struct ath3k_version {
    rom_version: __le32,
    build_version: __le32,
    ram_version: __le32,
    ref_clock: __u8,
    reserved: [__u8; 7],
}

static ath3k_table: [struct_usb_device_id; /* terminating entry included */ 60] = [
    USB_DEVICE(0x0CF3, 0x3000),
    USB_DEVICE(0x0489, 0xE027), USB_DEVICE(0x0489, 0xE03D),
    USB_DEVICE(0x04F2, 0xAFF1), USB_DEVICE(0x0930, 0x0215),
    USB_DEVICE(0x0CF3, 0x3002), USB_DEVICE(0x0CF3, 0xE019),
    USB_DEVICE(0x13d3, 0x3304), USB_DEVICE(0x03F0, 0x311D),
    USB_DEVICE(0x0489, 0xe04d), USB_DEVICE(0x0489, 0xe04e),
    USB_DEVICE(0x0489, 0xe057), USB_DEVICE(0x0489, 0xe056),
    USB_DEVICE(0x0489, 0xe05f), USB_DEVICE(0x0489, 0xe076),
    USB_DEVICE(0x0489, 0xe078), USB_DEVICE(0x0489, 0xe095),
    USB_DEVICE(0x04c5, 0x1330), USB_DEVICE(0x04CA, 0x3004),
    USB_DEVICE(0x04CA, 0x3005), USB_DEVICE(0x04CA, 0x3006),
    USB_DEVICE(0x04CA, 0x3007), USB_DEVICE(0x04CA, 0x3008),
    USB_DEVICE(0x04CA, 0x300b), USB_DEVICE(0x04CA, 0x300d),
    USB_DEVICE(0x04CA, 0x300f), USB_DEVICE(0x04CA, 0x3010),
    USB_DEVICE(0x04CA, 0x3014), USB_DEVICE(0x04CA, 0x3018),
    USB_DEVICE(0x0930, 0x0219), USB_DEVICE(0x0930, 0x021c),
    USB_DEVICE(0x0930, 0x0220), USB_DEVICE(0x0930, 0x0227),
    USB_DEVICE(0x0b05, 0x17d0), USB_DEVICE(0x0CF3, 0x0036),
    USB_DEVICE(0x0CF3, 0x3004), USB_DEVICE(0x0CF3, 0x3008),
    USB_DEVICE(0x0CF3, 0x311D), USB_DEVICE(0x0CF3, 0x311E),
    USB_DEVICE(0x0CF3, 0x311F), USB_DEVICE(0x0cf3, 0x3121),
    USB_DEVICE(0x0CF3, 0x817a), USB_DEVICE(0x0CF3, 0x817b),
    USB_DEVICE(0x0cf3, 0xe003), USB_DEVICE(0x0CF3, 0xE004),
    USB_DEVICE(0x0CF3, 0xE005), USB_DEVICE(0x0CF3, 0xE006),
    USB_DEVICE(0x13d3, 0x3362), USB_DEVICE(0x13d3, 0x3375),
    USB_DEVICE(0x13d3, 0x3393), USB_DEVICE(0x13d3, 0x3395),
    USB_DEVICE(0x13d3, 0x3402), USB_DEVICE(0x13d3, 0x3408),
    USB_DEVICE(0x13d3, 0x3423), USB_DEVICE(0x13d3, 0x3432),
    USB_DEVICE(0x13d3, 0x3472), USB_DEVICE(0x13d3, 0x3474),
    USB_DEVICE(0x13d3, 0x3487), USB_DEVICE(0x13d3, 0x3490),
    USB_DEVICE(0x0489, 0xE02C), USB_DEVICE(0x0489, 0xE036),
    USB_DEVICE(0x0489, 0xE03C), Default::default(),
];

const BTUSB_ATH3012: usize = 0x80;
// This table is to load patch and sysconfig files for AR3012.
static ath3k_blist_tbl: [struct_usb_device_id; 50] = [
    USB_DEVICE_INFO(0x0489, 0xe04e, BTUSB_ATH3012), USB_DEVICE_INFO(0x0489, 0xe04d, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x0489, 0xe056, BTUSB_ATH3012), USB_DEVICE_INFO(0x0489, 0xe057, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x0489, 0xe05f, BTUSB_ATH3012), USB_DEVICE_INFO(0x0489, 0xe076, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x0489, 0xe078, BTUSB_ATH3012), USB_DEVICE_INFO(0x0489, 0xe095, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x04c5, 0x1330, BTUSB_ATH3012), USB_DEVICE_INFO(0x04ca, 0x3004, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x04ca, 0x3005, BTUSB_ATH3012), USB_DEVICE_INFO(0x04ca, 0x3006, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x04ca, 0x3007, BTUSB_ATH3012), USB_DEVICE_INFO(0x04ca, 0x3008, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x04ca, 0x300b, BTUSB_ATH3012), USB_DEVICE_INFO(0x04ca, 0x300d, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x04ca, 0x300f, BTUSB_ATH3012), USB_DEVICE_INFO(0x04ca, 0x3010, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x04ca, 0x3014, BTUSB_ATH3012), USB_DEVICE_INFO(0x04ca, 0x3018, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x0930, 0x0219, BTUSB_ATH3012), USB_DEVICE_INFO(0x0930, 0x021c, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x0930, 0x0220, BTUSB_ATH3012), USB_DEVICE_INFO(0x0930, 0x0227, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x0b05, 0x17d0, BTUSB_ATH3012), USB_DEVICE_INFO(0x0CF3, 0x0036, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x0cf3, 0x3004, BTUSB_ATH3012), USB_DEVICE_INFO(0x0cf3, 0x3008, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x0cf3, 0x311D, BTUSB_ATH3012), USB_DEVICE_INFO(0x0cf3, 0x311E, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x0cf3, 0x311F, BTUSB_ATH3012), USB_DEVICE_INFO(0x0cf3, 0x3121, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x0CF3, 0x817a, BTUSB_ATH3012), USB_DEVICE_INFO(0x0CF3, 0x817b, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x0cf3, 0xe004, BTUSB_ATH3012), USB_DEVICE_INFO(0x0cf3, 0xe005, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x0cf3, 0xe006, BTUSB_ATH3012), USB_DEVICE_INFO(0x0cf3, 0xe003, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x13d3, 0x3362, BTUSB_ATH3012), USB_DEVICE_INFO(0x13d3, 0x3375, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x13d3, 0x3393, BTUSB_ATH3012), USB_DEVICE_INFO(0x13d3, 0x3395, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x13d3, 0x3402, BTUSB_ATH3012), USB_DEVICE_INFO(0x13d3, 0x3408, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x13d3, 0x3423, BTUSB_ATH3012), USB_DEVICE_INFO(0x13d3, 0x3432, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x13d3, 0x3472, BTUSB_ATH3012), USB_DEVICE_INFO(0x13d3, 0x3474, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x13d3, 0x3487, BTUSB_ATH3012), USB_DEVICE_INFO(0x13d3, 0x3490, BTUSB_ATH3012),
    USB_DEVICE_INFO(0x0489, 0xE036, BTUSB_ATH3012), USB_DEVICE_INFO(0x0489, 0xE03C, BTUSB_ATH3012),
    Default::default(),
];

#[inline]
unsafe fn ath3k_log_failed_loading(err: c_int, len: c_int, size: c_int, count: c_int) {
    BT_ERR!("Firmware loading err = %d, len = %d, size = %d, count = %d", err, len, size, count);
}

const USB_REQ_DFU_DNLOAD: u8 = 1;
const BULK_SIZE: usize = 4096;
const FW_HDR_SIZE: usize = 20;
const TIMEGAP_USEC_MIN: u32 = 50;
const TIMEGAP_USEC_MAX: u32 = 100;

unsafe fn ath3k_load_firmware(udev: *mut usb_device, firmware: *const firmware) -> c_int {
    let send_buf = kmalloc(BULK_SIZE, GFP_KERNEL) as *mut u8;
    let mut len = 0;
    let mut err;
    let mut sent = 0;
    let mut count = (*firmware).size as usize;
    BT_DBG!("udev %p", udev);
    if send_buf.is_null() { BT_ERR!("Can't allocate memory chunk for firmware"); return -ENOMEM; }
    err = usb_control_msg_send(udev, 0, USB_REQ_DFU_DNLOAD, USB_TYPE_VENDOR, 0, 0,
        (*firmware).data, FW_HDR_SIZE, USB_CTRL_SET_TIMEOUT, GFP_KERNEL);
    if err != 0 { BT_ERR!("Can't change to loading configuration err"); kfree(send_buf as *mut c_void); return err; }
    sent += FW_HDR_SIZE; count -= FW_HDR_SIZE;
    let pipe = usb_sndbulkpipe(udev, 0x02);
    while count != 0 {
        usleep_range(TIMEGAP_USEC_MIN, TIMEGAP_USEC_MAX);
        let size = core::cmp::min(count, BULK_SIZE);
        memcpy(send_buf, (*firmware).data.add(sent), size);
        err = usb_bulk_msg(udev, pipe, send_buf, size, &mut len, 3000);
        if err != 0 || len != size as c_int { ath3k_log_failed_loading(err, len, size as c_int, count as c_int); kfree(send_buf as *mut c_void); return err; }
        sent += size; count -= size;
    }
    kfree(send_buf as *mut c_void); err
}

unsafe fn ath3k_get_state(udev: *mut usb_device, state: *mut u8) -> c_int {
    usb_control_msg_recv(udev, 0, ATH3K_GETSTATE, USB_TYPE_VENDOR | USB_DIR_IN, 0, 0, state, 1, USB_CTRL_SET_TIMEOUT, GFP_KERNEL)
}

unsafe fn ath3k_get_version(udev: *mut usb_device, version: *mut ath3k_version) -> c_int {
    usb_control_msg_recv(udev, 0, ATH3K_GETVERSION, USB_TYPE_VENDOR | USB_DIR_IN, 0, 0, version as *mut c_void, core::mem::size_of::<ath3k_version>(), USB_CTRL_SET_TIMEOUT, GFP_KERNEL)
}

unsafe fn ath3k_load_fwfile(udev: *mut usb_device, firmware: *const firmware) -> c_int {
    let send_buf = kmalloc(BULK_SIZE, GFP_KERNEL) as *mut u8;
    if send_buf.is_null() { BT_ERR!("Can't allocate memory chunk for firmware"); return -ENOMEM; }
    let mut count = (*firmware).size as usize; let mut sent = 0; let mut len = 0;
    let size = core::cmp::min(count, FW_HDR_SIZE);
    let ret = usb_control_msg_send(udev, 0, ATH3K_DNLOAD, USB_TYPE_VENDOR, 0, 0, (*firmware).data, size, USB_CTRL_SET_TIMEOUT, GFP_KERNEL);
    if ret != 0 { BT_ERR!("Can't change to loading configuration err"); kfree(send_buf as *mut c_void); return ret; }
    sent += size; count -= size; let pipe = usb_sndbulkpipe(udev, 0x02);
    while count != 0 { usleep_range(TIMEGAP_USEC_MIN, TIMEGAP_USEC_MAX); let size = core::cmp::min(count, BULK_SIZE); memcpy(send_buf, (*firmware).data.add(sent), size); let err = usb_bulk_msg(udev, pipe, send_buf, size, &mut len, 3000); if err != 0 || len != size as c_int { ath3k_log_failed_loading(err, len, size as c_int, count as c_int); kfree(send_buf as *mut c_void); return err; } sent += size; count -= size; }
    kfree(send_buf as *mut c_void); 0
}

unsafe fn ath3k_switch_pid(udev: *mut usb_device) { usb_control_msg_send(udev, 0, USB_REG_SWITCH_VID_PID, USB_TYPE_VENDOR, 0, 0, core::ptr::null(), 0, USB_CTRL_SET_TIMEOUT, GFP_KERNEL); }

unsafe fn ath3k_set_normal_mode(udev: *mut usb_device) -> c_int {
    let mut fw_state = 0; let ret = ath3k_get_state(udev, &mut fw_state);
    if ret != 0 { BT_ERR!("Can't get state to change to normal mode err"); return ret; }
    if fw_state & ATH3K_MODE_MASK == ATH3K_NORMAL_MODE { BT_DBG!("firmware was already in normal mode"); return 0; }
    usb_control_msg_send(udev, 0, ATH3K_SET_NORMAL_MODE, USB_TYPE_VENDOR, 0, 0, core::ptr::null(), 0, USB_CTRL_SET_TIMEOUT, GFP_KERNEL)
}

unsafe fn ath3k_load_patch(udev: *mut usb_device) -> c_int {
    let mut state = 0; let mut version = core::mem::zeroed::<ath3k_version>();
    let ret = ath3k_get_state(udev, &mut state); if ret != 0 { BT_ERR!("Can't get state to change to load ram patch err"); return ret; }
    if state & ATH3K_PATCH_UPDATE != 0 { BT_DBG!("Patch was already downloaded"); return 0; }
    let ret = ath3k_get_version(udev, &mut version); if ret != 0 { BT_ERR!("Can't get version to change to load ram patch err"); return ret; }
    let filename = format!("ar3k/AthrBT_0x{:08x}.dfu", le32_to_cpu(version.rom_version));
    let mut firmware = core::ptr::null(); let ret = request_firmware(&mut firmware, filename.as_ptr(), &(*udev).dev); if ret < 0 { BT_ERR!("Patch file not found %s", filename.as_ptr()); return ret; }
    let data = (*firmware).data; let size = (*firmware).size;
    let rom = get_unaligned_le32(data.add(size - 8)); let build = get_unaligned_le32(data.add(size - 4));
    if rom != le32_to_cpu(version.rom_version) || build <= le32_to_cpu(version.build_version) { BT_ERR!("Patch file version did not match with firmware"); release_firmware(firmware); return -EINVAL; }
    let ret = ath3k_load_fwfile(udev, firmware); release_firmware(firmware); ret
}

unsafe fn ath3k_load_syscfg(udev: *mut usb_device) -> c_int {
    let mut state = 0; let mut version = core::mem::zeroed::<ath3k_version>();
    if ath3k_get_state(udev, &mut state) != 0 { BT_ERR!("Can't get state to change to load configuration err"); return -EBUSY; }
    let ret = ath3k_get_version(udev, &mut version); if ret != 0 { BT_ERR!("Can't get version to change to load ram patch err"); return ret; }
    let clk = match version.ref_clock { ATH3K_XTAL_FREQ_26M => 26, ATH3K_XTAL_FREQ_40M => 40, ATH3K_XTAL_FREQ_19P2 => 19, _ => 0 };
    let filename = format!("ar3k/ramps_0x{:08x}_{}.dfu", le32_to_cpu(version.rom_version), clk);
    let mut firmware = core::ptr::null(); let ret = request_firmware(&mut firmware, filename.as_ptr(), &(*udev).dev); if ret < 0 { BT_ERR!("Configuration file not found %s", filename.as_ptr()); return ret; }
    let ret = ath3k_load_fwfile(udev, firmware); release_firmware(firmware); ret
}

unsafe fn ath3k_probe(intf: *mut usb_interface, id: *const struct_usb_device_id) -> c_int {
    let udev = interface_to_usbdev(intf); if (*(*intf).cur_altsetting).desc.bInterfaceNumber != 0 { return -ENODEV; }
    let mut matched = id; if (*id).driver_info == 0 { let m = usb_match_id(intf, ath3k_blist_tbl.as_ptr()); if !m.is_null() { matched = m; } }
    if (*matched).driver_info & BTUSB_ATH3012 != 0 { if le16_to_cpu((*udev).descriptor.bcdDevice) > 1 { return -ENODEV; } let ret = ath3k_load_patch(udev); if ret < 0 { return ret; } let ret = ath3k_load_syscfg(udev); if ret < 0 { return ret; } let ret = ath3k_set_normal_mode(udev); if ret != 0 { return ret; } ath3k_switch_pid(udev); return 0; }
    let mut firmware = core::ptr::null(); let ret = request_firmware(&mut firmware, ATH3K_FIRMWARE.as_ptr(), &(*udev).dev); if ret < 0 { return ret; } let ret = ath3k_load_firmware(udev, firmware); release_firmware(firmware); ret
}

unsafe fn ath3k_disconnect(intf: *mut usb_interface) { BT_DBG!("%s intf %p", __func__, intf); }

// module_usb_driver(ath3k_driver) and MODULE_* declarations are kernel build metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
