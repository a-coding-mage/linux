// SPDX-License-Identifier: GPL-2.0
/*
 *    Time of day based timer functions.
 *
 *  S390 version
 *    Copyright IBM Corp. 1999, 2008
 *    Author(s): Hartmut Penner (hp@de.ibm.com),
 *               Martin Schwidefsky (schwidefsky@de.ibm.com),
 *               Denis Joseph Barrow (djbarrow@de.ibm.com,barrow_dj@yahoo.com)
 *
 *  Derived from "arch/i386/kernel/time.c"
 *    Copyright (C) 1991, 1992, 1995  Linus Torvalds
 */

// C headers and preprocessor definitions are supplied by the surrounding kernel.

#[repr(C)]
pub union tod_clock { pub tod: u64, pub eitod: u64, pub us: u64, pub sus: u64 }

extern "C" {
    static mut tod_clock_base: tod_clock;
    static mut clock_comparator_max: u64;
    static mut s390_epoch_delta_notifier: atomic_notifier_head;
    static mut ptff_function_mask: [u8; 16];
    static mut vdso_k_time_data: *mut vdso_time_data;
}

static mut lpar_offset: usize = 0;
static mut initial_leap_seconds: usize = 0;

static mut comparators: per_cpu<clock_event_device> = per_cpu::new();

#[no_mangle]
pub unsafe extern "C" fn time_early_init() {
    let mut qto: ptff_qto = core::mem::zeroed();
    let mut qui: ptff_qui = core::mem::zeroed();
    (*vdso_k_time_data).arch_data.tod_delta = tod_clock_base.tod;
    if !test_facility(28) { return; }
    ptff(&mut ptff_function_mask as *mut _, core::mem::size_of_val(&ptff_function_mask), PTFF_QAF);
    if ptff_query(PTFF_QTO) && ptff(&mut qto, core::mem::size_of::<ptff_qto>(), PTFF_QTO) == 0 { lpar_offset = qto.tod_epoch_difference; }
    if ptff_query(PTFF_QUI) && ptff(&mut qui, core::mem::size_of::<ptff_qui>(), PTFF_QUI) == 0 { initial_leap_seconds = (qui.old_leap as i64).wrapping_mul(4096000000i64) as usize; }
}

pub unsafe extern "C" fn sched_clock_noinstr() -> u64 { tod_to_ns(__get_tod_clock_monotonic()) }

pub unsafe extern "C" fn sched_clock() -> u64 { tod_to_ns(get_tod_clock_monotonic()) }

unsafe fn ext_to_timespec64(clk: *mut tod_clock, xt: *mut timespec64) {
    let mut sec = (*clk).us;
    let rem = sec % 1000000;
    sec /= 1000000;
    let nsec = (((*clk).sus + (rem << 12)) * 125) >> 9;
    (*xt).tv_sec = sec as _;
    (*xt).tv_nsec = nsec as _;
}

pub unsafe extern "C" fn clock_comparator_work() {
    (*get_lowcore()).clock_comparator = clock_comparator_max;
    let cd = this_cpu_ptr(&mut comparators);
    ((*cd).event_handler)(cd);
}

unsafe extern "C" fn s390_next_event(delta: usize, _evt: *mut clock_event_device) -> i32 {
    (*get_lowcore()).clock_comparator = get_tod_clock().wrapping_add(delta as u64);
    set_clock_comparator((*get_lowcore()).clock_comparator); 0
}

pub unsafe extern "C" fn init_cpu_timer() {
    (*get_lowcore()).clock_comparator = clock_comparator_max;
    set_clock_comparator((*get_lowcore()).clock_comparator);
    let cpu = smp_processor_id();
    let cd = per_cpu_ptr(&mut comparators, cpu);
    (*cd).name = b"comparator\0".as_ptr() as *const _;
    (*cd).features = CLOCK_EVT_FEAT_ONESHOT;
    (*cd).mult = 16777; (*cd).shift = 12; (*cd).min_delta_ns = 1; (*cd).min_delta_ticks = 1;
    (*cd).max_delta_ns = LONG_MAX; (*cd).max_delta_ticks = ULONG_MAX; (*cd).rating = 400;
    (*cd).cpumask = cpumask_of(cpu); (*cd).set_next_event = Some(s390_next_event);
    clockevents_register_device(cd);
    local_ctl_set_bit(0, CR0_CLOCK_COMPARATOR_SUBMASK_BIT);
    local_ctl_set_bit(0, CR0_ETR_SUBMASK_BIT);
}

