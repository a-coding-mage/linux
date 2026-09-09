// SPDX-License-Identifier: GPL-2.0-only

// External Linux/DRM declarations are supplied by other translated units.

const DSI_GEN_HDR: usize = 0x6c;
const DSI_GEN_PLD_DATA: usize = 0x70;
const DSI_CMD_PKT_STATUS: usize = 0x74;

const GEN_PLD_R_EMPTY: u32 = 1 << 4;
const GEN_PLD_W_FULL: u32 = 1 << 3;
const GEN_PLD_W_EMPTY: u32 = 1 << 2;
const GEN_CMD_FULL: u32 = 1 << 1;
const GEN_CMD_EMPTY: u32 = 1 << 0;
const GEN_RD_CMD_BUSY: u32 = 1 << 6;
const CMD_PKT_STATUS_TIMEOUT_US: u32 = 20000;

#[repr(C)]
struct adp_mipi_drv_private {
    dsi: mipi_dsi_host,
    bridge: drm_bridge,
    next_bridge: *mut drm_bridge,
    mipi: *mut core::ffi::c_void,
}

#[repr(C)] struct mipi_dsi_host { dev: *mut device, ops: *const mipi_dsi_host_ops }
#[repr(C)] struct drm_bridge { of_node: *mut device_node, bridge_type: u32 }
#[repr(C)] struct device { of_node: *mut device_node }
#[repr(C)] struct device_node;
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct drm_encoder;
#[repr(C)] struct mipi_dsi_device;
#[repr(C)] struct component_ops { bind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut core::ffi::c_void) -> i32>, unbind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut core::ffi::c_void)> }
#[repr(C)] struct mipi_dsi_host_ops { transfer: Option<unsafe extern "C" fn(*mut mipi_dsi_host, *const mipi_dsi_msg) -> isize>, attach: Option<unsafe extern "C" fn(*mut mipi_dsi_host, *mut mipi_dsi_device) -> i32>, detach: Option<unsafe extern "C" fn(*mut mipi_dsi_host, *mut mipi_dsi_device) -> i32> }
#[repr(C)] struct drm_bridge_funcs { atomic_create_state: *const core::ffi::c_void, atomic_destroy_state: *const core::ffi::c_void, atomic_duplicate_state: *const core::ffi::c_void, attach: Option<unsafe extern "C" fn(*mut drm_bridge, *mut drm_encoder, u32) -> i32> }
#[repr(C)] struct mipi_dsi_packet { payload: *const u8, payload_length: usize, header: *const u8, size: isize }
#[repr(C)] struct mipi_dsi_msg { rx_buf: *mut u8, rx_len: usize }

extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn mipi_dsi_create_packet(packet: *mut mipi_dsi_packet, msg: *const mipi_dsi_msg) -> i32;
    fn devm_drm_of_get_bridge(dev: *mut device, node: *mut device_node, port: u32, endpoint: u32) -> *mut drm_bridge;
    fn drm_bridge_add(bridge: *mut drm_bridge);
    fn drm_bridge_remove(bridge: *mut drm_bridge);
    fn component_add(dev: *mut device, ops: *const component_ops) -> i32;
    fn component_del(dev: *mut device, ops: *const component_ops);
    fn drm_bridge_attach(encoder: *mut drm_encoder, next: *mut drm_bridge, bridge: *mut drm_bridge, flags: u32) -> i32;
    fn devm_drm_bridge_alloc(dev: *mut device, size: usize, funcs: *const drm_bridge_funcs) -> *mut adp_mipi_drv_private;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut adp_mipi_drv_private);
    fn dev_get_drvdata(dev: *mut device) -> *mut adp_mipi_drv_private;
    fn mipi_dsi_host_register(host: *mut mipi_dsi_host) -> i32;
    fn mipi_dsi_host_unregister(host: *mut mipi_dsi_host);
    fn readl_poll_timeout(addr: *mut core::ffi::c_void, val: *mut u32, condition: bool, delay: u32, timeout: u32) -> i32;
}

