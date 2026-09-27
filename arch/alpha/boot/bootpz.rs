// SPDX-License-Identifier: GPL-2.0
/*
 * arch/alpha/boot/bootpz.c
 *
 * Copyright (C) 1997 Jay Estabrook
 *
 * This file is used for creating a compressed BOOTP file for the
 * Linux/AXP kernel
 *
 * based significantly on the arch/alpha/boot/main.c of Linus Torvalds
 * and the decompression code from MILO.
 */

// Dependencies supplied by the surrounding kernel build are intentionally
// left as external Rust items.

/* FIXME FIXME FIXME */
pub const MALLOC_AREA_SIZE: usize = 0x200000; /* 2MB for now */
/* FIXME FIXME FIXME */

/*
  WARNING NOTE

  It is very possible that turning on additional messages may cause
  kernel image corruption due to stack usage to do the printing.
*/

extern "C" {
    fn switch_to_osf_pal(nr: core::ffi::c_ulong, pcb_va: *mut pcb_struct,
                         pcb_pa: *mut pcb_struct, vptb: *mut core::ffi::c_ulong) -> core::ffi::c_ulong;
    fn decompress_kernel(destination: *mut core::ffi::c_void, source: *mut core::ffi::c_void,
                         ksize: usize, kzsize: usize) -> core::ffi::c_int;
    fn move_stack(new_stack: core::ffi::c_ulong);
    fn srm_printk(fmt: *const core::ffi::c_char, ...);
    fn __halt() -> !;
    fn tbia();
    fn callback_getenv(name: core::ffi::c_int, buf: *mut core::ffi::c_char, len: usize) -> core::ffi::c_long;
}

#[repr(C)]
pub struct pcb_struct {
    pub ksp: core::ffi::c_ulong,
    pub usp: core::ffi::c_ulong,
    pub ptbr: core::ffi::c_ulong,
    pub asn: core::ffi::c_ulong,
    pub pcc: core::ffi::c_ulong,
    pub unique: core::ffi::c_ulong,
    pub flags: core::ffi::c_ulong,
    pub res1: core::ffi::c_ulong,
    pub res2: core::ffi::c_ulong,
}

#[repr(C)]
pub struct percpu_struct {
    pub processor_offset: core::ffi::c_ulong,
    pub pal_revision: core::ffi::c_ulong,
    pub palcode_avail: [core::ffi::c_ulong; 3],
}

extern "C" {
    static mut hwrpb: *mut hwrpb_struct;
    static mut SP_on_entry: core::ffi::c_ulong;
    static _end: core::ffi::c_char;
}

#[repr(C)]
pub struct hwrpb_struct {
    pub pagesize: core::ffi::c_ulong,
    pub vptb: core::ffi::c_ulong,
    pub processor_offset: core::ffi::c_ulong,
}

pub const VPTB: *mut core::ffi::c_ulong = 0x200000000usize as *mut core::ffi::c_ulong;
pub const L1: *mut core::ffi::c_ulong = 0x200802000usize as *mut core::ffi::c_ulong;

#[inline]
pub unsafe fn find_pa(address: core::ffi::c_ulong) -> core::ffi::c_ulong {
    let mut result = *VPTB.add((address >> 13) as usize);
    result >>= 32;
    result <<= 13;
    result |= address & 0x1fff;
    result
}

pub unsafe fn check_range(vstart: core::ffi::c_ulong, vend: core::ffi::c_ulong,
                          kstart: core::ffi::c_ulong, kend: core::ffi::c_ulong) -> core::ffi::c_int {
    let mut vaddr = vstart;
    while vaddr <= vend {
        let kaddr = find_pa(vaddr) | PAGE_OFFSET;
        if kaddr >= kstart && kaddr <= kend { return 1; }
        vaddr = vaddr.wrapping_add(PAGE_SIZE);
    }
    0
}

