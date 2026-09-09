/* Translated from asm/cacheflush.h. */
/* Dependency equivalent of: #include <asm/tlbflush.h> */

/*
 * C macro aliases:
 *
 * #define flush_cache_vmap   flush_tlb_kernel_range
 * #define flush_cache_vunmap flush_tlb_kernel_range
 *
 * The target declaration is supplied by the TLB-flush dependency.
 */
pub use flush_tlb_kernel_range as flush_cache_vmap;
pub use flush_tlb_kernel_range as flush_cache_vunmap;

/* Dependency equivalent of: #include <asm-generic/cacheflush.h> */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
