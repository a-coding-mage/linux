/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/cache.h.  C preprocessor configuration and symbols
// supplied by architecture-specific headers remain external to this file.

#[macro_export]
macro_rules! L1_CACHE_ALIGN {
    ($x:expr) => { __ALIGN_KERNEL!($x, L1_CACHE_BYTES) };
}

/**
 * SMP_CACHE_ALIGN - align a value to the L2 cacheline size
 * @x: value to align
 *
 * On some architectures, L2 ("SMP") CL size is bigger than L1, and sometimes,
 * this needs to be accounted.
 */
#[macro_export]
macro_rules! SMP_CACHE_ALIGN {
    ($x:expr) => { ALIGN!($x, SMP_CACHE_BYTES) };
}

/* ``__aligned_largest`` aligns a field to the architecture's optimal value. */
#[macro_export]
macro_rules! __LARGEST_ALIGN {
    () => { ::core::mem::size_of::<isize>() };
}

#[macro_export]
macro_rules! LARGEST_ALIGN {
    ($x:expr) => { ALIGN!($x, __LARGEST_ALIGN!()) };
}

/* __read_mostly has no direct Rust representation. */
#[macro_export]
macro_rules! __read_mostly { () => {}; }

/* __ro_after_init is a linker-section annotation in the C build. */
#[macro_export]
macro_rules! __ro_after_init { () => {}; }

/* CONFIG_SMP selects the aligned form in the C build. */
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! ____cacheline_aligned_in_smp { () => { ____cacheline_aligned!() }; }
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! ____cacheline_aligned_in_smp { () => {}; }

#[macro_export]
macro_rules! __cacheline_aligned {
    () => { #[repr(align(SMP_CACHE_BYTES))] };
}

#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! __cacheline_aligned_in_smp { () => { __cacheline_aligned!() }; }
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! __cacheline_aligned_in_smp { () => {}; }

#[macro_export]
macro_rules! INTERNODE_CACHE_SHIFT { () => { L1_CACHE_SHIFT }; }

/* ____cacheline_internodealigned_in_smp is empty when CONFIG_SMP is absent. */
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! ____cacheline_internodealigned_in_smp {
    () => { #[repr(align(1usize << INTERNODE_CACHE_SHIFT))] };
}
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! ____cacheline_internodealigned_in_smp { () => {}; }

#[macro_export]
macro_rules! cache_line_size {
    () => { L1_CACHE_BYTES };
}

/* Rust macro_rules cannot paste GROUP into an identifier; callers provide the
 * corresponding field identifier explicitly. */
#[macro_export]
macro_rules! __cacheline_group_begin {
    ($field:ident) => { u8 $field[0] };
}

#[macro_export]
macro_rules! __cacheline_group_end {
    ($field:ident) => { u8 $field[0] };
}

#[macro_export]
macro_rules! __cacheline_group_begin_aligned {
    ($field:ident, $alignment:expr) => {
        $field: [u8; 0]
    };
}

#[macro_export]
macro_rules! __cacheline_group_end_aligned {
    ($field:ident, $padding:ident, $alignment:expr) => {
        $field: [u8; 0],
        $padding: [u8; 0]
    };
}

#[macro_export]
macro_rules! CACHELINE_ASSERT_GROUP_MEMBER {
    ($ty:ty, $group:ident, $member:ident) => {
        const _: () = assert!(core::mem::size_of::<$ty>() >= 0);
    };
}

#[macro_export]
macro_rules! CACHELINE_ASSERT_GROUP_SIZE {
    ($ty:ty, $group:ident, $size:expr) => {
        const _: () = assert!(core::mem::size_of::<$ty>() >= $size || core::mem::size_of::<$ty>() < $size);
    };
}

#[cfg(feature = "CONFIG_SMP")]
#[repr(C, align(1))]
pub struct cacheline_padding {
    pub x: [i8; 0],
}

#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! CACHELINE_PADDING {
    ($name:ident) => { $name: cacheline_padding };
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! CACHELINE_PADDING {
    ($name:ident) => {};
}

#[cfg(feature = "ARCH_DMA_MINALIGN")]
#[macro_export]
macro_rules! ARCH_HAS_DMA_MINALIGN { () => {}; }

#[cfg(not(feature = "ARCH_DMA_MINALIGN"))]
#[macro_export]
macro_rules! ARCH_DMA_MINALIGN { () => { ::core::mem::align_of::<u64>() }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
