// SPDX-License-Identifier: GPL-2.0
/*
 * Xilinx Zynq MPSoC Firmware layer for debugfs APIs
 *
 * Direct Rust translation of zynqmp-debug.c. Linux/kernel dependencies and
 * PM-API symbols are supplied by the surrounding kernel translation unit.
 */

const PM_API_NAME_LEN: usize = 50;

#[repr(C)]
struct PmApiInfo {
    api_id: u32,
    api_name: [u8; PM_API_NAME_LEN],
    api_name_len: i8,
}

static mut DEBUGFS_BUF: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

// PM_API(id) { id, #id, strlen(#id) }
static mut PM_API_LIST: [PmApiInfo; 24] = [
    pm_api_info(PM_FORCE_POWERDOWN, "PM_FORCE_POWERDOWN"),
    pm_api_info(PM_REQUEST_WAKEUP, "PM_REQUEST_WAKEUP"),
    pm_api_info(PM_SYSTEM_SHUTDOWN, "PM_SYSTEM_SHUTDOWN"),
    pm_api_info(PM_REQUEST_NODE, "PM_REQUEST_NODE"),
    pm_api_info(PM_RELEASE_NODE, "PM_RELEASE_NODE"),
    pm_api_info(PM_SET_REQUIREMENT, "PM_SET_REQUIREMENT"),
    pm_api_info(PM_GET_API_VERSION, "PM_GET_API_VERSION"),
    pm_api_info(PM_GET_NODE_STATUS, "PM_GET_NODE_STATUS"),
    pm_api_info(PM_REGISTER_NOTIFIER, "PM_REGISTER_NOTIFIER"),
    pm_api_info(PM_RESET_ASSERT, "PM_RESET_ASSERT"),
    pm_api_info(PM_RESET_GET_STATUS, "PM_RESET_GET_STATUS"),
    pm_api_info(PM_GET_CHIPID, "PM_GET_CHIPID"),
    pm_api_info(PM_PINCTRL_SET_FUNCTION, "PM_PINCTRL_SET_FUNCTION"),
    pm_api_info(PM_PINCTRL_CONFIG_PARAM_GET, "PM_PINCTRL_CONFIG_PARAM_GET"),
    pm_api_info(PM_PINCTRL_CONFIG_PARAM_SET, "PM_PINCTRL_CONFIG_PARAM_SET"),
    pm_api_info(PM_IOCTL, "PM_IOCTL"),
    pm_api_info(PM_CLOCK_ENABLE, "PM_CLOCK_ENABLE"),
    pm_api_info(PM_CLOCK_DISABLE, "PM_CLOCK_DISABLE"),
    pm_api_info(PM_CLOCK_GETSTATE, "PM_CLOCK_GETSTATE"),
    pm_api_info(PM_CLOCK_SETDIVIDER, "PM_CLOCK_SETDIVIDER"),
    pm_api_info(PM_CLOCK_GETDIVIDER, "PM_CLOCK_GETDIVIDER"),
    pm_api_info(PM_CLOCK_SETPARENT, "PM_CLOCK_SETPARENT"),
    pm_api_info(PM_CLOCK_GETPARENT, "PM_CLOCK_GETPARENT"),
    pm_api_info(PM_QUERY_DATA, "PM_QUERY_DATA"),
];

const fn pm_api_info(id: u32, name: &str) -> PmApiInfo {
    let mut bytes = [0u8; PM_API_NAME_LEN];
    let src = name.as_bytes();
    let mut i = 0;
    while i < src.len() { bytes[i] = src[i]; i += 1; }
    PmApiInfo { api_id: id, api_name: bytes, api_name_len: src.len() as i8 }
}

static mut firmware_debugfs_root: *mut dentry = core::ptr::null_mut();

unsafe fn zynqmp_pm_ioctl(node: u32, ioctl: u32, arg1: u32, arg2: u32,
                          arg3: u32, out: *mut u32) -> i32 {
    zynqmp_pm_invoke_fn(PM_IOCTL, out, 5, node as u64, ioctl as u64,
                        arg1 as u64, arg2 as u64, arg3 as u64)
}

unsafe fn zynqmp_pm_argument_value(arg: *mut u8) -> u64 {
    if arg.is_null() { return 0; }
    let mut value = 0u64;
    if kstrtou64(arg, 0, &mut value) == 0 { value } else { 0 }
}

unsafe fn get_pm_api_id(pm_api_req: *mut u8, pm_id: *mut u32) -> i32 {
    let mut i = 0usize;
    while i < PM_API_LIST.len() {
        if strncasecmp(pm_api_req, PM_API_LIST[i].api_name.as_ptr(),
                       PM_API_LIST[i].api_name_len as usize) == 0 {
            *pm_id = PM_API_LIST[i].api_id;
            break;
        }
        i += 1;
    }
    if i == PM_API_LIST.len() && kstrtouint(pm_api_req, 10, pm_id) != 0 { return -EINVAL; }
    0
}

