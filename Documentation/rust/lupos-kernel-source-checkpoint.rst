.. SPDX-License-Identifier: GPL-2.0

Lupos kernel compatibility source checkpoints
============================================

These are incomplete, inactive continuations of the existing Rust implementation
at ``126a30fae3bba11420ec2fcbde51a0a01bab1b5b``. Original C, native layout headers
and test oracles remain unchanged. Source correspondence is not a configured
build, ABI qualification or claim that original Linux tests pass.

Task-work ownership and retry semantics
--------------------------------------

Independent whole-owner review retains all eight Rust function identities and
the add/cancel/run algorithms. All three compare-exchange loops now pass the
expected pointer by address and consume the failure observation without an extra
reload. Cancellation retains its predecessor slot, and run reads each next link
before invoking its callback and performing the original reschedule check.

Native READ_ONCE and plain accesses remain distinct. Actual task fields, locks,
current-task and thread flags replace projected layouts and guessed constants.
NMI notification uses the initialized per-CPU hard IRQ-work object and the
original this-CPU selection. The original no-op interrupt callback, invalid-mode
warning, unconditional callback invocation and nullable function equality remain.

Two callback-bearing public signatures use exact native forwarding shells into
the sole Rust bodies. Synchronous native callback contexts avoid the known
nominal outer-KCFI mismatch of unadapted nullable Rust function-pointer arguments,
without guessing context-dependent callback encodings or weakening CFI.

Remaining obligations are substantive. Extracted warning and cond_resched sites
change diagnostic source attribution by construction; original attribution is
not preserved or claimed by this checkpoint. Configured generated layout and
enum representation, emitted ABI/CFI, instrumentation/callsite effects, native
concurrency/lifetime and runtime behavior remain unqualified.

The declaration recipe and module are inactive proposals. The selector is
default-off with RUST and BROKEN dependencies; selection-time and both source
errors remain. No objects are selected and no original C owner is excluded.

Namespace transition and staged cleanup
--------------------------------------

Independent review restores namespace-fd flag normalization, pidfd error checks,
fd lifetime on each return, and the original boundary between preparation
cleanup and caller cleanup. User and active-PID namespace snapshots, references,
installation and releases retain their native ordering. Successful credential
transfer precedes the original filesystem, IPC and time commits. Namespace exit
stores the nullable proxy pointer itself rather than dereferencing NULL.

Configured unsupported-namespace checks, native syscall metadata and the exact
native init-entry shell are retained. The existing Rust transition algorithms
remain the sole candidate owners, with explicit unsafe scopes and no diagnostic
exemptions. Native types and operations replace fabricated pointer/layout views.

The pinned source has an inherited preparation path where filesystem-copy
failure leaves a prepared credential outside flag-gated cleanup. This checkpoint
preserves that behavior instead of silently changing the C policy. Source
correspondence does not establish freedom from native bugs.

Generated types, syscall/ABI/CFI, full init-body attributes and relocations,
diagnostic/instrumentation identity, non-local cleanup and runtime remain open.
The default-off BROKEN selector, selection errors and source holds remain;
no new objects are selected and the original C owner stays unchanged.

Credential transitions and native contracts
------------------------------------------

All seventeen existing credential Rust bodies survive independent review.
Exec now assigns effective IDs to both saved and filesystem IDs. Thread clones
share credentials only under the original configured keyring condition; other
paths retain allocation, namespace/security hooks, references and error order.

Native pointer-return, atomic, integer union, const and flexible-array contracts
replace incompatible source projections. CONFIG_KEYS-disabled arguments remain
unevaluated as in the original macros. The RCU callback has an exact native
signature trampoline into the retained Rust destructor, and the initializer has
the original native __init entry around the unchanged Rust cache setup body.

Seven separate BUG_ON sites preserve their original operands and native reads;
the daemon warning again expands WARN_ON_ONCE(!daemon) on the native pointer.
Handwritten warning-policy exemptions are absent. Added unsafe scopes were
independently normalized and compared separately from actual source repairs.

