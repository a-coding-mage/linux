// SPDX-License-Identifier: GPL-2.0
/*
 * Defines interfaces for interacting with the Raspberry Pi firmware's
 * property channel.
 *
 * Copyright © 2015 Broadcom
 */

// Kernel dependencies supplied by other translation units/headers.

const MBOX_CHAN_PROPERTY: u32 = 8;

#[inline]
const fn mbox_msg(chan: u32, data28: u32) -> u32 { (data28 & !0xf) | (chan & 0xf) }
#[inline]
const fn mbox_chan(msg: u32) -> u32 { msg & 0xf }
#[inline]
const fn mbox_data28(msg: u32) -> u32 { msg & !0xf }

static mut RPI_HWMON: *mut platform_device = core::ptr::null_mut();
static mut RPI_CLK: *mut platform_device = core::ptr::null_mut();

#[repr(C)]
pub struct rpi_firmware {
    pub cl: mbox_client,
    pub chan: *mut mbox_chan, // The property channel.
    pub c: completion,
    pub enabled: u32,
    pub consumers: kref,
}

extern "C" {
    static mut transaction_lock: mutex;
    fn complete(c: *mut completion);
    fn reinit_completion(c: *mut completion);
    fn mbox_send_message(chan: *mut mbox_chan, msg: *mut u32) -> i32;
    fn wait_for_completion_timeout(c: *mut completion, timeout: u64) -> u64;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn dma_alloc_coherent(dev: *mut device, size: usize, addr: *mut dma_addr_t, flags: u32) -> *mut u32;
    fn dma_free_coherent(dev: *mut device, size: usize, vaddr: *mut u32, addr: dma_addr_t);
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn mbox_free_channel(chan: *mut mbox_chan);
    fn mbox_request_channel(cl: *mut mbox_client, index: i32) -> *mut mbox_chan;
    fn init_completion(c: *mut completion);
    fn kref_init(k: *mut kref);
    fn kref_put(k: *mut kref, release: unsafe extern "C" fn(*mut kref));
    fn kref_get_unless_zero(k: *mut kref) -> bool;
    fn put_device(dev: *mut device);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut rpi_firmware;
    fn of_find_device_by_node(node: *mut device_node) -> *mut platform_device;
    fn of_find_matching_node(from: *mut device_node, match_table: *const of_device_id) -> *mut device_node;
    fn of_get_compatible_child(node: *mut device_node, compatible: *const u8) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn platform_device_register_data(dev: *mut device, name: *const u8, id: i32, data: *const core::ffi::c_void, size: usize) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut core::ffi::c_void), data: *mut core::ffi::c_void) -> i32;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const u8) -> i32;
}

// External kernel types and firmware constants are supplied by the corresponding headers.
type dma_addr_t = u64;
type time64_t = i64;
#[repr(C)] pub struct device { pub node: *mut device_node, _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device, _private: [u8; 0] }
#[repr(C)] pub struct mbox_chan { pub mbox: *mut mbox_controller, _private: [u8; 0] }
#[repr(C)] pub struct mbox_controller { pub dev: *mut device, _private: [u8; 0] }
#[repr(C)] pub struct mbox_client { pub dev: *mut device, pub rx_callback: Option<unsafe extern "C" fn(*mut mbox_client, *mut core::ffi::c_void)>, pub tx_block: bool }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct kref { _private: [u8; 0] }
#[repr(C)] pub struct of_device_id { pub compatible: *const u8 }
extern "C" { static rpi_firmware_of_match: [of_device_id; 2]; }

unsafe extern "C" fn response_callback(cl: *mut mbox_client, _msg: *mut core::ffi::c_void) {
    let fw = (cl as *mut u8).sub(core::mem::offset_of!(rpi_firmware, cl)) as *mut rpi_firmware;
    complete(&mut (*fw).c);
}

unsafe fn rpi_firmware_transaction(fw: *mut rpi_firmware, chan: u32, data: u32) -> i32 {
    let mut message = mbox_msg(chan, data);
    mutex_lock(&mut transaction_lock);
    reinit_completion(&mut (*fw).c);
    let mut ret = mbox_send_message((*fw).chan, &mut message);
    if ret >= 0 {
        ret = if wait_for_completion_timeout(&mut (*fw).c, HZ) != 0 { 0 } else { -ETIMEDOUT };
    }
    mutex_unlock(&mut transaction_lock);
    ret
}

