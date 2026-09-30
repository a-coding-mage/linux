// SPDX-License-Identifier: GPL-2.0-only
/* Capture only the x86 static-call target; the original WARN macro and Rust
 * warning emit and pass their real target-native bug_entry records. */
#include <linux/bug.h>
#include <linux/static_call_types.h>
#include <linux/string.h>
#include <linux/stdarg.h>

extern int puts(const char *);
extern void abort(void);
extern long write(int, const void *, unsigned long);
bool c_warning(int);
bool rust_warning(int);
bool fixture_condition(int value);
int fixture_argument(void);

static int condition_calls, argument_calls, trap_calls, captured_argument;
static int stored_value;
struct static_call_key __SCK__WARN_trap;

static void fail(const char *reason)
{
	write(2, reason, strlen(reason));
	abort();
}

bool fixture_condition(int value)
{
	condition_calls++;
	stored_value = value;
	return value != 0;
}

int fixture_argument(void)
{
	argument_calls++;
	return stored_value ^ 0x4a17;
}

#ifdef CONFIG_BUG
void __SCT__WARN_trap(struct bug_entry *entry, ...)
{
	va_list args;
	const char *format = (const char *)&entry->format_disp + entry->format_disp;

	if (strcmp(format, "initcall warning argument=%d\n"))
		fail("warning format differs\n");
	if (entry->flags != (BUGFLAG_WARNING | BUGFLAG_ARGS | BUGFLAG_TAINT(TAINT_WARN)))
		fail("warning flags differ\n");
	va_start(args, entry);
	captured_argument = va_arg(args, int);
	va_end(args);
	trap_calls++;
}
#endif

int main(void)
{
	for (int value = -256; value <= 256; ++value) {
		condition_calls = argument_calls = trap_calls = captured_argument = 0;
		bool result = c_warning(value);
		int conditions = condition_calls, arguments = argument_calls;
		int traps = trap_calls, captured = captured_argument;
		condition_calls = argument_calls = trap_calls = captured_argument = 0;
		if (rust_warning(value) != result)
			fail("warning result differs\n");
		if (conditions != condition_calls || condition_calls != 1)
			fail("warning condition evaluations differ\n");
		if (arguments != argument_calls)
			fail("warning argument evaluations differ\n");
		if (traps != trap_calls)
			fail("warning trap calls differ\n");
		if (captured != captured_argument)
			fail("warning captured argument differs\n");
	}
	puts("INIT_MAIN_WARN_OK cases=513");
	return 0;
}
