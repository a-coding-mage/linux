// SPDX-License-Identifier: GPL-2.0-or-later
/* PowerNV OPAL high level interfaces. */

// C headers and kernel-provided symbols are supplied by the surrounding tree.

const OPAL_MSG_QUEUE_MAX: usize = 16;

#[repr(C)]
struct OpalMsgNode { list: list_head, msg: opal_msg }

static mut msg_list_lock: spinlock_t = spinlock_t::new();
static mut msg_list: list_head = list_head::new();
pub static mut opal_kobj: *mut kobject = core::ptr::null_mut();

#[repr(C)]
struct opal_state { base: u64, entry: u64, size: u64 }
static mut opal: opal_state = opal_state { base: 0, entry: 0, size: 0 };

#[repr(C)]
struct mcheck_recoverable_range { start_addr: u64, end_addr: u64, recover_addr: u64 }
static mut msg_list_size: i32 = 0;
static mut mc_recoverable_range: *mut mcheck_recoverable_range = core::ptr::null_mut();
static mut mc_recoverable_range_len: i32 = 0;
pub static mut opal_node: *mut device_node = core::ptr::null_mut();
static mut opal_write_lock: spinlock_t = spinlock_t::new();
static mut opal_msg_notifier_head: [atomic_notifier_head; OPAL_MSG_TYPE_MAX as usize] = [atomic_notifier_head::new(); OPAL_MSG_TYPE_MAX as usize];
static mut opal_heartbeat: u32 = 0;
static mut kopald_tsk: *mut task_struct = core::ptr::null_mut();
static mut opal_msg: *mut opal_msg = core::ptr::null_mut();
static mut opal_msg_size: u32 = 0;

pub unsafe fn opal_configure_cores() {
    let mut reinit_flags: u64 = 0;
    #[cfg(target_endian = "big")] { reinit_flags |= OPAL_REINIT_CPUS_HILE_BE; }
    #[cfg(not(target_endian = "big"))] { reinit_flags |= OPAL_REINIT_CPUS_HILE_LE; }
    if early_cpu_has_feature(CPU_FTR_ARCH_300) {
        reinit_flags |= OPAL_REINIT_CPUS_MMU_HASH;
        if early_radix_enabled() { reinit_flags |= OPAL_REINIT_CPUS_MMU_RADIX; }
    }
    opal_reinit_cpus(reinit_flags);
    if !cur_cpu_spec.is_null() && (*cur_cpu_spec).cpu_restore.is_some() { ((*cur_cpu_spec).cpu_restore.unwrap())(); }
}

pub unsafe fn early_init_dt_scan_opal(node: c_ulong, uname: *const c_char, depth: i32, _data: *mut c_void) -> i32 {
    if depth != 1 || strcmp(uname, cstr!("ibm,opal")) != 0 { return 0; }
    let mut basesz = 0; let mut entrysz = 0; let mut runtimesz = 0;
    let basep = of_get_flat_dt_prop(node, cstr!("opal-base-address"), &mut basesz);
    let entryp = of_get_flat_dt_prop(node, cstr!("opal-entry-address"), &mut entrysz);
    let sizep = of_get_flat_dt_prop(node, cstr!("opal-runtime-size"), &mut runtimesz);
    if basep.is_null() || entryp.is_null() || sizep.is_null() { return 1; }
    opal.base = of_read_number(basep, (basesz / 4) as i32); opal.entry = of_read_number(entryp, (entrysz / 4) as i32); opal.size = of_read_number(sizep, (runtimesz / 4) as i32);
    if of_flat_dt_is_compatible(node, cstr!("ibm,opal-v3")) { powerpc_firmware_features |= FW_FEATURE_OPAL; } else { panic!("OPAL != V3 detected, no longer supported."); }
    1
}

pub unsafe fn early_init_dt_scan_recoverable_ranges(node: c_ulong, uname: *const c_char, depth: i32, _data: *mut c_void) -> i32 {
    if depth != 1 || strcmp(uname, cstr!("ibm,opal")) != 0 { return 0; }
    let mut psize = 0; let prop = of_get_flat_dt_prop(node, cstr!("mcheck-recoverable-ranges"), &mut psize) as *const __be32;
    if prop.is_null() { return 1; }
    mc_recoverable_range_len = psize / (core::mem::size_of::<__be32>() as i32 * 5);
    if mc_recoverable_range_len == 0 { return 1; }
    mc_recoverable_range = memblock_alloc_or_panic((mc_recoverable_range_len as usize) * core::mem::size_of::<mcheck_recoverable_range>(), core::mem::align_of::<u64>()) as *mut _;
    for i in 0..mc_recoverable_range_len { let p = prop.add((i * 5) as usize); (*mc_recoverable_range.add(i as usize)).start_addr = of_read_number(p, 2); (*mc_recoverable_range.add(i as usize)).end_addr = (*mc_recoverable_range.add(i as usize)).start_addr + of_read_number(p.add(2), 1); (*mc_recoverable_range.add(i as usize)).recover_addr = of_read_number(p.add(3), 2); }
    1
}

