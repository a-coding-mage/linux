// SPDX-License-Identifier: GPL-2.0+
/* Raspberry Pi driver for firmware controlled clocks. */

// C dependencies supplied by the surrounding kernel translation.

const RPI_FIRMWARE_STATE_ENABLE_BIT: u32 = BIT(0);
const RPI_FIRMWARE_STATE_WAIT_BIT: u32 = BIT(1);

#[repr(C)]
pub struct raspberrypi_clk {
    dev: *mut device,
    firmware: *mut rpi_firmware,
    cpufreq: *mut platform_device,
}

#[repr(C)]
pub struct raspberrypi_clk_data {
    hw: clk_hw,
    id: u32,
    variant: *mut raspberrypi_clk_variant,
    rpi: *mut raspberrypi_clk,
}

#[inline]
unsafe fn clk_hw_to_data(hw: *const clk_hw) -> *const raspberrypi_clk_data {
    container_of(hw, raspberrypi_clk_data, hw)
}

#[repr(C)]
pub struct raspberrypi_clk_variant {
    export: bool,
    clkdev: *mut i8,
    min_rate: c_ulong,
    minimize: bool,
    maximize: bool,
    flags: u32,
}

static mut RPI_FIRMWARE_CLK_NAMES: [*mut i8; RPI_FIRMWARE_NUM_CLK_ID as usize] = [
    /* Indexed entries are supplied by the firmware clock identifiers. */
];

static mut RASPBERRYPI_CLK_VARIANTS: [raspberrypi_clk_variant; RPI_FIRMWARE_NUM_CLK_ID as usize] = [
    raspberrypi_clk_variant { export: false, clkdev: core::ptr::null_mut(), min_rate: 0, minimize: false, maximize: false, flags: 0 };
    RPI_FIRMWARE_NUM_CLK_ID as usize
];

#[repr(C, packed)]
struct raspberrypi_firmware_prop {
    id: __le32,
    val: __le32,
    disable_turbo: __le32,
}

unsafe fn raspberrypi_clock_property(
    firmware: *mut rpi_firmware,
    data: *const raspberrypi_clk_data,
    tag: u32,
    val: *mut u32,
) -> i32 {
    let mut msg = raspberrypi_firmware_prop {
        id: cpu_to_le32((*data).id),
        val: cpu_to_le32(*val),
        disable_turbo: 0,
    };
    let ret = rpi_firmware_property(firmware, tag, &mut msg as *mut _ as *mut c_void, core::mem::size_of::<raspberrypi_firmware_prop>());
    if ret != 0 { return ret; }
    *val = le32_to_cpu(msg.val);
    0
}

unsafe extern "C" fn raspberrypi_fw_is_prepared(hw: *mut clk_hw) -> i32 {
    let data = clk_hw_to_data(hw);
    let rpi = (*data).rpi;
    let mut val = 0u32;
    let ret = raspberrypi_clock_property((*rpi).firmware, data, RPI_FIRMWARE_GET_CLOCK_STATE, &mut val);
    if ret != 0 {
        dev_err_ratelimited((*rpi).dev, "Failed to get %s state: %d\n", clk_hw_get_name(hw), ret);
        return 0;
    }
    if val & RPI_FIRMWARE_STATE_ENABLE_BIT != 0 { 1 } else { 0 }
}

unsafe extern "C" fn raspberrypi_fw_get_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    let data = clk_hw_to_data(hw);
    let rpi = (*data).rpi;
    let mut val = 0u32;
    let ret = raspberrypi_clock_property((*rpi).firmware, data, RPI_FIRMWARE_GET_CLOCK_RATE, &mut val);
    if ret != 0 {
        dev_err_ratelimited((*rpi).dev, "Failed to get %s frequency: %d\n", clk_hw_get_name(hw), ret);
        return 0;
    }
    val as c_ulong
}

unsafe extern "C" fn raspberrypi_fw_set_rate(hw: *mut clk_hw, rate: c_ulong, _parent_rate: c_ulong) -> i32 {
    let data = clk_hw_to_data(hw);
    let rpi = (*data).rpi;
    let mut value = rate as u32;
    let ret = raspberrypi_clock_property((*rpi).firmware, data, RPI_FIRMWARE_SET_CLOCK_RATE, &mut value);
    if ret != 0 { dev_err_ratelimited((*rpi).dev, "Failed to change %s frequency: %d\n", clk_hw_get_name(hw), ret); }
    ret
}

