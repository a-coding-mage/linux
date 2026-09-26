/* SPDX-License-Identifier: GPL-2.0-only */
#define envp_init original_envp_init
#include "canonical.h"

initcall_entry_t __initcall0_start[0], __initcall1_start[0], __initcall2_start[0];
initcall_entry_t __initcall3_start[0], __initcall4_start[0], __initcall5_start[0];
initcall_entry_t __initcall6_start[0], __initcall7_start[0], __initcall_end[0];

struct snapshot {
	u64 size, align, low, high;
	int nid;
	unsigned int allocated, formatted, logged, freed;
	u64 free_size, logged_size;
	char format[160], argument[4096], memory[4096];
};
static struct snapshot captured;
static bool allocation_failure;
static char memory[4096];

void capture_reset(bool fail)
{
	memset(&captured, 0, sizeof(captured));
	memset(memory, 0xa5, sizeof(memory));
	allocation_failure = fail;
}
const struct snapshot *capture_result(void) { return &captured; }

void *memblock_alloc_try_nid(phys_addr_t size, phys_addr_t align,
		phys_addr_t low, phys_addr_t high, int nid)
{
	if (size > sizeof(memory) || captured.allocated)
		__builtin_trap();
	captured.size = size;
	captured.align = align;
	captured.low = low;
	captured.high = high;
	captured.nid = nid;
	captured.allocated++;
	return allocation_failure ? NULL : memory;
}

int sprintf(char *buffer, const char *format, ...)
{
	va_list args;
	const char *value;
	size_t length;
	if (strcmp(format, " %s") || captured.freed || allocation_failure)
		__builtin_trap();
	va_start(args, format);
	value = va_arg(args, const char *);
	length = strlen(value);
	va_end(args);
	if (buffer < memory || buffer + length + 2 > memory + captured.size)
		__builtin_trap();
	buffer[0] = ' ';
	memcpy(buffer + 1, value, length + 1);
	captured.formatted++;
	return length + 1;
}

#ifdef CONFIG_PRINTK
int _printk(const char *format, ...)
{
	va_list args;
	const char *value;
	if (captured.freed || captured.logged || strlen(format) >= sizeof(captured.format))
		__builtin_trap();
	strcpy(captured.format, format);
	va_start(args, format);
	value = va_arg(args, const char *);
	if (strlen(value) >= sizeof(captured.argument))
		__builtin_trap();
	strcpy(captured.argument, value);
	if (allocation_failure)
		captured.logged_size = va_arg(args, size_t);
	else if (value != memory + 1)
		__builtin_trap();
	va_end(args);
	captured.logged++;
	return 0;
}
#endif

void memblock_free(void *buffer, size_t size)
{
	if (buffer != memory || size != captured.size || captured.freed || allocation_failure)
		__builtin_trap();
	captured.free_size = size;
	memcpy(captured.memory, memory, sizeof(memory));
	captured.freed++;
	memset(memory, 0xdd, sizeof(memory));
}

#include "original.inc"

void original_unknown_prepare(const char *const *args, const char *const *envs, bool pending)
{
	memcpy(argv_init, args, sizeof(argv_init));
	memcpy(envp_init, envs, sizeof(envp_init));
	panic_later = pending ? "pending" : NULL;
}
void original_unknown_notice(void) { print_unknown_bootoptions(); }
