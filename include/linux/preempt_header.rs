/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/preempt.h. */

// Dependencies supplied by other translated headers are intentionally external.

pub const PREEMPT_BITS: u32 = 8;
pub const SOFTIRQ_BITS: u32 = 8;
pub const HARDIRQ_DISABLE_BITS: u32 = 8;
pub const HARDIRQ_BITS: u32 = 4;
pub const NMI_BITS: u32 = 1 + 3 * (IS_ENABLED!(CONFIG_HAS_SEPARATE_PREEMPT_RESCHED_BITS) as u32);

pub const PREEMPT_SHIFT: u32 = 0;
pub const SOFTIRQ_SHIFT: u32 = PREEMPT_SHIFT + PREEMPT_BITS;
pub const HARDIRQ_DISABLE_SHIFT: u32 = SOFTIRQ_SHIFT + SOFTIRQ_BITS;
pub const HARDIRQ_SHIFT: u32 = HARDIRQ_DISABLE_SHIFT + HARDIRQ_DISABLE_BITS;
pub const NMI_SHIFT: u32 = HARDIRQ_SHIFT + HARDIRQ_BITS;

#[inline(always)]
pub const fn __irq_mask(x: u32) -> usize { ((1usize << x) - 1) }
pub const PREEMPT_MASK: usize = __irq_mask(PREEMPT_BITS) << PREEMPT_SHIFT;
pub const SOFTIRQ_MASK: usize = __irq_mask(SOFTIRQ_BITS) << SOFTIRQ_SHIFT;
pub const HARDIRQ_DISABLE_MASK: usize = __irq_mask(HARDIRQ_DISABLE_BITS) << HARDIRQ_DISABLE_SHIFT;
pub const HARDIRQ_MASK: usize = __irq_mask(HARDIRQ_BITS) << HARDIRQ_SHIFT;
pub const NMI_MASK: usize = __irq_mask(NMI_BITS) << NMI_SHIFT;
pub const PREEMPT_OFFSET: usize = 1usize << PREEMPT_SHIFT;
pub const SOFTIRQ_OFFSET: usize = 1usize << SOFTIRQ_SHIFT;
pub const HARDIRQ_DISABLE_OFFSET: usize = 1usize << HARDIRQ_DISABLE_SHIFT;
pub const HARDIRQ_OFFSET: usize = 1usize << HARDIRQ_SHIFT;
pub const NMI_OFFSET: usize = 1usize << NMI_SHIFT;
pub const SOFTIRQ_DISABLE_OFFSET: usize = 2 * SOFTIRQ_OFFSET;
pub const PREEMPT_DISABLE_OFFSET: usize = if cfg!(CONFIG_PREEMPT_COUNT) { PREEMPT_OFFSET } else { 0 };
pub const PREEMPT_ENABLED: usize = 0;
pub const PREEMPT_DISABLED: usize = PREEMPT_DISABLE_OFFSET + PREEMPT_ENABLED;
pub const INIT_PREEMPT_COUNT: usize = PREEMPT_OFFSET;
pub const FORK_PREEMPT_COUNT: usize = 2 * PREEMPT_DISABLE_OFFSET + PREEMPT_ENABLED;
pub const PREEMPT_LOCK_OFFSET: usize = if cfg!(CONFIG_PREEMPT_RT) { 0 } else { PREEMPT_DISABLE_OFFSET };
pub const SOFTIRQ_LOCK_OFFSET: usize = SOFTIRQ_DISABLE_OFFSET + PREEMPT_LOCK_OFFSET;

pub unsafe fn interrupt_context_level() -> u8 {
    let pc = preempt_count();
    let mut level: u8 = 0;
    level += ((pc & NMI_MASK) != 0) as u8;
    level += ((pc & (NMI_MASK | HARDIRQ_MASK)) != 0) as u8;
    level += ((pc & (NMI_MASK | HARDIRQ_MASK | SOFTIRQ_OFFSET)) != 0) as u8;
    level
}

