// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Calxeda, Inc.
 *
 * This driver provides the clk notifier callbacks that are used when
 * the cpufreq-dt driver changes to frequency to alert the highbank
 * EnergyCore Management Engine (ECME) about the need to change
 * voltage. The ECME interfaces with the actual voltage regulators.
 */

// C preprocessor: pr_fmt(fmt) KBUILD_MODNAME ": " fmt

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

const HB_CPUFREQ_CHANGE_NOTE: u32 = 0x8000_0001;
const HB_CPUFREQ_IPC_LEN: usize = 7;
const HB_CPUFREQ_VOLT_RETRIES: i32 = 15;

const PRE_RATE_CHANGE: c_ulong = 0;
const POST_RATE_CHANGE: c_ulong = 1;
const NOTIFY_BAD: c_int = 0x0002;
const NOTIFY_DONE: c_int = 0x0000;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;

type c_int = i32;
type c_ulong = usize;

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct clk_notifier_data {
    pub clk: *mut clk,
    pub old_rate: u64,
    pub new_rate: u64,
}

#[repr(C)]
pub struct platform_device_info {
    pub name: *const u8,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct clk;

#[repr(C)]
pub struct device_node;

extern "C" {
    fn pl320_ipc_transmit(msg: *mut u32) -> c_int;
    fn of_machine_is_compatible(compat: *const u8) -> bool;
    fn get_cpu_device(cpu: c_int) -> *mut device;
    fn of_node_get(node: *mut device_node) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn clk_get(dev: *mut device, id: *const u8) -> *mut clk;
    fn clk_notifier_register(clk: *mut clk, nb: *mut notifier_block) -> c_int;
    fn platform_device_register_full(info: *const platform_device_info) -> *mut c_void;
    fn pr_err(fmt: *const u8, ...);
    fn is_err(ptr: *const c_void) -> bool;
    fn ptr_err(ptr: *const c_void) -> c_int;
}

unsafe fn hb_voltage_change(freq: u64) -> c_int {
    let mut msg: [u32; HB_CPUFREQ_IPC_LEN] = [0; HB_CPUFREQ_IPC_LEN];
    msg[0] = HB_CPUFREQ_CHANGE_NOTE;
    msg[1] = (freq / 1_000_000) as u32;

    pl320_ipc_transmit(msg.as_mut_ptr())
}

unsafe extern "C" fn hb_cpufreq_clk_notify(
    _nb: *mut notifier_block,
    action: c_ulong,
    hclk: *mut c_void,
) -> c_int {
    let clk_data = hclk as *mut clk_notifier_data;
    let mut i: i32 = 0;

    if action == PRE_RATE_CHANGE {
        if (*clk_data).new_rate > (*clk_data).old_rate {
            while hb_voltage_change((*clk_data).new_rate) != 0 {
                i += 1;
                if i - 1 > HB_CPUFREQ_VOLT_RETRIES {
                    return NOTIFY_BAD;
                }
            }
        }
    } else if action == POST_RATE_CHANGE {
        if (*clk_data).new_rate < (*clk_data).old_rate {
            while hb_voltage_change((*clk_data).new_rate) != 0 {
                i += 1;
                if i - 1 > HB_CPUFREQ_VOLT_RETRIES {
                    return NOTIFY_BAD;
                }
            }
        }
    }

    NOTIFY_DONE
}

static mut hb_cpufreq_clk_nb: notifier_block = notifier_block {
    notifier_call: Some(hb_cpufreq_clk_notify),
};

unsafe extern "C" fn hb_cpufreq_driver_init() -> c_int {
    let devinfo = platform_device_info {
        name: b"cpufreq-dt\0".as_ptr(),
    };
    let cpu_dev: *mut device;
    let cpu_clk: *mut clk;
    let np: *mut device_node;
    let ret: c_int;

    if !of_machine_is_compatible(b"calxeda,highbank\0".as_ptr())
        && !of_machine_is_compatible(b"calxeda,ecx-2000\0".as_ptr())
    {
        return -ENODEV;
    }

    cpu_dev = get_cpu_device(0);
    if cpu_dev.is_null() {
        pr_err(b"failed to get highbank cpufreq device\n\0".as_ptr());
        return -ENODEV;
    }

    np = of_node_get((*cpu_dev).of_node);
    if np.is_null() {
        pr_err(b"failed to find highbank cpufreq node\n\0".as_ptr());
        return -ENOENT;
    }

    cpu_clk = clk_get(cpu_dev, core::ptr::null());
    if is_err(cpu_clk as *const c_void) {
        ret = ptr_err(cpu_clk as *const c_void);
        pr_err(b"failed to get cpu0 clock: %d\n\0".as_ptr(), ret);
        of_node_put(np);
        return ret;
    }

    ret = clk_notifier_register(cpu_clk, &raw mut hb_cpufreq_clk_nb);
    if ret != 0 {
        pr_err(b"failed to register clk notifier: %d\n\0".as_ptr(), ret);
        of_node_put(np);
        return ret;
    }

    // Instantiate cpufreq-dt.
    platform_device_register_full(&devinfo);

    of_node_put(np);
    ret
}

// module_init(hb_cpufreq_driver_init);

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
}

#[allow(dead_code)]
static hb_cpufreq_of_match: [of_device_id; 3] = [
    of_device_id { compatible: b"calxeda,highbank\0".as_ptr() },
    of_device_id { compatible: b"calxeda,ecx-2000\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

// MODULE_DEVICE_TABLE(of, hb_cpufreq_of_match);
// MODULE_AUTHOR("Mark Langsdorf <mark.langsdorf@calxeda.com>");
// MODULE_DESCRIPTION("Calxeda Highbank cpufreq driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