unsafe extern "C" fn clock_comparator_interrupt(_: ext_code, _: u32, _: usize) {
    inc_irq_stat(IRQEXT_CLK);
    if (*get_lowcore()).clock_comparator == clock_comparator_max { set_clock_comparator((*get_lowcore()).clock_comparator); }
}
unsafe extern "C" fn timing_alert_interrupt(_: ext_code, param32: u32, _: usize) {
    inc_irq_stat(IRQEXT_TLA);
    if param32 & 0x00038000 != 0 { stp_timing_alert(&param32 as *const _ as *mut stp_irq_parm); }
}

pub unsafe extern "C" fn read_persistent_clock64(ts: *mut timespec64) {
    let delta = initial_leap_seconds as u64 + TOD_UNIX_EPOCH;
    let mut clk: tod_clock = core::mem::zeroed(); store_tod_clock_ext(&mut clk); clk.eitod = clk.eitod.wrapping_sub(delta); ext_to_timespec64(&mut clk, ts);
}

pub unsafe extern "C" fn read_persistent_wall_and_boot_offset(wall_time: *mut timespec64, boot_offset: *mut timespec64) {
    let mut boot_time: timespec64 = core::mem::zeroed(); let delta = initial_leap_seconds as u64 + TOD_UNIX_EPOCH;
    let mut clk = tod_clock_base; clk.eitod = clk.eitod.wrapping_sub(delta); ext_to_timespec64(&mut clk, &mut boot_time);
    read_persistent_clock64(wall_time); *boot_offset = timespec64_sub(*wall_time, boot_time);
}

unsafe extern "C" fn read_tod_clock(_: *mut clocksource) -> u64 { get_tod_clock_monotonic() }

static mut clocksource_tod: clocksource = clocksource { name: b"tod\0".as_ptr() as *const _, rating: 400, read: Some(read_tod_clock), mask: CLOCKSOURCE_MASK(64), mult: 4096000, shift: 24, flags: CLOCK_SOURCE_IS_CONTINUOUS, vdso_clock_mode: VDSO_CLOCKMODE_TOD, id: CSID_S390_TOD };

pub unsafe extern "C" fn clocksource_default_clock() -> *mut clocksource { &mut clocksource_tod }

pub unsafe extern "C" fn time_init() {
    stp_reset();
    if register_external_irq(EXT_IRQ_CLK_COMP, Some(clock_comparator_interrupt)) != 0 { panic!("Couldn't request external interrupt 0x1004"); }
    if register_external_irq(EXT_IRQ_TIMING_ALERT, Some(timing_alert_interrupt)) != 0 { panic!("Couldn't request external interrupt 0x1406"); }
    if __clocksource_register(&mut clocksource_tod) != 0 { panic!("Could not register TOD clock source"); }
    init_cpu_timer(); vtime_init();
}

static mut clock_sync_word: per_cpu<atomic_t> = per_cpu::new();
static mut stp_mutex: mutex = mutex::new();
static mut clock_sync_flags: usize = 0;
const CLOCK_SYNC_HAS_STP: usize = 0; const CLOCK_SYNC_STP: usize = 1; const CLOCK_SYNC_STPINFO_VALID: usize = 2;

pub unsafe extern "C" fn get_phys_clock(clock: *mut usize) -> i32 {
    let sw_ptr = get_cpu_var(&mut clock_sync_word); let sw0 = atomic_read(sw_ptr); *clock = get_tod_clock() as usize - lpar_offset; let sw1 = atomic_read(sw_ptr); put_cpu_var(&mut clock_sync_word);
    if sw0 == sw1 && sw0 & 0x80000000 != 0 { return 0; }
    if !test_bit(CLOCK_SYNC_HAS_STP, &clock_sync_flags) { return -EOPNOTSUPP; }
    if !test_bit(CLOCK_SYNC_STP, &clock_sync_flags) { return -EACCES; } -EAGAIN
}

unsafe fn disable_sync_clock(_: *mut core::ffi::c_void) { let p = this_cpu_ptr(&mut clock_sync_word); atomic_andnot(0x80000000, p); atomic_inc(p); }
unsafe fn enable_sync_clock() { atomic_or(0x80000000, this_cpu_ptr(&mut clock_sync_word)); }
unsafe fn check_sync_clock() -> i32 { let p = get_cpu_var(&mut clock_sync_word); let rc = ((atomic_read(p) & 0x80000000) != 0) as i32; put_cpu_var(&mut clock_sync_word); rc }

