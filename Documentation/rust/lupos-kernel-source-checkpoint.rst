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

Verification scope
------------------

Performed: exact baseline/source hashes, independent source comparisons,
callback and native declaration inventories, and text-diff whitespace checks.
No compiler, formatter, configuration, bindgen, build, probe, original test,
guest or CI execution occurred. The broad source phase remains active; native
qualification and general build fixes are later work.