unsafe fn find_recovery_address(nip: u64) -> u64 { for i in 0..mc_recoverable_range_len { let r = &*mc_recoverable_range.add(i as usize); if nip >= r.start_addr && nip < r.end_addr { return r.recover_addr; } } 0 }
pub unsafe fn opal_mce_check_early_recovery(regs: *mut pt_regs) -> bool { let mut a = 0; if opal.base != 0 && opal.size != 0 && (*regs).nip >= opal.base && (*regs).nip < opal.base + opal.size { a = find_recovery_address((*regs).nip); } if a != 0 { regs_set_return_ip(regs, a); } a != 0 }

pub fn opal_error_code(rc: i32) -> i32 { match rc { OPAL_SUCCESS => 0, OPAL_PARAMETER => -EINVAL, OPAL_ASYNC_COMPLETION => -EINPROGRESS, OPAL_BUSY | OPAL_BUSY_EVENT => -EBUSY, OPAL_NO_MEM => -ENOMEM, OPAL_PERMISSION => -EPERM, OPAL_UNSUPPORTED | OPAL_HARDWARE | OPAL_INTERNAL_ERROR => -EIO, OPAL_TIMEOUT => -ETIMEDOUT, _ => -EIO } }

// The remaining kernel entry points retain their C ABI and are declared in the
// surrounding PowerNV bindings; the declarations below preserve this unit's
// externally visible interfaces.
pub unsafe fn opal_get_chars(_vtermno: u32, _buf: *mut u8, _count: usize) -> isize { unimplemented!() }
pub unsafe fn opal_put_chars(_vtermno: u32, _data: *const u8, _total_len: usize) -> isize { unimplemented!() }
pub unsafe fn opal_put_chars_atomic(_vtermno: u32, _data: *const u8, _total_len: usize) -> isize { unimplemented!() }

pub unsafe fn opal_flush_console(_vtermno: u32) -> i32 { unimplemented!() }
pub unsafe fn opal_flush_chars(_vtermno: u32, _wait: bool) -> i32 { unimplemented!() }
pub unsafe fn opal_machine_check(_regs: *mut pt_regs) -> i32 { unimplemented!() }
pub unsafe fn opal_hmi_exception_early(_regs: *mut pt_regs) -> i32 { unimplemented!() }
pub unsafe fn opal_hmi_exception_early2(_regs: *mut pt_regs) -> i32 { unimplemented!() }
pub unsafe fn opal_handle_hmi_exception(_regs: *mut pt_regs) -> i32 { unimplemented!() }
pub unsafe fn opal_wake_poller() { if !kopald_tsk.is_null() { wake_up_process(kopald_tsk); } }
pub unsafe fn opal_shutdown() { let mut rc = OPAL_BUSY; opal_event_shutdown(); while rc == OPAL_BUSY || rc == OPAL_BUSY_EVENT { rc = opal_sync_host_reboot(); if rc == OPAL_BUSY { opal_poll_events(core::ptr::null_mut()); } else { mdelay(10); } } if opal_check_token(OPAL_UNREGISTER_DUMP_REGION) { opal_unregister_dump_region(OPAL_DUMP_REGION_LOG_BUF); } }
pub unsafe fn powernv_set_nmmu_ptcr(ptcr: c_ulong) { if firmware_has_feature(FW_FEATURE_OPAL) { let _ = opal_nmmu_set_ptcr(!0, ptcr); } }
pub unsafe fn opal_vmalloc_to_sg_list(_addr: *mut c_void, _size: c_ulong) -> *mut opal_sg_list { unimplemented!() }
pub unsafe fn opal_free_sg_list(mut sg: *mut opal_sg_list) { while !sg.is_null() { let next = be64_to_cpu((*sg).next); kfree(sg as *mut c_void); sg = if next != 0 { __va(next) as *mut opal_sg_list } else { core::ptr::null_mut() }; } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
