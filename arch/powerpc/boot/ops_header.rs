/* SPDX-License-Identifier: GPL-2.0 */
/* Global definition of all the bootwrapper operations. */

use core::ffi::{c_char, c_int, c_ulong, c_void};

/* C dependencies: types.h and string.h. */
pub type u8 = core::primitive::u8;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;

pub const BOOT_COMMAND_LINE_SIZE: usize = 2048;
pub const MAX_PATH_LEN: usize = 256;
pub const MAX_PROP_LEN: usize = 256;

pub type KernelEntryT = unsafe extern "C" fn(c_ulong, c_ulong, *mut c_void);

#[repr(C)]
pub struct PlatformOps {
    pub fixups: Option<unsafe extern "C" fn()>,
    pub image_hdr: Option<unsafe extern "C" fn(*const c_void)>,
    pub malloc: Option<unsafe extern "C" fn(c_ulong) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub realloc: Option<unsafe extern "C" fn(*mut c_void, c_ulong) -> *mut c_void>,
    pub exit: Option<unsafe extern "C" fn()>,
    pub vmlinux_alloc: Option<unsafe extern "C" fn(c_ulong) -> *mut c_void>,
    pub kentry: Option<unsafe extern "C" fn(c_ulong, *mut c_void)>,
}
unsafe extern "C" {
    pub static mut platform_ops: PlatformOps;
}

#[repr(C)]
pub struct DtOps {
    pub finddevice: Option<unsafe extern "C" fn(*const c_char) -> *mut c_void>,
    pub getprop: Option<unsafe extern "C" fn(*const c_void, *const c_char, *mut c_void, c_int) -> c_int>,
    pub setprop: Option<unsafe extern "C" fn(*const c_void, *const c_char, *const c_void, c_int) -> c_int>,
    pub del_node: Option<unsafe extern "C" fn(*const c_void) -> c_int>,
    pub get_parent: Option<unsafe extern "C" fn(*const c_void) -> *mut c_void>,
    pub create_node: Option<unsafe extern "C" fn(*const c_void, *const c_char) -> *mut c_void>,
    pub find_node_by_prop_value: Option<unsafe extern "C" fn(*const c_void, *const c_char, *const c_char, c_int) -> *mut c_void>,
    pub find_node_by_compatible: Option<unsafe extern "C" fn(*const c_void, *const c_char) -> *mut c_void>,
    pub finalize: Option<unsafe extern "C" fn() -> c_ulong>,
    pub get_path: Option<unsafe extern "C" fn(*const c_void, *mut c_char, c_int) -> *mut c_char>,
}
unsafe extern "C" { pub static mut dt_ops: DtOps; }

#[repr(C)]
pub struct ConsoleOps {
    pub open: Option<unsafe extern "C" fn() -> c_int>,
    pub write: Option<unsafe extern "C" fn(*const c_char, c_int)>,
    pub edit_cmdline: Option<unsafe extern "C" fn(*mut c_char, c_int, u32)>,
    pub close: Option<unsafe extern "C" fn()>,
    pub data: *mut c_void,
}
unsafe extern "C" { pub static mut console_ops: ConsoleOps; }

