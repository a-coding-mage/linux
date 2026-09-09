// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) Paul Mackerras 1997.
 *
 * Updates for PPC64 by Todd Inglett, Dave Engebretsen & Peter Bergner.
 */

use core::ffi::c_void;

#[repr(C)]
struct AddrRange {
    addr: *mut c_void,
    size: usize,
}

// Declarations supplied by the surrounding bootwrapper sources.
extern "C" {
    static mut _vmlinux_start: *mut u8;
    static mut _vmlinux_end: *mut u8;
    static mut _start: *mut c_void;
    static mut _end: *mut c_void;
    static mut _initrd_start: u8;
    static mut _initrd_end: u8;
    #[cfg(target_arch = "powerpc64")]
    static mut _esm_blob_start: u8;
    #[cfg(target_arch = "powerpc64")]
    static mut _esm_blob_end: u8;

    static mut platform_ops: PlatformOps;
    static mut dt_ops: DtOps;
    static mut console_ops: ConsoleOps;
    static mut loader_info: LoaderInfo;
    static mut cmdline: [u8; BOOT_COMMAND_LINE_SIZE];

    fn partial_decompress(src: *mut u8, src_size: usize, dst: *mut u8, dst_size: usize, offset: usize) -> isize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memmove(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn parse_elf64(header: *const u8, ei: *mut ElfInfo) -> i32;
    fn parse_elf32(header: *const u8, ei: *mut ElfInfo) -> i32;
    fn fatal(fmt: *const u8, ...);
    fn printf(fmt: *const u8, ...);
    fn flush_cache(addr: *mut c_void, size: usize);
    fn malloc(size: usize) -> *mut c_void;
    fn setprop_val(chosen: *mut c_void, name: *const u8, value: u32);
    fn getprop(chosen: *mut c_void, name: *const u8, value: *mut i32, size: usize) -> i32;
    fn setprop_str(chosen: *mut c_void, name: *const u8, value: *const u8);
    fn finddevice(path: *const u8) -> *mut c_void;
    fn create_node(parent: *mut c_void, name: *const u8) -> *mut c_void;
    fn get_sp() -> *mut c_void;
    fn exit() -> !;
    fn min(a: usize, b: usize) -> usize;
}

// These declarations correspond to types defined by the included bootwrapper headers.
#[repr(C)] struct ElfInfo { memsize: usize, loadsize: usize, elfoffset: usize }
#[repr(C)] struct PlatformOps {
    image_hdr: Option<unsafe extern "C" fn(*const u8)>,
    vmlinux_alloc: Option<unsafe extern "C" fn(usize) -> *mut c_void>,
    fixups: Option<unsafe extern "C" fn()>,
    kentry: Option<unsafe extern "C" fn(usize, *mut c_void)>,
}
#[repr(C)] struct DtOps { finalize: Option<unsafe extern "C" fn() -> usize> }
#[repr(C)] struct ConsoleOps {
    open: Option<unsafe extern "C" fn() -> i32>,
    edit_cmdline: Option<unsafe extern "C" fn(*mut u8, usize, u32)>,
    close: Option<unsafe extern "C" fn()>,
}
#[repr(C)] struct LoaderInfo { cmdline_len: usize, initrd_addr: usize, initrd_size: usize, promptr: *mut c_void }
type KernelEntry = unsafe extern "C" fn(usize, usize, *mut c_void);

// Build-time constant supplied by the bootwrapper headers.
const BOOT_COMMAND_LINE_SIZE: usize = 2048;

