// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  c 2001 PPC 64 Team, IBM Corp
 *
 * /dev/nvram driver for PPC
 */

// Dependencies supplied by the surrounding kernel translation.

static mut nvram_size: core::ffi::c_uint = 0;
static mut nvram_buf: [u8; 4] = [0; 4];
static mut nvram_lock: SpinLock = SpinLock::new();

// External kernel interfaces and types.
extern "C" {
    static mut current: *mut TaskStruct;
    static mut ppc_md: PpcMd;

    fn printk(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn spin_lock_irqsave(lock: *mut SpinLock, flags: *mut core::ffi::c_ulong);
    fn spin_unlock_irqrestore(lock: *mut SpinLock, flags: core::ffi::c_ulong);
    fn rtas_function_token(function: core::ffi::c_uint) -> core::ffi::c_int;
    fn rtas_call(
        token: core::ffi::c_int,
        nargs: core::ffi::c_int,
        nret: core::ffi::c_int,
        retbuf: *mut core::ffi::c_uint,
        ...,
    ) -> core::ffi::c_int;
    fn __pa(addr: *const u8) -> core::ffi::c_ulong;
    fn of_find_node_by_type(
        from: *mut DeviceNode,
        type_: *const core::ffi::c_char,
    ) -> *mut DeviceNode;
    fn of_get_property(
        node: *mut DeviceNode,
        name: *const core::ffi::c_char,
        lenp: *mut core::ffi::c_uint,
    ) -> *const Be32;
    fn of_node_put(node: *mut DeviceNode);
    fn be32_to_cpup(value: *const Be32) -> core::ffi::c_uint;
}

// These declarations correspond to types and constants provided by included headers.
#[repr(C)]
pub struct SpinLock {
    _private: [u8; 0],
}
impl SpinLock {
    const fn new() -> Self { Self { _private: [] } }
}
#[repr(C)] pub struct TaskStruct { pub comm: [core::ffi::c_char; 16] }
#[repr(C)] pub struct DeviceNode { _private: [u8; 0] }
#[repr(transparent)] pub struct Be32(pub u32);
#[repr(C)] pub struct PpcMd {
    pub nvram_read_val: Option<unsafe extern "C" fn(core::ffi::c_int) -> u8>,
    pub nvram_write_val: Option<unsafe extern "C" fn(core::ffi::c_int, u8)>,
    pub nvram_size: Option<unsafe extern "C" fn() -> isize>,
}
const RTAS_FN_NVRAM_FETCH: core::ffi::c_uint = 0;
const RTAS_FN_NVRAM_STORE: core::ffi::c_uint = 0;

unsafe fn chrp_nvram_read_val(addr: core::ffi::c_int) -> u8 {
    let mut done: core::ffi::c_uint;
    let mut flags: core::ffi::c_ulong = 0;
    let ret: u8;

    if addr >= nvram_size as core::ffi::c_int {
        printk(b"%s: read addr %d > nvram_size %u\n\0".as_ptr() as _, (*current).comm.as_ptr(), addr, nvram_size);
        return 0xff;
    }
    spin_lock_irqsave(&mut nvram_lock, &mut flags);
    if (rtas_call(rtas_function_token(RTAS_FN_NVRAM_FETCH), 3, 2, &mut done, addr,
                  __pa(nvram_buf.as_ptr()), 1) != 0) || 1 != done {
        ret = 0xff;
    } else {
        ret = nvram_buf[0];
    }
    spin_unlock_irqrestore(&mut nvram_lock, flags);
    ret
}

unsafe fn chrp_nvram_write_val(addr: core::ffi::c_int, val: u8) {
    let mut done: core::ffi::c_uint;
    let mut flags: core::ffi::c_ulong = 0;

    if addr >= nvram_size as core::ffi::c_int {
        printk(b"%s: write addr %d > nvram_size %u\n\0".as_ptr() as _, (*current).comm.as_ptr(), addr, nvram_size);
        return;
    }
    spin_lock_irqsave(&mut nvram_lock, &mut flags);
    nvram_buf[0] = val;
    if (rtas_call(rtas_function_token(RTAS_FN_NVRAM_STORE), 3, 2, &mut done, addr,
                  __pa(nvram_buf.as_ptr()), 1) != 0) || 1 != done {
        printk(b"rtas IO error storing 0x%02x at %d\0".as_ptr() as _, val, addr);
    }
    spin_unlock_irqrestore(&mut nvram_lock, flags);
}

unsafe fn chrp_nvram_size() -> isize {
    nvram_size as isize
}

pub unsafe extern "C" fn chrp_nvram_init() {
    let nvram: *mut DeviceNode;
    let nbytes_p: *const Be32;
    let mut proplen: core::ffi::c_uint;

    nvram = of_find_node_by_type(core::ptr::null_mut(), b"nvram\0".as_ptr() as _);
    if nvram.is_null() { return; }

    nbytes_p = of_get_property(nvram, b"#bytes\0".as_ptr() as _, &mut proplen);
    if nbytes_p.is_null() || proplen != core::mem::size_of::<core::ffi::c_uint>() as core::ffi::c_uint {
        of_node_put(nvram);
        return;
    }

    nvram_size = be32_to_cpup(nbytes_p);
    printk(b"CHRP nvram contains %u bytes\n\0".as_ptr() as _, nvram_size);
    of_node_put(nvram);

    ppc_md.nvram_read_val = Some(chrp_nvram_read_val);
    ppc_md.nvram_write_val = Some(chrp_nvram_write_val);
    ppc_md.nvram_size = Some(chrp_nvram_size);
}

// MODULE_DESCRIPTION("PPC NVRAM device driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
