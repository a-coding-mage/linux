/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_UTF8_CORE_BINDINGS_H
#define LUPOS_UTF8_CORE_BINDINGS_H

#include <linux/types.h>
#include <linux/errno.h>
#include <linux/parser.h>
#include <linux/slab.h>
#include "utf8n.h"

/* Only macro/header-inline boundaries; core control flow stays in Rust. */
struct unicode_map *rust_utf8_core_alloc(void);
const struct utf8data_table *rust_utf8_core_symbol_request(void);
void rust_utf8_core_symbol_put(void);
unsigned long rust_utf8_core_init_name_hash(const void *salt);
unsigned long rust_utf8_core_partial_name_hash(unsigned long c,
					      unsigned long hash);
unsigned int rust_utf8_core_end_name_hash(unsigned long hash);

#endif /* LUPOS_UTF8_CORE_BINDINGS_H */
