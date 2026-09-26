/* SPDX-License-Identifier: GPL-2.0-only */
/* Host transport for the unchanged C blacklist and the staged Rust owner.
 * Only memblock, symbol lookup, logging and corruption reporting are replaced.
 * The original string routines and list.h operations execute unchanged. */
#include <linux/init.h>
#include <linux/kallsyms.h>
#include <linux/list.h>
#include <linux/memblock.h>
#include <linux/stdarg.h>
#include <linux/string.h>

extern int printf(const char *, ...);
extern int vsnprintf(char *, size_t, const char *, va_list);
extern unsigned long strtoul(const char *, char **, int);
extern int posix_memalign(void **, size_t, size_t);
extern void exit(int) __attribute__((noreturn));

extern int original_blacklist(char *, initcall_t, int);
extern int blacklist_fixture(char *, initcall_t, int);
extern struct list_head *original_head(void), *rust_head(void);
extern char *original_entry_buffer(struct list_head *);

#define BOUNDARY(level) initcall_entry_t __initcall##level##_start[0]
BOUNDARY(0); BOUNDARY(1); BOUNDARY(2); BOUNDARY(3);
BOUNDARY(4); BOUNDARY(5); BOUNDARY(6); BOUNDARY(7);
initcall_entry_t __initcall_end[0];

static void *owners[64];
static size_t sizes[64];
static unsigned int allocations;
static struct list_head *active_head;
static struct list_head sentinel;
static bool report_result;
static const char *symbol_name;

#define CHECK(condition) do { if (!(condition)) { printf("FAIL %d\n", __LINE__); exit(90); } } while (0)

static unsigned int identity(const void *pointer)
{
	if (!pointer) return 0;
	if (pointer == active_head) return 100;
	if (pointer == &sentinel) return 101;
	for (unsigned int i = 1; i <= allocations; i++)
		if (pointer == owners[i]) return i;
	CHECK(false);
	return 0;
}

__attribute__((force_align_arg_pointer))
void *__memblock_alloc_or_panic(phys_addr_t size, phys_addr_t align, const char *function)
{
	void *memory = NULL;
	CHECK(++allocations < ARRAY_SIZE(owners));
	CHECK(align == SMP_CACHE_BYTES);
	CHECK(!strcmp(function, "initcall_blacklist"));
	CHECK(!posix_memalign(&memory, align, size));
	memset(memory, 0, size);
	owners[allocations] = memory;
	sizes[allocations] = size;
	printf("ALLOC %u %llu %llu %s\n", allocations,
	       (unsigned long long)size, (unsigned long long)align, function);
	return memory;
}

#ifdef CONFIG_PRINTK
__attribute__((force_align_arg_pointer))
int _printk(const char *format, ...)
{
	char buffer[2048];
	va_list arguments;
	va_start(arguments, format);
	int result = vsnprintf(buffer, sizeof(buffer), format, arguments);
	va_end(arguments);
	CHECK(result >= 0 && result < sizeof(buffer));
	printf("PRINT[%s]\n", buffer);
	return result;
}
#endif

static int callback(void) { return 0; }

#ifdef CONFIG_KALLSYMS
__attribute__((force_align_arg_pointer))
int sprint_symbol_no_offset(char *buffer, unsigned long address)
{
	CHECK(address == (unsigned long)callback);
	CHECK(strlen(symbol_name) < KSYM_SYMBOL_LEN);
	strcpy(buffer, symbol_name);
	printf("SYMBOL %d [%s]\n", !!address, symbol_name);
	return strlen(buffer);
}
#endif

#ifdef CONFIG_LIST_HARDENED
__attribute__((force_align_arg_pointer))
bool __list_valid_slowpath __list_add_valid_or_report(struct list_head *new,
						   struct list_head *prev,
							   struct list_head *next)
{
	printf("REPORT %u %u %u %d\n", identity(new), identity(prev), identity(next), report_result);
	return report_result;
}
#endif

static void snapshot(void)
{
	if (!active_head) {
		CHECK(!allocations);
		printf("NO_LIST\n");
		return;
	}
	printf("HEAD %u %u\n", identity(active_head->prev), identity(active_head->next));
	unsigned int count = 0;
	for (struct list_head *node = active_head->next; node != active_head; node = node->next) {
		CHECK(++count <= allocations / 2);
		char *buffer = original_entry_buffer(node);
		printf("NODE %u %u %u BUFFER %u [%s]\n", identity(node), identity(node->prev),
		       identity(node->next), identity(buffer), buffer);
	}
	for (unsigned int i = 2; i <= allocations; i += 2)
		printf("OWNED %u %zu [%s]\n", i, sizes[i], (char *)owners[i]);
}

int main(int argc, char **argv)
{
	CHECK(argc == 5);
	bool rust = argv[1][0] == 'r';
	unsigned int scenario = strtoul(argv[2], NULL, 10);
	bool corrupt = strtoul(argv[3], NULL, 10);
	report_result = strtoul(argv[4], NULL, 10);
	const char *inputs[] = {
		NULL, "", ",", ",,", "alpha", "alpha,beta", ",alpha,", "alpha,,beta",
		"alpha,alpha", "alpha [module]", "alpha beta", " alpha", "alpha ",
		"a-b,a_b", "upper,UPPER", "comma,,,tail,", "space\ttab,new\nline",
		"alpha,\"beta,gamma\"", "with=equals,\200,\377",
	};
	CHECK(scenario < ARRAY_SIZE(inputs));
	char input[512];
	char *value = NULL;
	size_t length = 0;
	if (inputs[scenario]) {
		strcpy(input, inputs[scenario]);
		length = strlen(input);
		value = input;
	}
	active_head = rust ? rust_head() : original_head();
	if (active_head && corrupt) active_head->prev = &sentinel;
	symbol_name = "alpha";
	int empty = rust ? blacklist_fixture(NULL, callback, 1) : original_blacklist(NULL, callback, 1);
	CHECK(!empty);
	printf("EMPTY %d\n", empty);
	int result = rust ? blacklist_fixture(value, callback, 0) : original_blacklist(value, callback, 0);
	printf("RESULT %d\n", result);
	printf("INPUT");
	if (value) for (size_t i = 0; i <= length; i++) printf(" %02x", (unsigned char)value[i]);
	printf("\n");
	/* The list owns copies after strsep has changed the caller's storage. */
	memset(input, '!', sizeof(input));
	snapshot();
	if (!corrupt) {
		char second[] = "later,alpha";
		result = rust ? blacklist_fixture(second, callback, 0) : original_blacklist(second, callback, 0);
		printf("SECOND %d\n", result);
		memset(second, '?', sizeof(second));
		snapshot();
	}
	const char *queries[] = {
		"", "alpha", "beta", "later", "alpha [module]", "beta [module]",
		"alpha [other module]", "alpha beta", " alpha", "alpha ", "a-b",
		"a_b", "upper", "UPPER", "comma", "tail", "missing", "space\ttab",
		"new\nline", " [module]", "\"beta", "gamma\"", "with=equals", "\200", "\377",
	};
	for (unsigned int i = 0; i < ARRAY_SIZE(queries); i++) {
		symbol_name = queries[i];
		initcall_t function = callback;
		result = rust ? blacklist_fixture(NULL, function, 1) : original_blacklist(NULL, function, 1);
		printf("MATCH %u %d\n", i, result);
	}
	printf("INIT_MAIN_BLACKLIST_OK\n");
	return 0;
}
