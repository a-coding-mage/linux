// SPDX-License-Identifier: GPL-2.0
/*
 * misc.c:  Miscellaneous prom functions that don't belong
 *          anywhere else.
 */

use core::ffi::c_char;

type Phandle = i32;

extern "C" {
    fn p1275_cmd_direct(args: *mut usize);
    fn prom_getproplen(node: Phandle, name: *const c_char) -> i32;
    fn prom_getproperty(node: Phandle, name: *const c_char, value: *mut c_char, buflen: i32) -> i32;
    fn prom_finddevice(path: *const c_char) -> Phandle;
    fn prom_getint(node: Phandle, property: *const c_char) -> i32;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    #[cfg(feature = "CONFIG_SMP")]
    fn smp_capture();
    #[cfg(feature = "CONFIG_SMP")]
    fn smp_release();
    #[cfg(feature = "CONFIG_SUN_LDOMS")]
    fn ldom_reboot(command: *const c_char);
    #[cfg(feature = "CONFIG_SUN_LDOMS")]
    fn ldom_power_off();

    static mut prom_mmu_ihandle_cache: i32;
    static prom_root_node: Phandle;
    static prom_chosen_path: *const c_char;
    static prom_mmu_name: *const c_char;
    static prom_callmethod_name: *const c_char;
    static prom_map_name: *const c_char;
    static prom_unmap_name: *const c_char;
    #[cfg(feature = "CONFIG_SUN_LDOMS")]
    static mut ldom_domaining_enabled: bool;
}

unsafe fn prom_service_exists(service_name: *const c_char) -> i32 {
    let mut args = [0usize; 5];
    args[0] = b"test\0".as_ptr() as usize;
    args[1] = 1;
    args[2] = 1;
    args[3] = service_name as usize;
    args[4] = usize::MAX;
    p1275_cmd_direct(args.as_mut_ptr());
    if args[4] != 0 { 0 } else { 1 }
}

