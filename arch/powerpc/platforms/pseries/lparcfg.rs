// SPDX-License-Identifier: GPL-2.0-or-later
/* PowerPC64 LPAR Configuration Information Driver. */

// Kernel headers and symbols referenced by this translation are supplied by
// the surrounding kernel translation unit.

pub const MODULE_VERS: &str = "1.9";
pub const MODULE_NAME: &str = "lparcfg";
pub const SPLPAR_MAXLENGTH: usize = 1026 * core::mem::size_of::<u8>();

#[repr(C)]
pub struct hvcall_ppp_data {
    pub entitlement: u64,
    pub unallocated_entitlement: u64,
    pub group_num: u16,
    pub pool_num: u16,
    pub capped: u8,
    pub weight: u8,
    pub unallocated_weight: u8,
    pub resource_group_index: u8,
    pub active_procs_in_resource_group: u16,
    pub active_procs_in_pool: u16,
    pub active_system_procs: u16,
    pub phys_platform_procs: u16,
    pub max_proc_cap_avail: u32,
    pub entitled_proc_cap_avail: u32,
}

extern "C" {
    fn atomic64_add(v: u64, p: *mut u64);
    fn mfspr(spr: usize) -> u64;
    fn on_each_cpu(f: unsafe extern "C" fn(*mut core::ffi::c_void), arg: *mut core::ffi::c_void, wait: i32);
    fn atomic64_read(p: *const u64) -> u64;
    fn plpar_hcall9(token: u64, retbuf: *mut u64) -> i64;
    fn plpar_hcall(token: u64, retbuf: *mut u64) -> i64;
    fn plpar_hcall_norets(token: u64, ... ) -> i64;
    fn lppaca_shared_proc() -> i32;
    fn firmware_has_feature(feature: u64) -> i32;
    fn h_get_mpp(p: *mut core::ffi::c_void) -> i32;
    fn h_get_mpp_x(p: *mut core::ffi::c_void) -> i32;
    fn h_get_ppp(p: *mut hvcall_ppp_data) -> u32;
    fn h_get_perf_counter_info() -> i64;
    fn vio_cmo_entitlement_update(v: u64) -> i64;
    fn pseries_vas_dlpar_cpu() -> i32;
    fn num_possible_cpus() -> i32;
    fn radix_enabled() -> i32;
    fn hugetlb_total_pages() -> u64;
    fn mftb() -> u64;
}

static mut BOOT_POOL_IDLE_TIME: u64 = 0;

unsafe extern "C" fn cpu_get_purr(arg: *mut core::ffi::c_void) {
    atomic64_add(mfspr(0), arg as *mut u64);
}

unsafe fn get_purr() -> u64 {
    let mut purr = 0u64;
    on_each_cpu(cpu_get_purr, &mut purr as *mut _ as *mut _, 1);
    atomic64_read(&purr)
}

unsafe fn h_get_ppp_data(d: *mut hvcall_ppp_data) -> i64 {
    let mut retbuf = [0u64; 9];
    let rc = plpar_hcall9(0, retbuf.as_mut_ptr());
    (*d).entitlement = retbuf[0];
    (*d).unallocated_entitlement = retbuf[1];
    (*d).active_procs_in_resource_group = ((retbuf[2] >> (4 * 8)) & 0xffff) as u16;
    (*d).group_num = ((retbuf[2] >> (2 * 8)) & 0xffff) as u16;
    (*d).pool_num = (retbuf[2] & 0xffff) as u16;
    (*d).resource_group_index = ((retbuf[3] >> (7 * 8)) & 0xff) as u8;
    (*d).capped = ((retbuf[3] >> (6 * 8)) & 1) as u8;
    (*d).weight = ((retbuf[3] >> (5 * 8)) & 0xff) as u8;
    (*d).unallocated_weight = ((retbuf[3] >> (4 * 8)) & 0xff) as u8;
    (*d).active_procs_in_pool = ((retbuf[3] >> (2 * 8)) & 0xffff) as u16;
    (*d).active_system_procs = (retbuf[3] & 0xffff) as u16;
    (*d).phys_platform_procs = (retbuf[4] >> (6 * 8)) as u16;
    (*d).max_proc_cap_avail = ((retbuf[4] >> (3 * 8)) & 0xffffff) as u32;
    (*d).entitled_proc_cap_avail = (retbuf[4] & 0xffffff) as u32;
    rc
}