unsafe fn prep_kernel() -> AddrRange {
    let mut elfheader = [0u8; 256];
    let vmlinuz_addr = _vmlinux_start;
    let vmlinuz_size = (_vmlinux_end as usize).wrapping_sub(_vmlinux_start as usize);
    let mut addr: *mut c_void = core::ptr::null_mut();
    let mut ei = ElfInfo { memsize: 0, loadsize: 0, elfoffset: 0 };
    let mut len = partial_decompress(vmlinuz_addr, vmlinuz_size, elfheader.as_mut_ptr(), elfheader.len(), 0);
    let mut uncompressed_image = false;
    if len == -1 {
        uncompressed_image = true;
        memcpy(elfheader.as_mut_ptr() as *mut c_void, vmlinuz_addr as *const c_void, elfheader.len());
        printf(b"No valid compressed data found, assume uncompressed data\n\r\0".as_ptr());
    }
    if parse_elf64(elfheader.as_ptr(), &mut ei) == 0 && parse_elf32(elfheader.as_ptr(), &mut ei) == 0 {
        fatal(b"Error: not a valid PPC32 or PPC64 ELF file!\n\r\0".as_ptr());
    }
    if let Some(f) = platform_ops.image_hdr { f(elfheader.as_ptr()); }
    printf(b"Allocating 0x%lx bytes for kernel...\n\r\0".as_ptr(), ei.memsize);
    if let Some(f) = platform_ops.vmlinux_alloc { addr = f(ei.memsize); }
    else {
        if (_start as usize) < ei.loadsize { fatal(b"Insufficient memory for kernel at address 0! (_start=%p, uncompressed size=%08lx)\n\r\0".as_ptr(), _start, ei.loadsize); }
        if (_end as usize) < ei.memsize { fatal(b"The final kernel image would overwrite the device tree\n\r\0".as_ptr()); }
    }
    if uncompressed_image {
        memcpy(addr, vmlinuz_addr.add(ei.elfoffset) as *const c_void, ei.loadsize);
        printf(b"0x%lx bytes of uncompressed data copied\n\r\0".as_ptr(), ei.loadsize);
    } else {
        printf(b"Decompressing (0x%p <- 0x%p:0x%p)...\n\r\0".as_ptr(), addr, vmlinuz_addr, vmlinuz_addr.add(vmlinuz_size));
        len = partial_decompress(vmlinuz_addr, vmlinuz_size, addr as *mut u8, ei.loadsize, ei.elfoffset);
        if len < 0 { fatal(b"Decompression failed with error code %ld\n\r\0".as_ptr(), len); }
        if len as usize != ei.loadsize { fatal(b"Decompression error: got 0x%lx bytes, expected 0x%lx.\n\r\0".as_ptr(), len, ei.loadsize); }
        printf(b"Done! Decompressed 0x%lx bytes\n\r\0".as_ptr(), len);
    }
    flush_cache(addr, ei.loadsize);
    AddrRange { addr, size: ei.memsize }
}

unsafe fn prep_initrd(vmlinux: AddrRange, chosen: *mut c_void, mut initrd_addr: usize, mut initrd_size: usize) -> AddrRange {
    if (&_initrd_end as *const u8) > (&_initrd_start as *const u8) {
        printf(b"Attached initrd image at 0x%p-0x%p\n\r\0".as_ptr(), &_initrd_start, &_initrd_end);
        initrd_addr = &_initrd_start as *const u8 as usize;
        initrd_size = (&_initrd_end as *const u8 as usize).wrapping_sub(initrd_addr);
    } else if initrd_size > 0 { printf(b"Using loader supplied ramdisk at 0x%lx-0x%lx\n\r\0".as_ptr(), initrd_addr, initrd_addr + initrd_size); }
    if initrd_size == 0 { return AddrRange { addr: core::ptr::null_mut(), size: 0 }; }
    if initrd_addr < vmlinux.size {
        let old_addr = initrd_addr as *mut c_void;
        printf(b"Allocating 0x%lx bytes for initrd ...\n\r\0".as_ptr(), initrd_size);
        initrd_addr = malloc(initrd_size) as usize;
        if initrd_addr == 0 { fatal(b"Can't allocate memory for initial ramdisk !\n\r\0".as_ptr()); }
        printf(b"Relocating initrd 0x%lx <- 0x%p (0x%lx bytes)\n\r\0".as_ptr(), initrd_addr, old_addr, initrd_size);
        memmove(initrd_addr as *mut c_void, old_addr, initrd_size);
    }
    printf(b"initrd head: 0x%lx\n\r\0".as_ptr(), *(initrd_addr as *const usize));
    setprop_val(chosen, b"linux,initrd-start\0".as_ptr(), initrd_addr as u32);
    setprop_val(chosen, b"linux,initrd-end\0".as_ptr(), (initrd_addr + initrd_size) as u32);
    AddrRange { addr: initrd_addr as *mut c_void, size: initrd_size }
}

