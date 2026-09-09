// SPDX-License-Identifier: GPL-2.0-or-later
/* Low-level PowerPC feature fixups; external kernel symbols are supplied by
 * the surrounding kernel translation unit. */

#[repr(C)]
pub struct fixup_entry { pub mask: c_ulong, pub value: c_ulong, pub start_off: c_long, pub end_off: c_long, pub alt_start_off: c_long, pub alt_end_off: c_long }

extern "C" {
    static mut static_key_feature_checks_initialized: bool;
    fn ppc_inst_read(p: *mut u32) -> ppc_inst_t;
    fn instr_is_relative_branch(i: ppc_inst_t) -> bool;
    fn branch_target(p: *mut u32) -> *mut u32;
    fn translate_branch(i: *mut ppc_inst_t, dst: *mut u32, src: *mut u32) -> c_int;
    fn raw_patch_instruction(dst: *mut u32, i: ppc_inst_t);
    fn ppc_inst_next(a: *mut u32, b: *mut u32) -> *mut u32;
    fn ppc_inst(x: u32) -> ppc_inst_t;
    fn ppc_raw_nop() -> u32;
    fn warn_on(x: bool);
    fn printk(fmt: *const c_char, ...);
    fn patch_instruction(dst: *mut u32, i: ppc_inst_t);
    fn patch_branch(dst: *mut u32, target: c_ulong, flags: c_ulong);
    fn cpu_has_feature(x: c_ulong) -> bool;
    fn stop_machine(f: unsafe extern "C" fn(*mut c_void) -> c_int, data: *mut c_void, arg: *mut c_void) -> c_int;
    fn mutex_lock(lock: *mut c_void); fn mutex_unlock(lock: *mut c_void);
    fn static_branch_enable(key: *mut c_void); fn static_branch_disable(key: *mut c_void);
    fn jump_label_init(); fn cpu_feature_keys_init(); fn mmu_feature_keys_init();
    fn init_section_contains(p: *mut c_void, size: usize) -> bool;
    fn pr_devel(fmt: *const c_char, ...); fn pr_info(fmt: *const c_char, ...);
    static mut interrupt_exit_not_reentrant: c_void;
    static mut exit_flush_lock: c_void;
    static mut stf_barrier_fallback: c_void; static mut entry_flush_fallback: c_void; static mut scv_entry_flush_fallback: c_void;
    static mut cur_cpu_spec: *mut cpu_spec; static mut powerpc_firmware_features: c_ulong;
    fn mmu_feature_keys_init();
}

type c_ulong = usize; type c_long = isize; type c_int = i32; type c_char = i8; type c_void = core::ffi::c_void;
#[repr(C)] pub struct ppc_inst_t { pub word: u64 }
#[repr(C)] pub struct cpu_spec { pub cpu_features: c_ulong, pub mmu_features: c_ulong }

unsafe fn calc_addr(f: *mut fixup_entry, offset: c_long) -> *mut u32 { (f as *mut u8).offset(offset) as *mut u32 }

unsafe fn patch_alt_instruction(src: *mut u32, dest: *mut u32, alt_start: *mut u32, alt_end: *mut u32) -> c_int {
    let mut instr = ppc_inst_read(src);
    if instr_is_relative_branch(ppc_inst_read(src)) {
        let target = branch_target(src);
        if (target < alt_start || target > alt_end) && translate_branch(&mut instr, dest, src) != 0 { return 1; }
    }
    raw_patch_instruction(dest, instr); 0
}

unsafe fn patch_feature_section_mask(value: c_ulong, mask: c_ulong, fcur: *mut fixup_entry) -> c_int {
    let start=calc_addr(fcur,(*fcur).start_off); let end=calc_addr(fcur,(*fcur).end_off);
    let alt_start=calc_addr(fcur,(*fcur).alt_start_off); let alt_end=calc_addr(fcur,(*fcur).alt_end_off);
    if alt_end.offset_from(alt_start) > end.offset_from(start) { return 1; }
    if (value & (*fcur).mask & mask) == ((*fcur).value & mask) { return 0; }
    let mut src=alt_start; let mut dest=start;
    while src < alt_end { if patch_alt_instruction(src,dest,alt_start,alt_end)!=0{return 1;} src=ppc_inst_next(src,src); dest=ppc_inst_next(dest,dest); }
    while dest < end { raw_patch_instruction(dest, ppc_inst(ppc_raw_nop())); dest=dest.add(1); } 0
}

unsafe fn do_feature_fixups_mask(value:c_ulong, mask:c_ulong, fixup_start:*mut c_void, fixup_end:*mut c_void) {
    let mut f=fixup_start as *mut fixup_entry; let end=fixup_end as *mut fixup_entry;
    while f < end { if patch_feature_section_mask(value,mask,f)!=0 { warn_on(true); printk(b"Unable to patch feature section\0".as_ptr() as *const c_char,calc_addr(f,(*f).start_off),calc_addr(f,(*f).end_off),calc_addr(f,(*f).alt_start_off),calc_addr(f,(*f).alt_end_off)); } f=f.add(1); }
}

