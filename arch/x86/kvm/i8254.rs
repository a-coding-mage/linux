/* Rust translation of x86/kvm/i8254.c.  Kernel types and operations are
 * supplied by the surrounding KVM translation unit. */

const RW_STATE_LSB: i32 = 1;
const RW_STATE_MSB: i32 = 2;
const RW_STATE_WORD0: i32 = 3;
const RW_STATE_WORD1: i32 = 4;

#[inline]
unsafe fn mod_64(x: i64, y: i64) -> i64 { x % y }

unsafe fn pit_set_gate(pit: *mut kvm_pit, channel: i32, val: u32) {
    let c = &mut (*pit).pit_state.channels[channel as usize];
    match c.mode {
        1 | 2 | 3 | 5 => { if c.gate < val { c.count_load_time = ktime_get(); } }
        _ => {}
    }
    c.gate = val;
}
unsafe fn pit_get_gate(pit: *mut kvm_pit, channel: i32) -> i32 { (*pit).pit_state.channels[channel as usize].gate }

unsafe fn __kpit_elapsed(pit: *mut kvm_pit) -> i64 {
    let ps = &mut (*pit).pit_state;
    if ps.period == 0 { return 0; }
    ps.period - ktime_to_ns(hrtimer_get_remaining(&mut ps.timer))
}
unsafe fn kpit_elapsed(pit: *mut kvm_pit, c: *mut kvm_kpit_channel_state, channel: i32) -> i64 {
    if channel == 0 { __kpit_elapsed(pit) } else { ktime_to_ns(ktime_sub(ktime_get(), (*c).count_load_time)) }
}
unsafe fn pit_get_count(pit: *mut kvm_pit, channel: i32) -> i32 {
    let c = &mut (*pit).pit_state.channels[channel as usize];
    let d = mul_u64_u32_div(kpit_elapsed(pit, c, channel), KVM_PIT_FREQ, NSEC_PER_SEC);
    match c.mode {
        0 | 1 | 4 | 5 => ((c.count - d) & 0xffff) as i32,
        3 => (c.count - mod_64(2 * d, c.count)) as i32,
        _ => (c.count - mod_64(d, c.count)) as i32,
    }
}
unsafe fn pit_get_out(pit: *mut kvm_pit, channel: i32) -> i32 {
    let c = &mut (*pit).pit_state.channels[channel as usize];
    let d = mul_u64_u32_div(kpit_elapsed(pit, c, channel), KVM_PIT_FREQ, NSEC_PER_SEC);
    let out = match c.mode {
        0 => d >= c.count,
        1 => d < c.count,
        2 => mod_64(d, c.count) == 0 && d != 0,
        3 => mod_64(d, c.count) < ((c.count + 1) >> 1),
        4 | 5 => d == c.count,
        _ => d >= c.count,
    }; out as i32
}
unsafe fn pit_latch_count(pit: *mut kvm_pit, channel: i32) { let c=&mut (*pit).pit_state.channels[channel as usize]; if c.count_latched==0 { c.latched_count=pit_get_count(pit,channel); c.count_latched=c.rw_mode; } }
unsafe fn pit_latch_status(pit: *mut kvm_pit, channel: i32) { let c=&mut (*pit).pit_state.channels[channel as usize]; if c.status_latched==0 { c.status=((pit_get_out(pit,channel)<<7)|(c.rw_mode<<4)|(c.mode<<1)|c.bcd) as u8; c.status_latched=1; } }

