/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/trace_recursion.h. */

#[cfg(CONFIG_TRACING)]
pub const TRACE_FTRACE_BIT: u32 = 0;
#[cfg(CONFIG_TRACING)]
pub const TRACE_FTRACE_NMI_BIT: u32 = 1;
#[cfg(CONFIG_TRACING)]
pub const TRACE_FTRACE_IRQ_BIT: u32 = 2;
#[cfg(CONFIG_TRACING)]
pub const TRACE_FTRACE_SIRQ_BIT: u32 = 3;
#[cfg(CONFIG_TRACING)]
pub const TRACE_FTRACE_TRANSITION_BIT: u32 = 4;
#[cfg(CONFIG_TRACING)]
pub const TRACE_INTERNAL_BIT: u32 = 5;
#[cfg(CONFIG_TRACING)]
pub const TRACE_INTERNAL_NMI_BIT: u32 = 6;
#[cfg(CONFIG_TRACING)]
pub const TRACE_INTERNAL_IRQ_BIT: u32 = 7;
#[cfg(CONFIG_TRACING)]
pub const TRACE_INTERNAL_SIRQ_BIT: u32 = 8;
#[cfg(CONFIG_TRACING)]
pub const TRACE_INTERNAL_TRANSITION_BIT: u32 = 9;
#[cfg(CONFIG_TRACING)]
pub const TRACE_INTERNAL_EVENT_BIT: u32 = 10;
#[cfg(CONFIG_TRACING)]
pub const TRACE_INTERNAL_EVENT_NMI_BIT: u32 = 11;
#[cfg(CONFIG_TRACING)]
pub const TRACE_INTERNAL_EVENT_IRQ_BIT: u32 = 12;
#[cfg(CONFIG_TRACING)]
pub const TRACE_INTERNAL_EVENT_SIRQ_BIT: u32 = 13;
#[cfg(CONFIG_TRACING)]
pub const TRACE_INTERNAL_EVENT_TRANSITION_BIT: u32 = 14;
#[cfg(CONFIG_TRACING)]
pub const TRACE_BRANCH_BIT: u32 = 15;
#[cfg(CONFIG_TRACING)]
pub const TRACE_IRQ_BIT: u32 = 16;
#[cfg(CONFIG_TRACING)]
pub const TRACE_RECORD_RECURSION_BIT: u32 = 17;

#[cfg(CONFIG_TRACING)]
pub const TRACE_CONTEXT_BITS: u32 = 4;
#[cfg(CONFIG_TRACING)]
pub const TRACE_FTRACE_START: u32 = TRACE_FTRACE_BIT;
#[cfg(CONFIG_TRACING)]
pub const TRACE_LIST_START: u32 = TRACE_INTERNAL_BIT;
#[cfg(CONFIG_TRACING)]
pub const TRACE_EVENT_START: u32 = TRACE_INTERNAL_EVENT_BIT;
#[cfg(CONFIG_TRACING)]
pub const TRACE_CONTEXT_MASK: u32 = (1 << (TRACE_LIST_START + TRACE_CONTEXT_BITS)) - 1;

#[cfg(CONFIG_TRACING)]
pub const TRACE_CTX_NMI: i32 = 0;
#[cfg(CONFIG_TRACING)]
pub const TRACE_CTX_IRQ: i32 = 1;
#[cfg(CONFIG_TRACING)]
pub const TRACE_CTX_SOFTIRQ: i32 = 2;
#[cfg(CONFIG_TRACING)]
pub const TRACE_CTX_NORMAL: i32 = 3;
#[cfg(CONFIG_TRACING)]
pub const TRACE_CTX_TRANSITION: i32 = 4;

#[cfg(CONFIG_TRACING)]
extern "C" {
    pub fn interrupt_context_level() -> u8;
    pub fn preempt_disable_notrace();
    pub fn preempt_enable_notrace();
    pub fn barrier();
    pub fn rcu_is_watching() -> bool;
    pub fn WARN_ONCE(condition: bool, format: *const core::ffi::c_char, ...);
    pub fn ftrace_record_recursion(ip: u64, parent_ip: u64);
    pub fn current_trace_recursion() -> *mut u32;
}

#[cfg(CONFIG_TRACING)]
#[inline(always)]
pub unsafe fn trace_recursion_set(bit: u32) {
    *current_trace_recursion() |= 1u32 << bit;
}

#[cfg(CONFIG_TRACING)]
#[inline(always)]
pub unsafe fn trace_recursion_clear(bit: u32) {
    *current_trace_recursion() &= !(1u32 << bit);
}

#[cfg(CONFIG_TRACING)]
#[inline(always)]
pub unsafe fn trace_recursion_test(bit: u32) -> u32 {
    *current_trace_recursion() & (1u32 << bit)
}

#[cfg(CONFIG_TRACING)]
#[inline(always)]
pub unsafe fn trace_get_context_bit() -> i32 {
    let bit = interrupt_context_level();
    TRACE_CTX_NORMAL - bit as i32
}

#[cfg(all(CONFIG_TRACING, CONFIG_FTRACE_RECORD_RECURSION))]
#[inline(always)]
pub unsafe fn do_ftrace_record_recursion(ip: u64, pip: u64) {
    if trace_recursion_test(TRACE_RECORD_RECURSION_BIT) == 0 {
        trace_recursion_set(TRACE_RECORD_RECURSION_BIT);
        ftrace_record_recursion(ip, pip);
        trace_recursion_clear(TRACE_RECORD_RECURSION_BIT);
    }
}

#[cfg(all(CONFIG_TRACING, not(CONFIG_FTRACE_RECORD_RECURSION)))]
#[inline(always)]
pub unsafe fn do_ftrace_record_recursion(_ip: u64, _pip: u64) {}

#[cfg(CONFIG_TRACING)]
#[inline(always)]
pub unsafe fn trace_test_and_set_recursion(ip: u64, pip: u64, start: i32) -> i32 {
    let mut val = core::ptr::read_volatile(current_trace_recursion());
    if cfg!(CONFIG_FTRACE_VALIDATE_RCU_IS_WATCHING) && !rcu_is_watching() {
        if trace_recursion_test(TRACE_RECORD_RECURSION_BIT) == 0 {
            trace_recursion_set(TRACE_RECORD_RECURSION_BIT);
            WARN_ONCE(true, b"RCU not on for: %pS\n\0".as_ptr() as *const _);
            trace_recursion_clear(TRACE_RECORD_RECURSION_BIT);
        }
        return -1;
    }
    let mut bit = trace_get_context_bit() + start;
    if val & (1u32 << bit) != 0 {
        bit = TRACE_CTX_TRANSITION + start;
        if val & (1u32 << bit) != 0 {
            do_ftrace_record_recursion(ip, pip);
            return -1;
        }
    }
    val |= 1u32 << bit;
    *current_trace_recursion() = val;
    barrier();
    preempt_disable_notrace();
    bit
}

#[cfg(CONFIG_TRACING)]
#[inline(always)]
pub unsafe fn trace_clear_recursion(bit: i32) {
    preempt_enable_notrace();
    barrier();
    trace_recursion_clear(bit as u32);
}

#[cfg(CONFIG_TRACING)]
#[inline(always)]
pub unsafe fn ftrace_test_recursion_trylock(ip: u64, parent_ip: u64) -> i32 {
    trace_test_and_set_recursion(ip, parent_ip, TRACE_FTRACE_START as i32)
}

#[cfg(CONFIG_TRACING)]
#[inline(always)]
pub unsafe fn ftrace_test_recursion_unlock(bit: i32) {
    trace_clear_recursion(bit);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
