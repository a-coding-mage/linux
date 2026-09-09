/*
 * Copyright (C) 2004 Microtronix Datacom Ltd
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License. See the file COPYING in the main directory of this
 * archive for more details.
 */

// Dependency intent from <linux/export.h>, <linux/string.h>,
// <linux/pgtable.h>, and <asm/cacheflush.h> is supplied by other files.

/* string functions */

// EXPORT_SYMBOL(memcpy);
// EXPORT_SYMBOL(memset);
// EXPORT_SYMBOL(memmove);
extern "C" {
    pub fn memcpy();
    pub fn memset();
    pub fn memmove();
}

/* memory management */

// EXPORT_SYMBOL(flush_icache_range);
extern "C" {
    pub fn flush_icache_range();
}

/*
 * libgcc functions - functions that are used internally by the
 * compiler...  (prototypes are not correct though, but that
 * doesn't really matter since they're not versioned).
 */

// #define DECLARE_EXPORT(name) extern void name(void); EXPORT_SYMBOL(name)

// DECLARE_EXPORT(__gcc_bcmp);
// DECLARE_EXPORT(__divsi3);
// DECLARE_EXPORT(__moddi3);
// DECLARE_EXPORT(__modsi3);
// DECLARE_EXPORT(__udivmoddi4);
// DECLARE_EXPORT(__udivsi3);
// DECLARE_EXPORT(__umoddi3);
// DECLARE_EXPORT(__umodsi3);
// DECLARE_EXPORT(__muldi3);
// DECLARE_EXPORT(__ucmpdi2);
// DECLARE_EXPORT(__lshrdi3);
// DECLARE_EXPORT(__ashldi3);
// DECLARE_EXPORT(__ashrdi3);
extern "C" {
    pub fn __gcc_bcmp();
    pub fn __divsi3();
    pub fn __moddi3();
    pub fn __modsi3();
    pub fn __udivmoddi4();
    pub fn __udivsi3();
    pub fn __umoddi3();
    pub fn __umodsi3();
    pub fn __muldi3();
    pub fn __ucmpdi2();
    pub fn __lshrdi3();
    pub fn __ashldi3();
    pub fn __ashrdi3();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
