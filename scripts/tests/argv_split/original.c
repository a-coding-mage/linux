/* SPDX-License-Identifier: GPL-2.0 */
/* Keep the unchanged algorithm and native allocation APIs as the oracle. */
#define __DISABLE_EXPORTS
#define argv_split argv_split_original
#define argv_free argv_free_original
#include <argv_split.c>
