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

An independently reviewed follow-on repairs explicit warning and cond_resched
file/line operands under a mandatory, separately verified original-file input.
The pinned x86 warning expansion retains native UD2, conditional bug-table
fields, distinct once sites, KUnit suppression and reset ownership. Unsupported
architecture/table profiles remain rejected; native !BUG behavior is retained.
The reschedule leaf preserves __might_resched followed by _cond_resched.

This does not restore the original trap PC, registers or stack: they still
belong to the extracted leaf. The original-file value, emitted records and
relocations, branch/profiling provenance, generated layout and enum ABI/CFI,
instrumentation, native concurrency/lifetime and runtime remain unqualified.
No default file string, generic warning slowpath or synthetic once state is
introduced, and the unconditional source hold remains.

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

Namespace-tree listing and user-cache ownership
----------------------------------------------

Independent namespace-tree review restores locked-reader and RCU protection,
lower-bound and exact lookup, request validation, cursor ownership and both
listing modes. Accepted entries hold a reference while RCU is dropped for
faultable user stores; RCU is reacquired before the next link is read. Native
put_user retains the value read inside its macro, and disabled VFS warnings
retain their original operand laziness. Readers and writers share one native
seqlock. Callback-bearing signatures use synchronous native contexts.

User-cache review retains the original allocation race and reference decisions.
IRQ-save outputs have native unsigned-long writable storage; final hash removal
precedes unlock/restore, epoll destruction and slab free. PREEMPT_RT keeps its
native zero-flags/no-hardware-IRQ-disable behavior. BINFMT_MISC y and m retain
their actual native storage and exports. Namespace/root initializers and the
native subsys-init entry preserve their configured source contracts.

Both owners remain inactive behind default-off BROKEN selectors and explicit
source/selection errors. Configured bindings, ABI/CFI, native memory ordering,
metadata/instrumentation and runtime still require later qualification.

Capability boot registration
----------------------------

The bounded boot repair retains one Rust file_caps_enabled integer initialized
to one. The exact native __setup("no_file_caps", file_caps_disable) entry calls
the retained Rust body, which ignores its input, stores zero and returns one.
The native char pointer uses the existing kernel FFI alias. Registration remains
independent of MULTIUSER, preserving native parser prefix/suffix behavior.

Only this boot prefix was reviewed. The remaining capability Rust body is
byte-preserved and is not credited as qualified coverage. Integration adds an
unconditional Rust source error alongside the native metadata error and the
inactive default-off/BROKEN selection guard. Init-body attributes, emitted setup
records, ABI and original tests remain open; no object selection changes.

CPU-thread and stopper lifecycle
-------------------------------

The reviewed smpboot continuation replaces the former successful-return
placeholder with the native thread state machine, callback decisions and
creation/parking/publication order. Unpark traverses forward, park traverses
backward, and registration failure joins all possible-CPU tasks before cleanup.
The separate configured idle-cache body reuses native per-CPU storage, fork
results and the original possible-CPU iterator.

The stop-machine continuation retains all twenty-seven Rust bodies. Its native
thread descriptor now addresses the real per-CPU stopper task member. The state
representation permits the original terminal EXIT-to-next-value acknowledgement;
callback invocation passes the original data. Nowait callback values are copied
before publication, without escaping a wrapper's stack context. Native warning
operands and the distinct NR_CPUS==1 possible-CPU expansion are retained.

Independent source review covers reference, callback, retry and unwind ordering.
It does not establish emitted ABI/CFI, exact diagnostic PCs, configured init
attributes, live CPU-hotplug synchronization or original-test equivalence.

IRQ work and notifier callbacks
-------------------------------

IRQ-work waiting now reloads the native BUSY atomic predicate on each iteration,
preserving prepare/state/check/signal/schedule/finish and synchronize_rcu order.
The non-RT hard-interrupt spin and RT/deferred wait cases remain distinct.
Native per-CPU storage and callbacks are retained, and a true weak architecture
fallback remains overridable by the original strong architecture owner.

