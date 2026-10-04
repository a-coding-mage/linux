/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_PANIC_SEQ_BUF_TEST_ROUTE_H
#define LUPOS_PANIC_SEQ_BUF_TEST_ROUTE_H

/* Test-only forced include for the unchanged lib/tests/seq_buf_kunit.c.
 * Parse the original inlines first, then route the test's own calls to the
 * actual Rust providers. This header must not be used for the explicit C
 * control selected by LUPOS_SEQ_BUF_KUNIT_C_CONTROL=y in the test build.
 */
#include <linux/kconfig.h>
#include <linux/module.h>
#include <linux/seq_buf.h>

#if !IS_BUILTIN(CONFIG_RUST_PANIC) || !IS_ENABLED(CONFIG_SEQ_BUF_KUNIT_TEST) || !IS_ENABLED(CONFIG_KUNIT)
#error "Rust seq_buf test routing requires RUST_PANIC=y and SEQ_BUF_KUNIT_TEST=y/m with KUnit"
#endif

void lupos_panic_seq_buf_clear(struct seq_buf *s);
void lupos_panic_seq_buf_init(struct seq_buf *s, char *buf, unsigned int size);
const char *lupos_panic_seq_buf_str(struct seq_buf *s);

/* Works for both suite forms; the companion emits only native export metadata
 * for these same Rust definitions in the official KUnit namespace.
 */
MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");

#define seq_buf_clear lupos_panic_seq_buf_clear
#define seq_buf_init lupos_panic_seq_buf_init
#define seq_buf_str lupos_panic_seq_buf_str

#endif
