// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external FFI declarations:
// stdlib.h, dwarf.h, elfutils/libdw.h, elfutils/libdwfl.h, elfutils/version.h

use core::ffi::{c_int, c_void};

type size_t = usize;
type ptrdiff_t = isize;
type Dwarf_Addr = u64;

const DWARF_C_READ: c_int = 1;

#[repr(C)]
pub struct Dwarf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwarf_Attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwarf_Op {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwarf_CFI {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn dwarf_begin(fildes: c_int, cmd: c_int) -> *mut Dwarf;
    fn dwfl_thread_getframes(thread: *mut c_void, callback: *mut c_void, arg: *mut c_void) -> c_int;
    fn dwarf_getlocations(
        attr: *mut Dwarf_Attribute,
        offset: ptrdiff_t,
        basep: *mut Dwarf_Addr,
        startp: *mut Dwarf_Addr,
        endp: *mut Dwarf_Addr,
        expr: *mut *mut Dwarf_Op,
        exprlen: *mut size_t,
    ) -> ptrdiff_t;
    fn dwarf_getcfi(dwarf: *mut Dwarf) -> *mut Dwarf_CFI;
    fn dwarf_cfi_end(cache: *mut Dwarf_CFI);
}

#[no_mangle]
pub unsafe extern "C" fn test_libdw() -> c_int {
    let dbg: *mut Dwarf = unsafe { dwarf_begin(0, DWARF_C_READ) };

    (dbg == core::ptr::null_mut()) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn test_libdw_unwind() -> c_int {
    /*
     * This function is guarded via: __nonnull_attribute__ (1, 2).
     * Passing '1' as arguments value. This code is never executed,
     * only compiled.
     */
    unsafe {
        dwfl_thread_getframes(
            1usize as *mut c_void,
            1usize as *mut c_void,
            core::ptr::null_mut(),
        );
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn test_libdw_getlocations() -> c_int {
    let mut base: Dwarf_Addr = 0;
    let mut start: Dwarf_Addr = 0;
    let mut end: Dwarf_Addr = 0;
    let mut attr = core::mem::MaybeUninit::<Dwarf_Attribute>::uninit();
    let mut op: *mut Dwarf_Op = core::ptr::null_mut();
    let mut nops: size_t = 0;
    let offset: ptrdiff_t = 0;

    unsafe {
        dwarf_getlocations(
            attr.as_mut_ptr(),
            offset,
            &mut base,
            &mut start,
            &mut end,
            &mut op,
            &mut nops,
        ) as c_int
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_libdw_getcfi() -> c_int {
    let dwarf: *mut Dwarf = core::ptr::null_mut();

    (unsafe { dwarf_getcfi(dwarf) } == core::ptr::null_mut()) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn test_elfutils() -> c_int {
    let cfi: *mut Dwarf_CFI = core::ptr::null_mut();

    unsafe {
        dwarf_cfi_end(cfi);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    unsafe {
        test_libdw()
            + test_libdw_unwind()
            + test_libdw_getlocations()
            + test_libdw_getcfi()
            + test_elfutils()
    }
}
