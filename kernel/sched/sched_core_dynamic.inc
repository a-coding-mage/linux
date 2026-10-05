/* SPDX-License-Identifier: GPL-2.0-only */
/* Exact target aliases, trampoline identities and static-key storage from
 * core.c. Rust owns dynamic policy/order and all scheduler algorithm bodies. */
#ifdef CONFIG_PREEMPT_DYNAMIC
#ifdef CONFIG_HAVE_PREEMPT_DYNAMIC_CALL
#ifdef CONFIG_PREEMPTION
#ifndef preempt_schedule_dynamic_enabled
#define preempt_schedule_dynamic_enabled preempt_schedule
#define preempt_schedule_dynamic_disabled NULL
#endif
DEFINE_STATIC_CALL(preempt_schedule, preempt_schedule_dynamic_enabled);
EXPORT_STATIC_CALL_TRAMP(preempt_schedule);
#ifndef preempt_schedule_notrace_dynamic_enabled
#define preempt_schedule_notrace_dynamic_enabled preempt_schedule_notrace
#define preempt_schedule_notrace_dynamic_disabled NULL
#endif
DEFINE_STATIC_CALL(preempt_schedule_notrace, preempt_schedule_notrace_dynamic_enabled);
EXPORT_STATIC_CALL_TRAMP(preempt_schedule_notrace);
#endif
#define cond_resched_dynamic_enabled __cond_resched
#define cond_resched_dynamic_disabled ((void *)&__static_call_return0)
DEFINE_STATIC_CALL_RET0(cond_resched, __cond_resched);
EXPORT_STATIC_CALL_TRAMP(cond_resched);
#define might_resched_dynamic_enabled __cond_resched
#define might_resched_dynamic_disabled ((void *)&__static_call_return0)
DEFINE_STATIC_CALL_RET0(might_resched, __cond_resched);
EXPORT_STATIC_CALL_TRAMP(might_resched);
#define LUPOS_CORE_DYNAMIC_ENABLE(f) static_call_update(f, f##_dynamic_enabled)
#define LUPOS_CORE_DYNAMIC_DISABLE(f) static_call_update(f, f##_dynamic_disabled)
#elif defined(CONFIG_HAVE_PREEMPT_DYNAMIC_KEY)
static DEFINE_STATIC_KEY_FALSE(sk_dynamic_cond_resched);
static DEFINE_STATIC_KEY_FALSE(sk_dynamic_might_resched);
#ifdef CONFIG_PREEMPTION
static DEFINE_STATIC_KEY_TRUE(sk_dynamic_preempt_schedule);
static DEFINE_STATIC_KEY_TRUE(sk_dynamic_preempt_schedule_notrace);
#endif
#define LUPOS_CORE_DYNAMIC_ENABLE(f) static_key_enable(&sk_dynamic_##f.key)
#define LUPOS_CORE_DYNAMIC_DISABLE(f) static_key_disable(&sk_dynamic_##f.key)
#else
#error "Unsupported PREEMPT_DYNAMIC mechanism"
#endif
#endif