#[no_mangle]
pub unsafe extern "C" fn prom_sun4v_guest_soft_state() {
    let svc = b"SUNW,soft-state-supported\0".as_ptr() as *const c_char;
    if prom_service_exists(svc) == 0 { return; }
    let mut args = [svc as usize, 0, 0];
    p1275_cmd_direct(args.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn prom_reboot(bcommand: *const c_char) {
    #[cfg(feature = "CONFIG_SUN_LDOMS")]
    if ldom_domaining_enabled { ldom_reboot(bcommand); }
    let mut args = [b"boot\0".as_ptr() as usize, 1, 0, bcommand as usize];
    p1275_cmd_direct(args.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn prom_feval(fstring: *const c_char) {
    if fstring.is_null() || *fstring == 0 { return; }
    let mut args = [b"interpret\0".as_ptr() as usize, 1, 1, fstring as usize, usize::MAX];
    p1275_cmd_direct(args.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn prom_cmdline() {
    let mut args = [b"enter\0".as_ptr() as usize, 0, 0];
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    #[cfg(feature = "CONFIG_SMP")]
    smp_capture();
    p1275_cmd_direct(args.as_mut_ptr());
    #[cfg(feature = "CONFIG_SMP")]
    smp_release();
    local_irq_restore(flags);
}

#[no_mangle]
pub unsafe extern "C" fn prom_halt() -> ! {
    #[cfg(feature = "CONFIG_SUN_LDOMS")]
    if ldom_domaining_enabled { ldom_power_off(); }
    loop {
        let mut args = [b"exit\0".as_ptr() as usize, 0, 0];
        p1275_cmd_direct(args.as_mut_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn prom_halt_power_off() {
    #[cfg(feature = "CONFIG_SUN_LDOMS")]
    if ldom_domaining_enabled { ldom_power_off(); }
    let mut args = [b"SUNW,power-off\0".as_ptr() as usize, 0, 0];
    p1275_cmd_direct(args.as_mut_ptr());
    prom_halt();
}

#[no_mangle]
pub unsafe extern "C" fn prom_get_idprom(idbuf: *mut c_char, num_bytes: i32) -> u8 {
    let len = prom_getproplen(prom_root_node, b"idprom\0".as_ptr() as *const c_char);
    if len > num_bytes || len == -1 { return 0xff; }
    if prom_getproperty(prom_root_node, b"idprom\0".as_ptr() as *const c_char, idbuf, num_bytes) == 0 {
        return *(idbuf as *const u8);
    }
    0xff
}

unsafe fn prom_get_mmu_ihandle() -> i32 {
    if prom_mmu_ihandle_cache != 0 { return prom_mmu_ihandle_cache; }
    let node = prom_finddevice(prom_chosen_path);
    let ret = prom_getint(node, prom_mmu_name);
    prom_mmu_ihandle_cache = if ret == -1 || ret == 0 { -1 } else { ret };
    ret
}

unsafe fn prom_get_memory_ihandle() -> i32 {
    static mut memory_ihandle_cache: i32 = 0;
    if memory_ihandle_cache != 0 { return memory_ihandle_cache; }
    let node = prom_finddevice(b"/chosen\0".as_ptr() as *const c_char);
    let ret = prom_getint(node, b"memory\0".as_ptr() as *const c_char);
    memory_ihandle_cache = if ret == -1 || ret == 0 { -1 } else { ret };
    ret
}

unsafe fn tlb_load(ty: *const c_char, index: usize, tte_data: usize, vaddr: usize) -> isize {
    let mut args = [0usize; 9];
    args[0] = prom_callmethod_name as usize; args[1] = 5; args[2] = 1;
    args[3] = ty as usize; args[4] = prom_get_mmu_ihandle() as u32 as usize;
    args[5] = vaddr; args[6] = tte_data; args[7] = index; args[8] = usize::MAX;
    p1275_cmd_direct(args.as_mut_ptr()); args[8] as isize
}

pub unsafe extern "C" fn prom_itlb_load(index: usize, tte_data: usize, vaddr: usize) -> isize { tlb_load(b"SUNW,itlb-load\0".as_ptr() as *const c_char, index, tte_data, vaddr) }
pub unsafe extern "C" fn prom_dtlb_load(index: usize, tte_data: usize, vaddr: usize) -> isize { tlb_load(b"SUNW,dtlb-load\0".as_ptr() as *const c_char, index, tte_data, vaddr) }

pub unsafe extern "C" fn prom_map(mode: i32, size: usize, vaddr: usize, paddr: usize) -> i32 {
    let mut args = [0usize; 11]; args[0]=prom_callmethod_name as usize; args[1]=7; args[2]=1;
    args[3]=prom_map_name as usize; args[4]=prom_get_mmu_ihandle() as u32 as usize;
    args[5]=mode as u32 as usize; args[6]=size; args[7]=vaddr; args[8]=0; args[9]=paddr; args[10]=usize::MAX;
    p1275_cmd_direct(args.as_mut_ptr()); let mut ret=args[10] as i32; if ret==0 {ret=-1;} ret
}

pub unsafe extern "C" fn prom_unmap(size: usize, vaddr: usize) { let mut args=[prom_callmethod_name as usize,4,0,prom_unmap_name as usize,prom_get_mmu_ihandle() as u32 as usize,size,vaddr]; p1275_cmd_direct(args.as_mut_ptr()); }

pub unsafe extern "C" fn prom_retain(name:*const c_char,size:usize,align:usize,paddr:*mut usize)->i32 { let mut a=[prom_callmethod_name as usize,5,3,b"SUNW,retain\0".as_ptr() as usize,prom_get_memory_ihandle() as u32 as usize,align,size,name as usize,usize::MAX,usize::MAX,usize::MAX]; p1275_cmd_direct(a.as_mut_ptr()); if a[8]!=0{return a[8] as i32;} *paddr=a[10]; 0 }
pub unsafe extern "C" fn prom_getunumber(syndrome_code:i32,phys_addr:usize,buf:*mut c_char,buflen:i32)->i32 { let mut a=[prom_callmethod_name as usize,7,2,b"SUNW,get-unumber\0".as_ptr() as usize,prom_get_memory_ihandle() as u32 as usize,buflen as usize,buf as usize,0,phys_addr,syndrome_code as u32 as usize,usize::MAX,usize::MAX]; p1275_cmd_direct(a.as_mut_ptr()); a[10] as i32 }

pub unsafe extern "C" fn prom_sleepself(){let mut a=[b"SUNW,sleep-self\0".as_ptr() as usize,0,0];p1275_cmd_direct(a.as_mut_ptr());}
pub unsafe extern "C" fn prom_sleepsystem()->i32{let mut a=[b"SUNW,sleep-system\0".as_ptr() as usize,0,1,usize::MAX];p1275_cmd_direct(a.as_mut_ptr());a[3] as i32}
pub unsafe extern "C" fn prom_wakeupsystem()->i32{let mut a=[b"SUNW,wakeup-system\0".as_ptr() as usize,0,1,usize::MAX];p1275_cmd_direct(a.as_mut_ptr());a[3] as i32}

#[cfg(feature = "CONFIG_SMP")]
pub unsafe extern "C" fn prom_startcpu(n:i32,pc:usize,arg:usize){let mut a=[b"SUNW,start-cpu\0".as_ptr() as usize,3,0,n as u32 as usize,pc,arg];p1275_cmd_direct(a.as_mut_ptr());}
#[cfg(feature = "CONFIG_SMP")]
pub unsafe extern "C" fn prom_startcpu_cpuid(n:i32,pc:usize,arg:usize){let mut a=[b"SUNW,start-cpu-by-cpuid\0".as_ptr() as usize,3,0,n as u32 as usize,pc,arg];p1275_cmd_direct(a.as_mut_ptr());}
#[cfg(feature = "CONFIG_SMP")]
pub unsafe extern "C" fn prom_stopcpu_cpuid(n:i32){let mut a=[b"SUNW,stop-cpu-by-cpuid\0".as_ptr() as usize,1,0,n as u32 as usize];p1275_cmd_direct(a.as_mut_ptr());}
#[cfg(feature = "CONFIG_SMP")]
pub unsafe extern "C" fn prom_stopself(){let mut a=[b"SUNW,stop-self\0".as_ptr() as usize,0,0];p1275_cmd_direct(a.as_mut_ptr());}
#[cfg(feature = "CONFIG_SMP")]
pub unsafe extern "C" fn prom_idleself(){let mut a=[b"SUNW,idle-self\0".as_ptr() as usize,0,0];p1275_cmd_direct(a.as_mut_ptr());}
#[cfg(feature = "CONFIG_SMP")]
pub unsafe extern "C" fn prom_resumecpu(n:i32){let mut a=[b"SUNW,resume-cpu\0".as_ptr() as usize,1,0,n as u32 as usize];p1275_cmd_direct(a.as_mut_ptr());}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