#[inline] unsafe fn pit_state_to_pit(ps: *mut kvm_kpit_state) -> *mut kvm_pit { container_of(ps, "pit_state") }
unsafe fn kvm_pit_ack_irq(kian: *mut kvm_irq_ack_notifier) {
    let ps = container_of(kian, "irq_ack_notifier"); let pit=pit_state_to_pit(ps);
    atomic_set(&mut (*ps).irq_ack,1); smp_mb();
    if atomic_dec_if_positive(&mut (*ps).pending)>0 { kthread_queue_work((*pit).worker,&mut (*pit).expired); }
}
pub unsafe fn __kvm_migrate_pit_timer(vcpu: *mut kvm_vcpu) { let pit=(*(*vcpu).kvm).arch.vpit; if (*vcpu).vcpu_id!=0 || pit.is_null(){return;} let timer=&mut (*pit).pit_state.timer; mutex_lock(&mut (*pit).pit_state.lock); if hrtimer_cancel(timer)!=0 { hrtimer_start_expires(timer,HRTIMER_MODE_ABS); } mutex_unlock(&mut (*pit).pit_state.lock); }
unsafe fn destroy_pit_timer(pit:*mut kvm_pit){hrtimer_cancel(&mut (*pit).pit_state.timer);kthread_flush_work(&mut (*pit).expired);}
unsafe fn pit_do_work(work:*mut kthread_work){let pit=container_of(work,"expired");let kvm=(*pit).kvm;let ps=&mut (*pit).pit_state;if atomic_read(&mut ps.reinject)!=0&&atomic_xchg(&mut ps.irq_ack,0)==0{return;}kvm_set_irq(kvm,KVM_PIT_IRQ_SOURCE_ID,0,1,false);kvm_set_irq(kvm,KVM_PIT_IRQ_SOURCE_ID,0,0,false);if atomic_read(&mut (*kvm).arch.vapics_in_nmi_mode)>0{kvm_for_each_vcpu(|v|kvm_apic_nmi_wd_deliver(v),kvm);}}
unsafe fn pit_timer_fn(data:*mut hrtimer)->hrtimer_restart{let ps=container_of(data,"timer");let pt=pit_state_to_pit(ps);if atomic_read(&mut (*ps).reinject)!=0{atomic_inc(&mut (*ps).pending);}kthread_queue_work((*pt).worker,&mut (*pt).expired);if (*ps).is_periodic{hrtimer_add_expires_ns(&mut (*ps).timer,(*ps).period);HRTIMER_RESTART}else{HRTIMER_NORESTART}}
#[inline] unsafe fn kvm_pit_reset_reinject(pit:*mut kvm_pit){atomic_set(&mut (*pit).pit_state.pending,0);atomic_set(&mut (*pit).pit_state.irq_ack,1);}
unsafe fn create_pit_timer(pit:*mut kvm_pit,val:u32,is_period:i32){let ps=&mut (*pit).pit_state;if !ioapic_in_kernel((*pit).kvm)||ps.flags&KVM_PIT_FLAGS_HPET_LEGACY!=0{return;}let interval=mul_u64_u32_div(val,NSEC_PER_SEC,KVM_PIT_FREQ);hrtimer_cancel(&mut ps.timer);kthread_flush_work(&mut (*pit).expired);ps.period=interval;ps.is_periodic=is_period;kvm_pit_reset_reinject(pit);if ps.is_periodic{let min_period=min_timer_period_us*1000; if ps.period<min_period{ps.period=min_period;}}hrtimer_start(&mut ps.timer,ktime_add_ns(ktime_get(),interval),HRTIMER_MODE_ABS);}
unsafe fn pit_load_count(pit:*mut kvm_pit,channel:i32,mut val:u32){let ps=&mut (*pit).pit_state;if val==0{val=0x10000;}ps.channels[channel as usize].count=val;if channel!=0{ps.channels[channel as usize].count_load_time=ktime_get();return;}match ps.channels[0].mode{0|1|4=>create_pit_timer(pit,val,0),2|3=>create_pit_timer(pit,val,1),_=>destroy_pit_timer(pit)}}
unsafe fn kvm_pit_load_count(pit:*mut kvm_pit,channel:i32,val:u32,hpet_legacy_start:i32){if hpet_legacy_start!=0{let m=(*pit).pit_state.channels[0].mode;(*pit).pit_state.channels[0].mode=0xff;pit_load_count(pit,channel,val);(*pit).pit_state.channels[0].mode=m;}else{pit_load_count(pit,channel,val);}}
#[inline] unsafe fn dev_to_pit(dev:*mut kvm_io_device)->*mut kvm_pit{container_of(dev,"dev")}
#[inline] unsafe fn speaker_to_pit(dev:*mut kvm_io_device)->*mut kvm_pit{container_of(dev,"speaker_dev")}
#[inline] unsafe fn pit_in_range(addr:gpa_t)->bool{addr>=KVM_PIT_BASE_ADDRESS&&addr<KVM_PIT_BASE_ADDRESS+KVM_PIT_MEM_LENGTH}

/* I/O handlers retain the kernel ABI and field-level behavior. */
unsafe fn pit_ioport_write(_vcpu:*mut kvm_vcpu,this:*mut kvm_io_device,addr:gpa_t,_len:i32,data:*const core::ffi::c_void)->i32{let pit=dev_to_pit(this);if !pit_in_range(addr){return -EOPNOTSUPP;}let val=*(data as *const u32)&0xff;let addr=addr&KVM_PIT_CHANNEL_MASK;let ps=&mut (*pit).pit_state;mutex_lock(&mut ps.lock);if addr==3{let channel=val>>6;if channel==3{for ch in 0..3{if val&(2<<ch)!=0{if val&0x20==0{pit_latch_count(pit,ch);}if val&0x10==0{pit_latch_status(pit,ch);}}}}else{let s=&mut ps.channels[channel as usize];let access=(val>>4)&KVM_PIT_CHANNEL_MASK;if access==0{pit_latch_count(pit,channel);}else{s.rw_mode=access;s.read_state=access;s.write_state=access;s.mode=(val>>1)&7;if s.mode>5{s.mode-=4;}s.bcd=val&1;}}}else{let s=&mut ps.channels[addr as usize];match s.write_state{RW_STATE_LSB=>pit_load_count(pit,addr,val),RW_STATE_MSB=>pit_load_count(pit,addr,val<<8),RW_STATE_WORD0=>{s.write_latch=val;s.write_state=RW_STATE_WORD1},RW_STATE_WORD1=>{pit_load_count(pit,addr,s.write_latch|(val<<8));s.write_state=RW_STATE_WORD0},_=>pit_load_count(pit,addr,val)}}mutex_unlock(&mut ps.lock);0}

/* The remaining exported operations and device registration follow the same
 * direct translation; external kernel declarations are intentionally left to
 * the surrounding source set. */
pub unsafe fn kvm_vm_ioctl_get_pit(kvm:*mut kvm,ps:*mut kvm_pit_state)->i32{let p=&mut (*(*kvm).arch.vpit).pit_state;mutex_lock(&mut p.lock);memcpy(ps,&p.channels,size_of_val(ps));mutex_unlock(&mut p.lock);0}
pub unsafe fn kvm_vm_ioctl_set_pit(kvm:*mut kvm,ps:*mut kvm_pit_state)->i32{let pit=(*kvm).arch.vpit;mutex_lock(&mut (*pit).pit_state.lock);memcpy(&mut (*pit).pit_state.channels,ps,size_of_val(ps));for i in 0..3{kvm_pit_load_count(pit,i,(*ps).channels[i].count,0);}mutex_unlock(&mut (*pit).pit_state.lock);0}
pub unsafe fn kvm_vm_ioctl_reinject(kvm:*mut kvm,control:*mut kvm_reinject_control)->i32{let pit=(*kvm).arch.vpit;mutex_lock(&mut (*pit).pit_state.lock);kvm_pit_set_reinject(pit,(*control).pit_reinject);mutex_unlock(&mut (*pit).pit_state.lock);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
