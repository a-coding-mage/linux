// SPDX-License-Identifier: GPL-2.0
/*
 * Native leaves for the Rust BUG()/WARN() reporting owner: warn_rcu_enter()
 * and warn_rcu_exit(), the KUnit warning-suppression static branch,
 * disable_trace_on_warning(), the RCU list primitives and the
 * architecture's __warn_args() va_list reconstruction for vprintk(). Bug
 * table lookup and the report sequence live in lib/bug.rs.
 */
#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/context_tracking.h>
#include <linux/ftrace.h>
#include <linux/printk.h>
#include <linux/rculist.h>
#include <kunit/test-bug.h>

/* lib/bug.rs implements this configured bug-table profile. */
#if !defined(CONFIG_GENERIC_BUG_RELATIVE_POINTERS) || \
    !defined(HAVE_ARCH_BUG_FORMAT) || !defined(HAVE_ARCH_BUG_FORMAT_ARGS) || \
    !defined(CONFIG_DEBUG_BUGVERBOSE)
#error "lib/bug.rs implements the x86-64 relative, formatted, verbose bug table"
#endif
static_assert(BUG_TRAP_TYPE_NONE == 0 && BUG_TRAP_TYPE_WARN == 1 &&
	      BUG_TRAP_TYPE_BUG == 2);
static_assert(BUGFLAG_WARNING == 1 && BUGFLAG_ONCE == 2 && BUGFLAG_DONE == 4 &&
	      BUGFLAG_NO_CUT_HERE == 8 && BUGFLAG_ARGS == 16);
static_assert(BUGFLAG_TAINT(1) == (1 << 8));
static_assert(sizeof(struct bug_entry) == 16);

bool lupos_bug_warn_rcu_enter(void);
void lupos_bug_warn_rcu_exit(bool rcu);
bool lupos_bug_kunit_is_suppressed_warning(bool count);
void lupos_bug_disable_trace_on_warning(void);
bool lupos_bug_warn_printf_args(const char *fmt, struct pt_regs *regs);
void lupos_bug_list_add_rcu(struct list_head *new, struct list_head *head);
void lupos_bug_list_del_rcu(struct list_head *entry);

bool lupos_bug_warn_rcu_enter(void)
{
	return warn_rcu_enter();
}

void lupos_bug_warn_rcu_exit(bool rcu)
{
	warn_rcu_exit(rcu);
}

bool lupos_bug_kunit_is_suppressed_warning(bool count)
{
	return kunit_is_suppressed_warning(count);
}

void lupos_bug_disable_trace_on_warning(void)
{
	disable_trace_on_warning();
}

/* __warn_printf(): print the format with arguments rebuilt from regs. */
bool lupos_bug_warn_printf_args(const char *fmt, struct pt_regs *regs)
{
	struct arch_va_list _args;
	va_list *args = __warn_args(&_args, regs);

	if (args) {
		vprintk(fmt, *args);
		return true;
	}
	return false;
}

void lupos_bug_list_add_rcu(struct list_head *new, struct list_head *head)
{
	list_add_rcu(new, head);
}

void lupos_bug_list_del_rcu(struct list_head *entry)
{
	list_del_rcu(entry);
}