pub unsafe extern "C" fn rpi_firmware_property_list(fw: *mut rpi_firmware, data: *mut core::ffi::c_void, tag_size: usize) -> i32 {
    let size = tag_size + 12;
    if size & 3 != 0 { return -EINVAL; }
    let mut bus_addr = 0;
    let buf = dma_alloc_coherent((*(*fw).chan).mbox.dev, page_align(size), &mut bus_addr, GFP_ATOMIC);
    if buf.is_null() { return -ENOMEM; }
    *buf = size as u32;
    *buf.add(1) = RPI_FIRMWARE_STATUS_REQUEST;
    core::ptr::copy_nonoverlapping(data as *const u8, buf.add(2) as *mut u8, tag_size);
    *buf.add(size / 4 - 1) = RPI_FIRMWARE_PROPERTY_END;
    let ret = rpi_firmware_transaction(fw, MBOX_CHAN_PROPERTY, bus_addr as u32);
    core::ptr::copy_nonoverlapping(buf.add(2) as *const u8, data as *mut u8, tag_size);
    dma_free_coherent((*(*fw).chan).mbox.dev, page_align(size), buf, bus_addr);
    ret
}

pub unsafe extern "C" fn rpi_firmware_property(fw: *mut rpi_firmware, tag: u32, tag_data: *mut core::ffi::c_void, buf_size: usize) -> i32 {
    let size = core::mem::size_of::<rpi_firmware_property_tag_header>() + buf_size;
    let data = kmalloc(size, GFP_KERNEL);
    if data.is_null() { return -ENOMEM; }
    let header = data as *mut rpi_firmware_property_tag_header;
    (*header).tag = tag; (*header).buf_size = buf_size as u32; (*header).req_resp_size = 0;
    core::ptr::copy_nonoverlapping(tag_data as *const u8, (data as *mut u8).add(core::mem::size_of::<rpi_firmware_property_tag_header>()), buf_size);
    let ret = rpi_firmware_property_list(fw, data, size);
    core::ptr::copy_nonoverlapping((data as *mut u8).add(core::mem::size_of::<rpi_firmware_property_tag_header>()), tag_data as *mut u8, buf_size);
    kfree(data); ret
}

#[repr(C)] pub struct rpi_firmware_property_tag_header { pub tag: u32, pub buf_size: u32, pub req_resp_size: u32 }
#[repr(C)] pub struct rpi_firmware_clk_rate_request { pub id: u32, pub rate: u32 }

pub unsafe extern "C" fn rpi_firmware_clk_get_max_rate(fw: *mut rpi_firmware, id: u32) -> u32 {
    let mut msg = rpi_firmware_clk_rate_request { id, rate: 0 };
    if rpi_firmware_property(fw, RPI_FIRMWARE_GET_MAX_CLOCK_RATE, &mut msg as *mut _ as *mut _, core::mem::size_of_val(&msg)) != 0 { return u32::MAX; }
    u32::from_le(msg.rate)
}

unsafe extern "C" fn rpi_firmware_delete(kref: *mut kref) {
    let fw = (kref as *mut u8).sub(core::mem::offset_of!(rpi_firmware, consumers)) as *mut rpi_firmware;
    mbox_free_channel((*fw).chan); kfree(fw as *mut _);
}
pub unsafe extern "C" fn rpi_firmware_put(fw: *mut rpi_firmware) { kref_put(&mut (*fw).consumers, rpi_firmware_delete); }

unsafe fn rpi_firmware_print_firmware_revision(fw: *mut rpi_firmware) {
    let mut packet = 0u32;
    if rpi_firmware_property(fw, RPI_FIRMWARE_GET_FIRMWARE_REVISION, &mut packet as *mut _ as *mut _, 4) != 0 { return; }
    let _date_and_time: time64_t = packet as time64_t;
}

