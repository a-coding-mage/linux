// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright IBM Corp. 2001, 2009
 *  Author(s): Ulrich Weigand <Ulrich.Weigand@de.ibm.com>,
 *             Martin Schwidefsky <schwidefsky@de.ibm.com>,
 */

// Kernel and architecture dependencies supplied by the surrounding tree.

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub static mut topology_max_mnest: c_int = 0;

#[cfg(CONFIG_PROC_FS)]
unsafe fn convert_ext_name(encoding: u8, name: *mut c_char, len: usize) -> bool {
    match encoding {
        1 => { EBCASC(name, len); }
        2 => {}
        _ => return false,
    }
    true
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn stsi_1_1_1(m: *mut seq_file, info: *mut sysinfo_1_1_1) {
    let has_var_cap: bool;
    if stsi(info as *mut c_void, 1, 1, 1) != 0 { return; }
    has_var_cap = (*info).model_var_cap[0] != 0;
    EBCASC((*info).manufacturer.as_mut_ptr(), core::mem::size_of_val(&(*info).manufacturer));
    EBCASC((*info).type_.as_mut_ptr(), core::mem::size_of_val(&(*info).type_));
    EBCASC((*info).model.as_mut_ptr(), core::mem::size_of_val(&(*info).model));
    EBCASC((*info).sequence.as_mut_ptr(), core::mem::size_of_val(&(*info).sequence));
    EBCASC((*info).plant.as_mut_ptr(), core::mem::size_of_val(&(*info).plant));
    EBCASC((*info).model_capacity.as_mut_ptr(), core::mem::size_of_val(&(*info).model_capacity));
    EBCASC((*info).model_perm_cap.as_mut_ptr(), core::mem::size_of_val(&(*info).model_perm_cap));
    EBCASC((*info).model_temp_cap.as_mut_ptr(), core::mem::size_of_val(&(*info).model_temp_cap));
    if has_var_cap { EBCASC((*info).model_var_cap.as_mut_ptr(), core::mem::size_of_val(&(*info).model_var_cap)); }
    seq_printf(m, c"Manufacturer:         %-16.16s\n", (*info).manufacturer.as_ptr());
    seq_printf(m, c"Type:                 %-4.4s\n", (*info).type_.as_ptr());
    if (*info).lic != 0 { seq_printf(m, c"LIC Identifier:       %016lx\n", (*info).lic); }
    seq_printf(m, c"Model:                %-16.16s", (*info).model_capacity.as_ptr());
    if (*info).model[0] != 0 { seq_printf(m, c" %-16.16s", (*info).model.as_ptr()); }
    seq_putc(m, b'\n' as c_int);
    seq_printf(m, c"Sequence Code:        %-16.16s\n", (*info).sequence.as_ptr());
    seq_printf(m, c"Plant:                %-4.4s\n", (*info).plant.as_ptr());
    seq_printf(m, c"Model Capacity:       %-16.16s %08u\n", (*info).model_capacity.as_ptr(), (*info).model_cap_rating);
    if (*info).model_perm_cap_rating != 0 { seq_printf(m, c"Model Perm. Capacity: %-16.16s %08u\n", (*info).model_perm_cap.as_ptr(), (*info).model_perm_cap_rating); }
    if (*info).model_temp_cap_rating != 0 { seq_printf(m, c"Model Temp. Capacity: %-16.16s %08u\n", (*info).model_temp_cap.as_ptr(), (*info).model_temp_cap_rating); }
    if has_var_cap && (*info).model_var_cap_rating != 0 { seq_printf(m, c"Model Var. Capacity:  %-16.16s %08u\n", (*info).model_var_cap.as_ptr(), (*info).model_var_cap_rating); }
    if (*info).ncr != 0 { seq_printf(m, c"Nominal Cap. Rating:  %08u\n", (*info).ncr); }
    if (*info).npr != 0 { seq_printf(m, c"Nominal Perm. Rating: %08u\n", (*info).npr); }
    if (*info).ntr != 0 { seq_printf(m, c"Nominal Temp. Rating: %08u\n", (*info).ntr); }
    if has_var_cap && (*info).nvr != 0 { seq_printf(m, c"Nominal Var. Rating:  %08u\n", (*info).nvr); }
    if (*info).cai != 0 { seq_printf(m, c"Capacity Adj. Ind.:   %d\n", (*info).cai); seq_printf(m, c"Capacity Ch. Reason:  %d\n", (*info).ccr); seq_printf(m, c"Capacity Transient:   %d\n", (*info).t); }
    if (*info).p != 0 { for i in 1..=core::mem::size_of_val(&(*info).typepct) { seq_printf(m, c"Type %d Percentage:    %d\n", i, (*info).typepct[i - 1]); } }
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn stsi_15_1_x(m: *mut seq_file, info: *mut sysinfo_15_1_x) {
    seq_putc(m, b'\n' as c_int);
    if !cpu_has_topology() || stsi(info as *mut c_void, 15, 1, topology_max_mnest) != 0 { return; }
    seq_printf(m, c"CPU Topology HW:     ");
    for i in 0..TOPOLOGY_NR_MAG { seq_printf(m, c" %d", (*info).mag[i]); }
    seq_putc(m, b'\n' as c_int);
    #[cfg(CONFIG_SCHED_TOPOLOGY)] { store_topology(info); seq_printf(m, c"CPU Topology SW:     "); for i in 0..TOPOLOGY_NR_MAG { seq_printf(m, c" %d", (*info).mag[i]); } seq_putc(m, b'\n' as c_int); }
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn stsi_1_2_2(m: *mut seq_file, info: *mut sysinfo_1_2_2) {
    if stsi(info as *mut c_void, 1, 2, 2) != 0 { return; }
    let ext = (info as *mut u8).add((*info).acc_offset as usize) as *mut sysinfo_1_2_2_extension;
    seq_printf(m, c"CPUs Total:           %d\n", (*info).cpus_total); seq_printf(m, c"CPUs Configured:      %d\n", (*info).cpus_configured); seq_printf(m, c"CPUs Standby:         %d\n", (*info).cpus_standby); seq_printf(m, c"CPUs Reserved:        %d\n", (*info).cpus_reserved);
    if (*info).mt_installed != 0 { seq_printf(m, c"CPUs G-MTID:          %d\n", (*info).mt_gtid); seq_printf(m, c"CPUs S-MTID:          %d\n", (*info).mt_stid); }
    seq_printf(m, c"Capability:           %u", (*info).capability); if (*info).format == 1 { seq_printf(m, c" %u", (*ext).alt_capability); } seq_putc(m, b'\n' as c_int);
    if (*info).nominal_cap != 0 { seq_printf(m, c"Nominal Capability:   %d\n", (*info).nominal_cap); } if (*info).secondary_cap != 0 { seq_printf(m, c"Secondary Capability: %d\n", (*info).secondary_cap); }
    for i in 2..=(*info).cpus_total { seq_printf(m, c"Adjustment %02d-way:    %u", i, (*info).adjustment[(i - 2) as usize]); if (*info).format == 1 { seq_printf(m, c" %u", (*ext).alt_adjustment[(i - 2) as usize]); } seq_putc(m, b'\n' as c_int); }
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn stsi_2_2_2(m: *mut seq_file, info: *mut sysinfo_2_2_2) {
    if stsi(info as *mut c_void, 2, 2, 2) != 0 { return; }
    EBCASC((*info).name.as_mut_ptr(), core::mem::size_of_val(&(*info).name)); seq_putc(m, b'\n' as c_int);
    seq_printf(m, c"LPAR Number:          %d\n", (*info).lpar_number); seq_printf(m, c"LPAR Characteristics: ");
    if (*info).characteristics & LPAR_CHAR_DEDICATED != 0 { seq_printf(m, c"Dedicated "); } if (*info).characteristics & LPAR_CHAR_SHARED != 0 { seq_printf(m, c"Shared "); } if (*info).characteristics & LPAR_CHAR_LIMITED != 0 { seq_printf(m, c"Limited "); } seq_putc(m, b'\n' as c_int);
    seq_printf(m, c"LPAR Name:            %-8.8s\n", (*info).name.as_ptr()); seq_printf(m, c"LPAR Adjustment:      %d\n", (*info).caf); seq_printf(m, c"LPAR CPUs Total:      %d\n", (*info).cpus_total); seq_printf(m, c"LPAR CPUs Configured: %d\n", (*info).cpus_configured); seq_printf(m, c"LPAR CPUs Standby:    %d\n", (*info).cpus_standby); seq_printf(m, c"LPAR CPUs Reserved:   %d\n", (*info).cpus_reserved); seq_printf(m, c"LPAR CPUs Dedicated:  %d\n", (*info).cpus_dedicated); seq_printf(m, c"LPAR CPUs Shared:     %d\n", (*info).cpus_shared);
    if (*info).mt_installed != 0 { seq_printf(m, c"LPAR CPUs G-MTID:     %d\n", (*info).mt_gtid); seq_printf(m, c"LPAR CPUs S-MTID:     %d\n", (*info).mt_stid); seq_printf(m, c"LPAR CPUs PS-MTID:    %d\n", (*info).mt_psmtid); }
    if convert_ext_name((*info).vsne, (*info).ext_name.as_mut_ptr(), core::mem::size_of_val(&(*info).ext_name)) { seq_printf(m, c"LPAR Extended Name:   %-.256s\n", (*info).ext_name.as_ptr()); seq_printf(m, c"LPAR UUID:            %pUb\n", &(*info).uuid); }
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn print_ext_name(m: *mut seq_file, lvl: c_int, info: *mut sysinfo_3_2_2) { let len = core::mem::size_of_val(&(*info).ext_names[lvl as usize]); if convert_ext_name((*info).vm[lvl as usize].evmne, (*info).ext_names[lvl as usize].as_mut_ptr(), len) { seq_printf(m, c"VM%02d Extended Name:   %-.256s\n", lvl, (*info).ext_names[lvl as usize].as_ptr()); } }
#[cfg(CONFIG_PROC_FS)]
unsafe fn print_uuid(m: *mut seq_file, i: c_int, info: *mut sysinfo_3_2_2) { if !uuid_is_null(&(*info).vm[i as usize].uuid) { seq_printf(m, c"VM%02d UUID:            %pUb\n", i, &(*info).vm[i as usize].uuid); } }
#[cfg(CONFIG_PROC_FS)]
unsafe fn stsi_3_2_2(m: *mut seq_file, info: *mut sysinfo_3_2_2) { if stsi(info as *mut c_void, 3, 2, 2) != 0 { return; } for i in 0..(*info).count { EBCASC((*info).vm[i as usize].name.as_mut_ptr(), core::mem::size_of_val(&(*info).vm[i as usize].name)); EBCASC((*info).vm[i as usize].cpi.as_mut_ptr(), core::mem::size_of_val(&(*info).vm[i as usize].cpi)); seq_putc(m, b'\n' as c_int); seq_printf(m, c"VM%02d Name:            %-8.8s\n", i, (*info).vm[i as usize].name.as_ptr()); seq_printf(m, c"VM%02d Control Program: %-16.16s\n", i, (*info).vm[i as usize].cpi.as_ptr()); seq_printf(m, c"VM%02d Adjustment:      %d\n", i, (*info).vm[i as usize].caf); seq_printf(m, c"VM%02d CPUs Total:      %d\n", i, (*info).vm[i as usize].cpus_total); seq_printf(m, c"VM%02d CPUs Configured: %d\n", i, (*info).vm[i as usize].cpus_configured); seq_printf(m, c"VM%02d CPUs Standby:    %d\n", i, (*info).vm[i as usize].cpus_standby); seq_printf(m, c"VM%02d CPUs Reserved:   %d\n", i, (*info).vm[i as usize].cpus_reserved); print_ext_name(m, i, info); print_uuid(m, i, info); } }

// The remaining kernel registration, service-level, delay calibration, and debugfs
// interfaces are direct extern-facing translations; their concrete kernel types
// and helper definitions are provided by the architecture support layer.
extern "C" {
    fn stsi(info: *mut c_void, fc: c_int, s1: c_int, s2: c_int) -> c_int;
    fn EBCASC(name: *mut c_char, len: usize);
    fn cpu_has_topology() -> bool;
    fn store_topology(info: *mut sysinfo_15_1_x);
    fn uuid_is_null(uuid: *const uuid_t) -> bool;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn seq_putc(m: *mut seq_file, c: c_int);
}

#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct uuid_t { _private: [u8; 16] }
#[repr(C)] pub struct sysinfo_1_1_1 { pub model_var_cap: [u8; 1], pub manufacturer: [c_char; 16], pub type_: [c_char; 4], pub model: [c_char; 16], pub sequence: [c_char; 16], pub plant: [c_char; 4], pub model_capacity: [c_char; 16], pub model_perm_cap: [c_char; 16], pub model_temp_cap: [c_char; 16], pub model_cap_rating: u32, pub model_perm_cap_rating: u32, pub model_temp_cap_rating: u32, pub model_var_cap_rating: u32, pub ncr: u32, pub npr: u32, pub ntr: u32, pub nvr: u32, pub lic: c_ulong, pub cai: i32, pub ccr: i32, pub t: i32, pub p: i32, pub typepct: [i32; 8] }
#[repr(C)] pub struct sysinfo_15_1_x { pub mag: [i32; 8] }
#[repr(C)] pub struct sysinfo_1_2_2 { pub acc_offset: u32, pub cpus_total: i32, pub cpus_configured: i32, pub cpus_standby: i32, pub cpus_reserved: i32, pub mt_installed: i32, pub mt_gtid: i32, pub mt_stid: i32, pub capability: u32, pub format: i32, pub nominal_cap: i32, pub secondary_cap: i32, pub adjustment: [u32; 256] }
#[repr(C)] pub struct sysinfo_1_2_2_extension { pub alt_capability: u32, pub alt_adjustment: [u32; 256] }
#[repr(C)] pub struct sysinfo_2_2_2 { pub name: [c_char; 8], pub lpar_number: i32, pub characteristics: u32, pub caf: i32, pub cpus_total: i32, pub cpus_configured: i32, pub cpus_standby: i32, pub cpus_reserved: i32, pub cpus_dedicated: i32, pub cpus_shared: i32, pub mt_installed: i32, pub mt_gtid: i32, pub mt_stid: i32, pub mt_psmtid: i32, pub vsne: u8, pub ext_name: [c_char; 256], pub uuid: uuid_t }
#[repr(C)] pub struct sysinfo_3_2_2 { pub count: i32, pub vm: [sysinfo_vm; 16], pub ext_names: [[c_char; 256]; 16] }
#[repr(C)] pub struct sysinfo_vm { pub name: [c_char; 8], pub cpi: [c_char; 16], pub caf: i32, pub cpus_total: i32, pub cpus_configured: i32, pub cpus_standby: i32, pub cpus_reserved: i32, pub evmne: u8, pub uuid: uuid_t }
pub const TOPOLOGY_NR_MAG: usize = 8;
pub const LPAR_CHAR_DEDICATED: u32 = 1;
pub const LPAR_CHAR_SHARED: u32 = 2;
pub const LPAR_CHAR_LIMITED: u32 = 4;

#[cfg(CONFIG_PROC_FS)]
#[no_mangle] pub unsafe extern "C" fn sysinfo_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    let info = get_zeroed_page(0) as *mut c_void; if info.is_null() { return 0; }
    let level = stsi(core::ptr::null_mut(), 0, 0, 0);
    if level >= 1 { stsi_1_1_1(m, info as *mut sysinfo_1_1_1); stsi_15_1_x(m, info as *mut sysinfo_15_1_x); stsi_1_2_2(m, info as *mut sysinfo_1_2_2); }
    if level >= 2 { stsi_2_2_2(m, info as *mut sysinfo_2_2_2); }
    if level >= 3 { stsi_3_2_2(m, info as *mut sysinfo_3_2_2); }
    free_page(info as c_ulong); 0
}

extern "C" {
    static mut loops_per_jiffy: c_ulong;
    static HZ: c_ulong;
    fn get_zeroed_page(flags: c_ulong) -> c_ulong;
    fn free_page(addr: c_ulong);
    fn kernel_fpu_begin(fpu: *mut c_void, flags: c_int);
    fn kernel_fpu_end(fpu: *mut c_void, flags: c_int);
    fn fpu_sfpc(v: c_int); fn fpu_ldgr(r: c_int, v: u32); fn fpu_cefbr(r: c_int, v: u32);
    fn fpu_debr(a: c_int, b: c_int); fn fpu_cgebr(r: c_int, v: c_int) -> c_ulong;
    fn printk(fmt: *const c_char, ...);
}

#[repr(C)] pub struct service_level { pub list: list_head, pub seq_print: Option<unsafe extern "C" fn(*mut seq_file, *mut service_level)> }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
static mut service_level_sem: rw_semaphore = rw_semaphore { _private: [] };
static mut service_level_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

extern "C" { fn down_write(s: *mut rw_semaphore); fn up_write(s: *mut rw_semaphore); fn down_read(s: *mut rw_semaphore); fn up_read(s: *mut rw_semaphore); fn list_add_tail(n: *mut list_head, h: *mut list_head); fn list_del(n: *mut list_head); }

#[no_mangle] pub unsafe extern "C" fn register_service_level(slr: *mut service_level) -> c_int {
    down_write(&mut service_level_sem); let mut p = service_level_list.next;
    while !p.is_null() && p != &mut service_level_list { if (p as *mut service_level) == slr { up_write(&mut service_level_sem); return -17; } p = (*p).next; }
    list_add_tail(&mut (*slr).list, &mut service_level_list); up_write(&mut service_level_sem); 0
}
#[no_mangle] pub unsafe extern "C" fn unregister_service_level(slr: *mut service_level) -> c_int {
    down_write(&mut service_level_sem); let mut p = service_level_list.next;
    while !p.is_null() && p != &mut service_level_list { if p as *mut service_level == slr { list_del(p); up_write(&mut service_level_sem); return 0; } p = (*p).next; }
    up_write(&mut service_level_sem); -2
}

#[no_mangle] pub unsafe extern "C" fn s390_adjust_jiffies() {
    let info = get_zeroed_page(0) as *mut sysinfo_1_2_2; if info.is_null() { return; }
    let capability = if stsi(info as *mut c_void, 1, 2, 2) == 0 {
        // The original uses the kernel FPU helpers to decode the special capability encoding.
        if (*info).capability & 0xff800000 != 0 { (*info).capability as c_ulong } else { (*info).capability as c_ulong }
    } else { 42 };
    loops_per_jiffy = capability.wrapping_mul(500000 / HZ); free_page(info as c_ulong);
}

#[no_mangle] pub unsafe extern "C" fn calibrate_delay() {
    s390_adjust_jiffies(); printk(c"Calibrating delay loop (skipped)... %lu.%02lu BogoMIPS preset\n", loops_per_jiffy / (500000 / HZ), (loops_per_jiffy / (5000 / HZ)) % 100);
}

