/*
 * ARAnyM console driver
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Linux kernel declarations and asm/natfeat symbols are supplied by other files.
type U8 = u8;
type SizeT = usize;
type SSizeT = isize;

#[repr(C)]
pub struct TtyPort {
    _private: [u8; 0],
}
#[repr(C)]
pub struct TtyDriver {
    _private: [u8; 0],
}
#[repr(C)]
pub struct TtyStruct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct File {
    _private: [u8; 0],
}
#[repr(C)]
pub struct Console {
    pub name: *const c_char,
    pub write: Option<unsafe extern "C" fn(*mut Console, *const c_char, c_uint)>,
    pub device: Option<unsafe extern "C" fn(*mut Console, *mut c_int) -> *mut TtyDriver>,
    pub flags: c_uint,
    pub index: c_int,
}
#[repr(C)]
pub struct TtyOperations {
    pub open: Option<unsafe extern "C" fn(*mut TtyStruct, *mut File) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut TtyStruct, *mut File)>,
    pub write: Option<unsafe extern "C" fn(*mut TtyStruct, *const U8, SizeT) -> SSizeT>,
    pub put_char: Option<unsafe extern "C" fn(*mut TtyStruct, U8) -> c_int>,
    pub write_room: Option<unsafe extern "C" fn(*mut TtyStruct) -> c_uint>,
}

extern "C" {
    fn virt_to_phys(addr: *const c_void) -> c_ulong;
    fn nf_call(id: c_int, phys: c_ulong);
    fn nf_get_id(name: *const c_char) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn tty_port_init(port: *mut TtyPort);
    fn tty_port_destroy(port: *mut TtyPort);
    fn tty_alloc_driver(count: c_uint, flags: c_ulong) -> *mut TtyDriver;
    fn tty_set_operations(driver: *mut TtyDriver, ops: *const TtyOperations);
    fn tty_port_link_device(port: *mut TtyPort, driver: *mut TtyDriver, index: c_int);
    fn tty_register_driver(driver: *mut TtyDriver) -> c_int;
    fn tty_unregister_driver(driver: *mut TtyDriver);
    fn tty_driver_kref_put(driver: *mut TtyDriver);
    fn register_console(con: *mut Console);
    fn unregister_console(con: *mut Console);
    fn console_is_registered(con: *const Console) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
}

static mut stderr_id: c_int = 0;
static mut nfcon_tty_port: TtyPort = TtyPort { _private: [] };
static mut nfcon_tty_driver: *mut TtyDriver = core::ptr::null_mut();

unsafe fn nfputs(str_: *const U8, mut count: SizeT) {
    let mut buf = [0u8; 68];
    let phys = virt_to_phys(buf.as_ptr() as *const c_void);
    buf[64] = 0;
    let mut current = str_;
    while count > 64 {
        core::ptr::copy_nonoverlapping(current, buf.as_mut_ptr(), 64);
        nf_call(stderr_id, phys);
        current = current.add(64);
        count -= 64;
    }
    core::ptr::copy_nonoverlapping(current, buf.as_mut_ptr(), count);
    buf[count] = 0;
    nf_call(stderr_id, phys);
}

unsafe extern "C" fn nfcon_write(_con: *mut Console, str_: *const c_char, count: c_uint) {
    nfputs(str_ as *const U8, count as SizeT);
}

unsafe extern "C" fn nfcon_device(_con: *mut Console, index: *mut c_int) -> *mut TtyDriver {
    *index = 0;
    nfcon_tty_driver
}

static mut nf_console: Console = Console {
    name: b"nfcon\0".as_ptr() as *const c_char,
    write: Some(nfcon_write),
    device: Some(nfcon_device),
    flags: 0x1,
    index: -1,
};

unsafe extern "C" fn nfcon_tty_open(_tty: *mut TtyStruct, _filp: *mut File) -> c_int { 0 }
unsafe extern "C" fn nfcon_tty_close(_tty: *mut TtyStruct, _filp: *mut File) {}
unsafe extern "C" fn nfcon_tty_write(_tty: *mut TtyStruct, buf: *const U8, count: SizeT) -> SSizeT {
    nfputs(buf, count);
    count as SSizeT
}
unsafe extern "C" fn nfcon_tty_put_char(_tty: *mut TtyStruct, ch: U8) -> c_int {
    let temp = [ch, 0];
    nf_call(stderr_id, virt_to_phys(temp.as_ptr() as *const c_void));
    1
}
unsafe extern "C" fn nfcon_tty_write_room(_tty: *mut TtyStruct) -> c_uint { 64 }

static nfcon_tty_ops: TtyOperations = TtyOperations {
    open: Some(nfcon_tty_open), close: Some(nfcon_tty_close), write: Some(nfcon_tty_write),
    put_char: Some(nfcon_tty_put_char), write_room: Some(nfcon_tty_write_room),
};

// The C source registers this function with early_param("debug", nf_debug_setup)
// when built without MODULE.  The registration is supplied by the kernel build.
#[cfg(not(feature = "module"))]
unsafe extern "C" fn nf_debug_setup(arg: *mut c_char) -> c_int {
    if strcmp(arg, b"nfcon\0".as_ptr() as *const c_char) != 0 { return 0; }
    stderr_id = nf_get_id(b"NF_STDERR\0".as_ptr() as *const c_char);
    if stderr_id != 0 {
        // The console is explicitly enabled for the non-standard debug=nfcon path.
        nf_console.flags |= 0x2;
        register_console(&raw mut nf_console);
    }
    0
}

unsafe extern "C" fn nfcon_init() -> c_int {
    stderr_id = nf_get_id(b"NF_STDERR\0".as_ptr() as *const c_char);
    if stderr_id == 0 { return -19; }
    let driver = tty_alloc_driver(1, 0x0001);
    if driver.is_null() { return -1; }
    tty_port_init(&raw mut nfcon_tty_port);
    tty_set_operations(driver, &nfcon_tty_ops);
    tty_port_link_device(&raw mut nfcon_tty_port, driver, 0);
    let res = tty_register_driver(driver);
    if res != 0 {
        pr_err(b"failed to register nfcon tty driver\n\0".as_ptr() as *const c_char);
        tty_driver_kref_put(driver);
        tty_port_destroy(&raw mut nfcon_tty_port);
        return res;
    }
    nfcon_tty_driver = driver;
    if console_is_registered(&raw const nf_console) == 0 { register_console(&raw mut nf_console); }
    0
}

unsafe extern "C" fn nfcon_exit() {
    unregister_console(&raw mut nf_console);
    tty_unregister_driver(nfcon_tty_driver);
    tty_driver_kref_put(nfcon_tty_driver);
    tty_port_destroy(&raw mut nfcon_tty_port);
}

// Equivalent to the C module_init(nfcon_init) and module_exit(nfcon_exit) macros.
// MODULE_DESCRIPTION("Atari NatFeat console driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