unsafe extern "C" fn raspberrypi_fw_dumb_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let data = clk_hw_to_data(hw);
    let variant = (*data).variant;
    (*req).rate = clamp((*req).rate, (*req).min_rate, (*req).max_rate);
    if (*variant).minimize && (*req).min_rate > 0 { (*req).rate = (*req).min_rate; }
    0
}

unsafe extern "C" fn raspberrypi_fw_prepare(hw: *mut clk_hw) -> i32 {
    let data = clk_hw_to_data(hw);
    let variant = (*data).variant;
    let rpi = (*data).rpi;
    let mut state = RPI_FIRMWARE_STATE_ENABLE_BIT;
    let mut ret = raspberrypi_clock_property((*rpi).firmware, data, RPI_FIRMWARE_SET_CLOCK_STATE, &mut state);
    if ret != 0 {
        dev_err_ratelimited((*rpi).dev, "Failed to set clock %s state to on: %d\n", clk_hw_get_name(hw), ret);
        return ret;
    }
    if (*variant).maximize {
        let (mut min_rate, mut max_rate) = (0, 0);
        clk_hw_get_rate_range(hw, &mut min_rate, &mut max_rate);
        ret = raspberrypi_fw_set_rate(hw, max_rate, 0);
    }
    ret
}

unsafe extern "C" fn raspberrypi_fw_unprepare(hw: *mut clk_hw) {
    let data = clk_hw_to_data(hw);
    let rpi = (*data).rpi;
    let (mut min_rate, mut max_rate) = (0, 0);
    clk_hw_get_rate_range(hw, &mut min_rate, &mut max_rate);
    raspberrypi_fw_set_rate(hw, min_rate, 0);
    let mut state = 0u32;
    let ret = raspberrypi_clock_property((*rpi).firmware, data, RPI_FIRMWARE_SET_CLOCK_STATE, &mut state);
    if ret != 0 { dev_err_ratelimited((*rpi).dev, "Failed to set clock %s state to off: %d\n", clk_hw_get_name(hw), ret); }
}

static RASPBERRYPI_FIRMWARE_CLK_OPS: clk_ops = clk_ops {
    prepare: Some(raspberrypi_fw_prepare),
    unprepare: Some(raspberrypi_fw_unprepare),
    is_prepared: Some(raspberrypi_fw_is_prepared),
    recalc_rate: Some(raspberrypi_fw_get_rate),
    determine_rate: Some(raspberrypi_fw_dumb_determine_rate),
    set_rate: Some(raspberrypi_fw_set_rate),
};

unsafe fn raspberrypi_clk_register(rpi: *mut raspberrypi_clk, _parent: u32, id: u32, variant: *mut raspberrypi_clk_variant) -> *mut clk_hw {
    let data = devm_kzalloc((*rpi).dev, core::mem::size_of::<raspberrypi_clk_data>(), GFP_KERNEL) as *mut raspberrypi_clk_data;
    if data.is_null() { return ERR_PTR(-ENOMEM); }
    (*data).rpi = rpi; (*data).id = id; (*data).variant = variant;
    let mut init: clk_init_data = core::mem::zeroed();
    init.name = devm_kasprintf((*rpi).dev, GFP_KERNEL, "fw-clk-%s", RPI_FIRMWARE_CLK_NAMES[id as usize]);
    if init.name.is_null() { return ERR_PTR(-ENOMEM); }
    init.ops = &RASPBERRYPI_FIRMWARE_CLK_OPS;
    init.flags = (*variant).flags | CLK_GET_RATE_NOCACHE;
    (*data).hw.init = &init;
    let mut min_rate = 0u32; let mut max_rate = 0u32;
    let mut ret = raspberrypi_clock_property((*rpi).firmware, data, RPI_FIRMWARE_GET_MIN_CLOCK_RATE, &mut min_rate);
    if ret != 0 { dev_err((*rpi).dev, "Failed to get clock %d min freq: %d\n", id, ret); return ERR_PTR(ret); }
    ret = raspberrypi_clock_property((*rpi).firmware, data, RPI_FIRMWARE_GET_MAX_CLOCK_RATE, &mut max_rate);
    if ret != 0 { dev_err((*rpi).dev, "Failed to get clock %d max freq: %d\n", id, ret); return ERR_PTR(ret); }
    ret = devm_clk_hw_register((*rpi).dev, &mut (*data).hw);
    if ret != 0 { return ERR_PTR(ret); }
    clk_hw_set_rate_range(&mut (*data).hw, min_rate as c_ulong, max_rate as c_ulong);
    if !(*variant).clkdev.is_null() {
        ret = devm_clk_hw_register_clkdev((*rpi).dev, &mut (*data).hw, core::ptr::null(), (*variant).clkdev);
        if ret != 0 { dev_err((*rpi).dev, "Failed to initialize clkdev\n"); return ERR_PTR(ret); }
    }
    if (*variant).min_rate != 0 {
        clk_hw_set_rate_range(&mut (*data).hw, (*variant).min_rate, max_rate as c_ulong);
        if raspberrypi_fw_get_rate(&mut (*data).hw, 0) < (*variant).min_rate {
            ret = raspberrypi_fw_set_rate(&mut (*data).hw, (*variant).min_rate, 0);
            if ret != 0 { return ERR_PTR(ret); }
        }
    }
    &mut (*data).hw
}

