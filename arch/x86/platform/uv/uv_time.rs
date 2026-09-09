// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SGI RTC clock/timer routines.
 *
 *  (C) Copyright 2020 Hewlett Packard Enterprise Development LP
 *  Copyright (c) 2009-2013 Silicon Graphics, Inc.  All Rights Reserved.
 *  Copyright (c) Dimitri Sivanich
 */

// C headers omitted; their supplied symbols remain external dependencies.

const RTC_NAME: &[u8] = b"sgi_rtc\0";

extern "C" {
    fn uv_read_rtc(cs: *mut clocksource) -> u64;
    fn uv_rtc_next_event(delta: c_ulong, ced: *mut clock_event_device) -> c_int;
    fn uv_rtc_shutdown(evt: *mut clock_event_device) -> c_int;
}

static mut CLOCKSOURCE_UV: clocksource = clocksource {
    name: RTC_NAME.as_ptr(),
    rating: 299,
    read: Some(uv_read_rtc),
    mask: UVH_RTC_REAL_TIME_CLOCK_MASK as u64,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

static mut CLOCK_EVENT_DEVICE_UV: clock_event_device = clock_event_device {
    name: RTC_NAME.as_ptr(),
    features: CLOCK_EVT_FEAT_ONESHOT,
    shift: 20,
    rating: 400,
    irq: -1,
    set_next_event: Some(uv_rtc_next_event),
    set_state_shutdown: Some(uv_rtc_shutdown),
    event_handler: None,
    ..unsafe { core::mem::zeroed() }
};

static mut CPU_CED: [clock_event_device; NR_CPUS] = unsafe { core::mem::zeroed() };

/* There is one of these allocated per node */
#[repr(C)]
struct uv_rtc_timer_head {
    lock: spinlock_t,
    /* next cpu waiting for timer, local node relative: */
    next_cpu: c_int,
    /* number of cpus on this node: */
    ncpus: c_int,
    cpu: [uv_rtc_timer_cpu; 0],
}

#[repr(C)]
struct uv_rtc_timer_cpu {
    lcpu: c_int,
    expires: u64,
}

/* Access to uv_rtc_timer_head via blade id. */
static mut BLADE_INFO: *mut *mut uv_rtc_timer_head = core::ptr::null_mut();
static mut UV_RTC_EVT_ENABLE: c_int = 0;

/* Hardware interface routines */

/* Send IPIs to another node */
unsafe fn uv_rtc_send_IPI(cpu: c_int) {
    let apicid = cpu_physical_id(cpu);
    let pnode = uv_apicid_to_pnode(apicid);
    let val = (1u64 << UVH_IPI_INT_SEND_SHFT)
        | ((apicid as u64) << UVH_IPI_INT_APIC_ID_SHFT)
        | ((X86_PLATFORM_IPI_VECTOR as u64) << UVH_IPI_INT_VECTOR_SHFT);
    uv_write_global_mmr64(pnode, UVH_IPI_INT, val);
}

/* Check for an RTC interrupt pending */
unsafe fn uv_intr_pending(pnode: c_int) -> c_int {
    (uv_read_global_mmr64(pnode, UVH_EVENT_OCCURRED2) & UVH_EVENT_OCCURRED2_RTC_1_MASK) as c_int
}

/* Setup interrupt and return non-zero if early expiration occurred. */
unsafe fn uv_setup_intr(cpu: c_int, expires: u64) -> c_int {
    let apicid = cpu_physical_id(cpu);
    let pnode = uv_cpu_to_pnode(cpu);
    uv_write_global_mmr64(pnode, UVH_RTC1_INT_CONFIG, UVH_RTC1_INT_CONFIG_M_MASK);
    uv_write_global_mmr64(pnode, UVH_INT_CMPB, (-1i64) as u64);
    uv_write_global_mmr64(pnode, UVH_EVENT_OCCURRED2_ALIAS, UVH_EVENT_OCCURRED2_RTC_1_MASK);
    let val = ((X86_PLATFORM_IPI_VECTOR as u64) << UVH_RTC1_INT_CONFIG_VECTOR_SHFT)
        | ((apicid as u64) << UVH_RTC1_INT_CONFIG_APIC_ID_SHFT);
    uv_write_global_mmr64(pnode, UVH_RTC1_INT_CONFIG, val);
    uv_write_global_mmr64(pnode, UVH_INT_CMPB, expires);
    if uv_read_rtc(core::ptr::null_mut()) <= expires { return 0; }
    if uv_intr_pending(pnode) == 0 { 1 } else { 0 }
}

/* Per-cpu timer tracking routines */

unsafe fn uv_rtc_deallocate_timers() {
    for bid in 0..uv_possible_blades {
        kfree(*BLADE_INFO.add(bid as usize) as *mut c_void);
    }
    kfree(BLADE_INFO as *mut c_void);
}

/* Allocate per-node list of cpu timer expiration times. */
unsafe fn uv_rtc_allocate_timers() -> c_int {
    BLADE_INFO = kcalloc(uv_possible_blades as usize, core::mem::size_of::<*mut c_void>(), GFP_KERNEL);
    if BLADE_INFO.is_null() { return -ENOMEM; }
    for_each_present_cpu!(cpu, {
        let nid = cpu_to_node(cpu);
        let bid = uv_cpu_to_blade_id(cpu);
        let bcpu = uv_cpu_blade_processor_id(cpu);
        let mut head = *BLADE_INFO.add(bid as usize);
        if head.is_null() {
            let n = uv_blade_nr_possible_cpus(bid);
            head = kmalloc_node(core::mem::size_of::<uv_rtc_timer_head>() + (n as usize) * core::mem::size_of::<uv_rtc_timer_cpu>(), GFP_KERNEL, nid);
            if head.is_null() { uv_rtc_deallocate_timers(); return -ENOMEM; }
            spin_lock_init(&mut (*head).lock);
            (*head).ncpus = n;
            (*head).next_cpu = -1;
            *BLADE_INFO.add(bid as usize) = head;
        }
        let entry = (*head).cpu.as_mut_ptr().add(bcpu as usize);
        (*entry).lcpu = cpu;
        (*entry).expires = u64::MAX;
    });
    0
}

/* Find and set the next expiring timer. */
unsafe fn uv_rtc_find_next_timer(head: *mut uv_rtc_timer_head, pnode: c_int) {
    let mut lowest = u64::MAX;
    let mut bcpu = -1;
    (*head).next_cpu = -1;
    for c in 0..(*head).ncpus {
        let exp = (*head).cpu.as_ptr().add(c as usize).read().expires;
        if exp < lowest { bcpu = c; lowest = exp; }
    }
    if bcpu >= 0 {
        (*head).next_cpu = bcpu;
        let c = (*head).cpu.as_ptr().add(bcpu as usize).read().lcpu;
        if uv_setup_intr(c, lowest) != 0 { uv_rtc_send_IPI(c); }
    } else {
        uv_write_global_mmr64(pnode, UVH_RTC1_INT_CONFIG, UVH_RTC1_INT_CONFIG_M_MASK);
    }
}

/* Set expiration time for current cpu. */
unsafe fn uv_rtc_set_timer(cpu: c_int, expires: u64) -> c_int {
    let pnode = uv_cpu_to_pnode(cpu);
    let head = *BLADE_INFO.add(uv_cpu_to_blade_id(cpu) as usize);
    let bcpu = uv_cpu_blade_processor_id(cpu);
    let t = &mut (*head).cpu.as_mut_ptr().add(bcpu as usize).as_mut().unwrap().expires;
    let mut flags = 0;
    spin_lock_irqsave(&mut (*head).lock, &mut flags);
    let next_cpu = (*head).next_cpu;
    *t = expires;
    if next_cpu < 0 || bcpu == next_cpu || expires < (*head).cpu.as_ptr().add(next_cpu as usize).read().expires {
        (*head).next_cpu = bcpu;
        if uv_setup_intr(cpu, expires) != 0 {
            *t = u64::MAX;
            uv_rtc_find_next_timer(head, pnode);
            spin_unlock_irqrestore(&mut (*head).lock, flags);
            return -ETIME;
        }
    }
    spin_unlock_irqrestore(&mut (*head).lock, flags);
    0
}

/* Unset expiration time for current cpu. */
unsafe fn uv_rtc_unset_timer(cpu: c_int, force: c_int) -> c_int {
    let pnode = uv_cpu_to_pnode(cpu);
    let head = *BLADE_INFO.add(uv_cpu_to_blade_id(cpu) as usize);
    let bcpu = uv_cpu_blade_processor_id(cpu);
    let t = &mut (*head).cpu.as_mut_ptr().add(bcpu as usize).as_mut().unwrap().expires;
    let mut flags = 0;
    let mut rc = 0;
    spin_lock_irqsave(&mut (*head).lock, &mut flags);
    if (((*head).next_cpu == bcpu) && uv_read_rtc(core::ptr::null_mut()) >= *t) || force != 0 { rc = 1; }
    if rc != 0 { *t = u64::MAX; if (*head).next_cpu == bcpu { uv_rtc_find_next_timer(head, pnode); } }
    spin_unlock_irqrestore(&mut (*head).lock, flags);
    rc
}

/* Read the RTC. */
unsafe fn uv_read_rtc(cs: *mut clocksource) -> u64 {
    let offset = if uv_get_min_hub_revision_id() == 1 { 0 } else { (uv_blade_processor_id() * L1_CACHE_BYTES) % PAGE_SIZE };
    uv_read_local_mmr(UVH_RTC | offset) as u64
}

/* Program the next event, relative to now */
unsafe fn uv_rtc_next_event(delta: c_ulong, ced: *mut clock_event_device) -> c_int {
    uv_rtc_set_timer(cpumask_first((*ced).cpumask), delta as u64 + uv_read_rtc(core::ptr::null_mut()))
}

/* Shutdown the RTC timer */
unsafe fn uv_rtc_shutdown(evt: *mut clock_event_device) -> c_int {
    uv_rtc_unset_timer(cpumask_first((*evt).cpumask), 1); 0
}

unsafe fn uv_rtc_interrupt() {
    let cpu = smp_processor_id();
    let ced = &mut CPU_CED[cpu as usize];
    if ced.event_handler.is_none() || uv_rtc_unset_timer(cpu, 0) != 1 { return; }
    (ced.event_handler.unwrap())(ced);
}

unsafe fn uv_enable_evt_rtc(_str: *mut c_char) -> c_int { UV_RTC_EVT_ENABLE = 1; 1 }

unsafe fn uv_rtc_register_clockevents(_dummy: *mut work_struct) {
    let ced = &mut CPU_CED[smp_processor_id() as usize];
    *ced = CLOCK_EVENT_DEVICE_UV;
    ced.cpumask = cpumask_of(smp_processor_id());
    clockevents_register_device(ced);
}

unsafe fn uv_rtc_setup_clock() -> c_int {
    if !is_uv_system() { return -ENODEV; }
    let mut rc = clocksource_register_hz(&mut CLOCKSOURCE_UV, sn_rtc_cycles_per_second);
    if rc != 0 { printk!(KERN_INFO, "UV RTC clocksource failed rc %d\n", rc); }
    else { printk!(KERN_INFO, "UV RTC clocksource registered freq %lu MHz\n", sn_rtc_cycles_per_second / 1E6 as c_ulong); }
    if rc != 0 || UV_RTC_EVT_ENABLE == 0 || !x86_platform_ipi_callback.is_null() { return rc; }
    rc = uv_rtc_allocate_timers();
    if rc != 0 { goto_error!(); }
    x86_platform_ipi_callback = Some(uv_rtc_interrupt);
    CLOCK_EVENT_DEVICE_UV.mult = div_sc(sn_rtc_cycles_per_second, NSEC_PER_SEC, CLOCK_EVENT_DEVICE_UV.shift);
    CLOCK_EVENT_DEVICE_UV.min_delta_ns = NSEC_PER_SEC / sn_rtc_cycles_per_second;
    CLOCK_EVENT_DEVICE_UV.min_delta_ticks = 1;
    CLOCK_EVENT_DEVICE_UV.max_delta_ns = CLOCKSOURCE_UV.mask * (NSEC_PER_SEC / sn_rtc_cycles_per_second);
    CLOCK_EVENT_DEVICE_UV.max_delta_ticks = CLOCKSOURCE_UV.mask;
    rc = schedule_on_each_cpu(uv_rtc_register_clockevents);
    if rc != 0 { x86_platform_ipi_callback = None; uv_rtc_deallocate_timers(); goto_error!(); }
    printk!(KERN_INFO, "UV RTC clockevents registered\n");
    return 0;
    goto_error!();
    clocksource_unregister(&mut CLOCKSOURCE_UV);
    printk!(KERN_INFO, "UV RTC clockevents failed rc %d\n", rc);
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
