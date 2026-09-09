// SPDX-License-Identifier: GPL-2.0
// Source-level translation of um/kernel/time.c. Kernel and UML dependencies
// are supplied by the surrounding translation unit.

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
use core::ptr;

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_mode: time_travel_mode = 0;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_start_set: bool = false;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_start: u64 = 0;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_time: u64 = 0;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_shm_offset: u64 = 0;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_events: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_irqs: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_timer_interval: u64 = 0;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_next_event: u64 = 0;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_timer_event: time_travel_event = unsafe { core::mem::zeroed() };
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_ext_fd: i32 = -1;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_ext_waiting: u32 = 0;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_ext_prev_request_valid: bool = false;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_ext_prev_request: u64 = 0;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_ext_free_until: *mut u64 = ptr::null_mut();
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut _time_travel_ext_free_until: u64 = 0;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_shm_id: u16 = 0;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_shm: *mut um_timetravel_schedshm = ptr::null_mut();
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut time_travel_shm_client: *mut um_timetravel_schedshm_client = ptr::null_mut();
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
pub static mut tt_extra_sched_jiffies: usize = 0;

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
pub unsafe fn sched_clock() -> u64 {
    (jiffies.wrapping_sub(INITIAL_JIFFIES).wrapping_add(tt_extra_sched_jiffies) as u64)
        .wrapping_mul(NSEC_PER_SEC / HZ)
}

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_set_time(ns: u64) {
    if ns < time_travel_time { panic!("time-travel: time goes backwards"); }
    if ns >= S64_MAX as u64 { panic!("The system was going to sleep forever, aborting"); }
    time_travel_time = ns;
}

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
#[repr(C)] enum time_travel_message_handling { TTMH_IDLE, TTMH_POLL, TTMH_READ, TTMH_READ_START_ACK }

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut bc_message: u64 = 0;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
pub static mut time_travel_should_print_bc_msg: i32 = 0;
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
pub unsafe fn _time_travel_print_bc_msg() {
    time_travel_should_print_bc_msg = 0;
    printk(KERN_INFO, "time-travel: received broadcast 0x%llx\n", bc_message);
}

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_setup_shm(fd: i32, id: u16) {
    let mut len: u32;
    time_travel_shm = os_mmap_rw_shared(fd, core::mem::size_of::<um_timetravel_schedshm>());
    if time_travel_shm.is_null() { os_close_file(fd); return; }
    len = (*time_travel_shm).len;
    if (*time_travel_shm).version != UM_TIMETRAVEL_SCHEDSHM_VERSION || len < struct_size!(time_travel_shm, clients, id as usize + 1) {
        os_unmap_memory(time_travel_shm as *mut _, core::mem::size_of::<um_timetravel_schedshm>());
        time_travel_shm = ptr::null_mut(); os_close_file(fd); return;
    }
    time_travel_shm = os_mremap_rw_shared(time_travel_shm, core::mem::size_of::<um_timetravel_schedshm>(), len as usize);
    if !time_travel_shm.is_null() {
        time_travel_shm_offset = (*time_travel_shm).current_time;
        time_travel_shm_client = &mut (*time_travel_shm).clients[id as usize];
        (*time_travel_shm_client).capa |= UM_TIMETRAVEL_SCHEDSHM_CAP_TIME_SHARE;
        time_travel_shm_id = id;
        time_travel_ext_free_until = &mut (*time_travel_shm).free_until;
    }
    os_close_file(fd);
}

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_handle_message(msg: *mut um_timetravel_msg, mode: time_travel_message_handling) {
    let mut resp: um_timetravel_msg = core::mem::zeroed(); resp.op = UM_TIMETRAVEL_ACK;
    let mut ret: i32;
    if mode as i32 != TTMH_READ as i32 {
        while os_poll(1, &mut time_travel_ext_fd) != 0 {}
    }
    if mode as i32 == TTMH_READ_START_ACK as i32 {
        let mut fd = [0i32; UM_TIMETRAVEL_SHARED_MAX_FDS];
        ret = os_rcv_fd_msg(time_travel_ext_fd, fd.as_mut_ptr(), fd.len(), msg, core::mem::size_of::<um_timetravel_msg>());
        if ret as usize == core::mem::size_of::<um_timetravel_msg>() {
            time_travel_setup_shm(fd[UM_TIMETRAVEL_SHARED_MEMFD], (*msg).time as u16 & UM_TIMETRAVEL_START_ACK_ID);
            os_close_file(fd[UM_TIMETRAVEL_SHARED_LOGFD]);
        }
    } else { ret = os_read_file(time_travel_ext_fd, msg, core::mem::size_of::<um_timetravel_msg>()); }
    if ret == 0 { panic!("time-travel external link is broken"); }
    if ret as usize != core::mem::size_of::<um_timetravel_msg>() { panic!("invalid time-travel message"); }
    match (*msg).op {
        UM_TIMETRAVEL_ACK => return,
        UM_TIMETRAVEL_RUN => { time_travel_set_time((*msg).time); if !time_travel_shm.is_null() { (*time_travel_shm_client).flags &= !UM_TIMETRAVEL_SCHEDSHM_FLAGS_REQ_RUN; return; } },
        UM_TIMETRAVEL_FREE_UNTIL => { if time_travel_shm.is_null() { time_travel_ext_free_until = &mut _time_travel_ext_free_until; _time_travel_ext_free_until = (*msg).time; } },
        UM_TIMETRAVEL_BROADCAST => { bc_message = (*msg).time; time_travel_should_print_bc_msg = 1; },
        _ => {},
    }
    resp.seq = (*msg).seq; os_write_file(time_travel_ext_fd, &resp, core::mem::size_of_val(&resp));
}

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_ext_req(op: u32, time: u64) -> u64 {
    static mut seq: i32 = 0; seq += 1;
    let mseq = seq; let mut msg: um_timetravel_msg = core::mem::zeroed(); msg.op = op; msg.time = time; msg.seq = mseq;
    block_signals_hard(); os_write_file(time_travel_ext_fd, &msg, core::mem::size_of_val(&msg));
    if !(msg.op == UM_TIMETRAVEL_WAIT && !time_travel_shm.is_null()) {
        while msg.op != UM_TIMETRAVEL_ACK { time_travel_handle_message(&mut msg, if op == UM_TIMETRAVEL_START { TTMH_READ_START_ACK } else { TTMH_READ }); }
    }
    if op == UM_TIMETRAVEL_GET { time_travel_set_time(msg.time); }
    unblock_signals_hard(); msg.time
}

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
pub unsafe fn __time_travel_wait_readable(fd: i32) { if time_travel_mode != TT_MODE_EXTERNAL { return; } let mut fds = [fd, time_travel_ext_fd]; while os_poll(2, fds.as_mut_ptr()) != 0 { let mut msg: um_timetravel_msg = core::mem::zeroed(); if os_poll(2, fds.as_mut_ptr()) == 1 { time_travel_handle_message(&mut msg, TTMH_READ); } } }

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_ext_update_request(time: u64) {
    if time_travel_mode != TT_MODE_EXTERNAL || (time_travel_ext_prev_request_valid && time == time_travel_ext_prev_request) { return; }
    if time_travel_ext_waiting == 0 && !time_travel_ext_free_until.is_null() && time < (*time_travel_ext_free_until - time_travel_shm_offset) { return; }
    time_travel_ext_prev_request = time; time_travel_ext_prev_request_valid = true;
    if !time_travel_shm.is_null() { let running = &mut (*time_travel_shm).clients[(*time_travel_shm).running_id as usize]; if running.capa & UM_TIMETRAVEL_SCHEDSHM_CAP_TIME_SHARE != 0 { (*time_travel_shm_client).flags |= UM_TIMETRAVEL_SCHEDSHM_FLAGS_REQ_RUN; time += time_travel_shm_offset; (*time_travel_shm_client).req_time = time; if time < (*time_travel_shm).free_until { (*time_travel_shm).free_until = time; } return; } }
    time_travel_ext_req(UM_TIMETRAVEL_REQUEST, time);
}

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
pub unsafe fn __time_travel_propagate_time() { static mut last: u64 = 0; if !time_travel_shm.is_null() { (*time_travel_shm).current_time = time_travel_time + time_travel_shm_offset; return; } if last != time_travel_time { time_travel_ext_req(UM_TIMETRAVEL_UPDATE, time_travel_time); last = time_travel_time; } }

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_ext_request(time: u64) -> bool { if time_travel_ext_waiting == 0 && !time_travel_ext_free_until.is_null() && time < (*time_travel_ext_free_until - time_travel_shm_offset) { return false; } time_travel_ext_update_request(time); true }

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_ext_wait(idle: bool) { let mut msg: um_timetravel_msg = core::mem::zeroed(); msg.op = UM_TIMETRAVEL_ACK; time_travel_ext_prev_request_valid = false; if time_travel_shm.is_null() { time_travel_ext_free_until = ptr::null_mut(); } time_travel_ext_waiting += 1; time_travel_ext_req(UM_TIMETRAVEL_WAIT, u64::MAX); while msg.op != UM_TIMETRAVEL_RUN { time_travel_handle_message(&mut msg, if idle { TTMH_IDLE } else { TTMH_POLL }); } time_travel_ext_waiting -= 1; time_travel_ext_prev_request_valid = false; }

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_ext_get_time() { if !time_travel_shm.is_null() { time_travel_set_time((*time_travel_shm).current_time - time_travel_shm_offset); } else { time_travel_ext_req(UM_TIMETRAVEL_GET, u64::MAX); } }

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn __time_travel_update_time(ns: u64, idle: bool) { if time_travel_mode == TT_MODE_EXTERNAL && time_travel_ext_request(ns) { time_travel_ext_wait(idle); } else { time_travel_set_time(ns); } }

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_first_event() -> *mut time_travel_event { list_first_entry_or_null!(&mut time_travel_events, time_travel_event, list) }

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn __time_travel_add_event(e: *mut time_travel_event, time: u64) { if (*e).pending { return; } (*e).pending = true; (*e).time = time; let mut flags = 0usize; local_irq_save(&mut flags); let mut tmp: *mut time_travel_event; list_for_each_entry!(tmp, &mut time_travel_events, list) { if (*tmp).time > (*e).time || ((*tmp).time == (*e).time && (*tmp).onstack && (*e).onstack) { list_add_tail!(&mut (*e).list, &mut (*tmp).list); return; } } list_add_tail!(&mut (*e).list, &mut time_travel_events); tmp = time_travel_first_event(); time_travel_ext_update_request((*tmp).time); time_travel_next_event = (*tmp).time; local_irq_restore(flags); }

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_add_event(e: *mut time_travel_event, time: u64) { if (*e).fn_.is_none() { return; } __time_travel_add_event(e, time); }
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
pub unsafe fn time_travel_add_event_rel(e: *mut time_travel_event, delay_ns: u64) { time_travel_add_event(e, time_travel_time + delay_ns); }
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_periodic_timer(_: *mut time_travel_event) { time_travel_add_event(&mut time_travel_timer_event, time_travel_time + time_travel_timer_interval); if tt_extra_sched_jiffies > 0 { tt_extra_sched_jiffies -= 1; } deliver_alarm(); }
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
pub unsafe fn deliver_time_travel_irqs() { if list_empty!(&time_travel_irqs) { return; } let mut flags=0usize; local_irq_save(&mut flags); irq_enter(); while let Some(e)=list_first_entry_or_null!(&mut time_travel_irqs,time_travel_event,list) { list_del!(&mut (*e).list); (*e).pending=false; if let Some(f)=(*e).fn_ { f(e); } } irq_exit(); local_irq_restore(flags); }

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_deliver_event(e: *mut time_travel_event) { if e == &mut time_travel_timer_event { if let Some(f)=(*e).fn_ { f(e); } } else if irqs_disabled() { list_add_tail!(&mut (*e).list,&mut time_travel_irqs); (*e).pending=true; } else { let mut flags=0usize; local_irq_save(&mut flags); irq_enter(); if let Some(f)=(*e).fn_ { f(e); } irq_exit(); local_irq_restore(flags); } }
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
pub unsafe fn time_travel_del_event(e: *mut time_travel_event) -> bool { if !(*e).pending { return false; } let mut flags=0usize; local_irq_save(&mut flags); list_del!(&mut (*e).list); (*e).pending=false; local_irq_restore(flags); true }

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_update_time(next:u64,idle:bool) { let mut ne:time_travel_event=core::mem::zeroed(); ne.onstack=true; let mut finished=idle; __time_travel_add_event(&mut ne,next); loop { let e=time_travel_first_event(); __time_travel_update_time((*e).time,idle); if e==time_travel_first_event() { time_travel_del_event(e); if time_travel_time != (*e).time { panic!(); } if e==&mut ne { finished=true; } else { time_travel_deliver_event(e); } } if let Some(x)=time_travel_first_event(){time_travel_ext_update_request((*x).time);} if !ne.pending || finished {break;} } time_travel_del_event(&mut ne); }
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_update_time_rel(offs:u64){let mut f=0usize;local_irq_save(&mut f);time_travel_update_time(time_travel_time+offs,false);local_irq_restore(f);}
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
pub unsafe fn time_travel_ndelay(nsec:usize){time_travel_update_time_rel(nsec as u64);}
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
pub unsafe fn time_travel_add_irq_event(e:*mut time_travel_event){time_travel_ext_get_time();time_travel_add_event(e,time_travel_time);}
#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe fn time_travel_oneshot_timer(_: *mut time_travel_event){if tt_extra_sched_jiffies>0{tt_extra_sched_jiffies-=1;}deliver_alarm();}

