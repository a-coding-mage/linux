/* SPDX-License-Identifier: GPL-2.0 */
/* Compile the complete retained implementation with only its symbol renamed. */
#define __DISABLE_EXPORTS
#define errname errname_original
#include <errname.c>
