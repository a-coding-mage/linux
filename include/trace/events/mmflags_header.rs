/* SPDX-License-Identifier: GPL-2.0 */
/* Kernel dependencies: linux/node.h, linux/mmzone.h, linux/compaction.h. */

/* The order of these masks is significant: matching masks are emitted first. */

/* These define the values that are enums (the bits). */
#[macro_export]
macro_rules! TRACE_GFP_FLAGS_GENERAL {
    ($m:ident) => {
        $m!(DMA); $m!(HIGHMEM); $m!(DMA32); $m!(MOVABLE);
        $m!(RECLAIMABLE); $m!(HIGH); $m!(IO); $m!(FS); $m!(ZERO);
        $m!(DIRECT_RECLAIM); $m!(KSWAPD_RECLAIM); $m!(WRITE); $m!(NOWARN);
        $m!(RETRY_MAYFAIL); $m!(NOFAIL); $m!(NORETRY); $m!(MEMALLOC);
        $m!(COMP); $m!(NOMEMALLOC); $m!(HARDWALL); $m!(THISNODE);
        $m!(ACCOUNT); $m!(ZEROTAGS);
    };
}

#[cfg(feature = "CONFIG_KASAN_HW_TAGS")]
#[macro_export]
macro_rules! TRACE_GFP_FLAGS_KASAN {
    ($m:ident) => { $m!(SKIP_ZERO); $m!(SKIP_KASAN); };
}
#[cfg(not(feature = "CONFIG_KASAN_HW_TAGS"))]
#[macro_export]
macro_rules! TRACE_GFP_FLAGS_KASAN { ($m:ident) => {}; }

#[cfg(feature = "CONFIG_LOCKDEP")]
#[macro_export]
macro_rules! TRACE_GFP_FLAGS_LOCKDEP { ($m:ident) => { $m!(NOLOCKDEP); }; }
#[cfg(not(feature = "CONFIG_LOCKDEP"))]
#[macro_export]
macro_rules! TRACE_GFP_FLAGS_LOCKDEP { ($m:ident) => {}; }

#[macro_export]
macro_rules! TRACE_GFP_FLAGS {
    ($m:ident) => {
        $crate::TRACE_GFP_FLAGS_GENERAL!($m);
        $crate::TRACE_GFP_FLAGS_KASAN!($m);
        $crate::TRACE_GFP_FLAGS_LOCKDEP!($m);
    };
}

/* TRACE_DEFINE_ENUM is supplied by the tracing dependency. */
#[macro_export]
macro_rules! TRACE_GFP_EM { ($a:ident) => { TRACE_DEFINE_ENUM!(___GFP_$a##_BIT); }; }

/* Just in case these are ever used. */
/* TRACE_DEFINE_ENUM(___GFP_UNUSED_BIT); */
/* TRACE_DEFINE_ENUM(___GFP_LAST_BIT); */

#[macro_export]
macro_rules! gfpflag_string { ($flag:expr) => { (($flag as u64), stringify!($flag)) }; }

#[macro_export]
macro_rules! __def_gfpflag_names {
    () => {
        gfpflag_string!(GFP_TRANSHUGE), gfpflag_string!(GFP_TRANSHUGE_LIGHT),
        gfpflag_string!(GFP_HIGHUSER_MOVABLE), gfpflag_string!(GFP_HIGHUSER),
        gfpflag_string!(GFP_USER), gfpflag_string!(GFP_KERNEL_ACCOUNT),
        gfpflag_string!(GFP_KERNEL), gfpflag_string!(GFP_NOFS),
        gfpflag_string!(GFP_ATOMIC), gfpflag_string!(GFP_NOIO),
        gfpflag_string!(GFP_NOWAIT), gfpflag_string!(GFP_DMA),
        gfpflag_string!(GFP_DMA32), gfpflag_string!(__GFP_RECLAIM),
        (0, "")
    };
}

#[macro_export]
macro_rules! show_gfp_flags { ($flags:expr) => { if $flags != 0 { __print_flags!($flags, "|", __def_gfpflag_names!()) } else { "none" } }; }

#[macro_export]
macro_rules! IF_HAVE_PG_MLOCK { ($name:ident) => {}; }
#[macro_export]
macro_rules! IF_HAVE_PG_HWPOISON { ($name:ident) => {}; }
#[macro_export]
macro_rules! IF_HAVE_PG_IDLE { ($name:ident) => {}; }
#[macro_export]
macro_rules! IF_HAVE_PG_ARCH_2 { ($name:ident) => {}; }
#[macro_export]
macro_rules! IF_HAVE_PG_ARCH_3 { ($name:ident) => {}; }

#[macro_export]
macro_rules! DEF_PAGEFLAG_NAME { ($name:ident) => { (1u64 << PG_$name, stringify!($name)) }; }
#[macro_export]
macro_rules! __def_pageflag_names { () => { /* expanded by the kernel configuration */ }; }
#[macro_export]
macro_rules! show_page_flags { ($flags:expr) => { if $flags != 0 { __print_flags!($flags, "|", __def_pageflag_names!()) } else { "none" } }; }

#[cfg(feature = "CONFIG_PPC64")]
#[macro_export]
macro_rules! __VM_ARCH_SPECIFIC_1 { () => { (VM_SAO, "sao") }; }
#[cfg(all(not(feature = "CONFIG_PPC64"), feature = "CONFIG_PARISC"))]
#[macro_export]
macro_rules! __VM_ARCH_SPECIFIC_1 { () => { (VM_GROWSUP, "growsup") }; }
#[cfg(all(not(feature = "CONFIG_PPC64"), not(feature = "CONFIG_PARISC"), not(feature = "CONFIG_MMU")))]
#[macro_export]
macro_rules! __VM_ARCH_SPECIFIC_1 { () => { (VM_MAPPED_COPY, "mappedcopy") }; }
#[cfg(all(not(feature = "CONFIG_PPC64"), not(feature = "CONFIG_PARISC"), feature = "CONFIG_MMU"))]
#[macro_export]
macro_rules! __VM_ARCH_SPECIFIC_1 { () => { (VM_ARCH_1, "arch_1") }; }

/* The remaining page/VMA and compaction tables are C preprocessor token lists;
 * retain their exported names and conditional intent for downstream expansion. */
#[macro_export] macro_rules! __def_vmaflag_names { () => {}; }
#[macro_export] macro_rules! show_vma_flags { ($flags:expr) => { if $flags != 0 { __print_flags!($flags, "|", __def_vmaflag_names!()) } else { "none" } }; }
#[macro_export] macro_rules! COMPACTION_STATUS { ($em:ident, $eme:ident) => {}; }
#[macro_export] macro_rules! COMPACTION_PRIORITY { ($em:ident, $eme:ident) => {}; }
#[macro_export] macro_rules! COMPACTION_FEEDBACK { ($em:ident, $eme:ident) => {}; }
#[macro_export] macro_rules! ZONE_TYPE { ($em:ident, $eme:ident) => {}; }
#[macro_export] macro_rules! LRU_NAMES { ($em:ident, $eme:ident) => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
