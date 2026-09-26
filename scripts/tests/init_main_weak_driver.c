/* SPDX-License-Identifier: GPL-2.0-only */
#include <linux/mm.h>
#include <linux/poison.h>

extern void exit(int) __attribute__((noreturn));
extern int puts(const char *);

char __init_begin[8], __init_end[8];
static unsigned int step;

#define HOOKS(X) \
	X(arch_post_acpi_subsys_init) X(smp_setup_processor_id) \
	X(smp_prepare_boot_cpu) X(thread_stack_cache_init) \
	X(poking_init) X(pgtable_cache_init) X(trap_init) X(free_initmem)
#define DECLARE(name) void name(void);
HOOKS(DECLARE)

#ifdef STRONG_OVERRIDES
#define DEFINE(name) void name(void) { step++; }
HOOKS(DEFINE)
#endif

__attribute__((force_align_arg_pointer))
unsigned long free_reserved_area(void *start, void *end, int poison, const char *name)
{
	if (start != __init_begin || end != __init_end || poison != POISON_FREE_INITMEM ||
	    strcmp(name, "unused kernel image (initmem)"))
		exit(90);
	step++;
	return 123; /* The original weak free_initmem ignores the page count. */
}

int main(void)
{
#define POINTER(name) name,
	void (*volatile callbacks[])(void) = {HOOKS(POINTER)};
	for (unsigned int i = 0; i < ARRAY_SIZE(callbacks); i++) {
		callbacks[i]();
#ifdef STRONG_OVERRIDES
		if (step != i + 1) exit(91);
#else
		if (step != (i == ARRAY_SIZE(callbacks) - 1)) exit(92);
#endif
	}
#ifdef STRONG_OVERRIDES
	puts("INIT_MAIN_WEAK_OK strong=8");
#else
	puts("INIT_MAIN_WEAK_OK default-free=1");
#endif
	return 0;
}