unsafe fn process_api_request(pm_id: u32, a: *mut u64, r: *mut u32) -> i32 {
    let mut version = 0u32;
    let mut qdata: zynqmp_pm_query_data = core::mem::zeroed();
    let ret = match pm_id {
        PM_GET_API_VERSION => {
            let x = zynqmp_pm_get_api_version(&mut version);
            sprintf(DEBUGFS_BUF.as_mut_ptr(), b"PM-API Version = %d.%d\0".as_ptr(), version >> 16, version & 0xffff);
            x
        }
        PM_FORCE_POWERDOWN => zynqmp_pm_force_pwrdwn(*a, if *a.add(1) != 0 { *a.add(1) } else { ZYNQMP_PM_REQUEST_ACK_NO }),
        PM_REQUEST_WAKEUP => zynqmp_pm_request_wake(*a, *a.add(1), *a.add(2), if *a.add(3) != 0 { *a.add(3) } else { ZYNQMP_PM_REQUEST_ACK_NO }),
        PM_SYSTEM_SHUTDOWN => zynqmp_pm_system_shutdown(*a, *a.add(1)),
        PM_REQUEST_NODE => zynqmp_pm_request_node(*a, if *a.add(1) != 0 { *a.add(1) } else { ZYNQMP_PM_CAPABILITY_ACCESS }, if *a.add(2) != 0 { *a.add(2) } else { 0 }, if *a.add(3) != 0 { *a.add(3) } else { ZYNQMP_PM_REQUEST_ACK_BLOCKING }),
        PM_RELEASE_NODE => zynqmp_pm_release_node(*a),
        PM_SET_REQUIREMENT => zynqmp_pm_set_requirement(*a, if *a.add(1) != 0 { *a.add(1) } else { ZYNQMP_PM_CAPABILITY_CONTEXT }, if *a.add(2) != 0 { *a.add(2) } else { 0 }, if *a.add(3) != 0 { *a.add(3) } else { ZYNQMP_PM_REQUEST_ACK_BLOCKING }),
        PM_GET_NODE_STATUS => zynqmp_pm_get_node_status(*a, r, r.add(1), r.add(2)),
        PM_REGISTER_NOTIFIER => zynqmp_pm_register_notifier(*a, *a.add(1), *a.add(2), *a.add(3)),
        PM_RESET_ASSERT => zynqmp_pm_reset_assert(*a, *a.add(1)),
        PM_RESET_GET_STATUS => zynqmp_pm_reset_get_status(*a, r),
        PM_GET_CHIPID => zynqmp_pm_get_chipid(r, r.add(1)),
        PM_PINCTRL_SET_FUNCTION => zynqmp_pm_pinctrl_set_function(*a, *a.add(1)),
        PM_PINCTRL_CONFIG_PARAM_GET => zynqmp_pm_pinctrl_get_config(*a, *a.add(1), r),
        PM_PINCTRL_CONFIG_PARAM_SET => zynqmp_pm_pinctrl_set_config(*a, *a.add(1), *a.add(2)),
        PM_IOCTL => zynqmp_pm_ioctl(*a as u32, *a.add(1) as u32, *a.add(2) as u32, *a.add(3) as u32, *a.add(4) as u32, r),
        PM_CLOCK_ENABLE => zynqmp_pm_clock_enable(*a),
        PM_CLOCK_DISABLE => zynqmp_pm_clock_disable(*a),
        PM_CLOCK_GETSTATE => zynqmp_pm_clock_getstate(*a, r),
        PM_CLOCK_SETDIVIDER => zynqmp_pm_clock_setdivider(*a, *a.add(1)),
        PM_CLOCK_GETDIVIDER => zynqmp_pm_clock_getdivider(*a, r),
        PM_CLOCK_SETPARENT => zynqmp_pm_clock_setparent(*a, *a.add(1)),
        PM_CLOCK_GETPARENT => zynqmp_pm_clock_getparent(*a, r),
        PM_QUERY_DATA => { qdata.qid=*a; qdata.arg1=*a.add(1); qdata.arg2=*a.add(2); qdata.arg3=*a.add(3); zynqmp_pm_query_data(qdata, r) }
        _ => { sprintf(DEBUGFS_BUF.as_mut_ptr(), b"Unsupported PM-API request\0".as_ptr()); -EINVAL }
    };
    ret
}

unsafe fn zynqmp_pm_debugfs_api_write(_file: *mut file, ptr: *const u8, len: usize, off: *mut loff_t) -> isize {
    strcpy(DEBUGFS_BUF.as_mut_ptr(), b"\0".as_ptr());
    if *off != 0 || len <= 1 || len > PAGE_SIZE - 1 { return -EINVAL as isize; }
    let mut kern_buff = memdup_user_nul(ptr, len);
    if IS_ERR(kern_buff) { return PTR_ERR(kern_buff) as isize; }
    let tmp_buff = kern_buff;
    let mut pm_id = 0u32;
    let mut args = [0u64; 5];
    let mut rets = [0u32; 4];
    let mut req = strsep(&mut kern_buff, b" \0".as_ptr());
    let mut ret = get_pm_api_id(req, &mut pm_id);
    if ret >= 0 {
        req = strsep(&mut kern_buff, b" \0".as_ptr());
        let mut i = 0usize;
        while i < args.len() && !req.is_null() {
            args[i] = zynqmp_pm_argument_value(req);
            i += 1;
            req = strsep(&mut kern_buff, b" \0".as_ptr());
        }
        ret = process_api_request(pm_id, args.as_mut_ptr(), rets.as_mut_ptr());
    }
    kfree(tmp_buff);
    if ret != 0 { return ret as isize; }
    len as isize
}
unsafe fn zynqmp_pm_debugfs_api_read(_file: *mut file, _ptr: *mut u8, _len: usize, _off: *mut loff_t) -> isize { simple_read_from_buffer(_ptr, _len, _off, DEBUGFS_BUF.as_ptr(), strlen(DEBUGFS_BUF.as_ptr())) }

pub unsafe fn zynqmp_pm_api_debugfs_init() {
    firmware_debugfs_root = debugfs_create_dir(b"zynqmp-firmware\0".as_ptr(), core::ptr::null_mut());
    debugfs_create_file(b"pm\0".as_ptr(), 0o660, firmware_debugfs_root, core::ptr::null_mut(), core::ptr::null());
}

pub unsafe fn zynqmp_pm_api_debugfs_exit() { debugfs_remove_recursive(firmware_debugfs_root); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
