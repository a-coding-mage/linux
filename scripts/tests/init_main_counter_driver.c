// SPDX-License-Identifier: GPL-2.0-only
/* The host's GS base is verified as zero before exercising the genuine native
 * per-CPU read/cmpxchg instructions against the canonical symbol. No IRQ-enable
 * instruction is executed by this userspace counter comparison. */
#include <linux/preempt.h>
#include <linux/percpu.h>
#include <asm/prctl.h>
#include <asm/unistd.h>

extern long syscall(long, ...);
extern int puts(const char *);
extern void exit(int) __attribute__((noreturn));
int c_count(void);
void c_set_count(int);
int rust_count(void);
void rust_set_count(int);

DEFINE_PER_CPU_CACHE_HOT(unsigned long, __preempt_count);

#define CHECK(condition) do { if (!(condition)) { puts(#condition); exit(__LINE__); } } while (0)

int main(void)
{
	unsigned long gs;
	const unsigned long before[] = { 0, 1, ~0UL, 0x1234567887654321UL,
		PREEMPT_NEED_RESCHED, PREEMPT_NEED_RESCHED | 1, 0xffffffffUL, 0xffffffff00000000UL };
	unsigned int cases = 0;

	CHECK(syscall(__NR_arch_prctl, ARCH_GET_GS, &gs) == 0 && gs == 0);
	for (int i = 0; i < ARRAY_SIZE(before); ++i) {
		for (int sample = -129; sample < 129; ++sample) {
			int value = sample * 0x1070707;
			__preempt_count = before[i];
			int expected_count = c_count();
			CHECK(rust_count() == expected_count);
			c_set_count(value);
			unsigned long expected = __preempt_count;
			__preempt_count = before[i];
			rust_set_count(value);
			CHECK(__preempt_count == expected);
			CHECK((__preempt_count & PREEMPT_NEED_RESCHED) == (before[i] & PREEMPT_NEED_RESCHED));
			cases++;
		}
	}
	CHECK(cases == 2064);
	puts("INIT_MAIN_COUNTER_OK cases=2064");
	return 0;
}
