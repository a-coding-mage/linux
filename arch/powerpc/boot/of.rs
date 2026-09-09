// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) Paul Mackerras 1997.
 */

// Declarations and constants supplied by the corresponding platform headers.

const PROG_START: usize = 0x0140_0000; // only used on 64-bit systems
const RAM_END: usize = 512usize << 20; // FIXME: use OF
const ONE_MB: usize = 0x100000;

static mut claim_base: usize = 0;

extern "C" {
    fn epapr_platform_init(
        r3: usize,
        r4: usize,
        r5: usize,
        r6: usize,
        r7: usize,
    );

    fn of_claim(virt: usize, size: usize, align: usize) -> usize;
    fn of_exit();
    fn of_vmlinux_alloc() -> *mut core::ffi::c_void;
    fn of_finddevice(name: *const core::ffi::c_char) -> i32;
    fn of_getprop(
        node: i32,
        prop: *const core::ffi::c_char,
        buf: *mut core::ffi::c_void,
        buflen: i32,
    ) -> i32;
    fn of_setprop(
        node: i32,
        prop: *const core::ffi::c_char,
        buf: *const core::ffi::c_void,
        buflen: i32,
    ) -> i32;
    fn of_console_init();
    fn of_init(promptr: *mut core::ffi::c_void);
}

extern "C" {
    static mut _end: u8;
    static mut platform_ops: PlatformOps;
    static mut dt_ops: DtOps;
    static mut loader_info: LoaderInfo;
}

unsafe fn of_try_claim(size: usize) -> *mut core::ffi::c_void {
    let mut addr: usize = 0;

    if claim_base == 0 {
        claim_base = ((&_end as *const u8 as usize) + ONE_MB - 1) & !(ONE_MB - 1);
    }

    while claim_base < RAM_END {
        // #ifdef DEBUG: the source prints the candidate address here.
        addr = of_claim(claim_base, size, 0);
        if addr != PROM_ERROR as usize {
            break;
        }
        claim_base = claim_base.wrapping_add(ONE_MB);
    }
    if addr == 0 {
        return core::ptr::null_mut();
    }
    claim_base = (claim_base.wrapping_add(size) + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    addr as *mut core::ffi::c_void
}

unsafe fn of_image_hdr(hdr: *const core::ffi::c_void) {
    let elf64 = hdr as *const Elf64_Ehdr;

    if (*elf64).e_ident[EI_CLASS] == ELFCLASS64 {
        /*
         * Maintain a "magic" minimum address. This keeps some older
         * firmware platforms running.
         */
        if claim_base < PROG_START {
            claim_base = PROG_START;
        }
    }
}

unsafe fn of_platform_init(a1: usize, a2: usize, promptr: *mut core::ffi::c_void) {
    platform_ops.image_hdr = Some(of_image_hdr);
    platform_ops.malloc = Some(of_try_claim);
    platform_ops.exit = Some(of_exit);
    platform_ops.vmlinux_alloc = Some(of_vmlinux_alloc);

    dt_ops.finddevice = Some(of_finddevice);
    dt_ops.getprop = Some(of_getprop);
    dt_ops.setprop = Some(of_setprop);

    of_console_init();

    of_init(promptr);
    loader_info.promptr = promptr;
    if a1 != 0 && a2 != 0 && a2 != 0xdeadbeef {
        loader_info.initrd_addr = a1;
        loader_info.initrd_size = a2;
    }
}

#[no_mangle]
pub unsafe extern "C" fn platform_init(
    r3: usize,
    r4: usize,
    r5: usize,
    r6: usize,
    r7: usize,
) {
    /* Detect OF vs. ePAPR boot */
    if r5 != 0 {
        of_platform_init(r3, r4, r5 as *mut core::ffi::c_void);
    } else {
        epapr_platform_init(r3, r4, r5, r6, r7);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
