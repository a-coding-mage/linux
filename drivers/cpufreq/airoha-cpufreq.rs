// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel bindings are intentionally
// referenced here rather than reimplemented in this translation.

#[repr(C)]
struct AirohaCpufreqPriv {
    opp_token: i32,
    pd_list: *mut DevPmDomainList,
    cpufreq_dt: *mut PlatformDevice,
}

extern "C" {
    static mut cpufreq_pdev: *mut PlatformDevice;

    fn get_cpu_device(cpu: i32) -> *mut Device;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dev_pm_opp_set_config(cpu_dev: *mut Device, config: *const DevPmOppConfig) -> i32;
    fn dev_pm_domain_attach_list(
        cpu_dev: *mut Device,
        attach_data: *const DevPmDomainAttachData,
        pd_list: *mut *mut DevPmDomainList,
    ) -> i32;
    fn platform_device_register_simple(
        name: *const core::ffi::c_char,
        id: i32,
        data: *const core::ffi::c_void,
        size: usize,
    ) -> *mut PlatformDevice;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut core::ffi::c_void);
    fn platform_get_drvdata(pdev: *mut PlatformDevice) -> *mut core::ffi::c_void;
    fn platform_device_unregister(pdev: *mut PlatformDevice);
    fn dev_pm_domain_detach_list(pd_list: *mut DevPmDomainList);
    fn dev_pm_opp_clear_config(token: i32);
    fn platform_driver_register(driver: *mut PlatformDriver) -> i32;
    fn platform_driver_unregister(driver: *mut PlatformDriver);
    fn platform_device_register_data(
        parent: *mut Device,
        name: *const core::ffi::c_char,
        id: i32,
        data: *const core::ffi::c_void,
        size: usize,
    ) -> *mut PlatformDevice;
    fn of_machine_get_match(match_list: *const OfDeviceId) -> *const OfDeviceId;
}

#[repr(C)]
struct Device;
#[repr(C)]
struct DevPmDomainList;
#[repr(C)]
struct PlatformDevice { dev: Device }
#[repr(C)]
struct OppTable;
#[repr(C)]
struct DevPmOpp;

type ConfigClks = unsafe extern "C" fn(
    *mut Device, *mut OppTable, *mut DevPmOpp, *mut core::ffi::c_void, bool,
) -> i32;

#[repr(C)]
struct DevPmDomainAttachData {
    pd_names: *const *const core::ffi::c_char,
    num_pd_names: usize,
    pd_flags: u32,
}

#[repr(C)]
struct DevPmOppConfig {
    clk_names: *const *const core::ffi::c_char,
    config_clks: Option<ConfigClks>,
}

#[repr(C)]
struct Driver {
    name: *const core::ffi::c_char,
}

#[repr(C)]
struct PlatformDriver {
    probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut PlatformDevice)>,
    driver: Driver,
}

#[repr(C)]
struct OfDeviceId {
    compatible: *const core::ffi::c_char,
}

const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const PD_FLAG_DEV_LINK_ON: u32 = 1 << 0;
const PD_FLAG_REQUIRED_OPP: u32 = 1 << 1;

unsafe extern "C" fn airoha_cpufreq_config_clks_nop(
    _dev: *mut Device,
    _opp_table: *mut OppTable,
    _opp: *mut DevPmOpp,
    _data: *mut core::ffi::c_void,
    _scaling_down: bool,
) -> i32 {
    0
}

static AIROHA_CPUFREQ_CLK_NAMES: [*const core::ffi::c_char; 2] = [
    b"cpu\0".as_ptr() as *const _,
    core::ptr::null(),
];
static AIROHA_CPUFREQ_PD_NAMES: [*const core::ffi::c_char; 1] = [b"perf\0".as_ptr() as *const _];

