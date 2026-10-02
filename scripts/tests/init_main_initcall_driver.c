// SPDX-License-Identifier: GPL-2.0-only
/* Observe privileged-state and diagnostics boundaries while invoking the
 * unchanged C orchestration, genuine Rust orchestration and original strlcat. */
#include <linux/init.h>
#include <linux/bug.h>
#include <linux/random.h>
#include <linux/string.h>
#include "init_main_fortify_observer.h"
#include <linux/stdarg.h>
#include <linux/kernel.h>

extern int puts(const char *);
extern void exit(int) __attribute__((noreturn));
int c_do_one_initcall(initcall_t);
int fixture_count(void);
void fixture_set_count(int);
bool fixture_irqs_disabled(void);
void fixture_irq_enable(void);
bool fixture_blacklisted(initcall_t);
void fixture_trace_start(initcall_t);
void fixture_trace_finish(initcall_t, int);

#define CHECK(condition) do { if (!(condition)) { puts(#condition); exit(__LINE__); } } while (0)

struct static_call_key __SCK__WARN_trap;
struct event { int kind, value, count, irq; char message[64]; };
static struct event events[16], expected[16];
static int used, count, disabled, denied, returned, after_count, after_irq, trace_mutation;

/* The only sprintf in do_one_initcall has a constant literal and no format
 * arguments. Keep this observation boundary safe for the kernel C stack ABI;
 * formatter implementation coverage belongs to its existing library tests. */
__attribute__((force_align_arg_pointer))
int sprintf(char *buffer, const char *format, ...)
{
	CHECK(!strcmp(format, "preemption imbalance "));
	strcpy(buffer, format);
	return strlen(format);
}

static void record(int kind, int value)
{
	CHECK(used < ARRAY_SIZE(events));
	events[used++] = (struct event){ .kind = kind, .value = value, .count = count, .irq = disabled };
}

__attribute__((force_align_arg_pointer))
int fixture_count(void) { record(1, count); return count; }
__attribute__((force_align_arg_pointer))
void fixture_set_count(int value) { record(2, value); count = value; }
__attribute__((force_align_arg_pointer))
bool fixture_irqs_disabled(void) { record(3, disabled); return disabled; }
__attribute__((force_align_arg_pointer))
void fixture_irq_enable(void) { record(4, 0); disabled = 0; }

static int init_function(void)
{
	record(5, returned);
	count = after_count;
	disabled = after_irq;
	return returned;
}

__attribute__((force_align_arg_pointer))
bool fixture_blacklisted(initcall_t function)
{
	CHECK(function == init_function);
	record(6, denied);
	return denied;
}

__attribute__((force_align_arg_pointer))
void fixture_trace_start(initcall_t function)
{
	CHECK(function == init_function);
	record(7, 0);
}

__attribute__((force_align_arg_pointer))
void fixture_trace_finish(initcall_t function, int result)
{
	CHECK(function == init_function && result == returned);
	record(8, result);
	if (trace_mutation) { count ^= 0x13; disabled = !disabled; }
}

__attribute__((force_align_arg_pointer))
void add_device_randomness(const void *buffer, size_t size)
{
	CHECK(!buffer && !size);
	record(9, 0);
}

#ifdef CONFIG_BUG
__attribute__((force_align_arg_pointer))
void __SCT__WARN_trap(struct bug_entry *entry, ...)
{
	const char *format = (const char *)&entry->format_disp + entry->format_disp;
	va_list args;

	CHECK(!strcmp(format, "initcall %pS returned with %s\n"));
	CHECK(entry->flags == (BUGFLAG_WARNING | BUGFLAG_ARGS | BUGFLAG_TAINT(TAINT_WARN)));
	va_start(args, entry);
	CHECK(va_arg(args, void *) == (void *)init_function);
	const char *message = va_arg(args, const char *);
	record(10, 0);
	CHECK(strlen(message) < sizeof(events[0].message));
	strcpy(events[used - 1].message, message);
	va_end(args);
}
#endif

int main(void)
{
	const int counts[] = { 0, 1, 0x10000000, -1 };
	const int results[] = { 0, 19, -5, (-2147483647 - 1) };
	int cases = 0;

	for (int before = 0; before < 4; ++before)
	for (int after = 0; after < 4; ++after)
	for (int mode = 0; mode < 16; ++mode)
	for (int result = 0; result < 4; ++result) {
		int expected_used = 0, expected_result = 0, expected_count = 0, expected_irq = 0;
		for (int owner = 0; owner < 2; ++owner) {
			used = 0; count = counts[before]; disabled = mode & 1;
			denied = (mode >> 1) & 1; after_irq = (mode >> 2) & 1;
			trace_mutation = (mode >> 3) & 1; after_count = counts[after]; returned = results[result];
			int value = owner ? do_one_initcall(init_function) : c_do_one_initcall(init_function);
			if (!owner) {
				expected_used = used; expected_result = value; expected_count = count; expected_irq = disabled;
				memcpy(expected, events, sizeof(expected));
			} else {
				CHECK(value == expected_result && count == expected_count && disabled == expected_irq);
				CHECK(used == expected_used);
				for (int i = 0; i < used; ++i) {
					CHECK(events[i].kind == expected[i].kind && events[i].value == expected[i].value);
					CHECK(events[i].count == expected[i].count && events[i].irq == expected[i].irq);
					CHECK(!strcmp(events[i].message, expected[i].message));
				}
			}
		}
		cases++;
	}
	CHECK(cases == 1024);
	puts("INIT_MAIN_INITCALL_OK cases=1024");
	return 0;
}
