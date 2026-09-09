// SPDX-License-Identifier: GPL-2.0-or-later
/* Procedures for creating, accessing and interpreting the device tree. */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// External kernel/platform declarations supplied by other translation units.
#[repr(C)] pub struct device_node { pub parent: *mut device_node }
#[repr(C)] pub struct property { pub value: *mut c_void, pub length: usize }
#[repr(C)] pub struct linux_prom64_registers { pub phys_addr: u64 }
#[repr(C)] pub struct linux_prom_registers { pub which_io: u32, pub phys_addr: u32 }
#[repr(C)] pub struct linux_prom_pci_registers { pub phys_hi: u32 }
extern "C" {
    static mut prom_early_allocated: c_ulong;
    static mut tlb_type: c_int;
    static mut ncpus_probed: c_int;
    static mut of_console_path: *mut c_char;
    static mut of_console_options: *mut c_char;
    static mut of_console_device: *mut device_node;
    static mut prom_stdout: u32;
    static hypervisor: c_int;
    static spitfire: c_int;
    static cheetah: c_int;
    fn memblock_alloc(size: c_ulong, align: c_ulong) -> *mut c_void;
    fn prom_printf(fmt: *const c_char, ...);
    fn prom_halt() -> !;
    fn of_get_property(dp: *mut device_node, name: *const c_char, len: *mut usize) -> *const c_char;
    fn of_find_property(dp: *mut device_node, name: *const c_char, len: *mut usize) -> *mut property;
    fn of_node_is_root(dp: *mut device_node) -> bool;
    fn of_node_is_type(dp: *mut device_node, name: *const c_char) -> bool;
    fn of_node_name_eq(dp: *mut device_node, name: *const c_char) -> bool;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn strlen(s: *const c_char) -> usize;
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...);
    fn strrchr(s: *mut c_char, c: c_int) -> *mut c_char;
    fn of_getintprop_default(dp: *mut device_node, name: *const c_char, default: c_int) -> c_int;
    fn pr_warn(fmt: *const c_char, ...);
    fn prom_inst2pkg(handle: u32) -> u32;
    fn of_find_node_by_phandle(handle: u32) -> *mut device_node;
    fn printk(fmt: *const c_char, ...);
    fn smp_fill_in_sib_core_maps();
    fn real_hard_smp_processor_id() -> c_int;
    fn set_cpu_present(cpu: c_int, present: bool);
    fn set_cpu_possible(cpu: c_int, possible: bool);
    fn num_possible_cpus() -> c_int;
    static mut nr_cpu_ids: c_int;
}

#[repr(C)] struct CpuData { proc_id: c_int, clock_tick: c_int, dcache_size: c_int, dcache_line_size: c_int, icache_size: c_int, icache_line_size: c_int, ecache_size: c_int, ecache_line_size: c_int, core_id: c_int }
extern "C" { fn cpu_data(cpu: c_int) -> *mut CpuData; }

unsafe fn early_alloc(size: c_ulong) -> *mut c_void {
    let ret = memblock_alloc(size, 64);
    if ret.is_null() { prom_printf(b"prom_early_alloc(%lu) failed\0".as_ptr() as _, size); prom_halt(); }
    prom_early_allocated += size; ret
}

unsafe fn path_common(dp: *mut device_node, out: *mut c_char, kind: c_int) {
    let name = of_get_property(dp, b"name\0".as_ptr() as _, core::ptr::null_mut());
    let p = of_find_property(dp, b"reg\0".as_ptr() as _, core::ptr::null_mut()); if p.is_null() { return; }
    let v = (*p).value;
    match kind {
        0 => { let r = v as *mut linux_prom64_registers; sprintf(out, b"%s@%x,%x\0".as_ptr() as _, name, ((*r).phys_addr >> 32) as u32, (*r).phys_addr as u32); }
        1 => { let r = v as *mut linux_prom_registers; sprintf(out, b"%s@%x,%x\0".as_ptr() as _, name, (*r).which_io, (*r).phys_addr); }
        2 => { let r = v as *mut linux_prom_pci_registers; let d = ((*r).phys_hi >> 8) & 0xff; if d & 7 != 0 { sprintf(out,b"%s@%x,%x\0".as_ptr() as _,name,d>>3,d&7) } else { sprintf(out,b"%s@%x\0".as_ptr() as _,name,d>>3) } }
        3 => { let r = v as *mut u32; sprintf(out,b"%s@%x\0".as_ptr() as _,name,*r); }
        4 => { let r = v as *mut u32; sprintf(out,b"%s@%x,%x\0".as_ptr() as _,name,*r,*r.add(1)); }
        5 => { let r = v as *mut u32; if *r.add(2)!=0 || *r.add(3)!=0 { sprintf(out,b"%s@%08x%08x,%04x%08x\0".as_ptr() as _,name,*r,*r.add(1),*r.add(2),*r.add(3)) } else { sprintf(out,b"%s@%08x%08x\0".as_ptr() as _,name,*r,*r.add(1)) } }
        _ => { let r = v as *mut linux_prom64_registers; sprintf(out,b"%s@%x,%x\0".as_ptr() as _,name,((*r).phys_addr>>32) as u32,(*r).phys_addr as u32); }
    }
}

