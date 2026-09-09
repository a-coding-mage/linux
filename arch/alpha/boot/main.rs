// SPDX-License-Identifier: GPL-2.0
/*
 * arch/alpha/boot/main.c
 *
 * Copyright (C) 1994, 1995 Linus Torvalds
 *
 * This file is the bootloader for the Linux/AXP kernel
 */

// C header dependencies are supplied by the surrounding kernel build.

extern "C" {
    fn switch_to_osf_pal(
        nr: ::core::ffi::c_ulong,
        pcb_va: *mut pcb_struct,
        pcb_pa: *mut pcb_struct,
        vptb: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;
    fn srm_printk(fmt: *const ::core::ffi::c_char, ...);
    fn __halt() -> !;
    fn tbia();
    fn callback_getenv(name: ::core::ffi::c_int, value: *mut ::core::ffi::c_char, length: usize) -> ::core::ffi::c_long;
    fn callback_open(name: *mut ::core::ffi::c_char, flags: ::core::ffi::c_long) -> ::core::ffi::c_long;
    fn callback_close(dev: ::core::ffi::c_long) -> ::core::ffi::c_long;
    fn callback_read(dev: ::core::ffi::c_long, count: ::core::ffi::c_ulong, addr: *mut ::core::ffi::c_void, blocks: ::core::ffi::c_ulong) -> ::core::ffi::c_long;
    fn strcpy(dst: *mut ::core::ffi::c_char, src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}

#[repr(C)]
pub struct pcb_struct {
    pub ksp: ::core::ffi::c_ulong,
    pub usp: ::core::ffi::c_ulong,
    pub ptbr: ::core::ffi::c_ulong,
    pub asn: ::core::ffi::c_ulong,
    pub pcc: ::core::ffi::c_ulong,
    pub unique: ::core::ffi::c_ulong,
    pub flags: ::core::ffi::c_ulong,
    pub res1: ::core::ffi::c_ulong,
    pub res2: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct hwrpb_struct {
    pub pagesize: ::core::ffi::c_ulong,
    pub processor_offset: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct percpu_struct {
    pub pal_revision: ::core::ffi::c_ulong,
    pub palcode_avail: [::core::ffi::c_ulong; 3],
}

// Build-time constants supplied by the boot environment.
extern "C" {
    static mut hwrpb: *mut hwrpb_struct;
}

static mut PCB_VA: [pcb_struct; 1] = [pcb_struct {
    ksp: 0, usp: 0, ptbr: 0, asn: 0, pcc: 0, unique: 0, flags: 0, res1: 0, res2: 0,
}];

#[inline]
unsafe fn find_pa(vptb: *mut ::core::ffi::c_ulong, ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let address = ptr as usize as ::core::ffi::c_ulong;
    let mut result = *vptb.add((address >> 13) as usize);
    result >>= 32;
    result <<= 13;
    result |= address & 0x1fff;
    result as usize as *mut ::core::ffi::c_void
}

const VPTB: *mut ::core::ffi::c_ulong = 0x200000000usize as *mut ::core::ffi::c_ulong;
const L1: *mut ::core::ffi::c_ulong = 0x200802000usize as *mut ::core::ffi::c_ulong;

#[no_mangle]
pub unsafe extern "C" fn pal_init() {
    let mut i: ::core::ffi::c_ulong;
    let mut rev: ::core::ffi::c_ulong;
    let mut percpu: *mut percpu_struct;
    let pcb_pa: *mut pcb_struct;

    PCB_VA[0].ksp = 0;
    PCB_VA[0].usp = 0;
    PCB_VA[0].ptbr = *L1.add(1) >> 32;
    PCB_VA[0].asn = 0;
    PCB_VA[0].pcc = 0;
    PCB_VA[0].unique = 0;
    PCB_VA[0].flags = 1;
    PCB_VA[0].res1 = 0;
    PCB_VA[0].res2 = 0;
    pcb_pa = find_pa(VPTB, PCB_VA.as_mut_ptr() as *mut ::core::ffi::c_void) as *mut pcb_struct;

    srm_printk(b"Switching to OSF PAL-code .. \0".as_ptr() as *const _);
    i = switch_to_osf_pal(2, PCB_VA.as_mut_ptr(), pcb_pa, VPTB);
    if i != 0 {
        srm_printk(b"failed, code %ld\n\0".as_ptr() as *const _, i);
        __halt();
    }
    percpu = ((*hwrpb).processor_offset + hwrpb as usize as ::core::ffi::c_ulong) as usize as *mut percpu_struct;
    rev = (*percpu).palcode_avail[2];
    (*percpu).pal_revision = rev;
    srm_printk(b"Ok (rev %lx)\n\0".as_ptr() as *const _, rev);
    tbia();
}

#[inline]
unsafe fn openboot() -> ::core::ffi::c_long {
    let mut bootdev = [0i8; 256];
    let result = callback_getenv(ENV_BOOTED_DEV, bootdev.as_mut_ptr(), 255);
    if result < 0 { return result; }
    callback_open(bootdev.as_mut_ptr(), result & 255)
}

#[inline]
unsafe fn close(dev: ::core::ffi::c_long) -> ::core::ffi::c_long { callback_close(dev) }

#[inline]
unsafe fn load(dev: ::core::ffi::c_long, addr: ::core::ffi::c_ulong, count: ::core::ffi::c_ulong) -> ::core::ffi::c_long {
    let mut bootfile = [0i8; 256];
    let boot_size = (&_end as *const _ as usize).wrapping_sub(BOOT_ADDR as usize) as ::core::ffi::c_ulong;
    let mut result = callback_getenv(ENV_BOOTED_FILE, bootfile.as_mut_ptr(), 255);
    if result < 0 { return result; }
    result &= 255;
    bootfile[result as usize] = 0;
    if result != 0 { srm_printk(b"Boot file specification (%s) not implemented\n\0".as_ptr() as *const _, bootfile.as_ptr()); }
    callback_read(dev, count, addr as usize as *mut ::core::ffi::c_void, boot_size / 512 + 1)
}

unsafe fn runkernel() {
    core::arch::asm!("bis {1},{1},$30", "bis {0},{0},$26", "ret ($26)", in(reg) START_ADDR, in(reg) PAGE_SIZE + INIT_STACK, options(noreturn));
}

#[no_mangle]
pub unsafe extern "C" fn start_kernel() {
    let mut i: ::core::ffi::c_long;
    let mut dev: ::core::ffi::c_long;
    let mut nbytes: ::core::ffi::c_int;
    let mut envval = [0i8; 256];
    srm_printk(b"Linux/AXP bootloader for Linux " .as_ptr() as *const _);
    if (*hwrpb).pagesize != 8192 { srm_printk(b"Expected 8kB pages, got %ldkB\n\0".as_ptr() as *const _, (*hwrpb).pagesize >> 10); return; }
    pal_init();
    dev = openboot();
    if dev < 0 { srm_printk(b"Unable to open boot device: %016lx\n\0".as_ptr() as *const _, dev); return; }
    dev &= 0xffffffff;
    srm_printk(b"Loading vmlinux ...\0".as_ptr() as *const _);
    i = load(dev, START_ADDR, KERNEL_SIZE);
    close(dev);
    if i != KERNEL_SIZE as ::core::ffi::c_long { srm_printk(b"Failed (%lx)\n\0".as_ptr() as *const _, i); return; }
    nbytes = callback_getenv(ENV_BOOTED_OSFLAGS, envval.as_mut_ptr(), envval.len());
    if nbytes < 0 { nbytes = 0; }
    envval[nbytes as usize] = 0;
    strcpy(ZERO_PGE as usize as *mut _, envval.as_ptr());
    srm_printk(b" Ok\nNow booting the kernel\n\0".as_ptr() as *const _);
    runkernel();
}

// External build-time symbols/constants from the included kernel headers.
extern "C" { static _end: u8; }
const ENV_BOOTED_DEV: ::core::ffi::c_int = 0;
const ENV_BOOTED_FILE: ::core::ffi::c_int = 1;
const ENV_BOOTED_OSFLAGS: ::core::ffi::c_int = 2;
const BOOT_ADDR: usize = 0;
const START_ADDR: usize = 0;
const PAGE_SIZE: usize = 8192;
const INIT_STACK: usize = 0;
const KERNEL_SIZE: usize = 0;
const ZERO_PGE: usize = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
