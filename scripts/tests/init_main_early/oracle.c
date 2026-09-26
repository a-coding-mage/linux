/* SPDX-License-Identifier: GPL-2.0-only */
#define parse_early_options original_early_options
#define parse_early_param original_early_param
#define boot_command_line original_boot_command_line
#include "canonical.h"

char boot_command_line[COMMAND_LINE_SIZE];
extern unsigned int parser_calls;
initcall_entry_t __initcall0_start[0], __initcall1_start[0], __initcall2_start[0];
initcall_entry_t __initcall3_start[0], __initcall4_start[0], __initcall5_start[0];
initcall_entry_t __initcall6_start[0], __initcall7_start[0], __initcall_end[0];

struct event {
	int kind;
	int null;
	char value[256];
};
static struct event events[1024];
static unsigned int event_count;

static void record(int kind, const char *value)
{
	struct event *event;
	if (event_count == ARRAY_SIZE(events))
		__builtin_trap();
	event = &events[event_count++];
	event->kind = kind;
	event->null = !value;
	memset(event->value, 0, sizeof(event->value));
	if (value)
		sized_strscpy(event->value, value, sizeof(event->value));
}
static int good(char *value) { record(1, value); return 0; }
static int bad(char *value) { record(2, value); return -22; }
static int later(char *value) { record(3, value); return 1; }
struct obs_kernel_param fixture_setup[] = {
	{ "early", good, 1 }, { "early", bad, 1 }, { "early", later, 0 },
	{ "bad", bad, 1 }, { "dash_name", good, 1 },
	{ "other", later, 0 }, { "", good, 1 }, { "obsolete", NULL, 0 },
};
asm(".globl __setup_start\n.set __setup_start, fixture_setup\n"
    ".globl __setup_end\n.set __setup_end, fixture_setup + 192\n");
static_assert(sizeof(fixture_setup) == 192);

int _printk(const char *format, ...)
{
	va_list args;
	if (strcmp(format, "\0014Malformed early option '%s'\n"))
		__builtin_trap();
	va_start(args, format);
	record(9, va_arg(args, const char *));
	va_end(args);
	return 0;
}

#include "original.inc"

void original_early_reset(void)
{
	event_count = 0;
	parser_calls = 0;
}
unsigned int original_event_count(void) { return event_count; }
unsigned int original_parser_calls(void) { return parser_calls; }
const struct event *original_events(void) { return events; }
int original_early_callback(char *parameter, char *value)
{
	return do_early_param(parameter, value, NULL, NULL);
}
