// SPDX-License-Identifier: GPL-2.0-only
/* Inactive native metadata for the existing Rust notifier owner. */
#include "notifier_native_bindings.h"
#include "notifier_rust_guard.h"
#include <linux/export.h>
#include <linux/kprobes.h>

/* Exactly one trace-definition owner would replace the original notifier.o. */
#define CREATE_TRACE_POINTS
#include <trace/events/notifier.h>

/* These entries reference actual Rust body symbols, not C forwarding wrappers.
 * The native macro expresses the original blacklist requirement. It does not
 * prove final Rust symbol ranges, clones, outlining or linker retention.
 */
NOKPROBE_SYMBOL(lupos_notifier_call_chain);
NOKPROBE_SYMBOL(atomic_notifier_call_chain);
NOKPROBE_SYMBOL(notify_die);

/* notrace is present only on notify_die in the original C owner. A C
 * declaration cannot apply that instrumentation policy to a Rust definition.
 * Do not add a wrapper, broad compiler exemption, no_sanitize or guessed Rust
 * attribute to pretend it does. notifier_rust_guard.h and the Rust source error
 * remain mandatory until an actual supported per-body contract is qualified.
 */

EXPORT_SYMBOL_GPL(atomic_notifier_chain_register);
EXPORT_SYMBOL_GPL(atomic_notifier_chain_register_unique_prio);
EXPORT_SYMBOL_GPL(atomic_notifier_chain_unregister);
EXPORT_SYMBOL_GPL(atomic_notifier_call_chain);
EXPORT_SYMBOL_GPL(blocking_notifier_chain_register);
EXPORT_SYMBOL_GPL(blocking_notifier_chain_register_unique_prio);
EXPORT_SYMBOL_GPL(blocking_notifier_chain_unregister);
EXPORT_SYMBOL_GPL(blocking_notifier_call_chain_robust);
EXPORT_SYMBOL_GPL(blocking_notifier_call_chain);
EXPORT_SYMBOL_GPL(raw_notifier_chain_register);
EXPORT_SYMBOL_GPL(raw_notifier_chain_unregister);
EXPORT_SYMBOL_GPL(raw_notifier_call_chain_robust);
EXPORT_SYMBOL_GPL(raw_notifier_call_chain);
EXPORT_SYMBOL_GPL(srcu_notifier_chain_register);
EXPORT_SYMBOL_GPL(srcu_notifier_chain_unregister);
EXPORT_SYMBOL_GPL(srcu_notifier_call_chain);
EXPORT_SYMBOL_GPL(srcu_init_notifier_head);
EXPORT_SYMBOL_GPL(register_die_notifier);
EXPORT_SYMBOL_GPL(unregister_die_notifier);
/* atomic_notifier_call_chain_is_empty and notify_die are not exported in C. */
