.. SPDX-License-Identifier: GPL-2.0-only

Allocator and reclaim source checkpoint
======================================

This checkpoint belongs to the broad non-driver source implementation phase.
It is not a full-build, boot or runtime acceptance result. General integration
and build fixes follow the combined source phase; the original C regression
programs, compiler options, assertions and timeouts remain authoritative.

Reverse mapping
---------------

The Rust implementation of ``mm/rmap.c`` was reconciled with that file at
``e1d84f501551943a11f4c5271e9f5c85d7e15168`` (SHA-256
``d0397d01dfe7baf82ececda7a9a88627368d4843a0e96b645661ded7a6311750``).
Its 72 distinct C function names have Rust definitions. Independent source
review covered anonymous-VMA ownership and RCU, reverse mapping and mapcounts,
unmap/migration rollback, TLB generation arithmetic and configuration branches.
The review corrected a native type-name collision and explicitly retained
C integer wrapping where ranges are not bounded by signed INT_MAX.

``CONFIG_RUST_RMAP`` supplies explicit Rust object and inspection recipes, with
native types/constants generated from the configured kernel headers. The
original C implementation is preserved and is not the selected object owner.
Header algorithms, architecture primitives, atomic/locking and diagnostic
boundaries remain in the native companion and are explicit remaining native
work. They are not described as metadata-only or as an all-Rust kernel.

Source formatting, original-C hash preservation and patch composition checks
passed. Actual configured binding generation, Rust type/lint checks, native
compiler protection, callback ABI/CFI, link ownership, inspection/dependency
mutations, all configuration alternatives and original C runtime tests are
pending. Four const-generic unchecked-tail guards also require later negative
compilation controls. No new full build or guest was started for this source
checkpoint. The previously validated kernel remains a separate frozen artifact.

Classic and multigeneration reclaim
----------------------------------

The combined ``mm/vmscan.c`` source candidate covers 204 original function
definition occurrences, including mutually exclusive configuration branches.
This is a source inventory, not 204 independent functions or test results.
Classic reclaim and the multigeneration LRU share one configured native
``scan_control`` and private-type authority. Anonymous native bitfields use
generated accessors. Reclaim, aging, eviction, protection, demotion, direct
reclaim and kswapd decisions are represented in Rust.

Reciprocal source review covered the shared interface, retry and allocation
unwinds, configured READ_ONCE/WRITE_ONCE widths, static-key addresses, PID and
generation arithmetic, disabled-MGLRU calls and the original compile-time
BUILD_BUG conditions. ``CONFIG_RUST_BUILD_ASSERT_ALLOW`` is not enabled to
bypass a surviving compile-time failure. Native header algorithms and leaf
boundaries remain explicitly outside a claim of complete Rust production.

The original C and test files are unchanged. Formatting and source composition
checks passed; this candidate has not undergone native compilation, generated
ABI/signature validation, compiler-instrumentation and link ownership checks,
configuration matrix coverage, boot or original C regression execution.
These remain gates for the later combined build-fix and acceptance phases.

SLUB allocator
--------------

The two SLUB source lanes are composed under one native header/private-type
authority. Their inventories map 224 original first-half and 174 second-half
function-definition occurrences, including configuration variants, plus the
macro-generated statistics callbacks. Allocation/free, sheaf and barn control,
slab state, cache geometry, debug/proc/sysfs and hotplug decisions have Rust
source bodies. Reciprocal source review repaired a stale allocation pointer
after disposal and CPU-iteration boundary mismatches. Counts describe source
inventory only; configured compilation and runtime behavior remain unverified.

``mm/slub_allocation_exports.c`` retains thin native runtime entry points for
caller-IP and macro-selected parameter ABI. Most capture ``_RET_IP_`` at the
public entry and pass it to a Rust algorithm body; the caller-tracking entry
passes its explicit caller argument instead. Capturing it in an ordinary leaf
called from Rust would identify an intermediate callsite. These veneers are
real C runtime code and own their public symbols. They are not metadata-only,
and this source checkpoint does not claim a completely Rust-emitted public
API. Header algorithms, native static storage and registration leaves also
remain explicit native work; none forwards to the retained ``mm/slub.c`` body.