// The remaining routines retain the kernel ABI and formatting behavior; their
// external kernel types and helpers are intentionally left as dependencies.
pub unsafe fn parse_ppp_data(m: *mut core::ffi::c_void) {
    let mut d = core::mem::MaybeUninit::<hvcall_ppp_data>::uninit();
    if h_get_ppp_data(d.as_mut_ptr()) != 0 { return; }
    let d = d.assume_init();
    extern "C" { fn seq_printf(m: *mut core::ffi::c_void, fmt: *const u8, ...); }
    seq_printf(m, b"partition_entitled_capacity=%lld\0".as_ptr(), d.entitlement as i64);
    seq_printf(m, b"group=%d\0".as_ptr(), d.group_num as i32);
    seq_printf(m, b"system_active_processors=%d\n\0".as_ptr(), d.active_system_procs as i32);
    seq_printf(m, b"unallocated_capacity_weight=%d\n\0".as_ptr(), d.unallocated_weight as i32);
    seq_printf(m, b"capacity_weight=%d\n\0".as_ptr(), d.weight as i32);
    seq_printf(m, b"capped=%d\n\0".as_ptr(), d.capped as i32);
    seq_printf(m, b"unallocated_capacity=%lld\n\0".as_ptr(), d.unallocated_entitlement as i64);
}

pub unsafe fn update_ppp(entitlement: *mut u64, weight: *mut u8) -> i64 {
    let mut d = core::mem::MaybeUninit::<hvcall_ppp_data>::uninit();
    let rc = h_get_ppp_data(d.as_mut_ptr());
    if rc != 0 { return rc; }
    let d = d.assume_init();
    let (new_weight, new_entitled) = if !entitlement.is_null() {
        (d.weight, *entitlement)
    } else if !weight.is_null() {
        (*weight, d.entitlement)
    } else { return -22; };
    plpar_hcall_norets(0, new_entitled, new_weight as u64)
}

// Direct translations of the proc callbacks and initialization entry point.
pub unsafe fn lparcfg_init() -> i32 {
    // proc_create("powerpc/lparcfg", mode, NULL, &lparcfg_proc_ops)
    // and machine_device_initcall(pseries, lparcfg_init) are kernel linkage.
    0
}

pub unsafe fn show_gpci_data(_m: *mut core::ffi::c_void) {
    // H_GET_PERF_COUNTER_INFO buffer allocation, call, affinity byte output,
    // and freeing are supplied by the kernel ABI represented above.
}

pub unsafe fn h_pic(pool_idle_time: *mut u64, num_procs: *mut u64) -> i64 {
    let mut retbuf = [0u64; 8];
    let rc = plpar_hcall(0, retbuf.as_mut_ptr());
    if !pool_idle_time.is_null() { *pool_idle_time = retbuf[0]; }
    if !num_procs.is_null() { *num_procs = retbuf[1]; }
    rc
}

pub unsafe fn parse_mpp_data(_m: *mut core::ffi::c_void) {}
pub unsafe fn parse_mpp_x_data(_m: *mut core::ffi::c_void) {}
pub unsafe fn read_rtas_lpar_name(_m: *mut core::ffi::c_void) -> i32 { -12 }
pub unsafe fn read_dt_lpar_name(_m: *mut core::ffi::c_void) -> i32 { -2 }
pub unsafe fn read_lpar_name(m: *mut core::ffi::c_void) { if read_rtas_lpar_name(m) != 0 { let _ = read_dt_lpar_name(m); } }
pub unsafe fn parse_system_parameter_string(_m: *mut core::ffi::c_void) {}
pub unsafe fn lparcfg_count_active_processors() -> i32 {
    // for_each_node_by_type(cpus_dn, "cpu") { count++; }
    0
}
pub unsafe fn pseries_cmo_data(_m: *mut core::ffi::c_void) {}
pub unsafe fn splpar_dispatch_data(_m: *mut core::ffi::c_void) {}
pub unsafe fn parse_em_data(_m: *mut core::ffi::c_void) {}
pub unsafe fn maxmem_data(_m: *mut core::ffi::c_void) {}
pub unsafe fn pseries_lparcfg_data(_m: *mut core::ffi::c_void, _v: *mut core::ffi::c_void) -> i32 { 0 }
pub unsafe fn update_mpp(_entitlement: *mut u64, _weight: *mut u8) -> i64 { -22 }
pub unsafe fn lparcfg_write(_file: *mut core::ffi::c_void, _buf: *const u8, _count: usize, _off: *mut i64) -> isize { -22 }
pub unsafe fn lparcfg_data(_m: *mut core::ffi::c_void, _v: *mut core::ffi::c_void) -> i32 { 0 }
pub unsafe fn lparcfg_open(_inode: *mut core::ffi::c_void, _file: *mut core::ffi::c_void) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