#[cfg(target_arch = "powerpc64")]
unsafe fn prep_esm_blob(vmlinux: AddrRange, chosen: *mut c_void) {
    if (&_esm_blob_end as *const u8) <= (&_esm_blob_start as *const u8) { return; }
    let mut addr = &_esm_blob_start as *const u8 as usize;
    let size = (&_esm_blob_end as *const u8 as usize).wrapping_sub(addr);
    printf(b"Attached ESM blob at 0x%p-0x%p\n\r\0".as_ptr(), &_esm_blob_start, &_esm_blob_end);
    if addr < vmlinux.size { let old = addr as *mut c_void; printf(b"Allocating 0x%lx bytes for esm_blob ...\n\r\0".as_ptr(), size); addr = malloc(size) as usize; if addr == 0 { fatal(b"Can't allocate memory for ESM blob !\n\r\0".as_ptr()); } memmove(addr as *mut c_void, old, size); }
    setprop_val(chosen, b"linux,esm-blob-start\0".as_ptr(), addr as u32);
    setprop_val(chosen, b"linux,esm-blob-end\0".as_ptr(), (addr + size) as u32);
}
#[cfg(not(target_arch = "powerpc64"))]
unsafe fn prep_esm_blob(_vmlinux: AddrRange, _chosen: *mut c_void) {}

#[link_section = "__builtin_cmdline"]
static mut CMDLINE: [u8; BOOT_COMMAND_LINE_SIZE] = [0; BOOT_COMMAND_LINE_SIZE];

unsafe fn prep_cmdline(chosen: *mut c_void) {
    let mut timeout = 5000u32; let mut v = 0i32;
    if getprop(chosen, b"linux,cmdline-timeout\0".as_ptr(), &mut v, core::mem::size_of::<i32>()) == core::mem::size_of::<i32>() as i32 { timeout = v as u32; }
    if CMDLINE[0] == 0 { getprop(chosen, b"bootargs\0".as_ptr(), CMDLINE.as_mut_ptr() as *mut i32, BOOT_COMMAND_LINE_SIZE - 1); }
    printf(b"\n\rLinux/PowerPC load: %s\0".as_ptr(), CMDLINE.as_ptr());
    if let Some(f) = console_ops.edit_cmdline { if timeout != 0 { f(CMDLINE.as_mut_ptr(), BOOT_COMMAND_LINE_SIZE, timeout); } }
    printf(b"\n\r\0".as_ptr()); setprop_str(chosen, b"bootargs\0".as_ptr(), CMDLINE.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn start() {
    let mut vmlinux: AddrRange; let mut initrd: AddrRange; let mut ft_addr = 0usize;
    if loader_info.cmdline_len > 0 && CMDLINE[0] == 0 { memmove(CMDLINE.as_mut_ptr() as *mut c_void, loader_info.cmdline as *const c_void, min(loader_info.cmdline_len, BOOT_COMMAND_LINE_SIZE - 1)); }
    if let Some(f) = console_ops.open { if f() < 0 { exit(); } }
    if let Some(f) = platform_ops.fixups { f(); }
    printf(b"\n\rzImage starting: loaded at 0x%p (sp: 0x%p)\n\r\0".as_ptr(), _start, get_sp());
    let mut chosen = finddevice(b"/chosen\0".as_ptr()); if chosen.is_null() { chosen = create_node(core::ptr::null_mut(), b"chosen\0".as_ptr()); }
    vmlinux = prep_kernel(); initrd = prep_initrd(vmlinux, chosen, loader_info.initrd_addr, loader_info.initrd_size); prep_esm_blob(vmlinux, chosen); prep_cmdline(chosen);
    printf(b"Finalizing device tree...\0".as_ptr()); if let Some(f) = dt_ops.finalize { ft_addr = f(); }
    if ft_addr != 0 { printf(b" flat tree at 0x%lx\n\r\0".as_ptr(), ft_addr); } else { printf(b" using OF tree (promptr=%p)\n\r\0".as_ptr(), loader_info.promptr); }
    if let Some(f) = console_ops.close { f(); }
    let kentry: KernelEntry = core::mem::transmute(vmlinux.addr);
    if ft_addr != 0 { if let Some(f) = platform_ops.kentry { f(ft_addr, vmlinux.addr); } else { kentry(ft_addr, 0, core::ptr::null_mut()); } } else { kentry(initrd.addr as usize, initrd.size, loader_info.promptr); }
    fatal(b"Error: Linux kernel returned to zImage boot wrapper!\n\r\0".as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