The public veneers are: ``kmem_cache_alloc_noprof``,
``kmem_cache_alloc_lru_noprof``, ``kmem_cache_alloc_node_noprof``,
``kmem_cache_alloc_from_sheaf_noprof``, ``__kmalloc_large_noprof``,
``__kmalloc_large_node_noprof``, ``__kmalloc_node_noprof``,
``__kmalloc_noprof``, ``__kmalloc_node_track_caller_noprof``,
``_kmalloc_nolock_noprof``, ``__kmalloc_cache_noprof``,
``__kmalloc_cache_node_noprof``, ``__kmalloc_flags_noprof``,
``kmem_cache_free``, ``kvfree_rcu_cb``, ``kfree``,
``krealloc_node_align_noprof``, ``__kvmalloc_node_noprof``,
``kvrealloc_node_align_noprof``, ``kmem_cache_free_bulk``,
``__kmem_cache_free_bulk``, ``__kmem_cache_alloc_bulk``,
``__refill_objects_node`` and configured ``memcg_alloc_abort_single``.

Native companions inherit the original SLUB KASAN, KCSAN and KCOV exclusions;
the two original KMSAN function exemptions remain local. This is no proof of
equivalent Rust sanitizer coverage. Configured native types, callback and
public ABI, stack/compiler protection, caller-IP profiling, static data and
section lifetimes, original C suites and all configuration branches remain
gates for the later integrated build-fix and acceptance phases.

Common slab cache and deferred free
----------------------------------

The ``mm/slab_common.c`` source candidate maps all 78 original function
definition occurrences, including configuration alternatives. Cache creation,
merge/refcount/destruction, kmalloc geometry and randomization, reporting,
sensitive freeing and both deferred RCU-free configurations have Rust bodies.
The configured kernel headers and original private RCU declarations supply
native types. Invented zero-sized layouts and Default lock initializers are
not used. Exact list, mutex, per-CPU raw-spinlock and registration initializers
remain native storage boundaries.

Reciprocal source review corrected allocation-profile site merging,
configuration-variant inventory and native enum-width coupling. The public
``kmem_cache_destroy`` entry remains native to capture its original caller's
``_RET_IP_``; configured ``bpf_get_kmem_cache`` remains native for the original
``__bpf_kfunc`` metadata. Both call Rust algorithm bodies. These two public
symbols and native header/storage leaves remain explicit native runtime work,
not metadata-only or a claim of complete direct Rust public ownership.

Formatting, source inventory, immutable original-C hashes and read-only patch
checks passed. No bindgen, compiler, configuration, native probe, kernel build,
guest or original C test was run for this source checkpoint. Native signature,
layout, compiler-protection and section/data/callback identity, all branch
configurations and runtime error/lock/RCU behavior remain unverified gates.

Memory compaction
-----------------

The compaction source candidate maps all 85 original function-definition
occurrences (77 unique names), including disabled configuration alternatives.
Independent body review covered isolation and rollback, free/migration scans,
allocation/free callbacks, direct and proactive policy, kcompactd, sysctl,
sysfs and hotplug. It corrected a draft's plain list accesses to the original
header READ_ONCE/WRITE_ONCE operations. Native ``compact_control`` counter and
search-order widths, node iteration limits, integer promotion and truncation
were retained explicitly. Disabled macros discard their original arguments.

Configured private kernel headers supply actual types. Original native header,
architecture/atomic/list/RCU operations, allocation-profile wrappers,
tracepoints, warning sites and init registration remain native boundaries.
No original compaction decision body is forwarded to C. The freezer wait macro
re-evaluates the Rust predicate at its native wait points; its operational ABI
and profiling/callback identity remain later validation gates.

The original ``compaction.o`` remains applicable even with COMPACTION disabled;
the selector therefore does not incorrectly require that feature. Unsupported
native compiler policies fail closed. Formatting, full source inventory,
independent source review and patch checks passed. Generated types/signatures,
compiler instrumentation, dependency/selection/inspection targets, link,
configuration and original C runtime tests are still pending. No per-owner
build or guest was run.

Page and folio migration
-----------------------

The migration candidate provides Rust bodies for all 57 ordinary functions in
the original ``mm/migrate.c``, plus the syscall body dispatched through native
``SYSCALL_DEFINE6`` metadata. Independent source review covered all 57 bodies:
six PTE/wait functions, sixteen move/batch functions, seven movable-owner
functions, thirteen mapping/flags/buffer/fallback functions, eleven syscall
functions and four NUMA functions. The syscall metadata thunk was also
compared. These counts are original source functions, not tests or independent
runtime scenarios.

