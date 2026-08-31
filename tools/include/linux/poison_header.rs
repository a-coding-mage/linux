/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/********** include/linux/list.h **********/

/*
 * Architectures might want to move the poison pointer offset
 * into some well-recognized area such as 0xdead000000000000,
 * that is also not mappable by user-space exploits:
 *
 * C conditional:
 *   #ifdef CONFIG_ILLEGAL_POINTER_VALUE
 *     POISON_POINTER_DELTA = _AC(CONFIG_ILLEGAL_POINTER_VALUE, UL)
 *   #else
 *     POISON_POINTER_DELTA = 0
 *   #endif
 */
pub const POISON_POINTER_DELTA: usize = 0;

/*
 * C++ used NULL for these macros. In C, these are non-NULL pointers that will
 * result in page faults under normal circumstances, used to verify that nobody
 * uses non-initialized list entries.
 */
pub const LIST_POISON1: *mut c_void =
    (0x100usize.wrapping_add(POISON_POINTER_DELTA)) as *mut c_void;
pub const LIST_POISON2: *mut c_void =
    (0x200usize.wrapping_add(POISON_POINTER_DELTA)) as *mut c_void;

/********** include/linux/timer.h **********/
/*
 * Magic number "tsta" to indicate a static timer initializer
 * for the object debugging code.
 */
pub const TIMER_ENTRY_STATIC: *mut c_void =
    (0x300usize.wrapping_add(POISON_POINTER_DELTA)) as *mut c_void;

/********** mm/page_poison.c **********/
pub const PAGE_POISON: u8 = 0xaa;

/********** mm/page_alloc.c ************/

pub const TAIL_MAPPING: *mut c_void =
    (0x400usize.wrapping_add(POISON_POINTER_DELTA)) as *mut c_void;

/********** mm/slab.c **********/
/*
 * Magic nums for obj red zoning.
 * Placed in the first word before and the first word after an obj.
 */
pub const SLUB_RED_INACTIVE: u8 = 0xbb; /* when obj is inactive */
pub const SLUB_RED_ACTIVE: u8 = 0xcc; /* when obj is active */

/* ...and for poisoning */
pub const POISON_INUSE: u8 = 0x5a; /* for use-uninitialised poisoning */
pub const POISON_FREE: u8 = 0x6b; /* for use-after-free poisoning */
pub const POISON_END: u8 = 0xa5; /* end-byte of poisoning */

/********** arch/$ARCH/mm/init.c **********/
pub const POISON_FREE_INITMEM: u8 = 0xcc;

/********** arch/ia64/hp/common/sba_iommu.c **********/
/*
 * arch/ia64/hp/common/sba_iommu.c uses a 16-byte poison string with a
 * value of "SBAIOMMU POISON\0" for spill-over poisoning.
 */

/********** fs/jbd/journal.c **********/
pub const JBD_POISON_FREE: u8 = 0x5b;
pub const JBD2_POISON_FREE: u8 = 0x5c;

/********** drivers/base/dmapool.c **********/
pub const POOL_POISON_FREED: u8 = 0xa7; /* !inuse */
pub const POOL_POISON_ALLOCATED: u8 = 0xa9; /* !initted */

/********** drivers/atm/ **********/
pub const ATM_POISON_FREE: u8 = 0x12;
pub const ATM_POISON: u32 = 0xdeadbeef;

/********** kernel/mutexes **********/
pub const MUTEX_DEBUG_INIT: u8 = 0x11;
pub const MUTEX_DEBUG_FREE: u8 = 0x22;

/********** security/ **********/
pub const KEY_DESTROY: u8 = 0xbd;