The extracted trace leaf still changes _RET_IP_ scope: with ordinary non-inlined
FFI it returns into Rust rather than identifying the original native callsite.
This is a concrete attribution obligation, not a passed comparison. Original
irq_work.o no-ftrace policy must cover every future Rust/native split object;
no new instrumentation exemption is granted by this inactive checkpoint.

Notifier review preserves all twenty-six existing function identities, including
DEBUG_NOTIFIERS rejection before callback/count/budget updates, callback-address
traces, ordered rollback and native synchronization. Native storage retains the
exact static ATOMIC_NOTIFIER_HEAD(die_chain) initializer and lock name. Twelve
lock/RCU/SRCU leaves carry their native acquire/release context contracts, and
four distinct boot-check unlikely sites remain distinct.

Actual Rust-body notrace, complete NOKPROBE coverage, branch-profiling provenance
and configured cross-language context analysis remain substantive blockers.
Native annotations alone do not establish them. All four selectors remain
default-off/BROKEN, with SMP and IRQ_WORK constraints where originally required,
unconditional source stops and selection errors. No object ownership changes.

Workqueue common foundation
---------------------------

The historical kernel/workqueue.rs file contains comments rather than an
executable worker-pool owner. It is preserved, as is the existing working
rust/kernel/workqueue.rs client API. A separate source-held entrypoint begins
the missing implementation without replacing those client abstractions.

The first independently reviewed common slice supplies original work-data
encoding, pending publication, PWQ/pool lookup, ID assignment and off-queue
packing decisions. Pending-clear publication preserves the original write
barrier, atomic replacement and full barrier. Native configured declarations
retain the real private layouts, alignment, flexible arrays, per-CPU and RCU
annotations; source constants are not guessed Rust substitutes.

One native storage proposal retains original static objects, initialized locks,
system pointers and declaration text. This does not implement the missing
resource/boot, dispatch, execution, completion or diagnostic algorithms. Native
inline/macro leaves provide primitives; no original C workqueue policy fallback
is introduced. The new Rust entrypoint and native companion both hard-fail.

The proposed workqueue_native binding graph remains absent and unqualified,
including reuse of existing public native type identities. Original KBUILD_MODNAME
must preserve workqueue.* parameter-name and parmtype metadata despite the new
translation-unit filename. Callback/ABI/CFI, source/profiling attribution, native
owner code-generation policy, symbol selection and original tests remain open.
No workqueue build enrollment or complete-owner coverage is claimed.

A separately reviewed primitive extension adds real native lock/IRQ-save,
list/container/RCU iterator, atomic/bit, task/per-CPU/mask, completion and trace
leaves, plus the exact dead-softirq work declaration. Algorithms remain Rust
decisions; these primitives do not supply missing queue or worker policy.
The native iterator retains its initial RCU check and each protected link load.
Non-atomic bit operations and relaxed compare-exchange remain distinct.

Keyed work initialization reuses the existing Rust-kernel helper. Its public
work_struct, work_func_t and lock-class-key identities must be shared by the
future binding graph; casts between separate type universes are not a substitute.
Native callback-value equality also remains an explicit callback-bearing FFI/CFI
qualification obligation. Distinct native static keys and original operations
do not prove emitted key, diagnostic, allocator or callsite metadata identity.

The separately reviewed completion source supplies all original functions in
the flush/cancel/disable/convenience span. It preserves linked inactive barriers,
color generations and overflow cascade, pool revalidation, full RCU flush,
nested disable depth, pending ownership and IRQ restoration. Stack barriers
remain at their final address; completion and callback paths do not subsequently
read objects whose owners may already release them. BH and PREEMPT_RT waiting
remain distinct, and convenience work uses the original system_percpu_wq target.

This body file is saved unselected. Dispatch/execution imports, remaining native
primitive requests, callback/public symbol ownership and emitted qualification
are still incomplete. Function correspondence is source-review scope, not a
claim that the workqueue can run or that its original tests pass.

Verification scope
------------------

Performed: exact baseline/source hashes, independent source comparisons,
callback and native declaration inventories, and text-diff whitespace checks.
No compiler, formatter, configuration, bindgen, build, probe, original test,
guest or CI execution occurred. The broad source phase remains active; native
qualification and general build fixes are later work.
