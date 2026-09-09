// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * devtree.c - convenience functions for device tree manipulation
 * Copyright 2007 David Gibson, IBM Corporation.
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Types, constants, and functions supplied by the surrounding boot environment.
extern "C" {
    static mut timebase_period_ns: u32;
    fn finddevice(path: *const c_char) -> *mut c_void;
    fn getprop(node: *mut c_void, name: *const c_char, buf: *mut c_void, buflen: usize) -> c_int;
    fn fatal(fmt: *const c_char, ...);
    fn create_node(parent: *mut c_void, name: *const c_char) -> *mut c_void;
    fn setprop_str(node: *mut c_void, name: *const c_char, value: *const c_char);
    fn printf(fmt: *const c_char, ...);
    fn setprop(node: *mut c_void, name: *const c_char, value: *const c_void, len: usize);
    fn find_node_by_devtype(node: *mut c_void, devtype: *const c_char) -> *mut c_void;
    fn setprop_val(node: *mut c_void, name: *const c_char, value: u32);
    fn find_node_by_alias(alias: *const c_char) -> *mut c_void;
    fn find_node_by_prop_value(node: *mut c_void, name: *const c_char, value: *const c_void, len: usize) -> *mut c_void;
    fn get_parent(node: *mut c_void) -> *mut c_void;
    fn be32_to_cpu(value: u32) -> u32;
    fn cpu_to_be32(value: u32) -> u32;
    fn memset(dest: *mut c_void, value: c_int, len: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strnlen(s: *const c_char, maxlen: usize) -> usize;
}

const MAX_PROP_LEN: usize = 4096;
const MAX_ADDR_CELLS: usize = 4;
const MHZ: fn(u32) -> u32 = |x| (x.wrapping_add(500_000)) / 1_000_000;

pub unsafe fn dt_fixup_memory(start: u64, size: u64) {
    let root = finddevice(b"/\0".as_ptr() as *const c_char);
    let mut naddr: i32 = 0;
    let mut nsize: i32 = 0;
    let mut memreg = [0u32; 4];
    if getprop(root, b"#address-cells\0".as_ptr() as *const c_char, &mut naddr as *mut _ as *mut c_void, 4) < 0 { naddr = 2; } else { naddr = be32_to_cpu(naddr as u32) as i32; }
    if naddr < 1 || naddr > 2 { fatal(b"Can't cope with #address-cells == %d in /\n\r\0".as_ptr() as *const c_char, naddr); }
    if getprop(root, b"#size-cells\0".as_ptr() as *const c_char, &mut nsize as *mut _ as *mut c_void, 4) < 0 { nsize = 1; } else { nsize = be32_to_cpu(nsize as u32) as i32; }
    if nsize < 1 || nsize > 2 { fatal(b"Can't cope with #size-cells == %d in /\n\r\0".as_ptr() as *const c_char, nsize); }
    let mut i = 0usize;
    if naddr == 2 { memreg[i] = cpu_to_be32((start >> 32) as u32); i += 1; }
    memreg[i] = cpu_to_be32(start as u32); i += 1;
    if nsize == 2 { memreg[i] = cpu_to_be32((size >> 32) as u32); i += 1; }
    memreg[i] = cpu_to_be32(size as u32);
    let mut memory = finddevice(b"/memory\0".as_ptr() as *const c_char);
    if memory.is_null() { memory = create_node(core::ptr::null_mut(), b"memory\0".as_ptr() as *const c_char); setprop_str(memory, b"device_type\0".as_ptr() as *const c_char, b"memory\0".as_ptr() as *const c_char); }
    printf(b"Memory <- <0x%x\0".as_ptr() as *const c_char, be32_to_cpu(memreg[0]));
    for j in 1..(naddr + nsize) as usize { printf(b" 0x%x\0".as_ptr() as *const c_char, be32_to_cpu(memreg[j])); }
    printf(b"> (%ldMB)\n\r\0".as_ptr() as *const c_char, (size >> 20) as c_ulong);
    setprop(memory, b"reg\0".as_ptr() as *const c_char, memreg.as_ptr() as *const c_void, ((naddr + nsize) as usize) * 4);
}

pub unsafe fn dt_fixup_cpu_clocks(cpu: u32, tb: u32, bus: u32) {
    printf(b"CPU clock-frequency <- 0x%x (%dMHz)\n\r\0".as_ptr() as *const c_char, cpu, MHZ(cpu));
    printf(b"CPU timebase-frequency <- 0x%x (%dMHz)\n\r\0".as_ptr() as *const c_char, tb, MHZ(tb));
    if bus > 0 { printf(b"CPU bus-frequency <- 0x%x (%dMHz)\n\r\0".as_ptr() as *const c_char, bus, MHZ(bus)); }
    let mut devp = core::ptr::null_mut();
    loop { devp = find_node_by_devtype(devp, b"cpu\0".as_ptr() as *const c_char); if devp.is_null() { break; } setprop_val(devp, b"clock-frequency\0".as_ptr() as *const c_char, cpu_to_be32(cpu)); setprop_val(devp, b"timebase-frequency\0".as_ptr() as *const c_char, cpu_to_be32(tb)); if bus > 0 { setprop_val(devp, b"bus-frequency\0".as_ptr() as *const c_char, cpu_to_be32(bus)); } }
    timebase_period_ns = 1_000_000_000 / tb;
}

pub unsafe fn dt_fixup_clock(path: *const c_char, freq: u32) { let devp = finddevice(path); if !devp.is_null() { printf(b"%s: clock-frequency <- %x (%dMHz)\n\r\0".as_ptr() as *const c_char, path, freq, MHZ(freq)); setprop_val(devp, b"clock-frequency\0".as_ptr() as *const c_char, cpu_to_be32(freq)); } }

pub unsafe fn dt_fixup_mac_address_by_alias(alias: *const c_char, addr: *const u8) { let devp = find_node_by_alias(alias); if !devp.is_null() { printf(b"%s: local-mac-address <- %02x:%02x:%02x:%02x:%02x:%02x\n\r\0".as_ptr() as *const c_char, alias, *addr, *addr.add(1), *addr.add(2), *addr.add(3), *addr.add(4), *addr.add(5)); setprop(devp, b"local-mac-address\0".as_ptr() as *const c_char, addr as *const c_void, 6); } }

pub unsafe fn dt_fixup_mac_address(index: u32, addr: *const u8) { let devp = find_node_by_prop_value(core::ptr::null_mut(), b"linux,network-index\0".as_ptr() as *const c_char, &index as *const _ as *const c_void, 4); if !devp.is_null() { printf(b"ENET%d: local-mac-address <- %02x:%02x:%02x:%02x:%02x:%02x\n\r\0".as_ptr() as *const c_char, index, *addr, *addr.add(1), *addr.add(2), *addr.add(3), *addr.add(4), *addr.add(5)); setprop(devp, b"local-mac-address\0".as_ptr() as *const c_char, addr as *const c_void, 6); } }

// C variadic traversal is ABI-specific; `args` represents the va_list supplied by the caller.
pub unsafe fn __dt_fixup_mac_addresses(startindex: u32, mut args: *mut *const u8) { let mut index = startindex; while !args.is_null() && !(*args).is_null() { dt_fixup_mac_address(index, *args); index += 1; args = args.add(1); } }

pub unsafe fn dt_get_reg_format(node: *mut c_void, naddr: *mut u32, nsize: *mut u32) { if getprop(node, b"#address-cells\0".as_ptr() as *const c_char, naddr as *mut c_void, 4) != 4 { *naddr = 2; } else { *naddr = be32_to_cpu(*naddr); } if getprop(node, b"#size-cells\0".as_ptr() as *const c_char, nsize as *mut c_void, 4) != 4 { *nsize = 1; } else { *nsize = be32_to_cpu(*nsize); } }

static mut PROP_BUF: [u32; MAX_PROP_LEN / 4] = [0; MAX_PROP_LEN / 4];

unsafe fn copy_val(dest: *mut u32, src: *const u32, naddr: usize) { let pad = MAX_ADDR_CELLS - naddr; memset(dest as *mut c_void, 0, pad * 4); memcpy(dest.add(pad) as *mut c_void, src as *const c_void, naddr * 4); }
unsafe fn sub_reg(reg: *mut u32, sub: *const u32) -> bool { let mut borrow = 0u32; for i in (0..MAX_ADDR_CELLS).rev() { let prev = borrow; borrow = (*reg.add(i) < (*sub.add(i)).wrapping_add(prev)) as u32; *reg.add(i) = (*reg.add(i)).wrapping_sub((*sub.add(i)).wrapping_add(prev)); } borrow == 0 }
unsafe fn add_reg(reg: *mut u32, add: *const u32, naddr: usize) -> bool { let mut carry = 0u64; for i in (MAX_ADDR_CELLS-naddr..MAX_ADDR_CELLS).rev() { let tmp = be32_to_cpu(*reg.add(i)) as u64 + be32_to_cpu(*add.add(i)) as u64 + carry; carry = tmp >> 32; *reg.add(i) = cpu_to_be32(tmp as u32); } carry == 0 }
unsafe fn compare_reg(reg: *const u32, range: *const u32, rangesize: *const u32) -> bool { let mut i = 0; while i < MAX_ADDR_CELLS { if be32_to_cpu(*reg.add(i)) < be32_to_cpu(*range.add(i)) { return false; } if be32_to_cpu(*reg.add(i)) > be32_to_cpu(*range.add(i)) { break; } i += 1; } let mut end=0; while i < MAX_ADDR_CELLS { end=be32_to_cpu(*range.add(i)).wrapping_add(be32_to_cpu(*rangesize.add(i))); if be32_to_cpu(*reg.add(i)) < end { break; } if be32_to_cpu(*reg.add(i)) > end { return false; } i += 1; } *reg.add(i) != end }
unsafe fn find_range(reg: *const u32, ranges: *const u32, nregaddr: usize, naddr: usize, nsize: usize, buflen: usize) -> isize { let nrange=nregaddr+naddr+nsize; let mut i=0; while i+nrange<=buflen { let mut a=[0u32;4]; let mut s=[0u32;4]; copy_val(a.as_mut_ptr(), ranges.add(i), nregaddr); copy_val(s.as_mut_ptr(), ranges.add(i+nregaddr+naddr), nsize); if compare_reg(reg,a.as_ptr(),s.as_ptr()) { return i as isize; } i+=nrange; } -1 }

unsafe fn dt_xlate(node: *mut c_void, res: c_int, reglen: c_int, addr: *mut c_ulong, size: *mut c_ulong) -> c_int { let mut last=[0u32;4]; let mut this=[0u32;4]; let mut parent=get_parent(node); if parent.is_null(){return 0} let mut naddr=0u32; let mut nsize=0u32; dt_get_reg_format(parent,&mut naddr,&mut nsize); if nsize>2{return 0} let offset=((naddr+nsize) as c_int)*res; if reglen<offset+naddr as c_int+nsize as c_int || MAX_PROP_LEN<((offset as usize+naddr as usize+nsize as usize)*4)){return 0} copy_val(last.as_mut_ptr(),PROP_BUF.as_ptr().add(offset as usize),naddr as usize); let mut ret_size=be32_to_cpu(PROP_BUF[(offset as usize+naddr as usize)] ) as u64; if nsize==2 {ret_size=(ret_size<<32)|be32_to_cpu(PROP_BUF[offset as usize+naddr as usize+1]) as u64;} loop { let prev_naddr=naddr; let prev_nsize=nsize; node=parent; parent=get_parent(node); if parent.is_null(){break} dt_get_reg_format(parent,&mut naddr,&mut nsize); let buflen=getprop(node,b"ranges\0".as_ptr() as *const c_char,PROP_BUF.as_mut_ptr() as *mut c_void,MAX_PROP_LEN); if buflen==0{continue} if buflen<0{return 0} let off=find_range(last.as_ptr(),PROP_BUF.as_ptr(),prev_naddr as usize,naddr as usize,prev_nsize as usize,buflen as usize/4); if off<0{return 0} copy_val(this.as_mut_ptr(),PROP_BUF.as_ptr().add(off as usize),prev_naddr as usize); if !sub_reg(last.as_mut_ptr(),this.as_ptr()){return 0} copy_val(this.as_mut_ptr(),PROP_BUF.as_ptr().add(off as usize+prev_naddr as usize),naddr as usize); if !add_reg(last.as_mut_ptr(),this.as_ptr(),naddr as usize){return 0} } if naddr>2{return 0} let ret_addr=((be32_to_cpu(last[2]) as u64)<<32)|be32_to_cpu(last[3]) as u64; if core::mem::size_of::<*const c_void>()==4 && (ret_addr>=0x1_0000_0000 || ret_size>0x1_0000_0000 || ret_addr+ret_size>0x1_0000_0000){return 0} *addr=ret_addr as c_ulong; if !size.is_null(){*size=ret_size as c_ulong} 1 }

