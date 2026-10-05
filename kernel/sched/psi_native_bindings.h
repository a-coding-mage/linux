/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_PSI_NATIVE_BINDINGS_H
#define LUPOS_PSI_NATIVE_BINDINGS_H
/* Generate only from the actual target configuration. No guessed Rust layouts. */
#include <linux/sched/clock.h>
#include <linux/sched/loadavg.h>
#include <linux/workqueue.h>
#include <linux/psi.h>
#include <linux/jiffies.h>
#include <linux/slab.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/uaccess.h>
#include "sched.h"

#ifndef CONFIG_PSI
#error "Lupos PSI native bindings require the native CONFIG_PSI types"
#endif

/* psi.c file-local constants, retained exactly from the source oracle. */
#define RUST_PSI_SOURCE_FREQ (2 * HZ + 1)
static const unsigned long RUST_PSI_FREQ = RUST_PSI_SOURCE_FREQ;
static const unsigned long RUST_PSI_EXP_10S = 1677;
static const unsigned long RUST_PSI_EXP_60S = 1981;
static const unsigned long RUST_PSI_EXP_300S = 2034;
static const unsigned int RUST_PSI_WINDOW_MAX_US = 10000000;
static const unsigned int RUST_PSI_UPDATES_PER_WINDOW = 10;

#define PSI_CONSTANT(name, value) static const unsigned int RUST_PSI_##name = value;
PSI_CONSTANT(NR_CPUS, NR_CPUS)
PSI_CONSTANT(NR_STATES, NR_PSI_STATES)
PSI_CONSTANT(NR_TASK_COUNTS, NR_PSI_TASK_COUNTS)
PSI_CONSTANT(NR_IOWAIT, NR_IOWAIT)
PSI_CONSTANT(NR_MEMSTALL, NR_MEMSTALL)
PSI_CONSTANT(NR_RUNNING, NR_RUNNING)
PSI_CONSTANT(NR_MEMSTALL_RUNNING, NR_MEMSTALL_RUNNING)
PSI_CONSTANT(AVGS, PSI_AVGS)
PSI_CONSTANT(POLL, PSI_POLL)
PSI_CONSTANT(NONIDLE, PSI_NONIDLE)
PSI_CONSTANT(IO, PSI_IO)
PSI_CONSTANT(MEM, PSI_MEM)
PSI_CONSTANT(CPU, PSI_CPU)
PSI_CONSTANT(IO_SOME, PSI_IO_SOME)
PSI_CONSTANT(IO_FULL, PSI_IO_FULL)
PSI_CONSTANT(MEM_SOME, PSI_MEM_SOME)
PSI_CONSTANT(MEM_FULL, PSI_MEM_FULL)
PSI_CONSTANT(CPU_SOME, PSI_CPU_SOME)
PSI_CONSTANT(CPU_FULL, PSI_CPU_FULL)
PSI_CONSTANT(ONCPU, PSI_ONCPU)
PSI_CONSTANT(TSK_IOWAIT, TSK_IOWAIT)
PSI_CONSTANT(TSK_MEMSTALL, TSK_MEMSTALL)
PSI_CONSTANT(TSK_RUNNING, TSK_RUNNING)
PSI_CONSTANT(TSK_MEMSTALL_RUNNING, TSK_MEMSTALL_RUNNING)
PSI_CONSTANT(TSK_ONCPU, TSK_ONCPU)
PSI_CONSTANT(STATE_RESCHEDULE, PSI_STATE_RESCHEDULE)
PSI_CONSTANT(PF_WQ_WORKER, PF_WQ_WORKER)
PSI_CONSTANT(EOPNOTSUPP, EOPNOTSUPP)
PSI_CONSTANT(EINVAL, EINVAL)
PSI_CONSTANT(ENOMEM, ENOMEM)
PSI_CONSTANT(EFAULT, EFAULT)
PSI_CONSTANT(EBUSY, EBUSY)
#ifdef CONFIG_IRQ_TIME_ACCOUNTING
PSI_CONSTANT(IRQ, PSI_IRQ)
PSI_CONSTANT(IRQ_FULL, PSI_IRQ_FULL)
#endif
#undef PSI_CONSTANT
static const u32 RUST_PSI_U32_MAX = U32_MAX;
static const u64 RUST_PSI_U64_MAX = ULLONG_MAX;
static const unsigned long RUST_PSI_FIXED_1 = FIXED_1;
static const long RUST_PSI_NSEC_PER_USEC = NSEC_PER_USEC;
static const __poll_t RUST_PSI_DEFAULT_POLLMASK = DEFAULT_POLLMASK;
static const __poll_t RUST_PSI_EPOLLERR = EPOLLERR;
static const __poll_t RUST_PSI_EPOLLPRI = EPOLLPRI;

extern int rust_psi_bug;
extern u64 rust_psi_period;
extern bool rust_psi_enable;

int rust_psi_setup(char *value);
void rust_psi_avgs_work(struct work_struct *work);
void rust_psi_poll_timer(struct timer_list *timer);
int rust_psi_rtpoll_worker(void *data);

#define PSI_LEAF(ret, name, args, body) ret rust_psi_##name args;
#include "psi_native_primitives.inc"
#undef PSI_LEAF

#ifdef CONFIG_PROC_FS
#define PSI_PROC_CALLBACKS(resource) \
	int rust_psi_##resource##_show(struct seq_file *m, void *data); \
	int rust_psi_##resource##_open(struct inode *inode, struct file *file); \
	ssize_t rust_psi_##resource##_write(struct file *file, \
		const char __user *buf, size_t size, loff_t *pos)
PSI_PROC_CALLBACKS(io);
PSI_PROC_CALLBACKS(memory);
PSI_PROC_CALLBACKS(cpu);
#ifdef CONFIG_IRQ_TIME_ACCOUNTING
PSI_PROC_CALLBACKS(irq);
#endif
#undef PSI_PROC_CALLBACKS
__poll_t rust_psi_fop_poll(struct file *file, poll_table *wait);
int rust_psi_fop_release(struct inode *inode, struct file *file);
int rust_psi_proc_init(void);
void rust_psi_proc_mkdir(void);
void rust_psi_proc_create_io(void);
void rust_psi_proc_create_memory(void);
void rust_psi_proc_create_cpu(void);
#ifdef CONFIG_IRQ_TIME_ACCOUNTING
void rust_psi_proc_create_irq(void);
#endif
#endif
#endif
