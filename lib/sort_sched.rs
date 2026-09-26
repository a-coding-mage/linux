// SPDX-License-Identifier: GPL-2.0
//! The original cond_resched expansion used by the nonatomic sort interfaces.

// Keep the actual static-call key visible to objtool, as __ADDRESSABLE does in
// static_call_mod(). The provider is built in, but retain the MODULE distinction
// of the original header for standalone consumers of the translated source.
#[cfg(all(
    CONFIG_PREEMPT_DYNAMIC,
    CONFIG_HAVE_PREEMPT_DYNAMIC_CALL,
    CONFIG_HAVE_STATIC_CALL_INLINE,
    not(MODULE)
))]
#[used]
#[link_section = ".discard.addressable"]
static mut COND_RESCHED_KEY: *const bindings::static_call_key =
    core::ptr::addr_of!(bindings::__SCK__cond_resched);

/// Execute the scheduler and atomic-context diagnostics selected by sched.h.
///
/// # Safety
/// Called only from the original nonatomic sort interfaces. `file` must point
/// to a live nul-terminated source location, as supplied by their call site.
#[inline(always)]
pub(super) unsafe fn cond_resched(file: *const u8, line: i32) {
    // The original __might_resched inline is empty without atomic-sleep debug.
    let _ = (file, line);
    #[cfg(CONFIG_DEBUG_ATOMIC_SLEEP)]
    // SAFETY: The call site supplies a static source filename and line number.
    unsafe {
        bindings::__might_resched(file.cast(), line, 0);
    }

    #[cfg(all(CONFIG_PREEMPT_DYNAMIC, CONFIG_HAVE_PREEMPT_DYNAMIC_CALL))]
    // SAFETY: This is the real kernel static-call trampoline. Its update and
    // objtool metadata retain the same scheduler-controlled dispatch as C.
    unsafe {
        bindings::__SCT__cond_resched();
    }

    #[cfg(all(
        CONFIG_PREEMPT_DYNAMIC,
        not(CONFIG_HAVE_PREEMPT_DYNAMIC_CALL),
        CONFIG_HAVE_PREEMPT_DYNAMIC_KEY
    ))]
    // SAFETY: The architecture's real dynamic-key implementation owns dispatch.
    unsafe {
        bindings::dynamic_cond_resched();
    }

    #[cfg(all(
        any(not(CONFIG_PREEMPTION), CONFIG_PREEMPT_DYNAMIC),
        not(all(CONFIG_PREEMPT_DYNAMIC, CONFIG_HAVE_PREEMPT_DYNAMIC_CALL)),
        not(all(CONFIG_PREEMPT_DYNAMIC, CONFIG_HAVE_PREEMPT_DYNAMIC_KEY))
    ))]
    // SAFETY: This is the original nonpreemptible scheduler yield point.
    unsafe {
        bindings::__cond_resched();
    }
    // CONFIG_PREEMPTION && !CONFIG_PREEMPT_DYNAMIC: _cond_resched returns zero.
}