unsafe fn clock_sync_global(delta: i64) { let mut qto: ptff_qto = core::mem::zeroed(); tod_clock_base.eitod = tod_clock_base.eitod.wrapping_add(delta as u64); (*vdso_k_time_data).arch_data.tod_delta = tod_clock_base.tod; if ptff_query(PTFF_QTO) && ptff(&mut qto, core::mem::size_of::<ptff_qto>(), PTFF_QTO) == 0 { lpar_offset = qto.tod_epoch_difference; } atomic_notifier_call_chain(&mut s390_epoch_delta_notifier, 0, &delta as *const _ as *mut _); }
unsafe fn clock_sync_local(delta: i64) { if (*get_lowcore()).clock_comparator != clock_comparator_max { (*get_lowcore()).clock_comparator = (*get_lowcore()).clock_comparator.wrapping_add(delta as u64); set_clock_comparator((*get_lowcore()).clock_comparator); } (*get_lowcore()).last_update_clock = (*get_lowcore()).last_update_clock.wrapping_add(delta as u64); }

static mut time_sync_wq: *mut workqueue_struct = core::ptr::null_mut();
unsafe fn time_init_wq() { if !time_sync_wq.is_null() { return; } time_sync_wq = create_singlethread_workqueue(b"timesync\0".as_ptr() as _); }
#[repr(C)] struct clock_sync_data { cpus: atomic_t, in_sync: i32, clock_delta: i64 }
static mut stp_online: bool = true; static mut stp_info: stp_sstpi = stp_sstpi::zeroed(); static mut stp_page: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn stp_reset() { stp_page = get_zeroed_page(GFP_ATOMIC) as _; let rc = chsc_sstpc(stp_page, STP_OP_CTRL, 0, core::ptr::null_mut()); if rc == 0 { set_bit(CLOCK_SYNC_HAS_STP, &mut clock_sync_flags); } else if stp_online { free_page(stp_page as usize); stp_page = core::ptr::null_mut(); stp_online = false; } }
pub unsafe extern "C" fn stp_enabled() -> bool { test_bit(CLOCK_SYNC_HAS_STP, &clock_sync_flags) && stp_online }
unsafe extern "C" fn stp_timeout(_: *mut timer_list) { queue_work(time_sync_wq, &mut stp_work); }
unsafe extern "C" fn stp_init() -> i32 { if !test_bit(CLOCK_SYNC_HAS_STP, &clock_sync_flags) { return 0; } timer_setup(&mut stp_timer, Some(stp_timeout), 0); time_init_wq(); if !stp_online { return 0; } queue_work(time_sync_wq, &mut stp_work); 0 }
unsafe extern "C" fn stp_timing_alert(p: *mut stp_irq_parm) { if (*p).tsc || (*p).lac || (*p).tcpc { queue_work(time_sync_wq, &mut stp_work); } }
pub unsafe extern "C" fn stp_sync_check() -> i32 { disable_sync_clock(core::ptr::null_mut()); 1 }
pub unsafe extern "C" fn stp_island_check() -> i32 { disable_sync_clock(core::ptr::null_mut()); 1 }
pub unsafe extern "C" fn stp_queue_work() { queue_work(time_sync_wq, &mut stp_work); }

unsafe fn __store_stpinfo() -> i32 { let rc = chsc_sstpi(stp_page, &mut stp_info, core::mem::size_of::<stp_sstpi>()); if rc != 0 { clear_bit(CLOCK_SYNC_STPINFO_VALID, &mut clock_sync_flags); } else { set_bit(CLOCK_SYNC_STPINFO_VALID, &mut clock_sync_flags); } rc }
unsafe fn stpinfo_valid() -> bool { stp_online && test_bit(CLOCK_SYNC_STPINFO_VALID, &clock_sync_flags) }

