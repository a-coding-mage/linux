/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_IRQ_CORE_BINDINGS_H
#define LUPOS_IRQ_CORE_BINDINGS_H
#include <linux/irq.h>
#include <linux/slab.h>
#include <linux/export.h>
#include <linux/interrupt.h>
#include <linux/kernel_stat.h>
#include <linux/maple_tree.h>
#include <linux/irqdomain.h>
#include <linux/sysfs.h>
#include <linux/string_choices.h>
#include <linux/msi.h>
#include <linux/module.h>
#include <linux/preempt.h>
#include <linux/random.h>
#include <linux/sched.h>
#include <linux/kstrtox.h>
#include <asm/irq_regs.h>
#include <trace/events/irq.h>
#include "internals.h"
static const gfp_t LUPOS_IRQ_GFP_KERNEL = GFP_KERNEL;
static const gfp_t LUPOS_IRQ_GFP_NOWAIT = GFP_NOWAIT;
static const unsigned int LUPOS_IRQ_NR_IRQS = NR_IRQS;
static const unsigned int LUPOS_IRQ_MAX_SPARSE_IRQS = MAX_SPARSE_IRQS;
extern struct maple_tree lupos_irq_sparse_irqs;
extern struct mutex lupos_irq_sparse_mutex;
extern struct static_key_false lupos_irq_duration_key;
#ifndef CONFIG_SPARSE_IRQ
extern struct irq_desc irq_desc[NR_IRQS] __cacheline_aligned_in_smp;
struct lupos_irq_flat_desc_storage { struct irq_desc descs[NR_IRQS]; }
    __aligned(__alignof__(irq_desc));
static const size_t LUPOS_IRQ_FLAT_STORAGE_SIZE = sizeof(struct lupos_irq_flat_desc_storage);
static const size_t LUPOS_IRQ_FLAT_ARRAY_SIZE = sizeof(irq_desc);
static const size_t LUPOS_IRQ_FLAT_ALIGN = __alignof__(irq_desc);
#endif
static const size_t LUPOS_IRQ_MUTEX_SIZE = sizeof(struct mutex);
static const size_t LUPOS_IRQ_MUTEX_ALIGN = __alignof__(struct mutex);
static const size_t LUPOS_IRQ_RAW_LOCK_SIZE = sizeof(raw_spinlock_t);
static const size_t LUPOS_IRQ_RAW_LOCK_ALIGN = __alignof__(raw_spinlock_t);
static const size_t LUPOS_IRQ_SPIN_LOCK_SIZE = sizeof(spinlock_t);
static const size_t LUPOS_IRQ_SPIN_LOCK_ALIGN = __alignof__(spinlock_t);
static const size_t LUPOS_IRQ_MAPLE_SIZE = sizeof(struct maple_tree);
static const size_t LUPOS_IRQ_MAPLE_ALIGN = __alignof__(struct maple_tree);
static const size_t LUPOS_IRQ_MAPLE_LOCK_OFFSET = offsetof(struct maple_tree, ma_lock);
static const unsigned int LUPOS_IRQ_MAPLE_FLAGS = MT_FLAGS_ALLOC_RANGE | MT_FLAGS_LOCK_EXTERN | MT_FLAGS_USE_RCU;
static const size_t LUPOS_IRQ_KEY_SIZE = sizeof(struct static_key_false);
static const size_t LUPOS_IRQ_KEY_ALIGN = __alignof__(struct static_key_false);
static const unsigned int LUPOS_IRQ_SPIN_MAGIC = SPINLOCK_MAGIC;
static const unsigned long LUPOS_IRQ_SPIN_OWNER = (unsigned long)SPINLOCK_OWNER_INIT;
static const unsigned int LUPOS_IRQ_SPIN_OWNER_CPU = -1;
static const unsigned int LUPOS_IRQ_LD_WAIT_SPIN = LD_WAIT_SPIN;
static const unsigned int LUPOS_IRQ_LD_WAIT_CONFIG = LD_WAIT_CONFIG;
static const unsigned int LUPOS_IRQ_LD_WAIT_SLEEP = LD_WAIT_SLEEP;
#ifdef CONFIG_LOCKDEP
static const size_t LUPOS_IRQ_MAPLE_EXTERNAL_OFFSET = offsetof(struct maple_tree, ma_external_lock);
#endif
#ifndef CONFIG_PREEMPT_RT
static const size_t LUPOS_IRQ_SPIN_RAW_OFFSET = offsetof(spinlock_t, rlock);
#endif
#if !defined(CONFIG_SMP) && defined(CONFIG_DEBUG_SPINLOCK)
static const unsigned int LUPOS_IRQ_UP_UNLOCKED = __ARCH_SPIN_LOCK_UNLOCKED;
#endif
#ifdef CONFIG_JUMP_LABEL
static const unsigned long LUPOS_IRQ_JUMP_TYPE_FALSE = JUMP_TYPE_FALSE;
#endif
extern const struct kobj_type lupos_irq_kobj_type;
/* Only calling-convention/config-sensitive primitives live in the companion. */
#define IRQ_PRIMITIVE(ret, name, args, body) ret lupos_irq_##name args;
#include "irq_core_primitives.inc"
#undef IRQ_PRIMITIVE
#endif
