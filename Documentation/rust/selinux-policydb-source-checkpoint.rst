SELinux policy database Rust source checkpoint
=============================================

Scope and state
---------------

This default-off provider reconstructs all 97 unique functions in retained
security/selinux/ss/policydb.c at baseline
bc846d0c26f566e6a586592c2a15d5a0aa9f5f94.  The previous 1,404-line Rust unit had
15 explicit body placeholders, 26 entirely omitted functions, incorrect local
ABI layouts, zero GFP flags, fictitious macro externs, incomplete bitmap
iteration and skipped boolean-index validation.  Merely removing placeholder
markers would not have made it a complete policydb implementation.

The replacement is divided into policydb-core.rs (44 C functions),
policydb-symbols.rs (22) and policydb-wire.rs (31).  The root policydb.rs owns
configured bindings, small allocation helpers and layout assertions.  The
private compatibility table remains a Rust-only value, not an invented shared
ABI.  Shared C structures and constants are generated from retained headers.

Behavior reconstructed
----------------------

* Compatibility table and all three transition-table hash/comparison/search
  paths; native unsigned wrapping is retained where C hash/count arithmetic
  wraps.
* Ownership callbacks, policy initialization/destruction, correctly paired
  slab versus virtual allocations, sparse symbol indexes, boolean-hole and MLS
  category checks, initial-SID loading/fallback and context predicates.
* Complete symbol/permission readers, permission masks/completeness, constraint
  expression op/attribute/stack validation, policy-version gates, class default
  validation, alias boolean checks, object_r duplicate cleanup and bounds walks.
* Complete range, legacy/compressed filename, genfs and object-context input,
  top-level policy input, corresponding output, bitmaps and all symbol writers.
  Cursor/remaining-length checks preserve original read/write batching; network
  node words remain unchanged and Infiniband prefixes use big endian.
* Original errno propagation, special EINVAL mappings, and partial-allocation
  ownership.  No runtime fallback calls the retained policydb.c algorithms.

The narrow C bridge invokes existing allocator/macros/header-inline primitives,
bitmap iteration, hashtab insert/search, context destruction and diagnostic
primitives.  Existing out-of-line SELinux subunits remain ordinary dependencies,
as in C.  This is a policydb-unit replacement, not a claim that all dependencies
are Rust.  Printing bridges preserve severity and message data but consolidate
source locations into policydb-glue.c, so dynamic-debug per-call-site metadata
is not identical.

Default-off composition
-----------------------

Add these lines without replacing other provider includes:

* security/selinux/Kconfig:
  source "security/selinux/ss/Kconfig.policydb-rust"
* End of security/selinux/Makefile:
  include $(srctree)/security/selinux/ss/policydb-provider.mk
* End of rust/Makefile:
  include $(srctree)/security/selinux/ss/policydb-bindgen.mk

CONFIG_RUST_SELINUX_POLICYDB depends on RUST and SECURITY_SELINUX and defaults
to n.  When selected, the existing ss/policydb.o composite slot is produced by
Rust, with policydb-glue.o added.  When disabled, the retained C producer remains
in place.  The provider uses canonical configured bindgen and normal Kbuild Rust
recipes/flags; no generated ABI is checked in.  RUST already excludes RANDSTRUCT
and GCC_PLUGIN_RANDSTRUCT.  The services provider owns flask.h generation when
both are selected; otherwise this provider uses the original generator recipe.

Source review and acceptance limits
-----------------------------------

The source inventory accounts for all 97 C routines (the two alternative debug
implementations count once each).  Review compared symbol validation, bounds,
initial SIDs, ownership/allocator pairing, transition formats, version gates and
serialization ordering against C.  Raw pointers are used where C callbacks can
access the enclosing policy object, avoiding exclusive Rust references across
those callbacks.  No explicit placeholders, fabricated layout objects, fake
external macro variables or zero configured allocation flags remain.

These are SOURCE-ONLY results.  No compiler, bindgen, make, test, configuration,
VM, policy load or security-setting operation was run.  The size/alignment/
offset assertions are obligations for a future configured build, not evidence
that one passed.  Native link, provider selection/rebuild behavior and runtime
security equivalence remain unverified.

Maintained C tests and gaps
--------------------------

The original tools/testing/selftests/lsm Makefile and its three C test programs
remain unchanged.  It retains -Wall -O2 $(KHDR_INCLUDES), common.c and ../lib.mk.
lsm_get_self_attr_test.c has an indirect SELinux context branch when that LSM is
active; lsm_set_self_attr_test.c covers bad pointers, sizes and flags;
lsm_list_modules_test.c covers active-LSM enumeration.  These are generic LSM ABI
checks, not direct policy database parser/serializer acceptance.  No direct
maintained C policydb unit test or policy-load corpus was located under
security/selinux, tools/testing or scripts/tests.  Existing genheaders tests use
retained genheaders.c as an oracle for generated constants only and do not
validate policydb execution.  No translated tests were substituted.

Future admitted execution must preserve original maintained C test sources,
Makefiles and flags, adapting only build/link paths.  Required uncovered areas
include retained-C differential policy parsing/round-trip bytes and remaining
length; all supported versions and formats; malformed/truncated/oversized inputs;
permission/constraint/bounds/sparse-index rejects; allocation-failure unwind; and
authorized policy load/reload/readback/security-decision checks.  A passing object
build alone cannot establish those properties.

Pre-existing C cleanup concerns
------------------------------

Exact retained behavior includes three unusual ownership cases: the symbol-loop
size_check/symtab_init/roles_init failures in policydb_read return via out without
policydb_destroy; range_read frees partially constructed range storage without
separately destroying successful bitmap members; and ocontext_destroy does not
release the ibendport union string.  These are separate existing C concerns, not
newly corrected behavior.  Source parity does not imply those paths are leak-free.
