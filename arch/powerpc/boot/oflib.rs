// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) Paul Mackerras 1997.
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the surrounding PowerPC boot environment.
type u32 = core::primitive::u32;
type prom_arg_t = u32;
type __be32 = u32;
type phandle = u32;
type ihandle = u32;

extern "C" {
    static mut _start: u8;
    static mut _end: u8;
    fn malloc(size: usize) -> *mut c_void;
    fn fatal(msg: *const c_char) -> !;
    fn printf(fmt: *const c_char, ...) -> c_int;
}

#[repr(C)]
struct prom_args {
    service: __be32,
    nargs: __be32,
    nret: __be32,
    args: [__be32; 10],
}

#[cfg(target_arch = "powerpc64")]
extern "C" {
    fn prom(args: *mut c_void) -> c_int;
}

#[cfg(not(target_arch = "powerpc64"))]
static mut prom: Option<unsafe extern "C" fn(*mut c_void) -> c_int> = None;

pub unsafe extern "C" fn of_init(promptr: *mut c_void) {
    #[cfg(not(target_arch = "powerpc64"))]
    {
        prom = Some(core::mem::transmute(promptr));
    }
}

unsafe fn prom_call(args: *mut prom_args) -> c_int {
    #[cfg(target_arch = "powerpc64")]
    { prom(args as *mut c_void) }
    #[cfg(not(target_arch = "powerpc64"))]
    { prom.unwrap()(args as *mut c_void) }
}

unsafe fn addr<T>(x: *const T) -> u32 { x as usize as u32 }

pub unsafe extern "C" fn of_call_prom(service: *const c_char, nargs: c_int, nret: c_int, ...) -> c_int {
    // The C varargs list is intentionally retained at the ABI boundary.
    let mut args = prom_args { service: addr(service), nargs: nargs as u32, nret: nret as u32, args: [0; 10] };
    if prom_call(&mut args) < 0 { return -1; }
    if nret > 0 { args.args[nargs as usize] as c_int } else { 0 }
}

unsafe fn of_call_prom_ret(service: *const c_char, nargs: c_int, nret: c_int, rets: *mut prom_arg_t, ...) -> c_int {
    let mut args = prom_args { service: addr(service), nargs: nargs as u32, nret: nret as u32, args: [0; 10] };
    if prom_call(&mut args) < 0 { return -1; }
    if !rets.is_null() {
        for i in 1..nret { *rets.add((i - 1) as usize) = args.args[(nargs + i) as usize]; }
    }
    if nret > 0 { args.args[nargs as usize] as c_int } else { 0 }
}

unsafe fn string_match(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
    while *s2 != 0 {
        if *s1 != *s2 { return 0; }
        s1 = s1.add(1); s2 = s2.add(1);
    }
    1
}

static mut need_map: c_int = -1;
static mut chosen_mmu: ihandle = 0;
static mut memory: ihandle = 0;

unsafe fn check_of_version() -> c_int {
    let oprom = of_finddevice(b"/openprom\0".as_ptr() as *const c_char);
    if oprom as isize == -1 { return 0; }
    let mut version = [0 as c_char; 64];
    if of_getprop(oprom, b"model\0".as_ptr() as *const c_char, version.as_mut_ptr() as *mut c_void, 64) <= 0 { return 0; }
    version[63] = 0;
    printf(b"OF version = '%s'\r\n\0".as_ptr() as *const c_char, version.as_ptr());
    if string_match(version.as_ptr(), b"Open Firmware, 1.\0".as_ptr() as *const c_char) == 0 && string_match(version.as_ptr(), b"FirmWorks,3.\0".as_ptr() as *const c_char) == 0 { return 0; }
    let mut chosen = of_finddevice(b"/chosen\0".as_ptr() as *const c_char);
    if chosen as isize == -1 { chosen = of_finddevice(b"/chosen@0\0".as_ptr() as *const c_char); if chosen as isize == -1 { printf(b"no chosen\n\0".as_ptr() as *const c_char); return 0; } }
    if of_getprop(chosen, b"mmu\0".as_ptr() as *const c_char, &mut chosen_mmu as *mut _ as *mut c_void, core::mem::size_of::<ihandle>() as c_int) <= 0 { printf(b"no mmu\n\0".as_ptr() as *const c_char); return 0; }
    memory = of_call_prom(b"open\0".as_ptr() as *const c_char, 1, 1, b"/memory\0".as_ptr());
    if memory == -1 { memory = of_call_prom(b"open\0".as_ptr() as *const c_char, 1, 1, b"/memory@0\0".as_ptr()); if memory == -1 { printf(b"no memory node\n\0".as_ptr() as *const c_char); return 0; } }
    printf(b"old OF detected\r\n\0".as_ptr() as *const c_char); 1
}

pub unsafe extern "C" fn of_claim(virt: usize, size: usize, align: usize) -> u32 {
    if need_map < 0 { need_map = check_of_version(); }
    if align != 0 || need_map == 0 { return of_call_prom(b"claim\0".as_ptr() as *const c_char, 3, 1, virt, size, align) as u32; }
    let mut result = 0;
    let mut ret = of_call_prom_ret(b"call-method\0".as_ptr() as *const c_char, 5, 2, &mut result, b"claim\0".as_ptr(), memory, align, size, virt);
    if ret != 0 || result == u32::MAX { return u32::MAX; }
    ret = of_call_prom_ret(b"call-method\0".as_ptr() as *const c_char, 5, 2, &mut result, b"claim\0".as_ptr(), chosen_mmu, align, size, virt);
    let _ = ret;
    let _ = of_call_prom(b"call-method\0".as_ptr() as *const c_char, 6, 1, b"map\0".as_ptr(), chosen_mmu, 0x12, size, virt, virt);
    virt as u32
}

pub unsafe extern "C" fn of_vmlinux_alloc(size: usize) -> *mut c_void {
    let start = &_start as *const u8 as usize; let end = &_end as *const u8 as usize;
    let addr = of_claim(start, end - start, 0) as usize;
    printf(b"Trying to claim from 0x%lx to 0x%lx (0x%lx) got %lx\r\n\0".as_ptr() as *const c_char, start, end, end - start, addr);
    let p = malloc(size); if p.is_null() { fatal(b"Can't allocate memory for kernel image!\n\r\0".as_ptr() as *const c_char); } p
}

pub unsafe extern "C" fn of_exit() { let _ = of_call_prom(b"exit\0".as_ptr() as *const c_char, 0, 0); }

pub unsafe extern "C" fn of_finddevice(name: *const c_char) -> *mut c_void { of_call_prom(b"finddevice\0".as_ptr() as *const c_char, 1, 1, name) as usize as *mut c_void }

pub unsafe extern "C" fn of_getprop(phandle: *const c_void, name: *const c_char, buf: *mut c_void, buflen: c_int) -> c_int { of_call_prom(b"getprop\0".as_ptr() as *const c_char, 4, 1, phandle, name, buf, buflen) }

pub unsafe extern "C" fn of_setprop(phandle: *const c_void, name: *const c_char, buf: *const c_void, buflen: c_int) -> c_int { of_call_prom(b"setprop\0".as_ptr() as *const c_char, 4, 1, phandle, name, buf, buflen) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