pub unsafe fn dt_xlate_reg(node:*mut c_void,res:c_int,addr:*mut c_ulong,size:*mut c_ulong)->c_int { dt_xlate(node,res,getprop(node,b"reg\0".as_ptr() as *const c_char,PROP_BUF.as_mut_ptr() as *mut c_void,MAX_PROP_LEN)/4,addr,size) }
pub unsafe fn dt_xlate_addr(node:*mut c_void,buf:*const u32,buflen:c_int,xlated_addr:*mut c_ulong)->c_int { if buflen as usize>MAX_PROP_LEN{return 0} memcpy(PROP_BUF.as_mut_ptr() as *mut c_void,buf as *const c_void,buflen as usize); dt_xlate(node,0,buflen/4,xlated_addr,core::ptr::null_mut()) }
pub unsafe fn dt_is_compatible(node:*mut c_void,compat:*const c_char)->c_int { let buf=PROP_BUF.as_mut_ptr() as *mut c_char; let len=getprop(node,b"compatible\0".as_ptr() as *const c_char,buf as *mut c_void,MAX_PROP_LEN); if len<0{return 0} let mut pos=0; while pos<len { if strcmp(buf.add(pos as usize),compat)==0{return 1} pos+=strnlen(buf.add(pos as usize), (len-pos) as usize) as c_int+1; } 0 }
pub unsafe fn dt_get_virtual_reg(node:*mut c_void,addr:*mut *mut c_void,nres:c_int)->c_int { let n=getprop(node,b"virtual-reg\0".as_ptr() as *const c_char,addr as *mut c_void,nres as usize*4); if n>0 {for i in 0..n/4 {*((addr as *mut u32).add(i as usize))=be32_to_cpu(*((addr as *mut u32).add(i as usize)));} return n/4} let mut count=0; while count<nres {let mut x=0;c_ulong; if dt_xlate_reg(node,count,&mut x,core::ptr::null_mut())==0{break} *addr.add(count as usize)=x as *mut c_void; count+=1;} count }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
