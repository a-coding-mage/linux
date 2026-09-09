/*
 * include/asm-xtensa/shmparam.h
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of this
 * archive for more details.
 */

/*
 * Xtensa can have variable size caches, and if
 * the size of single way is larger than the page size,
 * then we have to start worrying about cache aliasing
 * problems.
 */

/* Translated from: #define SHMLBA ((PAGE_SIZE > DCACHE_WAY_SIZE)? PAGE_SIZE : DCACHE_WAY_SIZE) */
pub const SHMLBA: usize = if PAGE_SIZE > DCACHE_WAY_SIZE {
    PAGE_SIZE
} else {
    DCACHE_WAY_SIZE
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