unsafe fn stp_sync_clock(data: *mut core::ffi::c_void) -> i32 {
    let sync = &mut *(data as *mut clock_sync_data); static mut first: i32 = 0; let mut clock_delta = 0i64; let mut flags; let mut rc;
    enable_sync_clock();
    if xchg(&mut first, 1) == 0 { while atomic_read(&sync.cpus) != 0 { cpu_relax(); } rc = 0; if stp_info.todoff != 0 || stp_info.tmd != 2 { flags = vdso_update_begin(); rc = chsc_sstpc(stp_page, STP_OP_SYNC, 0, &mut clock_delta); if rc == 0 { sync.clock_delta = clock_delta; clock_sync_global(clock_delta); rc = __store_stpinfo(); if rc == 0 && stp_info.tmd != 2 { rc = -EAGAIN; } } vdso_update_end(flags); } sync.in_sync = if rc != 0 { -EAGAIN } else { 1 }; xchg(&mut first, 0); } else { atomic_dec(&mut sync.cpus); while READ_ONCE(sync.in_sync) == 0 {} }
    if sync.in_sync != 1 { disable_sync_clock(core::ptr::null_mut()); } clock_sync_local(sync.clock_delta); 0
}

static mut stp_work: work_struct = work_struct::zeroed(); static mut stp_timer: timer_list = timer_list::zeroed();
unsafe extern "C" fn stp_work_fn(_: *mut work_struct) {
    let mut sync: clock_sync_data = core::mem::zeroed(); mutex_lock(&mut stp_mutex);
    if !stp_online { chsc_sstpc(stp_page, STP_OP_CTRL, 0, core::ptr::null_mut()); timer_delete_sync(&mut stp_timer); mutex_unlock(&mut stp_mutex); return; }
    if chsc_sstpc(stp_page, STP_OP_CTRL, 0xf0e0, core::ptr::null_mut()) != 0 { mutex_unlock(&mut stp_mutex); return; }
    if __store_stpinfo() != 0 || stp_info.c == 0 { mutex_unlock(&mut stp_mutex); return; }
    if check_sync_clock() == 0 { core::ptr::write_bytes(&mut sync, 0, 1); cpus_read_lock(); atomic_set(&mut sync.cpus, num_online_cpus() - 1); stop_machine_cpuslocked(Some(stp_sync_clock), &mut sync, cpu_online_mask); cpus_read_unlock(); }
    if check_sync_clock() == 0 { mod_timer(&mut stp_timer, jiffies + msecs_to_jiffies(MSEC_PER_SEC)); }
    mutex_unlock(&mut stp_mutex);
}

// STP sysfs accessors retain the source interface and depend on kernel device/sysfs types.
static mut stp_subsys: bus_type = bus_type { name: b"stp\0".as_ptr() as _, dev_name: b"stp\0".as_ptr() as _ };