#[macro_export] macro_rules! nmi_count { () => { preempt_count() & NMI_MASK }; }
#[macro_export] macro_rules! hardirq_count { () => { preempt_count() & HARDIRQ_MASK }; }
#[macro_export] macro_rules! softirq_count { () => { preempt_count() & SOFTIRQ_MASK }; }
#[macro_export] macro_rules! irq_count { () => { preempt_count() & (NMI_MASK | HARDIRQ_MASK | SOFTIRQ_MASK) }; }
#[macro_export] macro_rules! in_nmi { () => { nmi_count!() }; }
#[macro_export] macro_rules! in_hardirq { () => { hardirq_count!() }; }
#[macro_export] macro_rules! in_serving_softirq { () => { softirq_count!() & SOFTIRQ_OFFSET }; }
#[macro_export] macro_rules! in_task { () => { !(preempt_count() & (NMI_MASK | HARDIRQ_MASK | SOFTIRQ_OFFSET)) }; }
#[macro_export] macro_rules! in_softirq { () => { softirq_count!() }; }
#[macro_export] macro_rules! in_interrupt { () => { irq_count!() }; }
#[macro_export] macro_rules! hardirq_disable_count { () => { (preempt_count() & HARDIRQ_DISABLE_MASK) >> HARDIRQ_DISABLE_SHIFT }; }
#[macro_export] macro_rules! hardirq_disable_enter { () => { __preempt_count_add_return(HARDIRQ_DISABLE_OFFSET) }; }
#[macro_export] macro_rules! hardirq_disable_exit { () => { __preempt_count_sub_return(HARDIRQ_DISABLE_OFFSET) }; }
#[macro_export] macro_rules! in_atomic { () => { preempt_count() != 0 }; }
#[macro_export] macro_rules! in_atomic_preempt_off { () => { preempt_count() != PREEMPT_DISABLE_OFFSET }; }

extern "C" {
    pub fn preempt_count() -> usize;
    pub fn __preempt_count_add(val: i32);
    pub fn __preempt_count_sub(val: i32);
    pub fn __preempt_count_dec_and_test() -> bool;
    pub fn __preempt_schedule();
    pub fn __preempt_schedule_notrace();
    pub fn should_resched(flags: usize) -> bool;
    pub fn barrier();
    pub fn irqs_disabled() -> bool;
    pub fn set_preempt_need_resched();
    pub fn tif_need_resched() -> bool;
    pub fn lockdep_assert_preemption_disabled();
}

#[macro_export] macro_rules! preempt_count_add { ($v:expr) => { __preempt_count_add($v) }; }
#[macro_export] macro_rules! preempt_count_sub { ($v:expr) => { __preempt_count_sub($v) }; }
#[macro_export] macro_rules! __preempt_count_inc { () => { __preempt_count_add(1) }; }
#[macro_export] macro_rules! __preempt_count_dec { () => { __preempt_count_sub(1) }; }
#[macro_export] macro_rules! preempt_count_inc { () => { preempt_count_add!(1) }; }
#[macro_export] macro_rules! preempt_count_dec { () => { preempt_count_sub!(1) }; }
#[macro_export] macro_rules! preempt_disable { () => {{ preempt_count_inc!(); barrier(); }}; }
#[macro_export] macro_rules! sched_preempt_enable_no_resched { () => {{ barrier(); preempt_count_dec!(); }}; }
#[macro_export] macro_rules! preempt_enable_no_resched { () => { sched_preempt_enable_no_resched!() }; }
#[macro_export] macro_rules! preemptible { () => { preempt_count() == 0 && !irqs_disabled() }; }
#[macro_export] macro_rules! preempt_enable { () => {{ barrier(); if preempt_count_dec_and_test!() { __preempt_schedule(); } }}; }
#[macro_export] macro_rules! preempt_enable_notrace { () => {{ barrier(); if __preempt_count_dec_and_test() { __preempt_schedule_notrace(); } }}; }
#[macro_export] macro_rules! preempt_check_resched { () => {{ if should_resched(0) { __preempt_schedule(); } }}; }
#[macro_export] macro_rules! preempt_disable_notrace { () => {{ __preempt_count_inc!(); barrier(); }}; }
#[macro_export] macro_rules! preempt_enable_no_resched_notrace { () => {{ barrier(); __preempt_count_dec!(); }}; }
#[macro_export] macro_rules! preempt_count_dec_and_test { () => {{ preempt_count_sub!(1); should_resched(0) }}; }
#[macro_export] macro_rules! preempt_set_need_resched { () => {{ set_preempt_need_resched(); }}; }
#[macro_export] macro_rules! preempt_fold_need_resched { () => {{ if tif_need_resched() { set_preempt_need_resched(); } }}; }

#[inline(always)] pub unsafe fn preempt_disable_nested() { if cfg!(CONFIG_PREEMPT_RT) { preempt_disable!(); } else { lockdep_assert_preemption_disabled(); } }
#[inline(always)] pub unsafe fn preempt_enable_nested() { if cfg!(CONFIG_PREEMPT_RT) { preempt_enable!(); } }

extern "C" {
    pub fn preempt_model_none() -> bool;
    pub fn preempt_model_voluntary() -> bool;
    pub fn preempt_model_full() -> bool;
    pub fn preempt_model_lazy() -> bool;
    pub fn preempt_model_str() -> *const core::ffi::c_char;
}

#[inline] pub unsafe fn preempt_model_rt() -> bool { cfg!(CONFIG_PREEMPT_RT) }
#[inline] pub unsafe fn preempt_model_preemptible() -> bool { preempt_model_full() || preempt_model_lazy() || preempt_model_rt() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