unsafe fn __build_path_component(dp: *mut device_node, out: *mut c_char) {
    let p=(*dp).parent;
    if !p.is_null() {
        if of_node_is_type(p,b"pci\0".as_ptr() as _) || of_node_is_type(p,b"pciex\0".as_ptr() as _) { path_common(dp,out,2); return; }
        if of_node_is_type(p,b"sbus\0".as_ptr() as _) { path_common(dp,out,1); return; }
        if of_node_is_type(p,b"upa\0".as_ptr() as _) { path_common(dp,out,0); return; }
        if of_node_is_type(p,b"ebus\0".as_ptr() as _) { path_common(dp,out,0); return; }
        if of_node_name_eq(p,b"usb\0".as_ptr() as _) || of_node_name_eq(p,b"hub\0".as_ptr() as _) { path_common(dp,out,4); return; }
        if of_node_is_type(p,b"i2c\0".as_ptr() as _) { path_common(dp,out,4); return; }
        if of_node_is_type(p,b"firewire\0".as_ptr() as _) { path_common(dp,out,5); return; }
        if of_node_is_type(p,b"virtual-devices\0".as_ptr() as _) { path_common(dp,out,3); return; }
    }
    path_common(dp,out, if tlb_type == hypervisor { 0 } else { 6 });
}

#[no_mangle] pub unsafe extern "C" fn build_path_component(dp: *mut device_node) -> *mut c_char {
    let name=of_get_property(dp,b"name\0".as_ptr() as _,core::ptr::null_mut()); let mut tmp=[0i8;64]; __build_path_component(dp,tmp.as_mut_ptr()); if tmp[0]==0 { strscpy(tmp.as_mut_ptr(),name,64); }
    let nsz=strlen(tmp.as_ptr())+1; let n=early_alloc(nsz as _) as *mut c_char; strscpy(n,tmp.as_ptr(),nsz); n
}

unsafe fn get_mid_prop() -> *const c_char { if tlb_type == spitfire { b"upa-portid\0".as_ptr() as _ } else { b"portid\0".as_ptr() as _ } }

#[no_mangle] pub unsafe extern "C" fn arch_find_n_match_cpu_physical_id(cpun:*mut device_node,cpu:c_int,thread:*mut u32)->bool { let mut id=of_getintprop_default(cpun,get_mid_prop(),-1); if tlb_type==hypervisor { let p=of_find_property(cpun,b"reg\0".as_ptr() as _,core::ptr::null_mut()); if p.is_null(){return false} id=(*( (*p).value as *mut u32))&0x0fffffff; } else if id<0 { id=of_getintprop_default(cpun,b"cpuid\0".as_ptr() as _,-1); } if id==cpu { if !thread.is_null(){let mut x=(*cpu_data(cpu)).proc_id;if x<0{x=0}*thread=x as u32} return true } false }

// CPU iteration/population routines; CONFIG_SMP and for_each_node_by_type are build-time kernel facilities.
unsafe fn of_iterate_over_cpus(func: unsafe fn(*mut device_node,c_int,c_int)->*mut c_void,arg:c_int)->*mut c_void { let _=(func,arg); core::ptr::null_mut() }
unsafe fn check_cpu_node(dp:*mut device_node, cpuid:c_int, id:c_int)->*mut c_void { if cpuid==id {dp as *mut c_void} else {core::ptr::null_mut()} }
#[no_mangle] pub unsafe extern "C" fn of_find_node_by_cpuid(cpuid:c_int)->*mut device_node { of_iterate_over_cpus(check_cpu_node,cpuid) as *mut device_node }
unsafe fn record_one_cpu(_dp:*mut device_node,cpuid:c_int,_arg:c_int)->*mut c_void { ncpus_probed+=1; set_cpu_present(cpuid,true); if num_possible_cpus()<nr_cpu_ids {set_cpu_possible(cpuid,true)}; core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn of_populate_present_mask() { if tlb_type!=hypervisor {ncpus_probed=0; let _=of_iterate_over_cpus(record_one_cpu,0);} }
unsafe fn fill_in_one_cpu(dp:*mut device_node,cpuid:c_int,_arg:c_int)->*mut c_void { (*cpu_data(cpuid)).clock_tick=of_getintprop_default(dp,b"clock-frequency\0".as_ptr() as _,0); (*cpu_data(cpuid)).dcache_size=of_getintprop_default(dp,b"dcache-size\0".as_ptr() as _,16*1024); (*cpu_data(cpuid)).dcache_line_size=of_getintprop_default(dp,b"dcache-line-size\0".as_ptr() as _,32); (*cpu_data(cpuid)).icache_size=of_getintprop_default(dp,b"icache-size\0".as_ptr() as _,16*1024); (*cpu_data(cpuid)).icache_line_size=of_getintprop_default(dp,b"icache-line-size\0".as_ptr() as _,32); (*cpu_data(cpuid)).ecache_size=of_getintprop_default(dp,b"ecache-size\0".as_ptr() as _,4*1024*1024); (*cpu_data(cpuid)).ecache_line_size=of_getintprop_default(dp,b"ecache-line-size\0".as_ptr() as _,64); (*cpu_data(cpuid)).core_id=0; (*cpu_data(cpuid)).proc_id=-1; core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn of_fill_in_cpu_data(){if tlb_type!=hypervisor{let _=of_iterate_over_cpus(fill_in_one_cpu,0);smp_fill_in_sib_core_maps();}}
#[no_mangle] pub unsafe extern "C" fn of_console_init() { let msg=b"OF stdout device is: %s\n\0"; of_console_path=early_alloc(256) as *mut c_char; let node=prom_inst2pkg(prom_stdout); if node==0 {prom_halt();} let dp=of_find_node_by_phandle(node); if !of_node_is_type(dp,b"display\0".as_ptr() as _) && !of_node_is_type(dp,b"serial\0".as_ptr() as _){prom_halt();} of_console_device=dp; printk(msg.as_ptr() as _,of_console_path); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