#[no_mangle] pub unsafe extern "C" fn do_feature_fixups(value:c_ulong,start:*mut c_void,end:*mut c_void){do_feature_fixups_mask(value,!0,start,end)}

#[cfg(feature="CONFIG_PPC_BARRIER_NOSPEC")]
unsafe fn is_fixup_addr_valid(dest:*mut c_void,size:usize)->bool { true /* system_state < SYSTEM_FREEING_INITMEM || !init_section_contains(dest,size) */ }

#[cfg(feature="CONFIG_PPC_BARRIER_NOSPEC")]
unsafe fn do_patch_fixups(mut start:*mut c_long,end:*mut c_long,instrs:*mut u32,num:c_int)->c_int { let mut i=0; while start<end { let dest=(start as *mut u8).offset(*start) as *mut u32; if is_fixup_addr_valid(dest,4*num as usize){for j in 0..num{patch_instruction(dest.add(j as usize),ppc_inst(*instrs.add(j as usize)));}} start=start.add(1);i+=1;}i }

#[cfg(feature="CONFIG_PPC_BARRIER_NOSPEC")]
pub unsafe extern "C" fn do_barrier_nospec_fixups_range(enable:bool,start:*mut c_void,end:*mut c_void){let mut instr=ppc_raw_nop();if enable{instr=0;pr_info(b"barrier-nospec: using ORI speculation barrier\n\0".as_ptr() as *const c_char);}do_patch_fixups(start as *mut c_long,end as *mut c_long,&mut instr,1);}

static mut saved_cpu_features:c_ulong=0; static mut saved_mmu_features:c_ulong=0;
#[cfg(feature="CONFIG_PPC64")] static mut saved_firmware_features:c_ulong=0;

#[no_mangle] pub unsafe extern "C" fn apply_feature_fixups(){let spec=*cur_cpu_spec;saved_cpu_features=(*spec).cpu_features;saved_mmu_features=(*spec).mmu_features;do_feature_fixups((*spec).cpu_features,core::ptr::null_mut(),core::ptr::null_mut());do_feature_fixups((*spec).mmu_features,core::ptr::null_mut(),core::ptr::null_mut());do_final_fixups();}
#[no_mangle] pub unsafe extern "C" fn update_mmu_feature_fixups(mask:c_ulong){saved_mmu_features=(saved_mmu_features&!mask)|((*cur_cpu_spec).mmu_features&mask);}
#[no_mangle] pub unsafe extern "C" fn setup_feature_keys(){jump_label_init();cpu_feature_keys_init();mmu_feature_keys_init();static_key_feature_checks_initialized=true;}
unsafe fn do_final_fixups() {}

#[no_mangle] pub unsafe extern "C" fn do_lwsync_fixups(value:c_ulong,start:*mut c_void,end:*mut c_void){
    const CPU_FTR_LWSYNC:c_ulong=1<<0;
    if value&CPU_FTR_LWSYNC==0{return;} let mut p=start as *mut c_long;let e=end as *mut c_long;
    while p<e {let dest=(p as *mut u8).offset(*p) as *mut u32;raw_patch_instruction(dest,ppc_inst(0));p=p.add(1);}
}

/* The following entry points retain the source's conditional interfaces.  The
 * instruction encoders, linker section bounds, feature enums, and logging
 * primitives are external kernel dependencies. */
#[cfg(feature="CONFIG_PPC_BARRIER_NOSPEC")]
#[no_mangle] pub unsafe extern "C" fn do_barrier_nospec_fixups(enable:bool){do_barrier_nospec_fixups_range(enable,core::ptr::null_mut(),core::ptr::null_mut());}

#[cfg(feature="CONFIG_PPC_BARRIER_NOSPEC")]
#[no_mangle] pub unsafe extern "C" fn do_stf_barrier_fixups(types:u32){
    mutex_lock(&mut exit_flush_lock); static_branch_enable(&mut interrupt_exit_not_reentrant);
    /* stop_machine(__do_stf_barrier_fixups, &types, NULL); */
    if types!=0 { static_branch_disable(&mut interrupt_exit_not_reentrant); } mutex_unlock(&mut exit_flush_lock);
}

#[cfg(feature="CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub unsafe extern "C" fn do_uaccess_flush_fixups(_types:u32) {}
#[cfg(feature="CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub unsafe extern "C" fn do_entry_flush_fixups(_types:u32) {}
#[cfg(feature="CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub unsafe extern "C" fn do_rfi_flush_fixups(types:u32){mutex_lock(&mut exit_flush_lock);static_branch_enable(&mut interrupt_exit_not_reentrant);if types!=0{static_branch_disable(&mut interrupt_exit_not_reentrant);}mutex_unlock(&mut exit_flush_lock);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
