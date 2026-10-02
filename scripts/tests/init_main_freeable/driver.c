/* SPDX-License-Identifier: GPL-2.0-only */
#include "canonical.h"
#include <linux/stdarg.h>
/* The included Rust-only helper definitions have no public C header. */
refcount_t rust_helper_REFCOUNT_INIT(int n);
void rust_helper_refcount_set(refcount_t *r, int n);
void rust_helper_refcount_inc(refcount_t *r);
void rust_helper_refcount_dec(refcount_t *r);
bool rust_helper_refcount_dec_and_test(refcount_t *r);
#include "refcount-helper.inc"

extern int printf(const char *, ...);
extern int vsnprintf(char *, size_t, const char *, va_list);
extern void exit(int) __attribute__((noreturn));
extern char *rust_freeable(char *, bool);

struct task_struct *rust_helper_get_current(void);
void rust_helper_spin_lock(spinlock_t *lock);
void rust_helper_spin_unlock(spinlock_t *lock);
void fixture_pre_smp(void);
void fixture_basic_setup(void);
void fixture_console(void);

#define CHECK(condition) do { if (!(condition)) { printf("FAIL %d\n", __LINE__); exit(90); } } while (0)
static struct task_struct task;
static struct pid pid;
static unsigned long irq_state;
static unsigned int locked, sequence_active;
static int access_result;
static char *expected_command;
static char log[4096];
static unsigned int used;
gfp_t gfp_allowed_mask;
struct pid *cad_pid;
nodemask_t node_states[NR_NODE_STATES];
#ifdef CONFIG_SMP
unsigned int setup_max_cpus;
#endif
#define BOUNDARY(level) initcall_entry_t __initcall##level##_start[0]
BOUNDARY(0); BOUNDARY(1); BOUNDARY(2); BOUNDARY(3);
BOUNDARY(4); BOUNDARY(5); BOUNDARY(6); BOUNDARY(7);
initcall_entry_t __initcall_end[0];

static void event(char code)
{
	CHECK(gfp_allowed_mask == __GFP_BITS_MASK);
	CHECK(used + 1 < sizeof(log));
	log[used++] = code;
	log[used] = 0;
}

struct task_struct *rust_helper_get_current(void) { return &task; }
void rust_helper_spin_lock(spinlock_t *lock)
{
	CHECK(lock == &task.alloc_lock && !locked && !sequence_active);
	locked = 1;
	event('L');
}
void rust_helper_spin_unlock(spinlock_t *lock)
{
	CHECK(lock == &task.alloc_lock && locked && !sequence_active);
	locked = 0;
	event('U');
}
#ifdef CONFIG_CPUSETS
unsigned long rust_helper_local_irq_save(void)
{
	CHECK(locked && !sequence_active);
	unsigned long old = irq_state;
	irq_state = 0;
	event('I');
	return old;
}
void rust_helper_local_irq_restore(unsigned long flags)
{
	CHECK(locked && !sequence_active && !irq_state);
	irq_state = flags;
	event('R');
}
void rust_helper_write_seqcount_spinlock_begin(seqcount_spinlock_t *sequence)
{
	CHECK(sequence == &task.mems_allowed_seq && locked && !irq_state && !sequence_active);
	CHECK(task.mems_allowed.bits[0] == 0x55UL);
	sequence_active = 1;
	sequence->seqcount.sequence++;
	event('B');
}
void rust_helper_write_seqcount_spinlock_end(seqcount_spinlock_t *sequence)
{
	CHECK(sequence == &task.mems_allowed_seq && locked && !irq_state && sequence_active);
	CHECK(!memcmp(&task.mems_allowed, &node_states[N_MEMORY], sizeof(nodemask_t)));
	sequence_active = 0;
	sequence->seqcount.sequence++;
	event('E');
}
#endif

void refcount_warn_saturate(refcount_t *count, enum refcount_saturation_type type)
{
	CHECK(count == &pid.count);
	refcount_set(count, REFCOUNT_SATURATED);
	event('0' + type);
}

#ifdef CONFIG_SMP
static void prepare_cpus(unsigned int max)
{
	CHECK(max == setup_max_cpus);
	event('P');
}
struct smp_ops smp_ops = { .smp_prepare_cpus = prepare_cpus };
void smp_init(void) { event('S'); }
#elif defined(CONFIG_UP_LATE_INIT)
void up_late_init(void) { event('s'); }
#endif
void workqueue_init(void) { event('W'); }
void init_mm_internals(void) { event('M'); }
void fixture_pre_smp(void) { event('p'); }
#ifdef CONFIG_LOCKUP_DETECTOR
void lockup_detector_init(void) { event('D'); }
#endif
void sched_init_smp(void) { event('H'); }
void workqueue_init_topology(void) { event('T'); }
void async_init(void) { event('A'); }
#ifdef CONFIG_PADATA
void padata_init(void) { event('a'); }
#endif
void page_alloc_init_late(void) { event('G'); }
void fixture_basic_setup(void) { event('b'); }
#if IS_BUILTIN(CONFIG_KUNIT)
int kunit_run_all_tests(void) { event('K'); return -123; }
#endif
#ifdef CONFIG_BLK_DEV_INITRD
void wait_for_initramfs(void) { event('F'); }
#endif
void fixture_console(void) { event('C'); }
int init_eaccess(const char *command)
{
	CHECK(command == expected_command);
	event('X');
	return access_result;
}
void prepare_namespace(void) { event('N'); }
#ifdef CONFIG_INTEGRITY
void integrity_load_keys(void) { event('Y'); }
#endif
#ifdef CONFIG_PRINTK
__attribute__((force_align_arg_pointer))
int _printk(const char *format, ...)
{
	va_list arguments;
	va_start(arguments, format);
	int size = vsnprintf(log + used, sizeof(log) - used, format, arguments);
	va_end(arguments);
	CHECK(size >= 0 && size < sizeof(log) - used);
	used += size;
	return size;
}
#endif