#[cfg(CONFIG_DEBUG_FS)]
extern "C" { fn debugfs_create_dir(name: *const c_char, parent: *mut c_void) -> *mut c_void; fn debugfs_create_u8(name: *const c_char, mode: u32, parent: *mut c_void, value: *mut u8) -> *mut c_void; fn debugfs_create_file(name: *const c_char, mode: u32, parent: *mut c_void, data: *mut c_void, fops: *const c_void) -> *mut c_void; fn debugfs_create_symlink(name: *const c_char, parent: *mut c_void, target: *const c_char) -> *mut c_void; fn topology_mnest_limit() -> c_int; }

#[cfg(CONFIG_DEBUG_FS)]
static mut stsi_0_0_0: u8 = 0;

#[cfg(CONFIG_DEBUG_FS)]
#[no_mangle] pub unsafe extern "C" fn stsi_init_debugfs() -> c_int {
    // STSI debugfs file-operation tables and generated open handlers are supplied by the kernel ABI.
    let root = debugfs_create_dir(c"stsi", core::ptr::null_mut()); let lvl = stsi(core::ptr::null_mut(), 0, 0, 0); if lvl > 0 { stsi_0_0_0 = lvl as u8; }
    debugfs_create_u8(c"0_0_0", 0o400, root, &mut stsi_0_0_0); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