#[cfg(not(CONFIG_UML_TIME_TRAVEL_SUPPORT))]
static mut time_travel_start_set: i32 = 0;
#[cfg(not(CONFIG_UML_TIME_TRAVEL_SUPPORT))]
static mut time_travel_start: u64 = 0;
#[cfg(not(CONFIG_UML_TIME_TRAVEL_SUPPORT))]
static mut time_travel_time: u64 = 0;
#[cfg(not(CONFIG_UML_TIME_TRAVEL_SUPPORT))]
static mut time_travel_ext_waiting: i32 = 0;
#[cfg(not(CONFIG_UML_TIME_TRAVEL_SUPPORT))]
unsafe fn time_travel_update_time(_:u64,_:bool){}
#[cfg(not(CONFIG_UML_TIME_TRAVEL_SUPPORT))]
unsafe fn time_travel_update_time_rel(_:u64){}
#[cfg(not(CONFIG_UML_TIME_TRAVEL_SUPPORT))]
unsafe fn time_travel_handle_real_alarm(){}
#[cfg(not(CONFIG_UML_TIME_TRAVEL_SUPPORT))]
unsafe fn time_travel_set_interval(_:u64){}
#[cfg(not(CONFIG_UML_TIME_TRAVEL_SUPPORT))]
unsafe fn time_travel_set_start(){}