Review covered reference freezing and unlocking, swapcluster/xarray/IRQ order,
anonymous-VMA and page-table ownership, hugeTLB and buffer callbacks, UFFD/RWP
PTE bits, EAGAIN reinsertion and error rollback, paired batch lists, split and
retry accounting, compat/native user access, status arrays, ptrace/LSM/task/mm
lifetimes, node/cpuset validation and NUMA migration policy. Native integer
promotions preserve the actual header definition of HPAGE_PMD_NR.

The 339 declared native dependencies remain explicit: header algorithms,
architecture/uaccess/atomic/locking primitives, diagnostics/trace metadata and
external owner APIs. The native syscall entry dispatches to Rust; no ordinary
migrate.c body is forwarded to the retained original C implementation.
Configured signature/layout/CFI, compiler protection, callback/data identity,
link ownership and original C runtime tests remain unverified. Formatting,
inventories, source review and patch checks do not establish those gates.
No compiler, bindgen, configuration, probe, full build or guest was run.

VM statistics
-------------

The VM statistics source candidate maps 92 configured definition occurrences
(86 unique original names), plus the two seq-open macro bodies. Rust owns the
accounting/threshold/folding algorithms, NUMA reset, work/hotplug policy,
fragmentation calculations, proc/seq/debug output and registration sequence.
Native aligned atomic arrays, per-CPU objects, lock/work initializers and 101
configured primitive entries remain explicitly native runtime/storage work.

Source review corrected two concrete errors before compilation:

* ``scoped_guard`` generates an inner loop. Its original isolated-CPU
  ``continue`` releases RCU and still reaches the outer ``cond_resched``.
  The Rust shepherd now preserves that behavior, independently source-checked.
* The original text array infers its extent from designated initializers and
  compares it with the statistics count only under PROC_FS. One configured
  Rust initializer inventory now supplies both inferred extent and values,
  retaining the PROC_FS-only constant assertion and alternate-config storage.
  All 218 prior initializer expression/value pairs and their ordered cfg
  inventory remain unchanged. The repaired extent/assertion logic received
  an independent narrow review.

The original candidate and intermediate fixes are retained in source evidence.
The source review covers accounting/locking and byte-counter arithmetic, IRQ
fallback versus compare-exchange retry, zone/node folding, page-set expiry,
NUMA and CPU lifecycle, snapshot/error handling and diagnostic walks. It is not
exhaustive validation of every native leaf, generated type or configuration.
Formatting and source composition checks passed. No compiler, bindgen,
configuration, probe, kernel build, guest or original C tests ran. Native
layout/signature/CFI, compiler protection, data/callback/symbol identity,
dependency/inspection and runtime acceptance remain open.

Page cache and shared memory
----------------------------

The filemap candidate maps all 123 ordinary original definitions and the
cachestat syscall body. Independent source review covered the complete body
inventory. Repairs preserve C's LP64/ILP32 usual arithmetic conversions for
the zero-count direct-write last index and the three separate lexical folio
allocation tags. All original C and regression sources remain unchanged.

The shmem candidate maps 177 configured definition occurrences (158 distinct
names), independently reviewed in two source scopes. A separate recheck covers
the repair delta: primitive MAX names, 32-bit splice/statfs arithmetic, native
test_bit instrumentation, THP enum/huge-page config exposure, the exact !THP
fallback, kernel C-string pointers and six original allocation-tag sites.

Native storage/registration objects, header algorithms, architecture and macro
leaves, and shmem's generated static-inline wrappers are executable native
boundaries, not Rust completion or metadata-only objects. No original filemap.c
or shmem.c implementation is forwarded as a runtime fallback. Emitted tag and
allocation-partition identity, diagnostics, full ABI/layout/CFI, symbol and
section identity, security policy and all configuration/runtime gates remain
unverified. The shmem boot-string helper retains its sole caller's init lifetime
and the native initdata buffer; the later linked section audit remains required.

Generated declarations for filemap, shmem and reclaim reside in the existing
bindings crate under its unchanged generated-code policy. New owner-wide lint
allowances are removed from reclaim and shmem; handwritten diagnostics remain
visible for the general build-fix phase. Generation is an explicit bindings.o
prerequisite. Three additional filemap string boundaries use the kernel's
existing const-safe char-pointer conversion with unchanged literal bytes.

This is a reviewed source checkpoint. No compiler, bindgen, preprocessing,
configuration, probe, kernel build, guest or original C test was executed for
these new owners. Drivers remain deferred; this source checkpoint does not
release another per-owner build cycle.