unsafe fn adp_dsi_gen_pkt_hdr_write(adp: *mut adp_mipi_drv_private, hdr_val: u32) -> i32 {
    let mut val = 0;
    let ret = readl_poll_timeout((*adp).mipi.add(DSI_CMD_PKT_STATUS), &mut val, (val & GEN_CMD_FULL) == 0, 1000, CMD_PKT_STATUS_TIMEOUT_US);
    if ret != 0 { dev_err((*adp).dsi.dev, b"failed to get available command FIFO\0".as_ptr()); return ret; }
    writel(hdr_val, (*adp).mipi.add(DSI_GEN_HDR));
    let mask = GEN_CMD_EMPTY | GEN_PLD_W_EMPTY;
    let ret = readl_poll_timeout((*adp).mipi.add(DSI_CMD_PKT_STATUS), &mut val, (val & mask) == mask, 1000, CMD_PKT_STATUS_TIMEOUT_US);
    if ret != 0 { dev_err((*adp).dsi.dev, b"failed to write command FIFO\0".as_ptr()); return ret; }
    0
}

unsafe fn adp_dsi_write(adp: *mut adp_mipi_drv_private, packet: *const mipi_dsi_packet) -> i32 {
    let mut tx_buf = (*packet).payload;
    let mut len = (*packet).payload_length;
    let pld_data_bytes = core::mem::size_of::<u32>();
    let mut word: u32;
    let mut val = 0;
    while len != 0 {
        word = 0;
        let n = if len < pld_data_bytes { len } else { pld_data_bytes };
        core::ptr::copy_nonoverlapping(tx_buf, &mut word as *mut u32 as *mut u8, n);
        writel(u32::from_le(word), (*adp).mipi.add(DSI_GEN_PLD_DATA));
        if len < pld_data_bytes { len = 0; } else { tx_buf = tx_buf.add(pld_data_bytes); len -= pld_data_bytes; }
        let ret = readl_poll_timeout((*adp).mipi.add(DSI_CMD_PKT_STATUS), &mut val, (val & GEN_PLD_W_FULL) == 0, 1000, CMD_PKT_STATUS_TIMEOUT_US);
        if ret != 0 { dev_err((*adp).dsi.dev, b"failed to get available write payload FIFO\0".as_ptr()); return ret; }
    }
    word = 0;
    core::ptr::copy_nonoverlapping((*packet).header, &mut word as *mut u32 as *mut u8, core::mem::size_of::<[u8; 4]>());
    adp_dsi_gen_pkt_hdr_write(adp, u32::from_le(word))
}

unsafe fn adp_dsi_read(adp: *mut adp_mipi_drv_private, msg: *const mipi_dsi_msg) -> i32 {
    let len = (*msg).rx_len;
    let buf = (*msg).rx_buf;
    let mut val = 0;
    let mut ret = readl_poll_timeout((*adp).mipi.add(DSI_CMD_PKT_STATUS), &mut val, (val & GEN_RD_CMD_BUSY) == 0, 1000, CMD_PKT_STATUS_TIMEOUT_US);
    if ret != 0 { dev_err((*adp).dsi.dev, b"Timeout during read operation\n\0".as_ptr()); return ret; }
    let mut i = 0;
    while i < len {
        ret = readl_poll_timeout((*adp).mipi.add(DSI_CMD_PKT_STATUS), &mut val, (val & GEN_PLD_R_EMPTY) == 0, 1000, CMD_PKT_STATUS_TIMEOUT_US);
        if ret != 0 { dev_err((*adp).dsi.dev, b"Read payload FIFO is empty\n\0".as_ptr()); return ret; }
        val = readl((*adp).mipi.add(DSI_GEN_PLD_DATA));
        let mut j = 0;
        while j < 4 && j + i < len { *buf.add(i + j) = (val >> (8 * j)) as u8; j += 1; }
        i += 4;
    }
    ret
}

unsafe fn adp_dsi_host_transfer(host: *mut mipi_dsi_host, msg: *const mipi_dsi_msg) -> isize {
    let adp = host as *mut adp_mipi_drv_private;
    let mut packet = core::mem::MaybeUninit::<mipi_dsi_packet>::uninit();
    let mut ret = mipi_dsi_create_packet(packet.as_mut_ptr(), msg);
    if ret != 0 { dev_err((*adp).dsi.dev, b"failed to create packet: %d\n\0".as_ptr(), ret); return ret as isize; }
    let packet = packet.assume_init();
    ret = adp_dsi_write(adp, &packet);
    if ret != 0 { return ret as isize; }
    if !(*msg).rx_buf.is_null() && (*msg).rx_len != 0 { ret = adp_dsi_read(adp, msg); if ret != 0 { return ret as isize; } (*msg).rx_len as isize } else { packet.size }
}