pub unsafe fn pal_init() {
    static mut PCB_VA: [pcb_struct; 1] = [pcb_struct {
        ksp: 0, usp: 0, ptbr: 0, asn: 0, pcc: 0, unique: 0,
        flags: 0, res1: 0, res2: 0,
    }];
    let pcb_va = &mut PCB_VA[0];
    pcb_va.ksp = 0;
    pcb_va.usp = 0;
    pcb_va.ptbr = *L1.add(1) >> 32;
    pcb_va.asn = 0;
    pcb_va.pcc = 0;
    pcb_va.unique = 0;
    pcb_va.flags = 1;
    pcb_va.res1 = 0;
    pcb_va.res2 = 0;
    let pcb_pa = find_pa(pcb_va as *mut pcb_struct as core::ffi::c_ulong) as *mut pcb_struct;
    srm_printk(b"Switching to OSF PAL-code... \0".as_ptr() as *const core::ffi::c_char);
    let i = switch_to_osf_pal(2, pcb_va, pcb_pa, VPTB);
    if i != 0 {
        srm_printk(b"failed, code %ld\n\0".as_ptr() as *const core::ffi::c_char, i);
        __halt();
    }
    let percpu = ((*hwrpb).processor_offset + hwrpb as core::ffi::c_ulong)
        as *mut percpu_struct;
    (*percpu).pal_revision = (*percpu).palcode_avail[2];
    srm_printk(b"OK (rev %lx)\n\0".as_ptr() as *const core::ffi::c_char, (*percpu).pal_revision);
    tbia();
}

#[inline(never)]
pub unsafe fn runkernel() {
    core::arch::asm!("bis {0},{0},$27", "jmp ($27)", in(reg) START_ADDR, options(noreturn));
}

pub unsafe fn start_kernel() {
    let mut must_move = 0;
    let mut uncompressed_image_start = START_ADDR;
    let mut uncompressed_image_end = START_ADDR + KERNEL_SIZE;
    let mut initrd_image_start = NEXT_PAGE(uncompressed_image_end) + KERNEL_SIZE + PAGE_SIZE;
    let mut envval = [0i8; 256];
    let nbytes = callback_getenv(ENV_BOOTED_OSFLAGS, envval.as_mut_ptr(), envval.len());
    let nbytes = if nbytes < 0 || nbytes >= envval.len() as core::ffi::c_long { 0 } else { nbytes as usize };
    envval[nbytes] = 0;
    if (*INIT_HWRPB).pagesize != 8192 || (*INIT_HWRPB).vptb != VPTB as core::ffi::c_ulong { return; }
    pal_init();
    if check_range(BOOT_ADDR, (&_end as *const _ as core::ffi::c_ulong + 511) & !511,
                   ZERO_PGE, START_ADDR + KERNEL_SIZE) != 0 { __halt(); }
    if check_range(KERNEL_ORIGIN, SP_on_entry, START_ADDR, K_INITRD_START + REAL_INITRD_SIZE + MALLOC_AREA_SIZE) != 0 {
        uncompressed_image_start = NEXT_PAGE(START_ADDR + KERNEL_SIZE);
        uncompressed_image_end = K_COPY_IMAGE_END;
        must_move = 1;
        while check_range(KERNEL_ORIGIN, SP_on_entry, uncompressed_image_start, uncompressed_image_end) != 0 {
            uncompressed_image_start = uncompressed_image_start.wrapping_add(PAGE_SIZE);
            uncompressed_image_end = uncompressed_image_end.wrapping_add(PAGE_SIZE);
            initrd_image_start = initrd_image_start.wrapping_add(PAGE_SIZE);
        }
    }
    decompress_kernel(uncompressed_image_start as *mut _, KERNEL_ORIGIN as *mut _, KERNEL_SIZE, KERNEL_Z_SIZE);
    if must_move != 0 {
        move_stack(initrd_image_start - PAGE_SIZE);
        core::ptr::copy_nonoverlapping(uncompressed_image_start as *const u8, START_ADDR as *mut u8, KERNEL_SIZE);
    }
    core::ptr::write_bytes(ZERO_PGE as *mut u8, 0, PAGE_SIZE);
    core::ptr::copy_nonoverlapping(envval.as_ptr() as *const u8, ZERO_PGE as *mut u8, nbytes + 1);
    runkernel();
}

pub unsafe fn __kmalloc(_size: usize, _flags: gfp_t) -> *mut core::ffi::c_void { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
