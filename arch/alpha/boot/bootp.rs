// SPDX-License-Identifier: GPL-2.0
/*
 * arch/alpha/boot/bootp.c
 *
 * Copyright (C) 1997 Jay Estabrook
 *
 * This file is used for creating a bootp file for the Linux/AXP kernel
 *
 * based significantly on the arch/alpha/boot/main.c of Linus Torvalds
 */

// C dependencies supplied by the surrounding kernel build are intentionally
// left as external Rust declarations.

extern "C" {
    static mut INIT_HWRPB: *mut hwrpb_struct;
    static mut START_ADDR: usize;
    static mut KERNEL_SIZE: usize;
    static mut PAGE_SIZE: usize;
    static mut ZERO_PGE: usize;
    static mut UTS_RELEASE: *const core::ffi::c_char;
    fn switch_to_osf_pal(
        nr: usize,
        pcb_va: *mut pcb_struct,
        pcb_pa: *mut pcb_struct,
        vptb: *mut usize,
    ) -> usize;
    fn move_stack(new_stack: usize);
    fn srm_printk(fmt: *const core::ffi::c_char, ...);
    fn __halt() -> !;
    fn tbia();
    fn callback_getenv(
        name: *const core::ffi::c_char,
        value: *mut core::ffi::c_char,
        size: usize,
    ) -> isize;
    static mut ENV_BOOTED_OSFLAGS: *const core::ffi::c_char;
    static mut _end: core::ffi::c_char;
}

#[repr(C)]
pub struct pcb_struct {
    pub ksp: usize,
    pub usp: usize,
    pub ptbr: usize,
    pub asn: usize,
    pub pcc: usize,
    pub unique: usize,
    pub flags: usize,
    pub res1: usize,
    pub res2: usize,
}

#[repr(C)]
pub struct percpu_struct {
    pub pal_revision: usize,
    pub palcode_avail: [usize; 3],
}

#[repr(C)]
pub struct hwrpb_struct {
    pub pagesize: usize,
    pub vptb: usize,
    pub processor_offset: usize,
}

static mut pcb_va: [pcb_struct; 1] = [pcb_struct {
    ksp: 0, usp: 0, ptbr: 0, asn: 0, pcc: 0, unique: 0, flags: 0, res1: 0, res2: 0,
}];

#[inline]
unsafe fn find_pa(vptb: *mut usize, ptr: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let address = ptr as usize;
    let mut result = *vptb.add(address >> 13);
    result >>= 32;
    result <<= 13;
    result |= address & 0x1fff;
    result as *mut core::ffi::c_void
}

const VPTB: *mut usize = 0x200000000usize as *mut usize;
const L1: *mut usize = 0x200802000usize as *mut usize;

pub unsafe fn pal_init() {
    let mut i: usize;
    let rev: usize;
    let percpu: *mut percpu_struct;
    let pcb_pa: *mut pcb_struct;

    pcb_va[0].ksp = 0;
    pcb_va[0].usp = 0;
    pcb_va[0].ptbr = *L1.add(1) >> 32;
    pcb_va[0].asn = 0;
    pcb_va[0].pcc = 0;
    pcb_va[0].unique = 0;
    pcb_va[0].flags = 1;
    pcb_va[0].res1 = 0;
    pcb_va[0].res2 = 0;
    pcb_pa = find_pa(VPTB, pcb_va.as_mut_ptr() as *mut core::ffi::c_void) as *mut pcb_struct;

    // a0 = 2 (OSF); a1 is the PCB virtual address; a2 is its physical address;
    // a3 is the new virtual page table pointer; a4 is KSP (set by the assembly).
    srm_printk(b"Switching to OSF PAL-code .. \\0".as_ptr() as *const _);
    i = switch_to_osf_pal(2, pcb_va.as_mut_ptr(), pcb_pa, VPTB);
    if i != 0 {
        srm_printk(b"failed, code %ld\\n\\0".as_ptr() as *const _, i);
        __halt();
    }

    percpu = ((*INIT_HWRPB).processor_offset + INIT_HWRPB as usize) as *mut percpu_struct;
    rev = (*percpu).palcode_avail[2];
    (*percpu).pal_revision = rev;
    srm_printk(b"Ok (rev %lx)\\n\\0".as_ptr() as *const _, rev);
    tbia();
}

#[inline]
unsafe fn load(dst: usize, src: usize, count: usize) {
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, count);
}

#[inline(never)]
unsafe fn runkernel() -> ! {
    // Alpha assembly equivalent of the original non-returning jump.
    core::arch::asm!("bis {0},{0},$27", "jmp ($27)", in(reg) START_ADDR, options(noreturn));
}

pub unsafe fn start_kernel() {
    static mut nbytes: isize = 0;
    #[repr(align(8))]
    struct AlignedEnv([u8; 256]);
    static mut envval: AlignedEnv = AlignedEnv([0; 256]);
    static mut initrd_start: usize = 0;

    srm_printk(b"Linux/AXP bootp loader for Linux %s\\n\\0".as_ptr() as *const _, UTS_RELEASE);
    if (*INIT_HWRPB).pagesize != 8192 {
        srm_printk(b"Expected 8kB pages, got %ldkB\\n\\0".as_ptr() as *const _, (*INIT_HWRPB).pagesize >> 10);
        return;
    }
    if (*INIT_HWRPB).vptb != VPTB as usize {
        srm_printk(b"Expected vptb at %p, got %p\\n\\0".as_ptr() as *const _, VPTB, (*INIT_HWRPB).vptb as *mut core::ffi::c_void);
        return;
    }
    pal_init();

    initrd_start = ((START_ADDR + 5 * KERNEL_SIZE + PAGE_SIZE) | (PAGE_SIZE - 1)) + 1;
    #[cfg(feature = "INITRD_IMAGE_SIZE")]
    {
        extern "C" { static INITRD_IMAGE_SIZE: usize; }
        srm_printk(b"Initrd positioned at %#lx\n\0".as_ptr() as *const _, initrd_start);
        load(initrd_start, ((&_end as *const _ as usize + 511) & !511) + KERNEL_SIZE, INITRD_IMAGE_SIZE);
    }

    move_stack(initrd_start - PAGE_SIZE);
    nbytes = callback_getenv(ENV_BOOTED_OSFLAGS, envval.0.as_mut_ptr() as *mut _, envval.0.len());
    if nbytes < 0 || nbytes >= envval.0.len() as isize { nbytes = 0; }
    envval.0[nbytes as usize] = 0;
    srm_printk(b"Loading the kernel...'%s'\\n\\0".as_ptr() as *const _, envval.0.as_ptr());

    load(START_ADDR + 4 * KERNEL_SIZE, (&_end as *const _ as usize + 511) & !511, KERNEL_SIZE);
    load(START_ADDR, START_ADDR + 4 * KERNEL_SIZE, KERNEL_SIZE);
    core::ptr::write_bytes(ZERO_PGE as *mut u8, 0, PAGE_SIZE);
    core::ptr::copy_nonoverlapping(envval.0.as_ptr(), ZERO_PGE as *mut u8, envval.0.len());
    #[cfg(feature = "INITRD_IMAGE_SIZE")]
    {
        extern "C" { static INITRD_IMAGE_SIZE: usize; }
        (ZERO_PGE as *mut usize).add(256 / core::mem::size_of::<usize>()).write(initrd_start);
        (ZERO_PGE as *mut usize).add(256 / core::mem::size_of::<usize>() + 1).write(INITRD_IMAGE_SIZE);
    }
    runkernel();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
