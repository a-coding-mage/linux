// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2025 NXP
 */

// Dependencies supplied by the surrounding kernel bindings.

const SYSCTRL0: usize = 0x8;

#[repr(C)]
pub struct clk_imx8ulp_sim_lpav_data {
    pub lock: spinlock_t, // shared by MUX, clock gate and reset
    pub flags: c_ulong, // for spinlock usage
    pub clk_data: clk_hw_onecell_data, // keep last
}

#[repr(C)]
pub struct clk_imx8ulp_sim_lpav_gate {
    pub name: *const c_char,
    pub id: c_int,
    pub parent: clk_parent_data,
    pub bit: u8,
}

#[repr(C)]
struct clk_parent_data {
    pub fw_name: *const c_char,
}

#[repr(C)]
struct clk_hw_onecell_data {
    pub num: c_uint,
    pub hws: *mut *mut clk_hw,
}

#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub reg_stride: c_uint,
    pub lock: Option<unsafe extern "C" fn(*mut c_void)>,
    pub unlock: Option<unsafe extern "C" fn(*mut c_void)>,
    pub lock_arg: *mut c_void,
}

#[repr(C)]
struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct auxiliary_device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    _private: [u8; 0],
}

static mut gates: [clk_imx8ulp_sim_lpav_gate; 3] = [
    clk_imx8ulp_sim_lpav_gate { name: b"hifi_core_cg\0".as_ptr() as *const c_char, id: IMX8ULP_CLK_SIM_LPAV_HIFI_CORE, parent: clk_parent_data { fw_name: b"core\0".as_ptr() as *const c_char }, bit: 17 },
    clk_imx8ulp_sim_lpav_gate { name: b"hifi_pbclk_cg\0".as_ptr() as *const c_char, id: IMX8ULP_CLK_SIM_LPAV_HIFI_PBCLK, parent: clk_parent_data { fw_name: b"bus\0".as_ptr() as *const c_char }, bit: 18 },
    clk_imx8ulp_sim_lpav_gate { name: b"hifi_plat_cg\0".as_ptr() as *const c_char, id: IMX8ULP_CLK_SIM_LPAV_HIFI_PLAT, parent: clk_parent_data { fw_name: b"plat\0".as_ptr() as *const c_char }, bit: 19 },
];

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut c_void) -> *mut clk_imx8ulp_sim_lpav_data;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn dev_set_drvdata(dev: *mut c_void, data: *mut clk_imx8ulp_sim_lpav_data);
    fn devm_kzalloc(dev: *mut c_void, size: usize, flags: c_uint) -> *mut clk_imx8ulp_sim_lpav_data;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut c_void, base: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_clk_hw_register_gate_parent_data(dev: *mut c_void, name: *const c_char, parent: *const clk_parent_data, flags: c_ulong, reg: *mut c_void, bit_idx: u8, mask: u8, lock: *mut spinlock_t) -> *mut clk_hw;
    fn devm_auxiliary_device_create(dev: *mut c_void, name: *const c_char, id: *const c_void) -> *mut auxiliary_device;
    fn devm_of_clk_add_hw_provider(dev: *mut c_void, get: *const c_void, data: *mut clk_hw_onecell_data) -> c_int;
    fn devm_of_platform_populate(dev: *mut c_void) -> c_int;
}

unsafe extern "C" fn clk_imx8ulp_sim_lpav_lock(arg: *mut c_void) {
    let data = dev_get_drvdata(arg);
    spin_lock_irqsave(&mut (*data).lock, &mut (*data).flags);
}

unsafe extern "C" fn clk_imx8ulp_sim_lpav_unlock(arg: *mut c_void) {
    let data = dev_get_drvdata(arg);
    spin_unlock_irqrestore(&mut (*data).lock, (*data).flags);
}

unsafe extern "C" fn clk_imx8ulp_sim_lpav_probe(pdev: *mut platform_device) -> c_int {
    let regmap_config = regmap_config {
        reg_bits: 32,
        val_bits: 32,
        reg_stride: 4,
        lock: Some(clk_imx8ulp_sim_lpav_lock),
        unlock: Some(clk_imx8ulp_sim_lpav_unlock),
        lock_arg: pdev as *mut c_void,
    };
    let data = devm_kzalloc(pdev as *mut c_void, core::mem::size_of::<clk_imx8ulp_sim_lpav_data>(), 0);
    if data.is_null() { return -12; }
    dev_set_drvdata(pdev as *mut c_void, data);
    spin_lock_init(&mut (*data).lock);
    let base = devm_platform_ioremap_resource(pdev, 0);
    if base.is_null() { return -1; }
    let regmap = devm_regmap_init_mmio(pdev as *mut c_void, base, &regmap_config);
    if regmap.is_null() { return -1; }
    (*data).clk_data.num = 3;
    for i in 0..3 {
        let gate = &gates[i];
        let hw = devm_clk_hw_register_gate_parent_data(pdev as *mut c_void, gate.name, &gate.parent, 1, base.add(SYSCTRL0), gate.bit, 0, &mut (*data).lock);
        if hw.is_null() { return -1; }
        (*data).clk_data.hws.add(i).write(hw);
    }
    if devm_auxiliary_device_create(pdev as *mut c_void, b"reset\0".as_ptr() as *const c_char, core::ptr::null()).is_null() { return -19; }
    let ret = devm_of_clk_add_hw_provider(pdev as *mut c_void, core::ptr::null(), &mut (*data).clk_data);
    if ret != 0 { return ret; }
    devm_of_platform_populate(pdev as *mut c_void)
}

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_ulong = usize;
type c_void = core::ffi::c_void;

const IMX8ULP_CLK_SIM_LPAV_HIFI_CORE: c_int = 0;
const IMX8ULP_CLK_SIM_LPAV_HIFI_PBCLK: c_int = 1;
const IMX8ULP_CLK_SIM_LPAV_HIFI_PLAT: c_int = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