unsafe extern "C" fn airoha_cpufreq_probe(pdev: *mut PlatformDevice) -> i32 {
    let attach_data = DevPmDomainAttachData {
        pd_names: AIROHA_CPUFREQ_PD_NAMES.as_ptr(),
        num_pd_names: AIROHA_CPUFREQ_PD_NAMES.len(),
        pd_flags: PD_FLAG_DEV_LINK_ON | PD_FLAG_REQUIRED_OPP,
    };
    let config = DevPmOppConfig {
        clk_names: AIROHA_CPUFREQ_CLK_NAMES.as_ptr(),
        config_clks: Some(airoha_cpufreq_config_clks_nop),
    };
    let dev = &mut (*pdev).dev as *mut Device;
    let cpu_dev = get_cpu_device(0);
    if cpu_dev.is_null() { return -ENODEV; }

    let priv_ptr = devm_kzalloc(dev, core::mem::size_of::<AirohaCpufreqPriv>(), 0)
        as *mut AirohaCpufreqPriv;
    if priv_ptr.is_null() { return -ENOMEM; }

    (*priv_ptr).opp_token = dev_pm_opp_set_config(cpu_dev, &config);
    if (*priv_ptr).opp_token < 0 { return (*priv_ptr).opp_token; }

    let ret = dev_pm_domain_attach_list(cpu_dev, &attach_data, &mut (*priv_ptr).pd_list);
    if ret != 0 {
        dev_pm_opp_clear_config((*priv_ptr).opp_token);
        return ret;
    }

    let name = b"cpufreq-dt\0".as_ptr() as *const core::ffi::c_char;
    let cpufreq_dt = platform_device_register_simple(name, -1, core::ptr::null(), 0);
    if (cpufreq_dt as isize) < 0 {
        dev_pm_domain_detach_list((*priv_ptr).pd_list);
        dev_pm_opp_clear_config((*priv_ptr).opp_token);
        return cpufreq_dt as isize as i32;
    }
    (*priv_ptr).cpufreq_dt = cpufreq_dt;
    platform_set_drvdata(pdev, priv_ptr as *mut core::ffi::c_void);
    0
}

unsafe extern "C" fn airoha_cpufreq_remove(pdev: *mut PlatformDevice) {
    let priv_ptr = platform_get_drvdata(pdev) as *mut AirohaCpufreqPriv;
    platform_device_unregister((*priv_ptr).cpufreq_dt);
    dev_pm_domain_detach_list((*priv_ptr).pd_list);
    dev_pm_opp_clear_config((*priv_ptr).opp_token);
}

static mut AIROHA_CPUFREQ_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(airoha_cpufreq_probe),
    remove: Some(airoha_cpufreq_remove),
    driver: Driver { name: b"airoha-cpufreq\0".as_ptr() as *const _ },
};

static AIROHA_CPUFREQ_MATCH_LIST: [OfDeviceId; 3] = [
    OfDeviceId { compatible: b"airoha,an7583\0".as_ptr() as *const _ },
    OfDeviceId { compatible: b"airoha,en7581\0".as_ptr() as *const _ },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe extern "C" fn airoha_cpufreq_init() -> i32 {
    let match_ptr = of_machine_get_match(AIROHA_CPUFREQ_MATCH_LIST.as_ptr());
    if match_ptr.is_null() { return -ENODEV; }
    let mut ret = platform_driver_register(&mut AIROHA_CPUFREQ_DRIVER);
    if ret < 0 { return ret; }
    cpufreq_pdev = platform_device_register_data(
        core::ptr::null_mut(), b"airoha-cpufreq\0".as_ptr() as *const _, -1,
        match_ptr as *const core::ffi::c_void, core::mem::size_of::<OfDeviceId>(),
    );
    if (cpufreq_pdev as isize) < 0 {
        ret = cpufreq_pdev as isize as i32;
        platform_driver_unregister(&mut AIROHA_CPUFREQ_DRIVER);
    } else { ret = 0; }
    ret
}

unsafe extern "C" fn airoha_cpufreq_exit() {
    platform_device_unregister(cpufreq_pdev);
    platform_driver_unregister(&mut AIROHA_CPUFREQ_DRIVER);
}

// module_init(airoha_cpufreq_init);
// module_exit(airoha_cpufreq_exit);
// MODULE_DEVICE_TABLE(of, airoha_cpufreq_match_list);
// MODULE_AUTHOR("Christian Marangi <ansuelsmth@gmail.com>");
// MODULE_DESCRIPTION("CPUfreq driver for Airoha SoCs");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
