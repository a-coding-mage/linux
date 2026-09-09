// SPDX-License-Identifier: GPL-2.0
/*
 * Functions related to interrupt-poll handling in the block layer. This
 * is similar to NAPI for network devices.
 */

// Dependencies supplied by the surrounding kernel translation.

static mut IRQ_POLL_BUDGET: ::core::primitive::u32 = 256;

extern "C" {
    static mut blk_cpu_iopoll: PerCpu<ListHead>;

    fn test_bit(nr: ::core::primitive::c_ulong, addr: *const ::core::primitive::c_ulong) -> bool;
    fn test_and_set_bit(nr: ::core::primitive::c_ulong, addr: *mut ::core::primitive::c_ulong) -> bool;
    fn clear_bit_unlock(nr: ::core::primitive::c_ulong, addr: *mut ::core::primitive::c_ulong);
    fn set_bit(nr: ::core::primitive::c_ulong, addr: *mut ::core::primitive::c_ulong);
    fn smp_mb__before_atomic();
    fn local_irq_save(flags: *mut ::core::primitive::c_ulong);
    fn local_irq_restore(flags: ::core::primitive::c_ulong);
    fn local_irq_disable();
    fn local_irq_enable();
    fn raise_softirq_irqoff(nr: ::core::primitive::c_uint);
    fn __raise_softirq_irqoff(nr: ::core::primitive::c_uint);
    fn jiffies() -> ::core::primitive::c_ulong;
    fn time_after(a: ::core::primitive::c_ulong, b: ::core::primitive::c_ulong) -> bool;
    fn msleep(msecs: ::core::primitive::c_uint);
    fn memset(s: *mut ::core::ffi::c_void, c: ::core::primitive::c_int, n: usize) -> *mut ::core::ffi::c_void;
    fn list_add_tail(new: *mut ListHead, head: *mut ListHead);
    fn list_del(entry: *mut ListHead);
    fn list_move_tail(list: *mut ListHead, head: *mut ListHead);
    fn list_empty(head: *const ListHead) -> bool;
    fn list_splice_init(list: *mut ListHead, head: *mut ListHead);
    fn this_cpu_ptr<T>(var: *mut PerCpu<T>) -> *mut T;
    fn per_cpu<T>(var: *mut PerCpu<T>, cpu: ::core::primitive::uint) -> *mut T;
    fn open_softirq(nr: ::core::primitive::c_uint, action: unsafe extern "C" fn());
    fn cpuhp_setup_state_nocalls(state: ::core::primitive::int, name: *const ::core::primitive::c_char, startup: Option<unsafe extern "C" fn()>, teardown: Option<unsafe extern "C" fn(::core::primitive::uint) -> ::core::primitive::int>) -> ::core::primitive::int;
    fn local_bh_disable();
    fn local_bh_enable();
    fn BUG_ON(condition: bool);
}

#[repr(C)]
pub struct ListHead {
    pub next: *mut ListHead,
    pub prev: *mut ListHead,
}

pub struct PerCpu<T>(::core::marker::PhantomData<T>);

#[repr(C)]
pub struct IrqPoll {
    pub list: ListHead,
    pub state: ::core::primitive::c_ulong,
    pub weight: ::core::primitive::c_int,
    pub poll: Option<unsafe extern "C" fn(*mut IrqPoll, ::core::primitive::c_int) -> ::core::primitive::c_int>,
}

pub type IrqPollFn = unsafe extern "C" fn(*mut IrqPoll, ::core::primitive::c_int) -> ::core::primitive::c_int;

extern "C" {
    static IRQ_POLL_F_DISABLE: ::core::primitive::c_ulong;
    static IRQ_POLL_F_SCHED: ::core::primitive::c_ulong;
    static IRQ_POLL_SOFTIRQ: ::core::primitive::c_uint;
}

unsafe fn irq_poll_sched_impl(iop: *mut IrqPoll) {
    let mut flags: ::core::primitive::c_ulong = 0;

    if test_bit(IRQ_POLL_F_DISABLE, &(*iop).state) { return; }
    if test_and_set_bit(IRQ_POLL_F_SCHED, &mut (*iop).state) { return; }

    local_irq_save(&mut flags);
    list_add_tail(&mut (*iop).list, this_cpu_ptr(&mut blk_cpu_iopoll));
    raise_softirq_irqoff(IRQ_POLL_SOFTIRQ);
    local_irq_restore(flags);
}

