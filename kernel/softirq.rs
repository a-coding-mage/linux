// SPDX-License-Identifier: GPL-2.0-only
// Translation of linux/kernel/softirq.c. Kernel-provided symbols are external.

#[repr(C)]
pub struct softirq_action { pub action: Option<unsafe extern "C" fn()> }
#[repr(C)]
pub struct tasklet_head { pub head: *mut tasklet_struct, pub tail: *mut *mut tasklet_struct }
#[repr(C)]
pub struct tasklet_struct {
    pub next: *mut tasklet_struct, pub state: usize, pub count: atomic_t,
    pub func: Option<unsafe extern "C" fn(usize)>, pub callback: Option<unsafe extern "C" fn(*mut tasklet_struct)>,
    pub use_callback: bool, pub data: usize,
}
#[repr(C)] pub struct task_struct { pub flags: usize, pub softirq_disable_cnt: i32, pub preempt_disable_ip: usize }
#[repr(C)] pub struct atomic_t(pub i32);
#[repr(C)] pub struct smp_hotplug_thread { pub store: *mut *mut task_struct, pub setup: Option<unsafe extern "C" fn(u32)>, pub thread_should_run: Option<unsafe extern "C" fn(u32)->i32>, pub thread_fn: Option<unsafe extern "C" fn(u32)>, pub thread_comm: *const u8 }

extern "C" {
    static mut softirq_vec: [softirq_action; 10];
    static mut current: *mut task_struct;
    static mut ksoftirqd: *mut task_struct;
    static mut softirq_to_name: [*const u8; 10];
    fn __local_interrupt_disable(); fn __local_interrupt_enable();
    fn __this_cpu_read<T>(x: T) -> T; fn wake_up_process(t: *mut task_struct);
    fn __local_bh_disable_ip(ip: usize, cnt: u32); fn __local_bh_enable_ip(ip: usize, cnt: u32);
    fn __do_softirq(); fn local_softirq_pending() -> u32; fn set_softirq_pending(v: u32);
    fn local_irq_save(flags: *mut usize); fn local_irq_restore(flags: usize); fn local_irq_disable(); fn local_irq_enable();
    fn in_hardirq() -> bool; fn in_interrupt() -> bool; fn preemptible() -> bool; fn preempt_count() -> i32;
    fn softirq_count() -> u32; fn lockdep_assert_irqs_enabled(); fn lockdep_assert_irqs_disabled();
    fn __preempt_count_add(v: u32); fn __preempt_count_sub(v: u32); fn preempt_count_dec(); fn preempt_count_set(v: i32); fn preempt_check_resched();
    fn lockdep_softirqs_off(ip: usize); fn lockdep_softirqs_on(ip: usize); fn lockdep_softirq_enter(); fn lockdep_softirq_exit();
    fn lockdep_hardirq_context() -> bool; fn lockdep_hardirq_enter(); fn lockdep_hardirq_exit();
    fn account_softirq_enter(t: *mut task_struct); fn account_softirq_exit(t: *mut task_struct);
    fn ffs(v: u32) -> i32; fn jiffies() -> usize; fn time_before(a: usize,b:usize)->bool; fn need_resched()->bool;
    fn kstat_incr_softirqs_this_cpu(n: u32); fn trace_softirq_entry(n:u32); fn trace_softirq_exit(n:u32); fn trace_softirq_raise(n:u32);
    fn rcu_softirq_qs(); fn rcu_read_lock(); fn rcu_read_unlock(); fn migrate_disable(); fn migrate_enable();
    fn __raise_softirq_irqoff(n:u32); fn or_softirq_pending(v:u32); fn force_irqthreads()->bool; fn do_softirq_own_stack();
    fn ct_irq_enter(); fn ct_irq_exit(); fn __irq_enter_raw(); fn irq_count()->u32; fn smp_processor_id()->u32;
    fn tick_nohz_full_cpu(cpu:u32)->bool; fn is_idle_task(t:*mut task_struct)->bool; fn tick_irq_enter(); fn tick_nohz_irq_exit();
    fn hrtimer_rearm_deferred(); fn sched_core_idle_cpu(cpu:i32)->bool; fn hardirq_disable_count()->u32; fn in_nmi()->bool;
    fn tasklet_trylock(t:*mut tasklet_struct)->bool; fn tasklet_unlock(t:*mut tasklet_struct); fn atomic_read(a:*const atomic_t)->i32;
    fn test_and_clear_wake_up_bit(n:u32,p:*mut usize)->bool; fn clear_and_wake_up_bit(n:u32,p:*mut usize); fn wait_on_bit_lock(p:*mut usize,n:u32,state:u32); fn wait_on_bit(p:*mut usize,n:u32,state:u32);
    fn cpu_relax(); fn cond_resched(); fn warn_once(x:bool,msg:*const u8); fn pr_notice(msg:*const u8); fn sched_set_fifo_low(t:*mut task_struct);
    fn workqueue_softirq_action(high:bool); fn workqueue_softirq_dead(cpu:u32); fn cpuhp_setup_state_nocalls(a:i32,b:*const u8,c:Option<unsafe extern "C" fn(u32)->i32>,d:Option<unsafe extern "C" fn(u32)->i32>);
    fn smpboot_register_percpu_thread(t:*mut smp_hotplug_thread)->i32; fn local_timers_pending_force_th()->u32;
}