unsafe fn adp_dsi_bind(_: *mut device, _: *mut device, _: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn adp_dsi_unbind(_: *mut device, _: *mut device, _: *mut core::ffi::c_void) {}
static ADP_DSI_COMPONENT_OPS: component_ops = component_ops { bind: Some(adp_dsi_bind), unbind: Some(adp_dsi_unbind) };

// Remaining bridge/driver registration declarations and callbacks preserve the C interfaces.
unsafe extern "C" fn adp_dsi_host_attach(_: *mut mipi_dsi_host, _: *mut mipi_dsi_device) -> i32 { 0 }
unsafe extern "C" fn adp_dsi_host_detach(_: *mut mipi_dsi_host, _: *mut mipi_dsi_device) -> i32 { 0 }
static ADP_DSI_HOST_OPS: mipi_dsi_host_ops = mipi_dsi_host_ops { transfer: Some(adp_dsi_host_transfer), attach: Some(adp_dsi_host_attach), detach: Some(adp_dsi_host_detach) };

unsafe extern "C" fn adp_dsi_bridge_attach(bridge: *mut drm_bridge, encoder: *mut drm_encoder, flags: u32) -> i32 {
    let adp = (bridge as *mut u8).sub(core::mem::offset_of!(adp_mipi_drv_private, bridge)) as *mut adp_mipi_drv_private;
    drm_bridge_attach(encoder, (*adp).next_bridge, bridge, flags)
}

static ADP_DSI_BRIDGE_FUNCS: drm_bridge_funcs = drm_bridge_funcs {
    atomic_create_state: core::ptr::null(), atomic_destroy_state: core::ptr::null(), atomic_duplicate_state: core::ptr::null(), attach: Some(adp_dsi_bridge_attach),
};

unsafe extern "C" fn adp_mipi_probe(pdev: *mut platform_device) -> i32 {
    let adp = devm_drm_bridge_alloc(&mut (*pdev).dev, core::mem::size_of::<adp_mipi_drv_private>(), &ADP_DSI_BRIDGE_FUNCS);
    if adp.is_null() { return -1; }
    (*adp).mipi = devm_platform_ioremap_resource(pdev, 0);
    if (*adp).mipi.is_null() { dev_err(&mut (*pdev).dev, b"failed to map mipi mmio\0".as_ptr()); return -1; }
    (*adp).dsi.dev = &mut (*pdev).dev;
    (*adp).dsi.ops = &ADP_DSI_HOST_OPS;
    (*adp).bridge.of_node = (*pdev).dev.of_node;
    (*adp).bridge.bridge_type = 16;
    dev_set_drvdata(&mut (*pdev).dev, adp);
    mipi_dsi_host_register(&mut (*adp).dsi)
}

unsafe extern "C" fn adp_mipi_remove(pdev: *mut platform_device) {
    let adp = dev_get_drvdata(&mut (*pdev).dev);
    mipi_dsi_host_unregister(&mut (*adp).dsi);
}

#[repr(C)] struct of_device_id { compatible: *const u8 }
static ADP_MIPI_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"apple,h7-display-pipe-mipi\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];
#[repr(C)] struct platform_driver { name: *const u8, of_match_table: *const of_device_id, probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, remove: Option<unsafe extern "C" fn(*mut platform_device)> }
static ADP_MIPI_PLATFORM_DRIVER: platform_driver = platform_driver { name: b"adp-mipi\0".as_ptr(), of_match_table: ADP_MIPI_OF_MATCH.as_ptr(), probe: Some(adp_mipi_probe), remove: Some(adp_mipi_remove) };

// MODULE_DEVICE_TABLE(of, adp_mipi_of_match);
// module_platform_driver(adp_mipi_platform_driver);
// MODULE_DESCRIPTION("Apple Display Pipe MIPI driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