static mut timer_clockevent: [clock_event_device; NR_CPUS] = unsafe { core::mem::zeroed() };
pub unsafe fn timer_handler(_:i32,_:*mut siginfo,_:*mut uml_pt_regs){let mut f=0usize;if time_travel_mode==TT_MODE_BASIC{time_travel_handle_real_alarm();}local_irq_save(&mut f);do_IRQ(TIMER_IRQ,_);local_irq_restore(f);}
unsafe fn itimer_shutdown(evt:*mut clock_event_device)->i32{let cpu=evt.offset_from(timer_clockevent.as_mut_ptr());if time_travel_mode!=TT_MODE_OFF{time_travel_del_event(&mut time_travel_timer_event);}if time_travel_mode!=TT_MODE_INFCPU&&time_travel_mode!=TT_MODE_EXTERNAL{os_timer_disable(cpu as i32);}0}
unsafe fn itimer_set_periodic(evt:*mut clock_event_device)->i32{let interval=NSEC_PER_SEC/HZ;let cpu=evt.offset_from(timer_clockevent.as_mut_ptr());if time_travel_mode!=TT_MODE_OFF{time_travel_del_event(&mut time_travel_timer_event);time_travel_set_event_fn(&mut time_travel_timer_event,time_travel_periodic_timer);time_travel_set_interval(interval);time_travel_add_event(&mut time_travel_timer_event,time_travel_time+interval);}if time_travel_mode!=TT_MODE_INFCPU&&time_travel_mode!=TT_MODE_EXTERNAL{os_timer_set_interval(cpu as i32,interval);}0}
unsafe fn itimer_next_event(mut delta:u64,_:*mut clock_event_device)->i32{delta+=1;if time_travel_mode!=TT_MODE_OFF{time_travel_del_event(&mut time_travel_timer_event);time_travel_set_event_fn(&mut time_travel_timer_event,time_travel_oneshot_timer);time_travel_add_event(&mut time_travel_timer_event,time_travel_time+delta);}if time_travel_mode!=TT_MODE_INFCPU&&time_travel_mode!=TT_MODE_EXTERNAL{return os_timer_one_shot(raw_smp_processor_id(),delta);}0}
unsafe fn itimer_one_shot(evt:*mut clock_event_device)->i32{itimer_next_event(0,evt)}
static mut _timer_clockevent: clock_event_device = unsafe{core::mem::zeroed()};
unsafe fn um_timer(_:i32,_:*mut core::ffi::c_void)->irqreturn_t{let evt=&mut timer_clockevent[raw_smp_processor_id() as usize];if time_travel_mode!=TT_MODE_INFCPU&&time_travel_mode!=TT_MODE_EXTERNAL&&(*get_current()).mm!=ptr::null_mut(){os_alarm_process((*(*get_current()).mm).context.id.pid);}if let Some(f)=evt.event_handler{f(evt);}IRQ_HANDLED}
unsafe fn timer_read(_: *mut clocksource)->u64{if time_travel_mode!=TT_MODE_OFF{if !irqs_disabled()&&!in_interrupt()&&!in_softirq()&&time_travel_ext_waiting==0{time_travel_update_time_rel(TIMER_MULTIPLIER);}return time_travel_time/TIMER_MULTIPLIER;}os_nsecs()/TIMER_MULTIPLIER}
pub unsafe fn um_setup_timer()->i32{let cpu=raw_smp_processor_id();let evt=&mut timer_clockevent[cpu as usize];let err=os_timer_create();if err!=0{return err;}core::ptr::copy_nonoverlapping(&_timer_clockevent,evt,1);evt.cpumask=cpumask_of(cpu);clockevents_register_device(evt);0}
pub unsafe fn read_persistent_clock64(ts:*mut timespec64){time_travel_set_start();let nsecs=if time_travel_mode!=TT_MODE_OFF{time_travel_start+time_travel_time}else{os_persistent_clock_emulation()};set_normalized_timespec64(ts,nsecs/NSEC_PER_SEC,nsecs%NSEC_PER_SEC);}
pub unsafe fn time_init(){timer_set_signal_handler();late_time_init=Some(um_timer_init);}
unsafe fn um_timer_init(){let mut err=request_irq(TIMER_IRQ,um_timer,IRQF_TIMER,"hr timer",ptr::null_mut());if err!=0{printk(KERN_ERR,"register_timer : request_irq failed - errno = %d\n",-err);}err=um_setup_timer();if err!=0{return;}let _=clocksource_register_hz(&mut timer_clocksource,NSEC_PER_SEC/TIMER_MULTIPLIER);}
static mut timer_clocksource: clocksource = unsafe{core::mem::zeroed()};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