#[repr(C)]
pub struct SerialConsoleData {
    pub open: Option<unsafe extern "C" fn() -> c_int>,
    pub putc: Option<unsafe extern "C" fn(u8)>,
    pub getc: Option<unsafe extern "C" fn() -> u8>,
    pub tstc: Option<unsafe extern "C" fn() -> u8>,
    pub close: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct LoaderInfo {
    pub promptr: *mut c_void,
    pub initrd_addr: c_ulong,
    pub initrd_size: c_ulong,
    pub cmdline: *mut c_char,
    pub cmdline_len: c_int,
}
unsafe extern "C" { pub static mut loader_info: LoaderInfo; }

unsafe extern "C" {
    pub fn start(); pub fn fdt_init(blob: *mut c_void); pub fn serial_console_init() -> c_int;
    pub fn ns16550_console_init(devp: *mut c_void, scdp: *mut SerialConsoleData) -> c_int;
    pub fn cpm_console_init(devp: *mut c_void, scdp: *mut SerialConsoleData) -> c_int;
    pub fn mpc5200_psc_console_init(devp: *mut c_void, scdp: *mut SerialConsoleData) -> c_int;
    pub fn opal_console_init(devp: *mut c_void, scdp: *mut SerialConsoleData) -> c_int;
    pub fn simple_alloc_init(base: *mut c_char, heap_size: c_ulong, granularity: c_ulong, max_allocs: c_ulong) -> *mut c_void;
    pub fn flush_cache(ptr: *mut c_void, size: c_ulong);
    pub fn dt_xlate_reg(node: *mut c_void, res: c_int, addr: *mut c_ulong, size: *mut c_ulong) -> c_int;
    pub fn dt_xlate_addr(node: *mut c_void, buf: *mut u32, buflen: c_int, xlated_addr: *mut c_ulong) -> c_int;
    pub fn dt_is_compatible(node: *mut c_void, compat: *const c_char) -> c_int;
    pub fn dt_get_reg_format(node: *mut c_void, naddr: *mut u32, nsize: *mut u32);
    pub fn dt_get_virtual_reg(node: *mut c_void, addr: *mut *mut c_void, nres: c_int) -> c_int;
    pub fn strlen(s: *const c_char) -> usize;
}

#[inline] pub unsafe fn finddevice(name: *const c_char) -> *mut c_void { dt_ops.finddevice.map_or(core::ptr::null_mut(), |f| f(name)) }
#[inline] pub unsafe fn getprop(devp: *mut c_void, name: *const c_char, buf: *mut c_void, buflen: c_int) -> c_int { dt_ops.getprop.map_or(-1, |f| f(devp, name, buf, buflen)) }
#[inline] pub unsafe fn setprop(devp: *mut c_void, name: *const c_char, buf: *const c_void, buflen: c_int) -> c_int { dt_ops.setprop.map_or(-1, |f| f(devp, name, buf, buflen)) }
#[macro_export] macro_rules! setprop_val { ($devp:expr, $name:expr, $val:expr) => {{ let mut x = $val; $crate::setprop($devp, $name, (&mut x as *mut _).cast(), core::mem::size_of_val(&x) as c_int); }} }
#[inline] pub unsafe fn setprop_str(devp: *mut c_void, name: *const c_char, buf: *const c_char) -> c_int { dt_ops.setprop.map_or(-1, |f| f(devp, name, buf.cast(), strlen(buf) as c_int + 1)) }
#[inline] pub unsafe fn del_node(devp: *const c_void) -> c_int { dt_ops.del_node.map_or(-1, |f| f(devp)) }
#[inline] pub unsafe fn get_parent(devp: *const c_char) -> *mut c_void { dt_ops.get_parent.map_or(core::ptr::null_mut(), |f| f(devp.cast())) }
#[inline] pub unsafe fn create_node(parent: *const c_void, name: *const c_char) -> *mut c_void { dt_ops.create_node.map_or(core::ptr::null_mut(), |f| f(parent, name)) }
#[inline] pub unsafe fn find_node_by_prop_value(prev: *const c_void, propname: *const c_char, propval: *const c_char, proplen: c_int) -> *mut c_void { dt_ops.find_node_by_prop_value.map_or(core::ptr::null_mut(), |f| f(prev, propname, propval, proplen)) }
#[inline] pub unsafe fn find_node_by_prop_value_str(prev: *const c_void, propname: *const c_char, propval: *const c_char) -> *mut c_void { find_node_by_prop_value(prev, propname, propval, strlen(propval) as c_int + 1) }
#[inline] pub unsafe fn find_node_by_devtype(prev: *const c_void, ty: *const c_char) -> *mut c_void { find_node_by_prop_value_str(prev, b"device_type\0".as_ptr().cast(), ty) }
#[inline] pub unsafe fn find_node_by_alias(alias: *const c_char) -> *mut c_void { let devp = finddevice(b"/aliases\0".as_ptr().cast()); if !devp.is_null() { let mut path = [0 as c_char; MAX_PATH_LEN]; if getprop(devp, alias, path.as_mut_ptr().cast(), MAX_PATH_LEN as c_int) > 0 { return finddevice(path.as_ptr()); } } core::ptr::null_mut() }
#[inline] pub unsafe fn find_node_by_compatible(prev: *const c_void, compat: *const c_char) -> *mut c_void { dt_ops.find_node_by_compatible.map_or(core::ptr::null_mut(), |f| f(prev, compat)) }

unsafe extern "C" { pub fn dt_fixup_memory(start: u64, size: u64); pub fn dt_fixup_cpu_clocks(cpufreq: u32, tbfreq: u32, busfreq: u32); pub fn dt_fixup_clock(path: *const c_char, freq: u32); pub fn dt_fixup_mac_address_by_alias(alias: *const c_char, addr: *const u8); pub fn dt_fixup_mac_address(index: u32, addr: *const u8); pub fn __dt_fixup_mac_addresses(startindex: u32, ...); }
#[inline] pub unsafe fn get_path(phandle: *const c_void, buf: *mut c_char, len: c_int) -> *mut c_char { dt_ops.get_path.map_or(core::ptr::null_mut(), |f| f(phandle, buf, len)) }
#[inline] pub unsafe fn malloc(size: c_ulong) -> *mut c_void { platform_ops.malloc.map_or(core::ptr::null_mut(), |f| f(size)) }
#[inline] pub unsafe fn free(ptr: *mut c_void) { if let Some(f) = platform_ops.free { f(ptr); } }
#[inline] pub unsafe fn exit() -> ! { if let Some(f) = platform_ops.exit { f(); } loop {} }
#[macro_export] macro_rules! fatal { ($($args:tt)*) => {{ printf!($($args)*); $crate::exit(); }} }
#[macro_export] macro_rules! dt_fixup_mac_addresses { ($($addr:expr),* $(,)?) => {{ $crate::__dt_fixup_mac_addresses(0, $($addr),*, core::ptr::null::<core::ffi::c_void>()); }} }
#[macro_export] macro_rules! BSS_STACK { ($size:expr) => { static mut _bss_stack: [c_char; $size] = [0; $size]; static mut _platform_stack_top: *mut c_void = unsafe { _bss_stack.as_ptr().add($size) as *mut c_void }; } }

unsafe extern "C" { pub static mut timebase_period_ns: c_ulong; pub fn udelay(delay: isize); pub static mut _start: [c_char; 0]; pub static mut __bss_start: [c_char; 0]; pub static mut _end: [c_char; 0]; pub static mut _vmlinux_start: [c_char; 0]; pub static mut _vmlinux_end: [c_char; 0]; pub static mut _initrd_start: [c_char; 0]; pub static mut _initrd_end: [c_char; 0]; pub static mut _dtb_start: [c_char; 0]; pub static mut _dtb_end: [c_char; 0]; pub static mut _esm_blob_start: [c_char; 0]; pub static mut _esm_blob_end: [c_char; 0]; pub fn partial_decompress(inbuf: *mut c_void, input_size: c_ulong, outbuf: *mut c_void, output_size: c_ulong, skip: c_ulong) -> c_long; }
pub type c_long = isize;

#[inline] pub const unsafe fn __ilog2_u32(n: u32) -> c_int { 31 - n.leading_zeros() as c_int }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
