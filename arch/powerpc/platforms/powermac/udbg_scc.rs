// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * udbg for zilog scc ports as found on Apple PowerMacs
 *
 * Copyright (C) 2001-2005 PPC 64 Team, IBM Corp
 */

// External declarations supplied by the surrounding kernel translation.
extern "C" {
    fn real_readb(addr: *const u8) -> u8;
    fn real_writeb(data: u8, addr: *mut u8);
    fn in_8(addr: *const u8) -> u8;
    fn out_8(addr: *mut u8, data: u8);
    fn ioremap(addr: usize, size: usize) -> *mut u8;
    fn mb();
    fn udbg_puts(s: *const u8);
}

type U8IoMem = u8;

const SCC_TXRDY: u8 = 4;
const SCC_RXRDY: u8 = 1;

static mut sccc: *mut U8IoMem = core::ptr::null_mut();
static mut sccd: *mut U8IoMem = core::ptr::null_mut();

extern "C" {
    static mut udbg_putc: Option<unsafe extern "C" fn(c: i8)>;
    static mut udbg_getc: Option<unsafe extern "C" fn() -> i32>;
    static mut udbg_getc_poll: Option<unsafe extern "C" fn() -> i32>;
}

unsafe extern "C" fn udbg_scc_putc(c: i8) {
    if !sccc.is_null() {
        while (in_8(sccc) & SCC_TXRDY) == 0 {}
        out_8(sccd, c as u8);
        if c == b'\n' as i8 {
            udbg_scc_putc(b'\r' as i8);
        }
    }
}

unsafe extern "C" fn udbg_scc_getc_poll() -> i32 {
    if !sccc.is_null() {
        if (in_8(sccc) & SCC_RXRDY) != 0 {
            return in_8(sccd) as i32;
        } else {
            return -1;
        }
    }
    -1
}

unsafe extern "C" fn udbg_scc_getc() -> i32 {
    if !sccc.is_null() {
        while (in_8(sccc) & SCC_RXRDY) == 0 {}
        return in_8(sccd) as i32;
    }
    -1
}

static mut scc_inittab: [u8; 14] = [
    13, 0,       /* set baud rate divisor */
    12, 0,
    14, 1,       /* baud rate gen enable, src=rtxc */
    11, 0x50,    /* clocks = br gen */
    5,  0xea,    /* tx 8 bits, assert DTR & RTS */
    4,  0x46,    /* x16 clock, 1 stop */
    3,  0xc1,    /* rx enable, 8 bits */
];

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

const PAGE_MASK: usize = !((4096usize) - 1);
const PAGE_SIZE: usize = 4096;

unsafe extern "C" fn of_find_node_by_name(node: *mut DeviceNode, name: *const u8) -> *mut DeviceNode;
unsafe extern "C" fn of_get_parent(node: *mut DeviceNode) -> *mut DeviceNode;
unsafe extern "C" fn of_get_property(node: *mut DeviceNode, name: *const u8, len: *mut usize) -> *const u32;
unsafe extern "C" fn of_find_node_by_path(path: *const u8) -> *mut DeviceNode;
unsafe extern "C" fn of_node_put(node: *mut DeviceNode);
unsafe extern "C" fn of_node_get(node: *mut DeviceNode) -> *mut DeviceNode;
unsafe extern "C" fn of_node_name_eq(node: *mut DeviceNode, name: *const u8) -> bool;
unsafe extern "C" fn pmac_call_feature(feature: u32, node: *mut DeviceNode, flags: u32, value: u32);
unsafe extern "C" fn of_machine_is_compatible(name: *const u8) -> bool;

static mut of_chosen: *mut DeviceNode = core::ptr::null_mut();
const PMAC_FTR_SCC_ENABLE: u32 = 0;
const PMAC_SCC_ASYNC: u32 = 0;
const PMAC_SCC_FLAG_XMON: u32 = 0;

