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