unsafe fn rpi_register_hwmon_driver(dev: *mut device, fw: *mut rpi_firmware) {
    let mut packet = 0u32;
    if rpi_firmware_property(fw, RPI_FIRMWARE_GET_THROTTLED, &mut packet as *mut _ as *mut _, 4) != 0 { return; }
    RPI_HWMON = platform_device_register_data(dev, b"raspberrypi-hwmon\0".as_ptr(), -1, core::ptr::null(), 0);
}

unsafe fn rpi_register_clk_driver(dev: *mut device) {
    let firmware = of_get_compatible_child((*dev).node, b"raspberrypi,firmware-clocks\0".as_ptr());
    if !firmware.is_null() { of_node_put(firmware); return; }
    RPI_CLK = platform_device_register_data(dev, b"raspberrypi-clk\0".as_ptr(), -1, core::ptr::null(), 0);
}

unsafe extern "C" fn rpi_firmware_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let fw = kzalloc(core::mem::size_of::<rpi_firmware>(), GFP_KERNEL) as *mut rpi_firmware;
    if fw.is_null() { return -ENOMEM; }
    (*fw).cl.dev = dev; (*fw).cl.rx_callback = Some(response_callback); (*fw).cl.tx_block = true;
    (*fw).chan = mbox_request_channel(&mut (*fw).cl, 0);
    if (*fw).chan.is_null() { kfree(fw as *mut _); return -ENOMEM; }
    init_completion(&mut (*fw).c); kref_init(&mut (*fw).consumers); platform_set_drvdata(pdev, fw as *mut _);
    rpi_firmware_print_firmware_revision(fw); rpi_register_hwmon_driver(dev, fw); rpi_register_clk_driver(dev); 0
}

unsafe extern "C" fn rpi_firmware_shutdown(pdev: *mut platform_device) {
    let fw = platform_get_drvdata(pdev); if !fw.is_null() { rpi_firmware_property(fw, RPI_FIRMWARE_NOTIFY_REBOOT, core::ptr::null_mut(), 0); }
}

unsafe extern "C" fn rpi_firmware_remove(pdev: *mut platform_device) {
    let fw = platform_get_drvdata(pdev);
    platform_device_unregister(RPI_HWMON); RPI_HWMON = core::ptr::null_mut();
    platform_device_unregister(RPI_CLK); RPI_CLK = core::ptr::null_mut();
    rpi_firmware_put(fw);
}

pub unsafe extern "C" fn rpi_firmware_find_node() -> *mut device_node { of_find_matching_node(core::ptr::null_mut(), rpi_firmware_of_match.as_ptr()) }

pub unsafe extern "C" fn rpi_firmware_get(node: *mut device_node) -> *mut rpi_firmware {
    let pdev = of_find_device_by_node(node); if pdev.is_null() { return core::ptr::null_mut(); }
    let fw = platform_get_drvdata(pdev);
    if fw.is_null() || !kref_get_unless_zero(&mut (*fw).consumers) { put_device(&mut (*pdev).dev); return core::ptr::null_mut(); }
    put_device(&mut (*pdev).dev); fw
}

pub unsafe extern "C" fn devm_rpi_firmware_get(dev: *mut device, node: *mut device_node) -> *mut rpi_firmware {
    let fw = rpi_firmware_get(node); if fw.is_null() { return core::ptr::null_mut(); }
    if devm_add_action_or_reset(dev, devm_rpi_firmware_put, fw as *mut _) != 0 { return core::ptr::null_mut(); } fw
}

unsafe extern "C" fn devm_rpi_firmware_put(data: *mut core::ffi::c_void) { rpi_firmware_put(data as *mut rpi_firmware); }

extern "C" {
    static HZ: u64;
    static EINVAL: i32; static ENOMEM: i32; static ETIMEDOUT: i32;
    static GFP_ATOMIC: u32; static GFP_KERNEL: u32;
    static RPI_FIRMWARE_STATUS_REQUEST: u32; static RPI_FIRMWARE_STATUS_SUCCESS: u32;
    static RPI_FIRMWARE_PROPERTY_END: u32; static RPI_FIRMWARE_GET_MAX_CLOCK_RATE: u32;
    fn page_align(size: usize) -> usize;
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
