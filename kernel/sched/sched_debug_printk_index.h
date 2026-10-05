/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_DEBUG_PRINTK_INDEX_H
#define LUPOS_SCHED_DEBUG_PRINTK_INDEX_H

/*
 * Native PRINTK_INDEX boundary for the Rust debug owners. The record type,
 * packing and section attributes come from the real printk/compiler headers;
 * no Rust replica, size or alignment is introduced here.
 *
 * This is the __printk_index_emit contract in include/linux/printk.h at pinned
 * 126a30fa, with explicit ORIGINAL owner provenance instead of this bridge's
 * __func__/__FILE__/__LINE__. Level is embedded in fmt, as for the original
 * pr_cont/pr_err/pr_info calls; level and subsys_fmt_prefix remain NULL.
 *
 * The original compilation's __FILE__ spelling is not knowable from a source
 * checkout alone (out-of-tree paths and compiler prefix maps can change it).
 * Admission must supply its exact string literal as LUPOS_DEBUG_ORIGINAL_FILE.
 * Do not substitute this bridge's path or a guessed canonical source path.
 */
#ifdef CONFIG_PRINTK_INDEX
#ifndef LUPOS_DEBUG_ORIGINAL_FILE
#error "Lupos debug index requires the original native debug.c __FILE__ spelling"
#endif
#define LUPOS_DEBUG_INDEX(_fmt, _func, _line)                          \
 do {                                                               \
  if (__builtin_constant_p(_fmt) && __builtin_constant_p(NULL)) {      \
   static const struct pi_entry _entry __used = {                    \
    .fmt = __builtin_constant_p(_fmt) ? (_fmt) : NULL,                \
    .func = (_func),                                                 \
    .file = LUPOS_DEBUG_ORIGINAL_FILE,                               \
    .line = (_line),                                                 \
    .level = __builtin_constant_p(NULL) ? NULL : NULL,                \
    .subsys_fmt_prefix = NULL,                                       \
   };                                                               \
   static const struct pi_entry *_entry_ptr                          \
   __used __section(".printk_index") = &_entry;                       \
  }                                                                 \
 } while (0)
#else
#define LUPOS_DEBUG_INDEX(...) do {} while (0)
#endif

/*
 * The source-line IDs are private provenance keys, not Linux ABI constants.
 * Every current out!/ns! expansion has an entry in the included manifest.
 * Index records are compile-time objects, not runtime events: the switch does
 * not make their retention depend on which calls or arms execute. As in the
 * native macro, __used and the native compiler determine emitted objects.
 */
static void lupos_debug_print_index(unsigned int source_line)
{
 switch (source_line) {
#define LUPOS_DEBUG_SITE(_line, _func, _fmt)                           \
 case _line:                                                        \
  LUPOS_DEBUG_INDEX(KERN_CONT _fmt, _func, _line);                     \
  break;
/*
 * These sites are inside a Rust schedstat_enabled decision. With SCHEDSTATS
 * enabled that decision remains in the owner, without another static-key
 * check in this metadata leaf. With it disabled, preserve the native if (0)
 * lexical scope rather than guessing how __used behaves in eliminated code.
 */
#ifdef CONFIG_SCHEDSTATS
#define LUPOS_DEBUG_STAT_SITE(_line, _func, _fmt) \
 LUPOS_DEBUG_SITE(_line, _func, _fmt)
#else
#define LUPOS_DEBUG_STAT_SITE(_line, _func, _fmt)                      \
 case _line:                                                        \
  if (schedstat_enabled()) {                                         \
   LUPOS_DEBUG_INDEX(KERN_CONT _fmt, _func, _line);                    \
  }                                                                 \
  break;
#endif
/* The original task-group-path macro has two separately scoped expansions. */
#define LUPOS_DEBUG_GROUP_SITE(_line, _func, _fmt)                     \
 case _line:                                                        \
  LUPOS_DEBUG_INDEX(KERN_CONT _fmt, _func, _line);                     \
  LUPOS_DEBUG_INDEX(KERN_CONT _fmt, _func, _line);                     \
  break;
/* Keep both native lexical scopes and the original native sizeof predicate. */
#define LUPOS_DEBUG_SIZE_SITE(_line, _field)                           \
 case _line:                                                        \
  if (sizeof(((struct rq *)0)->_field) == 4) {                        \
   LUPOS_DEBUG_INDEX(KERN_CONT "  .%-30s: %d\n", "print_cpu", _line);  \
  } else {                                                          \
   LUPOS_DEBUG_INDEX(KERN_CONT "  .%-30s: %Ld\n", "print_cpu", _line); \
  }                                                                 \
  break;
#include "sched_debug_printk_sites.h"
#undef LUPOS_DEBUG_SIZE_SITE
#undef LUPOS_DEBUG_GROUP_SITE
#undef LUPOS_DEBUG_STAT_SITE
#undef LUPOS_DEBUG_SITE
 }
}
#endif /* LUPOS_SCHED_DEBUG_PRINTK_INDEX_H */