unsafe fn stp_sysfs_value(buf: *mut i8, fmt: *const i8, value: i64) -> isize { sysfs_emit(buf, fmt, value) }
unsafe fn ctn_id_show(_: *mut device, _: *mut device_attribute, buf: *mut i8) -> isize { let mut ret = -ENODATA; mutex_lock(&mut stp_mutex); if stpinfo_valid() { ret = sysfs_emit(buf, b"%016lx\n\0".as_ptr() as _, *(stp_info.ctnid.as_ptr() as *const usize)); } mutex_unlock(&mut stp_mutex); ret }
unsafe fn ctn_type_show(_: *mut device, _: *mut device_attribute, buf: *mut i8) -> isize { let mut ret = -ENODATA; mutex_lock(&mut stp_mutex); if stpinfo_valid() { ret = sysfs_emit(buf, b"%i\n\0".as_ptr() as _, stp_info.ctn); } mutex_unlock(&mut stp_mutex); ret }
unsafe fn dst_offset_show(_: *mut device, _: *mut device_attribute, buf: *mut i8) -> isize { let mut ret = -ENODATA; mutex_lock(&mut stp_mutex); if stpinfo_valid() && stp_info.vbits & 0x2000 != 0 { ret = sysfs_emit(buf, b"%i\n\0".as_ptr() as _, stp_info.dsto as i16 as i32); } mutex_unlock(&mut stp_mutex); ret }
unsafe fn leap_seconds_show(_: *mut device, _: *mut device_attribute, buf: *mut i8) -> isize { let mut ret = -ENODATA; mutex_lock(&mut stp_mutex); if stpinfo_valid() && stp_info.vbits & 0x8000 != 0 { ret = sysfs_emit(buf, b"%i\n\0".as_ptr() as _, stp_info.leaps as i16 as i32); } mutex_unlock(&mut stp_mutex); ret }
unsafe fn stratum_show(_: *mut device, _: *mut device_attribute, buf: *mut i8) -> isize { let mut ret = -ENODATA; mutex_lock(&mut stp_mutex); if stpinfo_valid() { ret = sysfs_emit(buf, b"%i\n\0".as_ptr() as _, stp_info.stratum as i16 as i32); } mutex_unlock(&mut stp_mutex); ret }
unsafe fn time_offset_show(_: *mut device, _: *mut device_attribute, buf: *mut i8) -> isize { let mut ret = -ENODATA; mutex_lock(&mut stp_mutex); if stpinfo_valid() && stp_info.vbits & 0x0800 != 0 { ret = sysfs_emit(buf, b"%i\n\0".as_ptr() as _, stp_info.tto); } mutex_unlock(&mut stp_mutex); ret }
unsafe fn time_zone_offset_show(_: *mut device, _: *mut device_attribute, buf: *mut i8) -> isize { let mut ret = -ENODATA; mutex_lock(&mut stp_mutex); if stpinfo_valid() && stp_info.vbits & 0x4000 != 0 { ret = sysfs_emit(buf, b"%i\n\0".as_ptr() as _, stp_info.tzo as i16 as i32); } mutex_unlock(&mut stp_mutex); ret }
unsafe fn timing_mode_show(_: *mut device, _: *mut device_attribute, buf: *mut i8) -> isize { let mut ret = -ENODATA; mutex_lock(&mut stp_mutex); if stpinfo_valid() { ret = sysfs_emit(buf, b"%i\n\0".as_ptr() as _, stp_info.tmd); } mutex_unlock(&mut stp_mutex); ret }
unsafe fn timing_state_show(_: *mut device, _: *mut device_attribute, buf: *mut i8) -> isize { let mut ret = -ENODATA; mutex_lock(&mut stp_mutex); if stpinfo_valid() { ret = sysfs_emit(buf, b"%i\n\0".as_ptr() as _, stp_info.tst); } mutex_unlock(&mut stp_mutex); ret }
unsafe fn online_show(_: *mut device, _: *mut device_attribute, buf: *mut i8) -> isize { sysfs_emit(buf, b"%i\n\0".as_ptr() as _, stp_online as i32) }
unsafe fn online_store(_: *mut device, _: *mut device_attribute, buf: *const i8, count: usize) -> isize { let value = simple_strtoul(buf, core::ptr::null_mut(), 0); if value != 0 && value != 1 { return -EINVAL; } if !test_bit(CLOCK_SYNC_HAS_STP, &clock_sync_flags) { return -EOPNOTSUPP; } mutex_lock(&mut stp_mutex); stp_online = value != 0; if stp_online { set_bit(CLOCK_SYNC_STP, &mut clock_sync_flags); } else { clear_bit(CLOCK_SYNC_STP, &mut clock_sync_flags); } queue_work(time_sync_wq, &mut stp_work); mutex_unlock(&mut stp_mutex); count as isize }

unsafe fn leap_seconds_scheduled_show(_: *mut device, _: *mut device_attribute, buf: *mut i8) -> isize {
    let mut stzi: stp_stzi = core::mem::zeroed(); mutex_lock(&mut stp_mutex);
    if !stpinfo_valid() || stp_info.vbits & 0x8000 == 0 || !stp_info.lu { mutex_unlock(&mut stp_mutex); return -ENODATA; }
    let ret = chsc_stzi(stp_page, &mut stzi, core::mem::size_of::<stp_stzi>()); mutex_unlock(&mut stp_mutex);
    if ret < 0 { return ret as isize; }
    if !stzi.lsoib.p { return sysfs_emit(buf, b"0,0\n\0".as_ptr() as _); }
    sysfs_emit(buf, b"%lu,%d\n\0".as_ptr() as _, tod_to_ns(stzi.lsoib.nlsout - TOD_UNIX_EPOCH) / NSEC_PER_SEC, stzi.lsoib.nlso - stzi.lsoib.also)
}

// The source declares DEVICE_ATTR_{RO,RW}, the stp attribute array, ATTRIBUTE_GROUPS(stp_dev),
// and device_initcall(stp_init_sysfs); these expand through the kernel's device/sysfs API.
unsafe extern "C" fn stp_init_sysfs() -> i32 { subsys_system_register(&mut stp_subsys, stp_dev_groups) }

// DEVICE_ATTR_RO/DEVICE_ATTR_RW, ATTRIBUTE_GROUPS, early_param, arch_initcall, and device_initcall
// are represented by the corresponding kernel registration macros in the containing build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