These source repairs do not establish native memory-model, generated ABI/CFI,
full init-body attributes, optimizer/LTO, original file/line/PC attribution or
allocation instrumentation parity. All such obligations remain open. The
selector is default-off and BROKEN, with source and selection errors intact.
Original credential C, native layout headers and tests remain unchanged.

Supplementary groups and user-access boundaries
----------------------------------------------

Independent whole-owner review replaces fabricated credential/group layouts,
numeric type projections and zero allocation flags with configured native
authority. Rust retains allocation decisions, user-array loops, sorted lookup,
permission/LSM decisions, references and failure cleanup. Native primitives
retain the real flexible-array allocation and GFP_KERNEL_ACCOUNT behavior.

The native sort boundary names the retained Rust comparator and the original
NULL swap callback, without a nullable callback argument crossing Rust FFI.
Native syscall and export metadata preserve the original public contracts;
syscall bodies forward to the Rust long-returning owners. Credential entry
calls use their real native declarations, without cross-module handle casts.

Opaque native storage does not qualify a Rust-plus-RANDSTRUCT profile: the
pinned global Rust restrictions remain unchanged. Configured value/layout ABI,
CFI, uaccess, syscall metadata, references, allocation attribution and runtime
still require qualification. All default-off/BROKEN and source/selection holds
remain, with the MULTIUSER condition preserved and no object selection changes.

Usermode-helper policy, callbacks and completion storage
-------------------------------------------------------

Independent review restores the literal configured static-helper path, including
the original empty-string policy, and the exact two-row sized sysctl registration.
The native early-init entry retains its original __init and initcall metadata
around the existing Rust setup body. Work and thread callbacks retain C ABI;
the public callback-bearing setup signature uses a native forwarding context.

Rust retains the original permission, credential, wait/completion, locking,
cleanup and error decisions. Native completion initialization preserves the
CONFIG_LOCKDEP split and self-links to final storage, without moving initialized
objects. All seventeen unsafe bodies have explicit scopes and caller contracts,
without handwritten diagnostic exemptions.

Integration moves the generated declarations from a local include into the
existing bindings crate, retaining its established generated-code and CFI policy.
No operational Rust body or native primitive changes in that registration step.
Explicit source errors protect the Rust owner and all three native companions;
the default-off BROKEN selector and selection errors choose no objects.

Configured layout, ABI/CFI, init-body instrumentation and relocations, completion
lifetime, source/callsite attribution and runtime/original tests remain open.
No helper policy is enabled or changed in the selected original C build.

User and resource-limit accounting completion
--------------------------------------------

Independent review retains eleven existing bodies and supplies eight missing
decrement, resource-limit and bootstrap bodies. The Rust decisions preserve
hierarchical limit propagation, zero-to-one reference acquisition, failure
rollback, final put ordering and the native LONG_MAX clamp. Native atomic and
RCU-reference primitives retain their original ordering rather than adopting
a stronger invented protocol.

Sysctl root registration remains conditional, while bootstrap hash insertion
and the initial NPROC charge remain unconditional. Four decrement warning sites
and both initialization BUG_ON operands retain their exact native expressions.
Typed native signatures and storage serve the sole Rust decision bodies.

Source comparison of all nineteen bodies does not establish generated by-value
ABI/CFI, symbol/section ownership, configured instrumentation, native concurrency
or runtime parity. All default-off/BROKEN, selection and source holds remain;
original C, layout headers and tests are unchanged.

Verification scope
------------------

Performed: exact baseline/source hashes, independent source comparisons,
callback and native declaration inventories, and text-diff whitespace checks.
No compiler, formatter, configuration, bindgen, build, probe, original test,
guest or CI execution occurred. The broad source phase remains active; native
qualification and general build fixes are later work.
