/* SPDX-License-Identifier: GPL-2.0-only */
#define saved_command_line original_saved_command_line
#define saved_command_line_len original_saved_command_line_len
#define do_one_initcall fixture_do_one
#include "canonical.h"
#undef saved_command_line
#undef saved_command_line_len
#undef do_one_initcall
#include <linux/ctype.h>

struct event { int kind; int argument; char value[160]; };
static struct event events[1024];
static unsigned int event_count;
static bool fail_allocation, mutate_entry;
static char saved[2048], allocated[2048];
char *original_saved_command_line;
unsigned int original_saved_command_line_len;

static void record(int kind, int argument, const char *value)
{
	struct event *event;
	if (event_count == ARRAY_SIZE(events))
		__builtin_trap();
	event = &events[event_count++];
	event->kind = kind;
	event->argument = argument;
	memset(event->value, 0, sizeof(event->value));
	if (value) {
		size_t length = strlen(value);
		if (length >= sizeof(event->value))
			__builtin_trap();
		memcpy(event->value, value, length);
	}
}

initcall_entry_t fixture_entries[19];
#ifdef CONFIG_HAVE_ARCH_PREL32_RELOCATIONS
#define ENTRY_SIZE "4"
#else
#define ENTRY_SIZE "8"
#endif
#define RANGE_SYMBOL(name, index) \
	asm(".globl " #name "\n.set " #name ", fixture_entries + " #index " * " ENTRY_SIZE "\n")
RANGE_SYMBOL(__initcall_start, 0);
RANGE_SYMBOL(__initcall0_start, 3);
RANGE_SYMBOL(__initcall1_start, 5);
RANGE_SYMBOL(__initcall2_start, 5);
RANGE_SYMBOL(__initcall3_start, 8);
RANGE_SYMBOL(__initcall4_start, 9);
RANGE_SYMBOL(__initcall5_start, 9);
RANGE_SYMBOL(__initcall6_start, 13);
RANGE_SYMBOL(__initcall7_start, 15);
RANGE_SYMBOL(__initcall_end, 19);

static void set_entry(unsigned int index, initcall_t callback)
{
#ifdef CONFIG_HAVE_ARCH_PREL32_RELOCATIONS
	long offset = (long)callback - (long)&fixture_entries[index];
	if (offset != (int)offset)
		__builtin_trap();
	fixture_entries[index] = offset;
#else
	fixture_entries[index] = callback;
#endif
}
static int first(void) { record(3, 1, NULL); return -12; }
static int last(void) { record(3, 9, NULL); return 42; }
static int middle(void)
{
	record(3, 5, NULL);
	if (mutate_entry)
		set_entry(7, last);
	return 0;
}
int fixture_do_one(initcall_t function)
{
	record(2, 0, NULL);
	return function();
}
void fixture_trace(const char *name) { record(4, 0, name); }
#define do_trace_initcall_level fixture_trace
void fixture_ctors(void) { record(5, 0, NULL); }
#define do_ctors fixture_ctors
#ifdef CONFIG_CPUSETS
void cpuset_init_smp(void) { record(6, 0, NULL); }
#endif
void ksysfs_init(void) { record(7, 0, NULL); }
void driver_init(void) { record(8, 0, NULL); }
#ifdef CONFIG_PROC_FS
void init_irq_proc(void) { record(9, 0, NULL); }
#endif

void *__kmalloc_noprof(DECL_TOKEN_PARAMS(size, token), gfp_t flags)
{
	if (flags != (GFP_KERNEL | __GFP_ZERO))
		__builtin_trap();
	record(10, size, NULL);
	if (fail_allocation)
		return NULL;
	if (size > sizeof(allocated))
		__builtin_trap();
	memset(allocated, 0, sizeof(allocated));
	return allocated;
}
void kfree(const void *pointer)
{
	if (pointer != allocated)
		__builtin_trap();
	record(11, 0, pointer);
}
extern void _exit(int status) __noreturn;
extern int puts(const char *);
void panic(const char *format, ...)
{
	va_list arguments;
	if (strcmp(format, "%s: Failed to allocate %zu bytes\n") ||
	    event_count != 1 || events[0].kind != 10)
		__builtin_trap();
	va_start(arguments, format);
	if (strcmp(va_arg(arguments, const char *), "do_initcalls") ||
	    va_arg(arguments, size_t) != (size_t)(original_saved_command_line_len + 1))
		__builtin_trap();
	va_end(arguments);
	puts("INIT_MAIN_LEVELS_ALLOCATION_FAILURE_OK");
	/* exit flushes the test result; the production panic remains nonreturning. */
	extern void exit(int) __noreturn;
	exit(0);
}

static int set_parameter(const char *value, const struct kernel_param *parameter)
{
	record(12, parameter->level, value);
	return value && !strcmp(value, "bad") ? -EINVAL : 0;
}
static const struct kernel_param_ops parameter_ops = { .set = set_parameter };
#define PARAMETER(number) { .name = "level" #number, .ops = &parameter_ops, .level = number }
const struct kernel_param fixture_parameters[] = {
	{ .name = "early_only", .ops = &parameter_ops, .level = -1 },
	PARAMETER(0), PARAMETER(1), PARAMETER(2), PARAMETER(3),
	PARAMETER(4), PARAMETER(5), PARAMETER(6), PARAMETER(7),
};
static_assert(sizeof(fixture_parameters) == 360);
asm(".globl __start___param\n.set __start___param, fixture_parameters\n"
    ".globl __stop___param\n.set __stop___param, fixture_parameters + 360\n");

static bool param_check_unsafe(const struct kernel_param *parameter)
{
	if (parameter->flags)
		__builtin_trap();
	return true;
}
void kernel_param_lock(struct module *module) { record(13, 0, NULL); }
void kernel_param_unlock(struct module *module) { record(14, 0, NULL); }
#undef irqs_disabled
#define irqs_disabled() 1
#define parse_args original_parse_args
#include "params.inc"
#undef parse_args
char *parse_args(const char *name, char *line, const struct kernel_param *parameters,
		unsigned int count, s16 minimum, s16 maximum, void *argument,
		parse_unknown_fn unknown)
{
	if (parameters != fixture_parameters || count != ARRAY_SIZE(fixture_parameters) ||
	    minimum != maximum || minimum < 0 || minimum > 7 || argument || !unknown)
		__builtin_trap();
	record(1, minimum, line);
	return original_parse_args(name, line, parameters, count, minimum, maximum,
				   argument, unknown);
}
int _printk(const char *format, ...)
{
	record(15, 0, format);
	return 0;
}
#include "strings.inc"
#define saved_command_line original_saved_command_line
#define saved_command_line_len original_saved_command_line_len
#define do_one_initcall fixture_do_one
#include "original.inc"
#undef saved_command_line
#undef saved_command_line_len
#undef do_one_initcall

char *fixture_reset(const char *line, bool fail, bool mutate, unsigned int length)
{
	if (strlen(line) >= sizeof(saved))
		__builtin_trap();
	strcpy(saved, line);
	memset(events, 0, sizeof(events));
	event_count = 0;
	fail_allocation = fail;
	mutate_entry = mutate;
	original_saved_command_line = saved;
	original_saved_command_line_len = length;
	for (unsigned int i = 0; i < ARRAY_SIZE(fixture_entries); i++)
		set_entry(i, i % 3 == 0 ? first : i % 3 == 1 ? middle : last);
	return saved;
}
const struct event *fixture_events(void) { return events; }
unsigned int fixture_event_count(void) { return event_count; }
void fixture_original(unsigned int mode)
{
	if (mode == 0)
		do_initcalls();
	else if (mode == 1)
		do_basic_setup();
	else
		do_pre_smp_initcalls();
}
