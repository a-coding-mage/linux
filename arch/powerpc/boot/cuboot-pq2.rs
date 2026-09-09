// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for PowerQUICC II
 * (a.k.a. 82xx with CPM, not the 8240 family of chips)
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

use core::mem::size_of;

type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type c_int = core::ffi::c_int;

#[repr(C)]
struct bd_t {
    bi_memstart: usize,
    bi_memsize: usize,
    bi_enetaddr: [u8; 6],
    bi_enet1addr: [u8; 6],
    bi_intfreq: usize,
    bi_busfreq: usize,
    bi_cpmfreq: usize,
    bi_brgfreq: usize,
}

#[repr(C)]
struct cs_range {
    csnum: u32,
    base: u32, /* must be zero */
    addr: u32,
    size: u32,
}

#[repr(C)]
struct pci_range {
    flags: u32,
    pci_addr: [u32; 2],
    phys_addr: u32,
    size: [u32; 2],
}

extern "C" {
    static mut bd: bd_t;
    static _dtb_start: core::ffi::c_uchar;
    static platform_ops: platform_ops;

    fn finddevice(path: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn dt_is_compatible(node: *mut core::ffi::c_void, compat: *const core::ffi::c_char) -> c_int;
    fn dt_get_reg_format(node: *mut core::ffi::c_void, naddr: *mut u32, nsize: *mut u32);
    fn get_parent(node: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn dt_xlate_reg(node: *mut core::ffi::c_void, index: c_int, addr: *mut usize, size: *mut usize) -> c_int;
    fn getprop(node: *mut core::ffi::c_void, name: *const core::ffi::c_char, buf: *mut core::ffi::c_void, len: usize) -> c_int;
    fn in_be32(addr: *const u32) -> u32;
    fn out_be32(addr: *mut u32, value: u32);
    fn in_le32(addr: *const u32) -> u32;
    fn out_le32(addr: *mut u32, value: u32);
    fn out_8(addr: *mut u8, value: u8);
    fn fsl_get_immr() -> *mut core::ffi::c_void;
    fn __ilog2_u32(value: u32) -> u32;
    fn udelay(value: u32);
    fn printf(format: *const core::ffi::c_char, ...);
    fn dt_fixup_memory(start: usize, size: usize);
    fn dt_fixup_mac_addresses(addr0: *const u8, addr1: *const u8);
    fn dt_fixup_cpu_clocks(intfreq: usize, busfreq_div4: usize, busfreq: usize);
    fn setprop(node: *mut core::ffi::c_void, name: *const core::ffi::c_char, value: *const core::ffi::c_void, len: usize);
    fn fdt_init(dtb: *const core::ffi::c_uchar);
    fn serial_console_init();
    fn cuboot_init();
}

#[repr(C)]
struct platform_ops {
    fixups: Option<unsafe extern "C" fn()>,
}

const MAX_PROP_LEN: usize = 1024;

static mut cs_ranges_buf: [cs_range; MAX_PROP_LEN / size_of::<cs_range>()] =
    unsafe { core::mem::zeroed() };
static mut pci_ranges_buf: [pci_range; MAX_PROP_LEN / size_of::<pci_range>()] =
    unsafe { core::mem::zeroed() };

unsafe fn update_cs_ranges() {
    let bus_node = finddevice(b"/localbus\0".as_ptr() as *const _);
    if bus_node.is_null() || dt_is_compatible(bus_node, b"fsl,pq2-localbus\0".as_ptr() as *const _) == 0 { return; }
    let (mut naddr, mut nsize) = (0, 0);
    dt_get_reg_format(bus_node, &mut naddr, &mut nsize);
    if naddr != 2 || nsize != 1 { return bad_localbus(); }
    let parent_node = get_parent(bus_node);
    if parent_node.is_null() { return bad_localbus(); }
    dt_get_reg_format(parent_node, &mut naddr, &mut nsize);
    if naddr != 1 || nsize != 1 { return bad_localbus(); }
    let mut ctrl_addr: *mut u32 = core::ptr::null_mut();
    let mut ctrl_size = 0usize;
    if dt_xlate_reg(bus_node, 0, &mut ctrl_addr as *mut _ as *mut usize, &mut ctrl_size) == 0 { return bad_localbus(); }
    let len = getprop(bus_node, b"ranges\0".as_ptr() as *const _, cs_ranges_buf.as_mut_ptr() as *mut _, size_of::<[cs_range; MAX_PROP_LEN / size_of::<cs_range>()]>());
    for i in 0..(len as usize / size_of::<cs_range>()) {
        let cs = cs_ranges_buf[i].csnum as usize;
        if cs >= ctrl_size / 8 || cs_ranges_buf[i].base != 0 { return bad_localbus(); }
        let mut base = in_be32(ctrl_addr.add(cs * 2));
        let option;
        if base & 1 != 0 { base &= 0x7fff; option = in_be32(ctrl_addr.add(cs * 2 + 1)) & 0x7fff; }
        else { base = 0x1801; option = 0x10; }
        out_be32(ctrl_addr.add(cs * 2), 0);
        out_be32(ctrl_addr.add(cs * 2 + 1), option | !(cs_ranges_buf[i].size - 1));
        out_be32(ctrl_addr.add(cs * 2), base | cs_ranges_buf[i].addr);
    }
    return;
    unsafe fn bad_localbus() { printf(b"Bad /localbus node\r\n\0".as_ptr() as *const _); }
}

unsafe fn fixup_pci() {
    /* Older u-boots don't set PCI up properly; update hardware from the device tree. */
    let node = finddevice(b"/pci\0".as_ptr() as *const _);
    if node.is_null() || dt_is_compatible(node, b"fsl,pq2-pci\0".as_ptr() as *const _) == 0 { return; }
    let mut pci_regs = [core::ptr::null_mut::<u32>(); 3];
    for i in 0..3 { if dt_xlate_reg(node, i, &mut pci_regs[i] as *mut _ as *mut usize, core::ptr::null_mut()) == 0 { printf(b"Bad PCI node -- using existing firmware setup.\r\n\0".as_ptr() as *const _); return; } }
    let soc_regs = fsl_get_immr() as *mut u8;
    if soc_regs.is_null() { printf(b"Unsupported PCI node -- using existing firmware setup.\r\n\0".as_ptr() as *const _); return; }
    let (mut naddr, mut nsize) = (0, 0);
    dt_get_reg_format(node, &mut naddr, &mut nsize);
    if naddr != 3 || nsize != 2 { printf(b"Bad PCI node -- using existing firmware setup.\r\n\0".as_ptr() as *const _); return; }
    let parent = get_parent(node);
    if parent.is_null() { printf(b"Bad PCI node -- using existing firmware setup.\r\n\0".as_ptr() as *const _); return; }
    dt_get_reg_format(parent, &mut naddr, &mut nsize);
    if naddr != 1 || nsize != 1 { printf(b"Unsupported PCI node -- using existing firmware setup.\r\n\0".as_ptr() as *const _); return; }
    let len = getprop(node, b"ranges\0".as_ptr() as *const _, pci_ranges_buf.as_mut_ptr() as *mut _, size_of::<[pci_range; MAX_PROP_LEN / size_of::<pci_range>()]>());
    let (mut mem, mut mmio, mut io): (*mut pci_range, *mut pci_range, *mut pci_range) = (core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    for i in 0..(len as usize / size_of::<pci_range>()) { let p = &mut pci_ranges_buf[i]; match p.flags & 0x43000000 { 0x42000000 => mem = p, 0x02000000 => mmio = p, 0x01000000 => io = p, _ => {} } }
    if mem.is_null() || mmio.is_null() || io.is_null() || (*mem).size[1] != (*mmio).size[1] || ((*mem).size[1] & ((*mem).size[1] - 1)) != 0 || ((*io).size[1] & ((*io).size[1] - 1)) != 0 { printf(b"Unsupported PCI node -- using existing firmware setup.\r\n\0".as_ptr() as *const _); return; }
    let mem_base = if (*mem).phys_addr.wrapping_add((*mem).size[1]) == (*mmio).phys_addr { mem } else if (*mmio).phys_addr.wrapping_add((*mmio).size[1]) == (*mem).phys_addr { mmio } else { printf(b"Unsupported PCI node -- using existing firmware setup.\r\n\0".as_ptr() as *const _); return; };
    out_be32(pci_regs[1], (*mem_base).phys_addr | 1); out_be32(pci_regs[2], !((*mem).size[1].wrapping_add((*mmio).size[1]).wrapping_sub(1)));
    out_be32(pci_regs[1].add(1), (*io).phys_addr | 1); out_be32(pci_regs[2].add(1), !((*io).size[1] - 1));
    for (r, x, shift, flag) in [(mem, 0usize, 12, 0xa0000000u32), (mmio, 6, 12, 0x80000000), (io, 12, 12, 0xc0000000)] { out_le32(pci_regs[0].add(x), (*r).pci_addr[1] >> shift); out_le32(pci_regs[0].add(x+2), (*r).phys_addr >> shift); out_le32(pci_regs[0].add(x+4), (!( (*r).size[1]-1) >> shift) | flag); }
    out_le32(pci_regs[0].add(58), 0); out_le32(pci_regs[0].add(60), 0);
    let mem_pow2 = 1u32 << (__ilog2_u32((bd.bi_memsize as u32) - 1) + 1); out_le32(pci_regs[0].add(62), 0xa0000000 | (!(mem_pow2 - 1) >> 12));
    if in_le32(pci_regs[0].add(32)) & 1 == 0 { udelay(100000); out_le32(pci_regs[0].add(32), 1); udelay(1020000); }
    out_le32(pci_regs[0].add(64), 0x80000004); out_le32(pci_regs[0].add(65), in_le32(pci_regs[0].add(65)) | 6);
    out_8(soc_regs.add(0x10028), 3); out_be32(soc_regs.add(0x1002c) as *mut u32, 0x01236745);
}

unsafe extern "C" fn pq2_platform_fixups() {
    dt_fixup_memory(bd.bi_memstart, bd.bi_memsize);
    dt_fixup_mac_addresses(bd.bi_enetaddr.as_ptr(), bd.bi_enet1addr.as_ptr());
    dt_fixup_cpu_clocks(bd.bi_intfreq, bd.bi_busfreq / 4, bd.bi_busfreq);
    update_cs_ranges();
    fixup_pci();
}

#[no_mangle]
pub unsafe extern "C" fn platform_init(_r3: usize, _r4: usize, _r5: usize, _r6: usize, _r7: usize) {
    cuboot_init();
    fdt_init(&_dtb_start);
    serial_console_init();
    (*(core::ptr::addr_of_mut!(platform_ops))).fixups = Some(pq2_platform_fixups);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
