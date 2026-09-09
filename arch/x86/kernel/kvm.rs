// SPDX-License-Identifier: GPL-2.0-or-later
/* KVM paravirt_ops implementation. C headers and configuration gates are
 * supplied by the surrounding kernel translation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const KVM_TASK_SLEEP_HASHBITS: u32 = 8;
const KVM_TASK_SLEEP_HASHSIZE: usize = 1 << KVM_TASK_SLEEP_HASHBITS;

static mut kvmapf: i32 = 1;
static mut steal_acc: i32 = 1;
static mut has_steal_clock: i32 = 0;
static mut has_guest_poll: i32 = 0;

#[repr(C)]
struct kvm_task_sleep_node {
    link: hlist_node,
    wq: swait_queue_head,
    token: u32,
    cpu: i32,
    dummy: bool,
}

#[repr(C)]
struct kvm_task_sleep_head {
    lock: raw_spinlock_t,
    list: hlist_head,
}

static mut async_pf_sleepers: [kvm_task_sleep_head; KVM_TASK_SLEEP_HASHSIZE] =
    [const { unsafe { core::mem::zeroed() } }; KVM_TASK_SLEEP_HASHSIZE];

unsafe fn _find_apf_task(b: *mut kvm_task_sleep_head, token: u32) -> *mut kvm_task_sleep_node {
    let mut p = (*b).list.first;
    while !p.is_null() {
        let n = hlist_entry(p, kvm_task_sleep_node, link);
        if (*n).token == token { return n; }
        p = (*p).next;
    }
    core::ptr::null_mut()
}

unsafe fn kvm_async_pf_queue_task(token: u32, n: *mut kvm_task_sleep_node) -> bool {
    let key = hash_32(token, KVM_TASK_SLEEP_HASHBITS) as usize;
    let b = &mut async_pf_sleepers[key] as *mut _;
    raw_spin_lock(&mut (*b).lock);
    let e = _find_apf_task(b, token);
    if !e.is_null() {
        let mut dummy = core::ptr::null_mut();
        if (*e).dummy { hlist_del(&mut (*e).link); dummy = e; }
        raw_spin_unlock(&mut (*b).lock); kfree(dummy as *mut core::ffi::c_void); return false;
    }
    (*n).token = token; (*n).cpu = smp_processor_id(); (*n).dummy = false;
    init_swait_queue_head(&mut (*n).wq); hlist_add_head(&mut (*n).link, &mut (*b).list);
    raw_spin_unlock(&mut (*b).lock); true
}

pub unsafe fn kvm_async_pf_task_wait_schedule(token: u32) {
    let mut n: kvm_task_sleep_node = core::mem::zeroed();
    let mut wait: swait_queue = core::mem::zeroed();
    lockdep_assert_irqs_disabled();
    if !kvm_async_pf_queue_task(token, &mut n) { return; }
    loop {
        prepare_to_swait_exclusive(&mut n.wq, &mut wait, TASK_UNINTERRUPTIBLE);
        if hlist_unhashed(&n.link) { break; }
        local_irq_enable(); schedule(); local_irq_disable();
    }
    finish_swait(&mut n.wq, &mut wait);
}

unsafe fn apf_task_wake_one(n: *mut kvm_task_sleep_node) {
    hlist_del_init(&mut (*n).link);
    if swq_has_sleeper(&(*n).wq) { swake_up_one(&mut (*n).wq); }
}

unsafe fn apf_task_wake_all() {
    for i in 0..KVM_TASK_SLEEP_HASHSIZE {
        let b = &mut async_pf_sleepers[i]; raw_spin_lock(&mut b.lock);
        let mut p = b.list.first; while !p.is_null() { let next = (*p).next;
            let n = hlist_entry(p, kvm_task_sleep_node, link);
            if (*n).cpu == smp_processor_id() { apf_task_wake_one(n); } p = next;
        } raw_spin_unlock(&mut b.lock);
    }
}

unsafe fn kvm_async_pf_task_wake(token: u32) {
    if token == !0 { apf_task_wake_all(); return; }
    let key = hash_32(token, KVM_TASK_SLEEP_HASHBITS) as usize;
    let b = &mut async_pf_sleepers[key]; let mut dummy: *mut kvm_task_sleep_node = core::ptr::null_mut();
    'again: loop { raw_spin_lock(&mut b.lock); let n = _find_apf_task(b, token);
        if n.is_null() { if dummy.is_null() { raw_spin_unlock(&mut b.lock); dummy = kzalloc_obj::<kvm_task_sleep_node>(GFP_ATOMIC); if dummy.is_null() { cpu_relax(); } continue 'again; }
            (*dummy).token=token; (*dummy).cpu=smp_processor_id(); (*dummy).dummy=true; init_swait_queue_head(&mut (*dummy).wq); hlist_add_head(&mut (*dummy).link,&mut b.list); dummy=core::ptr::null_mut();
        } else { apf_task_wake_one(n); } raw_spin_unlock(&mut b.lock); break;
    } kfree(dummy as *mut core::ffi::c_void);
}

pub unsafe fn kvm_read_and_reset_apf_flags() -> u32 { let mut flags=0; if this_cpu_read(async_pf_enabled) { flags=this_cpu_read(apf_reason.flags); this_cpu_write(apf_reason.flags,0); } flags }

pub unsafe fn __kvm_handle_async_pf(regs: *mut pt_regs, token: u32) -> bool {
    let flags=kvm_read_and_reset_apf_flags(); if flags==0{return false;} let state=irqentry_enter(regs); instrumentation_begin();
    if (*regs).flags & X86_EFLAGS_IF == 0 { panic!("Host injected async #PF in interrupt disabled region\n"); }
    if flags & KVM_PV_REASON_PAGE_NOT_PRESENT != 0 { if !user_mode(regs) { panic!("Host injected async #PF in kernel mode\n"); } kvm_async_pf_task_wait_schedule(token); } else { WARN_ONCE!(true,"Unexpected async PF flags: %x\n",flags); }
    instrumentation_end(); irqentry_exit(regs,state); true
}

pub unsafe extern "C" fn sysvec_kvm_asyncpf_interrupt(regs: *mut pt_regs) { let old=set_irq_regs(regs); apic_eoi(); inc_irq_stat(HYPERVISOR_CALLBACK); if this_cpu_read(async_pf_enabled) { let token=this_cpu_read(apf_reason.token); kvm_async_pf_task_wake(token); this_cpu_write(apf_reason.token,0); wrmsrq(MSR_KVM_ASYNC_PF_ACK,1); } set_irq_regs(old); }

unsafe fn paravirt_ops_setup(){ pv_info.name=b"KVM\0".as_ptr(); if kvm_para_has_feature(KVM_FEATURE_NOP_IO_DELAY){pv_info.io_delay=false;} }
unsafe fn kvm_register_steal_time(){ if has_steal_clock==0{return;} let cpu=smp_processor_id(); let st=&mut per_cpu(steal_time,cpu); wrmsrq(MSR_KVM_STEAL_TIME,slow_virt_to_phys(st)|KVM_MSR_ENABLED); }
unsafe fn kvm_guest_cpu_init(){ /* remaining per-cpu MSR setup is a direct dependency on kernel per-cpu primitives */ if has_steal_clock!=0{kvm_register_steal_time();} }
unsafe fn kvm_pv_disable_apf(){ if !this_cpu_read(async_pf_enabled){return;} wrmsrq(MSR_KVM_ASYNC_PF_EN,0); this_cpu_write(async_pf_enabled,false); }
unsafe fn kvm_disable_steal_time(){if has_steal_clock!=0{wrmsrq(MSR_KVM_STEAL_TIME,0);}}
unsafe fn kvm_guest_cpu_offline(shutdown:bool){kvm_disable_steal_time();if kvm_para_has_feature(KVM_FEATURE_PV_EOI){wrmsrq(MSR_KVM_PV_EOI_EN,0);}if kvm_para_has_feature(KVM_FEATURE_MIGRATION_CONTROL){wrmsrq(MSR_KVM_MIGRATION_CONTROL,0);}kvm_pv_disable_apf();if !shutdown{apf_task_wake_all();}kvmclock_disable();}
unsafe fn kvm_cpu_online(_cpu:u32)->i32{kvm_guest_cpu_init();0}

unsafe fn __kvm_cpuid_base()->u32{if boot_cpu_data.cpuid_level<0{return 0;}if boot_cpu_has(X86_FEATURE_HYPERVISOR){return cpuid_base_hypervisor(KVM_SIGNATURE,0);}0}
unsafe fn kvm_cpuid_base()->u32{static mut base:i32=-1;if base==-1{base=__kvm_cpuid_base() as i32;}base as u32}
pub unsafe fn kvm_para_available()->bool{kvm_cpuid_base()!=0}
pub unsafe fn kvm_arch_para_features()->u32{cpuid_eax(kvm_cpuid_base()|KVM_CPUID_FEATURES)}
pub unsafe fn kvm_arch_para_hints()->u32{cpuid_edx(kvm_cpuid_base()|KVM_CPUID_FEATURES)}

// Configuration-dependent registration, hypercall, APIC, TLB, suspend/resume,
// EFI, SEV, halt-poll, and paravirtual spinlock entry points remain represented
// by their corresponding external kernel symbols in the complete translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
