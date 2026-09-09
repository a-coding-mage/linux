// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  RouterBoard 500 specific prom routines
 *
 *  Copyright (C) 2003, Peter Sadik <peter.sadik@idt.com>
 *  Copyright (C) 2005-2006, P.Christeas <p_christ@hol.gr>
 *  Copyright (C) 2007, Gabor Juhos <juhosg@openwrt.org>
 *\t\t\tFelix Fietkau <nbd@openwrt.org>
 *\t\t\tFlorian Fainelli <florian@openwrt.org>
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static mut fw_arg0: core::ffi::c_int;
    static mut fw_arg1: usize;
    static mut mips_machtype: core::ffi::c_int;
    static mut arcs_cmdline: *mut core::ffi::c_char;

    fn strlen(s: *const core::ffi::c_char) -> usize;
    fn strncmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char, n: usize) -> core::ffi::c_int;
    fn simple_strtoul(s: *const core::ffi::c_char, end: *mut *mut core::ffi::c_char, base: core::ffi::c_uint) -> core::ffi::c_ulong;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn strscpy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, n: usize) -> isize;
    fn ioremap(start: phys_addr_t, size: phys_addr_t) -> *mut ddr_ram;
    fn printk(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn memblock_add(start: phys_addr_t, size: phys_addr_t) -> core::ffi::c_int;
}

pub type phys_addr_t = usize;

pub const mut_idt_cpu_freq: core::ffi::c_uint = 132000000;
#[no_mangle]
pub static mut idt_cpu_freq: core::ffi::c_uint = 132000000;

#[repr(C)]
pub struct resource {
    pub name: *const core::ffi::c_char,
    pub start: phys_addr_t,
    pub end: phys_addr_t,
    pub flags: core::ffi::c_ulong,
}

#[repr(C)]
pub struct ddr_ram {
    pub ddrbase: u32,
    pub ddrmask: u32,
}

extern "C" {
    static DDR0_PHYS_ADDR: phys_addr_t;
    static IORESOURCE_MEM: core::ffi::c_ulong;
    static COMMAND_LINE_SIZE: usize;
    static FREQ_TAG: *const core::ffi::c_char;
    static MEM_TAG: *const core::ffi::c_char;
    static BOARD_TAG: *const core::ffi::c_char;
    static BOARD_RB532A: *const core::ffi::c_char;
    static MACH_MIKROTIK_RB532A: core::ffi::c_int;
    static MACH_MIKROTIK_RB532: core::ffi::c_int;
}

static mut ddr_reg: [resource; 1] = [resource {
    name: b"ddr-reg\0".as_ptr() as *const core::ffi::c_char,
    start: 0,
    end: 0,
    flags: 0,
}];

#[inline]
unsafe fn match_tag(arg: *mut core::ffi::c_char, tag: *const core::ffi::c_char) -> bool {
    strncmp(arg, tag, strlen(tag)) == 0
}

#[inline]
unsafe fn tag2ul(arg: *mut core::ffi::c_char, tag: *const core::ffi::c_char) -> core::ffi::c_ulong {
    let num = arg.add(strlen(tag));
    simple_strtoul(num, core::ptr::null_mut(), 10)
}

unsafe fn prom_setup_cmdline() {
    static mut cmd_line: [core::ffi::c_char; 512] = [0; 512];
    let mut cp = cmd_line.as_mut_ptr();
    let prom_argc = fw_arg0;
    let prom_argv = fw_arg1 as *mut *mut core::ffi::c_char;

    /* Note: it is common that parameters start
     * at argv[1] and not argv[0],
     * however, our elf loader starts at [0] */
    for i in 0..prom_argc {
        let arg = *prom_argv.add(i as usize);
        if match_tag(arg, FREQ_TAG) {
            idt_cpu_freq = tag2ul(arg, FREQ_TAG) as core::ffi::c_uint;
            continue;
        }
        // IGNORE_CMDLINE_MEM is a build-time condition from the C source.
        if i > 0 {
            *cp = b' ' as core::ffi::c_char;
            cp = cp.add(1);
        }
        if match_tag(arg, BOARD_TAG) {
            let board = arg.add(strlen(BOARD_TAG));
            if match_tag(board, BOARD_RB532A) {
                mips_machtype = MACH_MIKROTIK_RB532A;
            } else {
                mips_machtype = MACH_MIKROTIK_RB532;
            }
        }
        let len = strlen(arg);
        memcpy(cp as *mut core::ffi::c_void, arg as *const core::ffi::c_void, len + 1);
        cp = cp.add(len);
    }
    *cp = b' ' as core::ffi::c_char;
    cp = cp.add(1);

    let len = strlen(arcs_cmdline);
    if len > 0 {
        *cp = b' ' as core::ffi::c_char;
        cp = cp.add(1);
        memcpy(cp as *mut core::ffi::c_void, arcs_cmdline as *const core::ffi::c_void, len + 1);
        cp = cp.add(len);
    }
    cmd_line[511] = 0;
    strscpy(arcs_cmdline, cmd_line.as_ptr(), 512);
}

#[no_mangle]
pub unsafe extern "C" fn prom_init() {
    let ddr = ioremap(ddr_reg[0].start, ddr_reg[0].end - ddr_reg[0].start);
    if ddr.is_null() {
        printk(b"Unable to remap DDR register\n\0".as_ptr() as *const core::ffi::c_char);
        return;
    }
    let ddrbase = &(*ddr).ddrbase as *const _ as phys_addr_t;
    let mut memsize = &(*ddr).ddrmask as *const _ as phys_addr_t;
    memsize = 0usize.wrapping_sub(memsize);
    prom_setup_cmdline();
    /* give all RAM to boot allocator,
     * except for the first 0x400 and the last 0x200 bytes */
    memblock_add(ddrbase.wrapping_add(0x400), memsize.wrapping_sub(0x600));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