#[repr(C)] struct rpi_firmware_get_clocks_response { parent: u32, id: u32 }

unsafe fn raspberrypi_discover_clocks(rpi: *mut raspberrypi_clk, data: *mut clk_hw_onecell_data) -> i32 {
    let count = RPI_FIRMWARE_NUM_CLK_ID as usize + 1;
    let clks = devm_kcalloc((*rpi).dev, count, core::mem::size_of::<rpi_firmware_get_clocks_response>(), GFP_KERNEL) as *mut rpi_firmware_get_clocks_response;
    if clks.is_null() { return -ENOMEM; }
    let ret = rpi_firmware_property((*rpi).firmware, RPI_FIRMWARE_GET_CLOCKS, clks as *mut c_void, core::mem::size_of::<rpi_firmware_get_clocks_response>() * RPI_FIRMWARE_NUM_CLK_ID as usize);
    if ret != 0 { return ret; }
    let mut cur = clks;
    while (*cur).id != 0 {
        if (*cur).id >= RPI_FIRMWARE_NUM_CLK_ID { dev_err((*rpi).dev, "Unknown clock id: %u (max: %u)\n", (*cur).id, RPI_FIRMWARE_NUM_CLK_ID - 1); return -EINVAL; }
        let variant = &mut RASPBERRYPI_CLK_VARIANTS[(*cur).id as usize];
        if variant.export {
            let hw = raspberrypi_clk_register(rpi, (*cur).parent, (*cur).id, variant);
            if IS_ERR(hw) { return PTR_ERR(hw); }
            (*data).num = (*cur).id + 1;
            (*data).hws[(*cur).id as usize] = hw;
        }
        cur = cur.add(1);
    }
    0
}

unsafe extern "C" fn raspberrypi_clk_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let firmware_node = if !dev.of_node.is_null() { of_get_parent(dev.of_node) } else { of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "raspberrypi,bcm2835-firmware") };
    if firmware_node.is_null() { dev_err(dev, "Missing firmware node\n"); return -ENOENT; }
    let firmware = devm_rpi_firmware_get(pdev, firmware_node); of_node_put(firmware_node);
    if firmware.is_null() { return -EPROBE_DEFER; }
    let rpi = devm_kzalloc(dev, core::mem::size_of::<raspberrypi_clk>(), GFP_KERNEL) as *mut raspberrypi_clk;
    if rpi.is_null() { return -ENOMEM; }
    (*rpi).dev = dev; (*rpi).firmware = firmware; platform_set_drvdata(pdev, rpi as *mut c_void);
    let data = devm_kzalloc(dev, struct_size_clk_hw_onecell_data(RPI_FIRMWARE_NUM_CLK_ID as usize), GFP_KERNEL) as *mut clk_hw_onecell_data;
    if data.is_null() { return -ENOMEM; }
    let ret = raspberrypi_discover_clocks(rpi, data); if ret != 0 { return ret; }
    let ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get, data); if ret != 0 { return ret; }
    (*rpi).cpufreq = platform_device_register_data(dev, "raspberrypi-cpufreq", -1, core::ptr::null(), 0); 0
}

unsafe extern "C" fn raspberrypi_clk_remove(pdev: *mut platform_device) {
    let rpi = platform_get_drvdata(pdev) as *mut raspberrypi_clk;
    platform_device_unregister((*rpi).cpufreq);
}

static mut RASPBERRYPI_CLK_DRIVER: platform_driver = platform_driver { driver: driver { name: "raspberrypi-clk", of_match_table: core::ptr::null() }, probe: Some(raspberrypi_clk_probe), remove: Some(raspberrypi_clk_remove) };
module_platform_driver!(RASPBERRYPI_CLK_DRIVER);
MODULE_AUTHOR!("Nicolas Saenz Julienne <nsaenzjulienne@suse.de>");
MODULE_DESCRIPTION!("Raspberry Pi firmware clock driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
