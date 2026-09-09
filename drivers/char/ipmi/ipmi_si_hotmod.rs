// SPDX-License-Identifier: GPL-2.0+
/*
 * ipmi_si_hotmod.c
 *
 * Handling for dynamically adding/removing IPMI devices through
 * a module parameter (and thus sysfs).
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Kernel and project-provided declarations are supplied by the surrounding translation.
extern "C" {
    fn strchr(s: *mut c_char, c: c_int) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn kstrdup(s: *const c_char, flags: c_uint) -> *mut c_char;
    fn kfree(p: *mut c_void);
    fn strstrip(s: *mut c_char) -> *mut c_char;
    fn simple_strtoul(s: *const c_char, end: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn kstrtoul(s: *const c_char, base: c_uint, result: *mut u64) -> c_int;
    fn ipmi_platform_add(name: *const c_char, id: c_int, data: *const ipmi_plat_data);
    fn ipmi_si_remove_by_data(space: c_uint, ty: c_uint, addr: u64) -> *mut device;
    fn dev_is_platform(dev: *mut device) -> bool;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn platform_device_unregister(dev: *mut platform_device);
    fn put_device(dev: *mut device);
    fn ipmi_remove_platform_device_by_name(name: *const c_char);
}

// Values and types defined by the kernel/project headers.
const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const SI_KCS: c_uint = 0;
const SI_SMIC: c_uint = 1;
const SI_BT: c_uint = 2;
const IPMI_MEM_ADDR_SPACE: c_uint = 0;
const IPMI_IO_ADDR_SPACE: c_uint = 1;
const IPMI_PLAT_IF_SI: c_uint = 0;
const SI_HOTMOD: c_uint = 0;

#[repr(C)]
pub struct ipmi_plat_data {
    pub iftype: c_uint,
    pub type_: c_uint,
    pub space: c_uint,
    pub addr: u64,
    pub regspacing: c_uint,
    pub regsize: c_uint,
    pub regshift: c_uint,
    pub irq: c_uint,
    pub slave_addr: c_uint,
    pub addr_source: c_uint,
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub name: *const c_char }

#[repr(C)]
struct hotmod_vals { name: *const c_char, val: c_int }

#[repr(i32)]
enum hotmod_op { HM_ADD, HM_REMOVE }

static mut HOTMOD_NR: c_int = 0;

static HOTMOD_OPS: &[hotmod_vals] = &[
    hotmod_vals { name: b"add\0".as_ptr() as *const c_char, val: 0 },
    hotmod_vals { name: b"remove\0".as_ptr() as *const c_char, val: 1 },
    hotmod_vals { name: core::ptr::null(), val: 0 },
];
static HOTMOD_SI: &[hotmod_vals] = &[
    hotmod_vals { name: b"kcs\0".as_ptr() as *const c_char, val: SI_KCS as c_int },
    hotmod_vals { name: b"smic\0".as_ptr() as *const c_char, val: SI_SMIC as c_int },
    hotmod_vals { name: b"bt\0".as_ptr() as *const c_char, val: SI_BT as c_int },
    hotmod_vals { name: core::ptr::null(), val: 0 },
];
static HOTMOD_AS: &[hotmod_vals] = &[
    hotmod_vals { name: b"mem\0".as_ptr() as *const c_char, val: IPMI_MEM_ADDR_SPACE as c_int },
    hotmod_vals { name: b"i/o\0".as_ptr() as *const c_char, val: IPMI_IO_ADDR_SPACE as c_int },
    hotmod_vals { name: core::ptr::null(), val: 0 },
];

unsafe fn parse_str(v: *const hotmod_vals, val: *mut c_uint, name: *const c_char, curr: *mut *const c_char) -> c_int {
    let mut s = strchr(*curr as *mut c_char, b',' as c_int);
    if s.is_null() { return -EINVAL; }
    *s = 0; s = s.add(1);
    let mut i = 0;
    while !(*v.add(i)).name.is_null() {
        if strcmp(*curr, (*v.add(i)).name) == 0 { *val = (*v.add(i)).val as c_uint; *curr = s; return 0; }
        i += 1;
    }
    -EINVAL
}

unsafe fn check_hotmod_int_op(curr: *const c_char, option: *const c_char, name: *const c_char, val: *mut c_uint) -> c_int {
    if strcmp(curr, name) == 0 {
        if option.is_null() { return -EINVAL; }
        let mut n = core::ptr::null_mut();
        *val = simple_strtoul(option, &mut n, 0) as c_uint;
        if *n != 0 || *option == 0 { return -EINVAL; }
        return 1;
    }
    0
}

unsafe fn parse_hotmod_str(mut curr: *const c_char, op: *mut hotmod_op, h: *mut ipmi_plat_data) -> c_int {
    (*h).iftype = IPMI_PLAT_IF_SI;
    let mut val = 0; let mut rv = parse_str(HOTMOD_OPS.as_ptr(), &mut val, core::ptr::null(), &mut curr); if rv != 0 { return rv; } *op = core::mem::transmute(val as i32);
    rv = parse_str(HOTMOD_SI.as_ptr(), &mut val, core::ptr::null(), &mut curr); if rv != 0 { return rv; } (*h).type_ = val;
    rv = parse_str(HOTMOD_AS.as_ptr(), &mut val, core::ptr::null(), &mut curr); if rv != 0 { return rv; } (*h).space = val;
    let mut s = strchr(curr as *mut c_char, b',' as c_int); if !s.is_null() { *s = 0; s = s.add(1); }
    rv = kstrtoul(curr, 0, &mut (*h).addr); if rv != 0 { return rv; }
    while !s.is_null() {
        curr = s; s = strchr(curr as *mut c_char, b',' as c_int); if !s.is_null() { *s = 0; s = s.add(1); }
        let mut o = strchr(curr as *mut c_char, b'=' as c_int); if !o.is_null() { *o = 0; o = o.add(1); }
        let opts = [(b"rsp\0", &mut (*h).regspacing), (b"rsi\0", &mut (*h).regsize), (b"rsh\0", &mut (*h).regshift), (b"irq\0", &mut (*h).irq), (b"ipmb\0", &mut (*h).slave_addr)];
        let mut found = false; for (n, p) in opts { rv = check_hotmod_int_op(curr, o, n.as_ptr() as *const c_char, p); if rv < 0 { return rv; } if rv != 0 { found = true; break; } }
        if !found { return -EINVAL; }
    }
    (*h).addr_source = SI_HOTMOD; 0
}

#[no_mangle]
pub unsafe extern "C" fn hotmod_handler(val: *const c_char, _kp: *const c_void) -> c_int {
    let str_ = kstrdup(val, GFP_KERNEL); if str_.is_null() { return -ENOMEM; }
    let mut curr = strstrip(str_); let mut rv = 0;
    while !curr.is_null() {
        let mut next = strchr(curr, b':' as c_int); if !next.is_null() { *next = 0; next = next.add(1); }
        let mut h: ipmi_plat_data = core::mem::zeroed(); let mut op = hotmod_op::HM_ADD;
        rv = parse_hotmod_str(curr, &mut op, &mut h); if rv != 0 { break; }
        if matches!(op, hotmod_op::HM_ADD) { HOTMOD_NR += 1; ipmi_platform_add(b"hotmod-ipmi-si\0".as_ptr() as _, HOTMOD_NR, &h); }
        else { let dev = ipmi_si_remove_by_data(h.space, h.type_, h.addr); if !dev.is_null() && dev_is_platform(dev) { let p = to_platform_device(dev); if strcmp((*p).name, b"hotmod-ipmi-si\0".as_ptr() as _) == 0 { platform_device_unregister(p); } } put_device(dev); }
        curr = next;
    }
    if rv == 0 { rv = strlen(val) as c_int; } kfree(str_ as *mut c_void); rv
}

#[no_mangle]
pub unsafe extern "C" fn ipmi_si_hotmod_exit() { ipmi_remove_platform_device_by_name(b"hotmod-ipmi-si\0".as_ptr() as _); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