pub unsafe extern "C" fn irq_poll_sched(iop: *mut IrqPoll) { irq_poll_sched_impl(iop); }

unsafe fn irq_poll_complete_impl(iop: *mut IrqPoll) {
    list_del(&mut (*iop).list);
    smp_mb__before_atomic();
    clear_bit_unlock(IRQ_POLL_F_SCHED, &mut (*iop).state);
}

pub unsafe extern "C" fn irq_poll_complete(iop: *mut IrqPoll) {
    let mut flags: ::core::primitive::c_ulong = 0;
    local_irq_save(&mut flags);
    irq_poll_complete_impl(iop);
    local_irq_restore(flags);
}

unsafe extern "C" fn irq_poll_softirq() {
    let list = this_cpu_ptr(&mut blk_cpu_iopoll);
    let mut rearm = 0;
    let mut budget = IRQ_POLL_BUDGET as ::core::primitive::c_int;
    let start_time = jiffies();

    local_irq_disable();
    while !list_empty(list) {
        if budget <= 0 || time_after(jiffies(), start_time) { rearm = 1; break; }
        local_irq_enable();
        let iop = ( (*list).next as *mut IrqPoll );
        let weight = (*iop).weight;
        let mut work = 0;
        if test_bit(IRQ_POLL_F_SCHED, &(*iop).state) {
            work = ((*iop).poll.expect("irq_poll poll callback"))(iop, weight);
        }
        budget -= work;
        local_irq_disable();
        if work >= weight {
            if test_bit(IRQ_POLL_F_DISABLE, &(*iop).state) { irq_poll_complete_impl(iop); }
            else { list_move_tail(&mut (*iop).list, list); }
        }
    }
    if rearm != 0 { __raise_softirq_irqoff(IRQ_POLL_SOFTIRQ); }
    local_irq_enable();
}

pub unsafe extern "C" fn irq_poll_disable(iop: *mut IrqPoll) {
    set_bit(IRQ_POLL_F_DISABLE, &mut (*iop).state);
    while test_and_set_bit(IRQ_POLL_F_SCHED, &mut (*iop).state) { msleep(1); }
    clear_bit(IRQ_POLL_F_DISABLE, &mut (*iop).state);
}

pub unsafe extern "C" fn irq_poll_enable(iop: *mut IrqPoll) {
    BUG_ON(!test_bit(IRQ_POLL_F_SCHED, &(*iop).state));
    smp_mb__before_atomic();
    clear_bit_unlock(IRQ_POLL_F_SCHED, &mut (*iop).state);
}

pub unsafe extern "C" fn irq_poll_init(iop: *mut IrqPoll, weight: ::core::primitive::c_int, poll_fn: Option<IrqPollFn>) {
    memset(iop as *mut ::core::ffi::c_void, 0, ::core::mem::size_of::<IrqPoll>());
    (*iop).list = ListHead { next: &mut (*iop).list, prev: &mut (*iop).list };
    (*iop).weight = weight;
    (*iop).poll = poll_fn;
}

unsafe extern "C" fn irq_poll_cpu_dead(cpu: ::core::primitive::uint) -> ::core::primitive::int {
    local_bh_disable();
    local_irq_disable();
    list_splice_init(per_cpu(&mut blk_cpu_iopoll, cpu), this_cpu_ptr(&mut blk_cpu_iopoll));
    __raise_softirq_irqoff(IRQ_POLL_SOFTIRQ);
    local_irq_enable();
    local_bh_enable();
    0
}

unsafe extern "C" fn irq_poll_setup() -> ::core::primitive::int {
    // for_each_possible_cpu(i)
    let mut i: ::core::primitive::int = 0;
    while i < 0 {
        let head = per_cpu(&mut blk_cpu_iopoll, i as ::core::primitive::uint);
        unsafe { (*head).next = head as *mut ListHead; (*head).prev = head as *mut ListHead; }
        i += 1;
    }
    open_softirq(IRQ_POLL_SOFTIRQ, irq_poll_softirq);
    cpuhp_setup_state_nocalls(0, b"irq_poll:dead\0".as_ptr() as *const ::core::primitive::c_char, None, Some(irq_poll_cpu_dead));
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