/* Keep the original state assignment; observe its required primitive calls. */
#undef current
#define current (&task)
#ifdef CONFIG_CPUSETS
#define set_mems_allowed fixture_set_mems_allowed
#define task_lock(p) rust_helper_spin_lock(&(p)->alloc_lock)
#define task_unlock(p) rust_helper_spin_unlock(&(p)->alloc_lock)
#undef local_irq_save
#undef local_irq_restore
#define local_irq_save(flags) ((flags) = rust_helper_local_irq_save())
#define local_irq_restore(flags) rust_helper_local_irq_restore(flags)
#undef write_seqcount_begin
#undef write_seqcount_end
#define write_seqcount_begin(sequence) rust_helper_write_seqcount_spinlock_begin(sequence)
#define write_seqcount_end(sequence) rust_helper_write_seqcount_spinlock_end(sequence)
#include "set-mems.inc"
#endif
#ifndef CONFIG_SMP
static inline void smp_prepare_cpus(unsigned int max) {}
#endif
#define do_pre_smp_initcalls fixture_pre_smp
#define do_basic_setup fixture_basic_setup
#define console_on_rootfs fixture_console
static char *ramdisk_execute_command;
static bool ramdisk_execute_command_set;
#include "original.inc"

static void reset(bool has_pid, unsigned int count, unsigned long flags)
{
	memset(&task, 0, sizeof(task));
	memset(&pid, 0, sizeof(pid));
	refcount_set(&pid.count, count);
	task.thread_pid = has_pid ? &pid : NULL;
#ifdef CONFIG_CPUSETS
	task.mems_allowed.bits[0] = 0x55UL;
	task.mems_allowed_seq.seqcount.sequence = ~1U;
#endif
	memset(&node_states[N_MEMORY], 0xa6, sizeof(nodemask_t));
	gfp_allowed_mask = 0;
	cad_pid = (struct pid *)0x10UL;
	irq_state = flags;
	locked = sequence_active = 0;
	used = 0; log[0] = 0;
#ifdef CONFIG_SMP
	setup_max_cpus = count;
#endif
}

static void compare(char *command, bool explicit, bool has_pid, unsigned int count,
		    unsigned long flags)
{
	/* Serial fixture scratch; keep the kernel frame-size diagnostic enabled. */
	static char expected[sizeof(log)];
	expected_command = command;
	reset(has_pid, count, flags);
	ramdisk_execute_command = command;
	ramdisk_execute_command_set = explicit;
	kernel_init_freeable();
	memcpy(expected, log, sizeof(expected));
	unsigned int expected_count = refcount_read(&pid.count);
	CHECK(cad_pid == (has_pid ? &pid : NULL));
	CHECK(irq_state == flags && !locked && !sequence_active);
	char *expected_result = ramdisk_execute_command;
	reset(has_pid, count, flags);
	char *(*volatile call)(char *, bool) = rust_freeable;
	CHECK(call(command, explicit) == expected_result);
	CHECK(!strcmp(log, expected));
	CHECK(refcount_read(&pid.count) == expected_count);
	CHECK(cad_pid == (has_pid ? &pid : NULL));
	CHECK(irq_state == flags && !locked && !sequence_active);
#ifdef CONFIG_CPUSETS
	CHECK(task.mems_allowed_seq.seqcount.sequence == 0);
	CHECK(!memcmp(&task.mems_allowed, &node_states[N_MEMORY], sizeof(nodemask_t)));
#endif
}

int main(void)
{
	char *commands[] = { NULL, "", "/init", "/custom init", "/init%p%s" };
	int results[] = { 0, -2, -13, 1, -2147483647 - 1 };
	unsigned int counts[] = { 0, 1, 17, REFCOUNT_MAX, REFCOUNT_SATURATED, ~0U };
	unsigned int cases = 0;
	for (unsigned int c = 0; c < ARRAY_SIZE(commands); c++)
		for (unsigned int r = 0; r < ARRAY_SIZE(results); r++)
			for (unsigned int explicit = 0; explicit < 2; explicit++)
				for (unsigned int has_pid = 0; has_pid < 2; has_pid++)
					for (unsigned int n = 0; n < ARRAY_SIZE(counts); n++)
						for (unsigned int irq = 0; irq < 2; irq++) {
							access_result = results[r];
							compare(commands[c], explicit, has_pid, counts[n], irq ? 0x202UL : 0);
							cases++;
						}
	printf("INIT_MAIN_FREEABLE_OK cases=%u\n", cases);
	return 0;
}