pub unsafe extern "C" fn udbg_scc_init(force_scc: i32) {
    let mut reg: *const u32;
    let mut addr: usize;
    let mut stdout: *mut DeviceNode = core::ptr::null_mut();
    let mut escc: *mut DeviceNode = core::ptr::null_mut();
    let mut macio: *mut DeviceNode = core::ptr::null_mut();
    let mut ch: *mut DeviceNode;
    let mut ch_def: *mut DeviceNode = core::ptr::null_mut();
    let mut ch_a: *mut DeviceNode = core::ptr::null_mut();

    escc = of_find_node_by_name(core::ptr::null_mut(), b"escc\0".as_ptr());
    if escc.is_null() { goto_bail(macio, escc, stdout, ch_def, ch_a); return; }
    macio = of_get_parent(escc);
    if macio.is_null() { goto_bail(macio, escc, stdout, ch_def, ch_a); return; }
    let path = of_get_property(of_chosen, b"linux,stdout-path\0".as_ptr(), core::ptr::null_mut()) as *const u8;
    if !path.is_null() { stdout = of_find_node_by_path(path); }

    // for_each_child_of_node(escc, ch)
    ch = core::ptr::null_mut();
    while !ch.is_null() {
        if ch == stdout { of_node_put(ch_def); ch_def = of_node_get(ch); }
        if of_node_name_eq(ch, b"ch-a\0".as_ptr()) { of_node_put(ch_a); ch_a = of_node_get(ch); }
        break;
    }
    if ch_def.is_null() && force_scc == 0 { goto_bail(macio, escc, stdout, ch_def, ch_a); return; }
    ch = if !ch_def.is_null() { ch_def } else { ch_a };
    reg = of_get_property(escc, b"reg\0".as_ptr(), core::ptr::null_mut());
    if reg.is_null() { goto_bail(macio, escc, stdout, ch_def, ch_a); return; }
    addr = *reg as usize;
    reg = of_get_property(macio, b"assigned-addresses\0".as_ptr(), core::ptr::null_mut());
    if reg.is_null() { goto_bail(macio, escc, stdout, ch_def, ch_a); return; }
    addr = addr.wrapping_add(*reg.add(2) as usize);
    pmac_call_feature(PMAC_FTR_SCC_ENABLE, ch, PMAC_SCC_ASYNC | PMAC_SCC_FLAG_XMON, 1);
    if ch == ch_a { addr = addr.wrapping_add(0x20); }
    sccc = ioremap(addr & PAGE_MASK, PAGE_SIZE).add(addr & !PAGE_MASK);
    sccd = sccc.add(0x10);
    mb();
    let mut i = 20000;
    while i != 0 { in_8(sccc); i -= 1; }
    out_8(sccc, 0x09); out_8(sccc, 0xc0);
    if !ch_def.is_null() {
        out_8(sccc, 13); scc_inittab[1] = in_8(sccc);
        out_8(sccc, 12); scc_inittab[3] = in_8(sccc);
    } else if of_machine_is_compatible(b"RackMac1,1\0".as_ptr()) || of_machine_is_compatible(b"RackMac1,2\0".as_ptr()) || of_machine_is_compatible(b"MacRISC4\0".as_ptr()) {
        scc_inittab[1] = 0; scc_inittab[3] = 0;
    } else { scc_inittab[1] = 0; scc_inittab[3] = 1; }
    i = 0; while i < scc_inittab.len() { out_8(sccc, scc_inittab[i]); i += 1; }
    udbg_putc = Some(udbg_scc_putc); udbg_getc = Some(udbg_scc_getc); udbg_getc_poll = Some(udbg_scc_getc_poll);
    udbg_puts(b"Hello World !\n\0".as_ptr());
    goto_bail(macio, escc, stdout, ch_def, ch_a);
}

unsafe fn goto_bail(macio: *mut DeviceNode, escc: *mut DeviceNode, stdout: *mut DeviceNode, ch_def: *mut DeviceNode, ch_a: *mut DeviceNode) {
    of_node_put(macio); of_node_put(escc); of_node_put(stdout); of_node_put(ch_def); of_node_put(ch_a);
}

// CONFIG_PPC64
unsafe extern "C" fn udbg_real_scc_putc(c: i8) {
    while (real_readb(sccc) & SCC_TXRDY) == 0 {}
    real_writeb(c as u8, sccd);
    if c == b'\n' as i8 { udbg_real_scc_putc(b'\r' as i8); }
}

pub unsafe extern "C" fn udbg_init_pmac_realmode() {
    sccc = 0x80013020usize as *mut U8IoMem;
    sccd = 0x80013030usize as *mut U8IoMem;
    udbg_putc = Some(udbg_real_scc_putc);
    udbg_getc = None;
    udbg_getc_poll = None;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