static mut SOFTIRQ_VEC: [softirq_action; 10] = [softirq_action { action: None }; 10];
static mut SOFTIRQ_TO_NAME: [*const u8; 10] = [b"HI\0".as_ptr(),b"TIMER\0".as_ptr(),b"NET_TX\0".as_ptr(),b"NET_RX\0".as_ptr(),b"BLOCK\0".as_ptr(),b"IRQ_POLL\0".as_ptr(),b"TASKLET\0".as_ptr(),b"SCHED\0".as_ptr(),b"HRTIMER\0".as_ptr(),b"RCU\0".as_ptr()];

unsafe fn wakeup_softirqd() { let t = ksoftirqd; if !t.is_null() { wake_up_process(t); } }
pub unsafe extern "C" fn _local_interrupt_disable() { __local_interrupt_disable(); }
pub unsafe extern "C" fn _local_interrupt_enable() { __local_interrupt_enable(); }

pub unsafe extern "C" fn do_softirq() { if in_interrupt() { return; } let mut f=0; local_irq_save(&mut f); if local_softirq_pending()!=0 { do_softirq_own_stack(); } local_irq_restore(f); }
pub unsafe extern "C" fn raise_softirq_irqoff(n:u32) { __raise_softirq_irqoff(n); if !in_interrupt() { wakeup_softirqd(); } }
pub unsafe extern "C" fn raise_softirq(n:u32) { let mut f=0; local_irq_save(&mut f); raise_softirq_irqoff(n); local_irq_restore(f); }
pub unsafe extern "C" fn __raise_softirq_irqoff(n:u32) { trace_softirq_raise(n); or_softirq_pending(1u32.wrapping_shl(n)); }
pub unsafe extern "C" fn open_softirq(n:i32, action:Option<unsafe extern "C" fn()>) { SOFTIRQ_VEC[n as usize].action=action; }

unsafe fn handle_softirqs(_ksirqd:bool) {
    let end=jiffies().wrapping_add(2); let old=(*current).flags; let mut restart=10; (*current).flags &= !0x00100000usize;
    let mut pending=local_softirq_pending(); __local_bh_disable_ip(0,1); lockdep_softirq_enter(); account_softirq_enter(current);
    loop { set_softirq_pending(0); local_irq_enable(); let mut bit=ffs(pending); while bit!=0 { let n=(bit-1) as usize; let h=&SOFTIRQ_VEC[n]; if let Some(a)=h.action { a(); } pending >>= bit; bit=ffs(pending); } local_irq_disable(); pending=local_softirq_pending(); if pending!=0 && time_before(jiffies(),end) && !need_resched() { restart-=1; if restart>0 { continue; } wakeup_softirqd(); } break; }
    account_softirq_exit(current); lockdep_softirq_exit(); __local_bh_enable_ip(0,1); (*current).flags=old;
}
pub unsafe extern "C" fn __do_softirq() { handle_softirqs(false); }

pub unsafe extern "C" fn irq_enter_rcu() { __irq_enter_raw(); hrtimer_rearm_deferred(); if tick_nohz_full_cpu(smp_processor_id()) || (is_idle_task(current) && irq_count()==1) { tick_irq_enter(); } account_hardirq_enter(current); }
pub unsafe extern "C" fn irq_enter() { ct_irq_enter(); irq_enter_rcu(); }
extern "C" { fn account_hardirq_enter(t:*mut task_struct); fn account_hardirq_exit(t:*mut task_struct); fn preempt_count_sub(v:u32); }
pub unsafe extern "C" fn irq_exit_rcu() { local_irq_disable(); account_hardirq_exit(current); preempt_count_sub(1); if !in_interrupt() && hardirq_disable_count()==0 && local_softirq_pending()!=0 { hrtimer_rearm_deferred(); wakeup_softirqd(); } lockdep_hardirq_exit(); }
pub unsafe extern "C" fn irq_exit() { irq_exit_rcu(); ct_irq_exit(); }

pub unsafe extern "C" fn tasklet_setup(t:*mut tasklet_struct, cb:Option<unsafe extern "C" fn(*mut tasklet_struct)>) { (*t).next=core::ptr::null_mut(); (*t).state=0; (*t).count.0=0; (*t).callback=cb; (*t).use_callback=true; (*t).data=0; }
pub unsafe extern "C" fn tasklet_init(t:*mut tasklet_struct, f:Option<unsafe extern "C" fn(usize)>, d:usize) { (*t).next=core::ptr::null_mut(); (*t).state=0; (*t).count.0=0; (*t).func=f; (*t).use_callback=false; (*t).data=d; }
pub unsafe extern "C" fn tasklet_kill(t:*mut tasklet_struct) { if in_interrupt(){pr_notice(b"Attempt to kill tasklet from interrupt\n\0".as_ptr());} wait_on_bit_lock(&mut (*t).state,0,2); wait_on_bit(&mut (*t).state,1,2); clear_and_wake_up_bit(0,&mut (*t).state); }
pub unsafe extern "C" fn tasklet_unlock_spin_wait(t:*mut tasklet_struct) { while ((*t).state & 2)!=0 { cpu_relax(); } }

pub unsafe extern "C" fn softirq_init() { open_softirq(6,None); open_softirq(0,None); }
pub unsafe extern "C" fn ksoftirqd_should_run(_cpu:u32)->i32 { local_softirq_pending() as i32 }
pub unsafe extern "C" fn run_ksoftirqd(_cpu:u32) { if local_softirq_pending()!=0 { handle_softirqs(true); cond_resched(); } }
pub unsafe extern "C" fn early_irq_init()->i32 { 0 }
pub unsafe extern "C" fn arch_probe_nr_irqs()->i32 { 16 }
pub unsafe extern "C" fn arch_early_irq_init()->i32 { 0 }
pub unsafe extern "C" fn arch_dynirq_lower_bound(from:u32)->u32 { from }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
