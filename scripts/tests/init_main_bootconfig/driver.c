/* SPDX-License-Identifier: GPL-2.0-only */
/* The unchanged XBC parser and opt-in parser execute for both callers. Only
 * memblock allocation and output formatting are instrumented host boundaries.
 * This does not model physical reservations or the complete early boot phase. */
#include "canonical.h"
#include <linux/stdarg.h>

extern int printf(const char *, ...);
extern int vsnprintf(char *, size_t, const char *, va_list);
extern unsigned long strtoul(const char *, char **, int);
extern int posix_memalign(void **, size_t, size_t);
extern void free(void *);
extern void exit(int) __attribute__((noreturn));

extern void original_setup(const char *), rust_setup(const char *);
extern char *original_extra(void), *original_args(void), *rust_extra(void), *rust_args(void);
extern size_t original_offset(void), rust_offset(void);
extern bool original_found(void), rust_found(void);
extern void *original_trailer(size_t *), *rust_trailer(size_t *);
extern void original_exit(void), rust_exit(void);
extern int original_warn(void), rust_warn(void);

unsigned long initrd_start, initrd_end;
#define BOUNDARY(level) initcall_entry_t __initcall##level##_start[0]
BOUNDARY(0); BOUNDARY(1); BOUNDARY(2); BOUNDARY(3);
BOUNDARY(4); BOUNDARY(5); BOUNDARY(6); BOUNDARY(7);
initcall_entry_t __initcall_end[0];
static unsigned int allocations, fail_at;
static void *owners[32];

#define CHECK(condition) do { if (!(condition)) { printf("FAIL %d\n", __LINE__); exit(90); } } while (0)

__attribute__((force_align_arg_pointer))
void *memblock_alloc_try_nid(phys_addr_t size, phys_addr_t align,
			   phys_addr_t low, phys_addr_t high, int node)
{
	void *memory = NULL;
	allocations++;
	CHECK(allocations < ARRAY_SIZE(owners));
	printf("ALLOC %u %llu %llu %llu %llu %d\n", allocations,
	       (unsigned long long)size, (unsigned long long)align,
	       (unsigned long long)low, (unsigned long long)high, node);
	CHECK(align == SMP_CACHE_BYTES && low == MEMBLOCK_LOW_LIMIT &&
	      high == MEMBLOCK_ALLOC_ACCESSIBLE && node == NUMA_NO_NODE);
	if (allocations == fail_at)
		return NULL;
	CHECK(!posix_memalign(&memory, align, size));
	memset(memory, 0, size);
	owners[allocations] = memory;
	return memory;
}

__attribute__((force_align_arg_pointer))
void memblock_free(void *pointer, size_t size)
{
	unsigned int owner = 0;
	if (pointer) {
		for (owner = 1; owner <= allocations; owner++)
			if (owners[owner] == pointer)
				break;
		CHECK(owner <= allocations);
		owners[owner] = NULL;
	}
	printf("FREE %u %zu\n", owner, size);
	free(pointer);
}

__attribute__((force_align_arg_pointer))
int _printk(const char *format, ...)
{
	char output[2048];
	va_list arguments;
	va_start(arguments, format);
	int result = vsnprintf(output, sizeof(output), format, arguments);
	va_end(arguments);
	CHECK(result >= 0 && result < sizeof(output));
	printf("PRINT[%s]\n", output);
	return result;
}

/* The C parser uses only its documented string formats here. Its real kernel
 * caller stack is aligned before entering host libc's variadic formatter. */
__attribute__((force_align_arg_pointer))
int fixture_snprintf(char *buffer, size_t size, const char *format, ...)
{
	va_list arguments;
	va_start(arguments, format);
	int result = vsnprintf(buffer, size, format, arguments);
	va_end(arguments);
	return result;
}

static void le32(unsigned char *bytes, unsigned int value)
{
	for (unsigned int i = 0; i < 4; i++)
		bytes[i] = value >> (8 * i);
}

int main(int argc, char **argv)
{
	CHECK(argc == 4);
	bool rust = argv[1][0] == 'r';
	unsigned int scenario = strtoul(argv[2], NULL, 10);
	fail_at = strtoul(argv[3], NULL, 10);
	unsigned char initrd[66048] __attribute__((aligned(16)));
	char command[COMMAND_LINE_SIZE] = "bootconfig root=old -- user=old";
	const char *text = "kernel { console = ttyS0; flag; quote = 'two words'; arr = one,two; }\ninit { name = value; bare; }\n";
	if (scenario == 1) command[0] = 0;
	if (scenario == 2) strcpy(command, "root=old -- bootconfig");
	if (scenario == 3) strcpy(command, "bootconfig=1 root=old");
	if (scenario == 4) strcpy(command, "\"bootconfig\" -- user");
	if (scenario == 5) text = "kernel = value\ninit {}\n";
	if (scenario == 6) text = "kernel { broken = \"unterminated\n";
	if (scenario == 7) text = "kernel.flag\n";
	if (scenario == 8) text = "init.flag\n";
	if (scenario == 9) text = "";
	memset(initrd, 0xa5, sizeof(initrd));
	unsigned char *start = initrd + 64;
	size_t size = strlen(text);
	if (scenario == 10 || scenario == 11) {
		size = XBC_DATA_MAX + (scenario == 11);
		memset(start, ' ', size);
	} else {
		memcpy(start, text, size);
	}
	unsigned int checksum = xbc_calc_checksum(start, size);
	le32(start + size, scenario == 12 ? 131072 : size);
	le32(start + size + 4, checksum + (scenario == 13));
	memcpy(start + size + 8, BOOTCONFIG_MAGIC, BOOTCONFIG_MAGIC_LEN);
	if (scenario == 14) start[size + 8] ^= 1;
	unsigned int padding = scenario >= 15 && scenario <= 19 ? scenario - 15 : 0;
	initrd_start = (unsigned long)start;
	initrd_end = (unsigned long)(start + size + 8 + BOOTCONFIG_MAGIC_LEN + padding);
	if (scenario == 20 || scenario == 21 || scenario == 22) initrd_end = 0;
#ifdef CONFIG_CMDLINE_FROM_BOOTCONFIG
	if (scenario == 21) xbc_prepend_embedded_cmdline(command, sizeof(command));
#endif
	if (scenario == 22) command[0] = 0;
	if (scenario >= 23) {
		size_t found_size = 99173;
		void *result = rust ? rust_trailer(scenario == 24 ? NULL : &found_size) :
			original_trailer(scenario == 24 ? NULL : &found_size);
		printf("TRAILER %ld %zu\n", result ? (long)((unsigned char *)result - start) : -1, found_size);
	} else {
		if (rust) rust_setup(command); else original_setup(command);
		char *extra = rust ? rust_extra() : original_extra();
		char *args = rust ? rust_args() : original_args();
		printf("STATE found=%d offset=%zu extra=[%s] args=[%s] command=[%s]\n",
		       rust ? rust_found() : original_found(), rust ? rust_offset() : original_offset(),
		       extra ? extra : "<null>", args ? args : "<null>", command);
		printf("WARN %d\n", rust ? rust_warn() : original_warn());
		if (rust) rust_exit(); else original_exit();
	}
	printf("INITRD %ld\n", initrd_end ? (long)(initrd_end - initrd_start) : -1);
	return 0;
}
