.. SPDX-License-Identifier: GPL-2.0

Lupos scheduler source recovery checkpoint
==========================================

This is an incomplete, inactive source checkpoint. It is not a configured
build, ABI qualification, runtime result, or claim that Linux tests pass.
The existing Rust implementation remains the editable starting point;
original C, native header oracles, and original tests remain unchanged.

Recovery authority
------------------

The recovered repository baseline is
``126a30fae3bba11420ec2fcbde51a0a01bab1b5b``. Its original scheduler C files
match the archived semantic authority
``e1d84f501551943a11f4c5271e9f5c85d7e15168``:

* core.c: ``c8b8f7a75dd209a94335fd74f738d6f1f63e253f8eabac7d7eb111f24ea9e2d2``
* fair.c: ``b79a0a6c7ba8fb270678533ac39b61466439252f93580bc43288063cef422512``
* rt.c: ``ce8e59a2652ade9c4c20033fd6be906c35f8e2d23ad19548e78c7ef848388802``
* deadline.c: ``9853e5f322ba0892406701ef7500d0bcf8fc85a0b813d3addd3cdf0b185133a6``

Core source was recovered from core-composed-source-v3.tar.gz, SHA256
``c6095452e327338466d0a5b468ea70bf27d7e5e995a6cdb88a5654aabf9635cc``;
all 45 source-manifest entries matched. Fair source was recovered from
fair-composed-review-002.tar.gz, SHA256
``1c9d85c8dce3264ce9e9ec6a05c285f7fb31f44f557d6078f7364337d9c3f89a``;
its complete file manifest matched, including corrections F001 through F015.
Complete foundation and tail backups were separately recovered.

The RT bodies archive was recovered separately and its missing native interface
layer has been newly supplied and independently source-reviewed. The deadline
early archive contains only Kconfig and review/oracle context, not later Rust
bodies. Missing work and later private policy fixes are never described as
byte-identical recovery.

Source repairs after recovery
-----------------------------

Core now re-reads rq->core across scheduler class callbacks that can drop
the runqueue lock. CPU hotplug can move the leader and its in-flight state;
the former cached leader would address the previous runqueue. Initial
clock-state sampling now matches C before prev_balance. The root gains
no_std, missing def_root_domain/calc_load_tasks binding allowlist entries
are restored, and the native HK_TYPE_KERNEL_NOISE constant is visible for
the unconditional sched_tick use.

Core also corrects callback/predicate C truth conversions, native CPU argument
types and the lazy-preempt configuration fallback. Fifteen independently
reviewed selection/core-hotplug bodies gain explicit unsafe scopes; removing
only these added scopes recovers the previous source bytes exactly. The
explicit no_std attribute is redundant under Kbuild's existing crate flag,
not evidence of a demonstrated compilation failure.

Core and fair generated declarations now live in cfg-gated modules in the
existing bindings crate. Only generated code inherits that crate's existing
diagnostic policy. The handwritten owners gain no warning exemption.
Generated-file dependencies of bindings.o are conditional on each owner.

Fair gains explicit unsafe scopes for 252 foundation and 187 tail bodies.
Independent whole-file comparison removed only the newly added outer scopes,
their safety comments and indentation; both files then matched the recovered
inputs byte-for-byte. All previous statements, control flow, literal text
and comments were preserved by that scope repair. This is source-change
verification, not proof of the unsafe preconditions or runtime safety.

Fair READ_ONCE/WRITE_ONCE sites now use typed native macro leaves instead
of Rust volatile operations. The native configured access expansion remains
authoritative. Callsite/instrumentation identity still requires qualification.
An unused generic allocation leaf is removed, and allocation-site comments
no longer claim unverified profiling/caller identity.

Further fair repairs retain native disabled-config inline/macro alternatives,
express C integer truth and signed/unsigned conversions explicitly, preserve
word-size promotion in util-est arithmetic, and correct cpufreq flags and the
deadline-server callback type declaration. These have independent source
review against pinned headers; emitted ABI/CFI behavior is still unqualified.

A further bounded core review adds explicit scopes to 24 clock/wake/statistics
and change-context bodies. Scope-only readback preserves their previous source;
the separately reviewed prefetch fix computes a nullable hint address with
native-derived offset_of and wrapping byte arithmetic, avoiding an in-bounds
Rust field projection through a possibly null CFS current pointer.

Further fair interface corrections match signed sched_clock_cpu arguments,
signed server runtime and exec-max accounting, native thread-count/folio-size
signatures, and word-width timestamp/balance-interval conversions. These are
source repairs only, with no change to the inactive admission policy.

Existing cpupri and cpudeadline Rust source receives bounded compatibility
repairs: allocation-failure unwind bounds, raw-pointer indexing and shared
field addresses, boolean mask checks, native IRQ flag types, allocation
pointer casts, callback extraction, and ten public C ABI symbol definitions.
The original heap, fallback and barrier algorithms are retained. These
support sources now have a shared configured binding input and sixty distinct
native primitive boundaries. Both CPUMASK_OFFSTACK representations retain their
native field allocation/free/decay, and CPU iteration preserves configured
limits and the original one-CPU alternative. Independent source comparison
preserves all nineteen Rust function bodies' statement text across explicit
unsafe scopes. Both roots import the existing bindings crate; no new generated
or handwritten diagnostic allowance is added.

Support admission remains blocked by BROKEN and hard errors in source make
fragments. No aggregate C inclusion or replacement-object selection changes.
Single ownership, native policy, nullable callback KCFI and cross-language
shared-memory/lifetime semantics still need later qualification.

The core affinity/TTWU increment captures the pending-refcount address before
its final release, preserves the original native !SMP set_task_cpu alternative,
and makes 51 reviewed unsafe scopes explicit. Exact de-scoping preserves prior
bodies; two existing cfg alternatives remain unchanged when disabled. These
repairs do not admit the owner or establish callback/native policy equivalence.

RT source checkpoint
--------------------

The five recovered RT body modules remain the implementation baseline. Native
header/macro declarations, private storage, class/sysctl metadata and configured
binding inputs are newly supplied. Source inventories find all 119 distinct
original C function names and all 172 referenced native boundaries, consisting
of 143 header/macro leaves, 28 original diagnostic sites and registration.
These counts establish source presence only.

Independent source corrections restore private static-inline helper linkage,
route move_queued_task_locked through its actual header operation, preserve
CPU-mask advancement after runqueue unlock and at original continue points,
and fix configured CPU limits and argument types. The 125 unsafe functions
now have explicit regions and caller contracts; independent token comparison
preserved every pre-scope body. Thirteen genuine disabled-config alternatives
keep the unsafe region inside their enabled configuration only.

RT generated declarations use the existing bindings crate policy. The hidden
default-off selector and hard errors in both RT make fragments remain intact.
No original build_policy.c inclusion is changed, no replacement object is
selected, and no original RT scheduling algorithm is used as a new C fallback.
Generated ABI, shared-memory/locking/RCU semantics, callback CFI and native
protection/caller/registration policy remain unqualified.

Deadline source checkpoint
--------------------------

The existing deadline Rust skeleton has been repaired and its missing owner
bodies filled incrementally. All 151 distinct original names now have Rust
bodies: eight retained baseline helpers and 143 newly supplied missing bodies.
The 23 class callbacks, per-CPU storage, native primitives and registration
declarations are present. A source inventory matches 234 native reference names
with declaration records; it is not generated ABI or link evidence.

Separate source audits cover the bandwidth/entity/lifecycle and EDF/runqueue/
migration paths against the pinned original C. They found a missing init cold
attribute and a contract comment that omitted the supported null console
destination; both are corrected. No additional concrete control-flow mismatch
was identified within those reviewed paths, which does not prove equivalence.
Retained-body scope edits have independent token/literal-preservation evidence.

Generated declarations remain in the existing bindings crate. The default-off
BROKEN selector and explicit make errors block activation. Original aggregate
C inclusions remain unchanged. Generated bitfield accessors still require
alias/lifetime qualification around embedded concurrently managed hrtimers;
callback CFI, memory ordering, lock context, instrumentation, warning/caller
attribution, single ownership and runtime behavior remain unqualified.

The additional core queue/fork/switch increment adds 55 reviewed unsafe scopes
and separates six existing cfg alternatives. Exact de-scoping and cfg-body
reconstruction preserve prior body text; context-switch ABI remains an open
native gate.

Pressure-stall source checkpoint
--------------------------------

The existing PSI skeleton's missing aggregation, task-state, IRQ, cgroup,
trigger and procfs behavior is now supplied in Rust. Invented zero-sized
layouts are removed in favor of native-header declarations. The previous
ticks-versus-nanoseconds initialization mismatch is corrected. Native storage,
header primitives, formatting and registration remain explicit runtime
boundaries; no original psi.c body is included or forwarded.

Source review covers seqcount snapshots, weighted aggregation and delayed
averages, trigger thresholds and file-credential capability checks, worker
publication, RCU teardown, poll events, nested memstall, cgroup migration and
task/common-ancestor accounting. Corrections preserve the native one-CPU
iteration alternative, ordinary scalar loads inside unchanged seqcount
barriers, and unsigned-long wrap before widening in missed-period arithmetic.
The native init cold hints are retained with their init-text sections.

PSI's Rust and native C files retain unconditional hard errors. Default-off
BROKEN wiring, binding-input registration and callback declaration metadata
are source proposals only, with no object selection. Configured binding and
enum/callback ABI/CFI, Rust/C concurrency and aliasing, instrumentation,
source/caller attribution, and build/runtime behavior remain unqualified.

A further core foundation pass adds 78 reviewed explicit scopes and preserves
ten existing cfg alternatives. Native unsigned-int NUMA timestamp assignment
is corrected without widening. Scope/cfg reconstruction retains prior body
text; none of these source checks qualifies the native owner.

CPU-accounting source checkpoint
--------------------------------

The existing cputime Rust implementation now uses native per-CPU storage and
header-derived interfaces. Missing IRQ tick, generic/native vtime, NO_HZ,
thread-group iteration and remote cpustat-reader behavior is supplied. Source
repairs preserve threshold flush/reset, seqcount-validated state, lockless then
IRQ-save retry, locked 32-bit runtime reads and output-alias update order.

Independent review removes an architecture-idle dependency absent on the
original s390 path and restores the native PROVE_RCU_LIST traversal diagnostic.
Thirty-five public C function spellings have Rust definitions and 76 helper
names have native declarations/definitions; these are text inventories only.
Configured types, enum/callback CFI, memory ordering, instrumentation and
build_policy ownership remain unqualified. Default-off BROKEN hooks and
source-stage errors retain the admission hold and original C selection.

Additional core source review corrects signed SCX-policy and configured
cpuhp_tasks_frozen/in_lock_functions boundaries. Scheduling/preemption/hotplug,
init/proxy/MM-CID bodies receive explicit reviewed unsafe scopes with exact
source readback, without admitting the native owner or altering its policies.

Waiting and completion source checkpoint
---------------------------------------

The four existing completion/swait/wait/wait_bit Rust owners now use actual
native header types/constants and preserve their 74 function names and 58
native export records. Source repairs replace invented layouts and member
offsets, restore list initialization and traversal, native state/lock checks,
waiter lifetime, callback handling and kernel-specific FFI integer types.
Thirteen original task-state macro sites have separate native expansions;
this preserves distinct site state without qualifying caller/IP attribution.

Independent review corrects raw self-linked temporary-list provenance, signed
count wrapping, the declared nullable bit-action domain, and plain-versus-
READ_ONCE scalar boundaries for shared completion/flags. Rust retains all
decisions and lock order. Generated nullable representation, outer callback
KCFI, cross-language memory model, source/caller attribution and native
instrumentation remain unqualified. Rust/C unconditional hard errors and
default-off BROKEN hooks remain; original objects are not replaced.

The final bounded core groups/bandwidth/debug review corrects an unsigned
cgroup parser argument and makes existing unsafe operations explicit. The
clock updater uses cfg-local declarations for all eight IRQ/paravirt/averaging
combinations, with exact prior-source reconstruction and unchanged accounting
operations. These are source checks, not native or configured acceptance.

Scheduler syscall source checkpoint
----------------------------------

The existing syscall skeleton is continued with its missing permission,
credential, RLIMIT, LSM, affinity/cpuset, deadline admission, PI/uclamp,
usercopy/versioning, yield and RR behavior. Native registration supplies the
original 14 syscall sites and seven exports. Source inventory matches 113
native declarations/definitions; this is not generated ABI or link evidence.

Independent source review covers validation/permission ordering, user-memory
access and failure cleanup, including the exact original IRQ guard expansion.
Configured anonymous rq.curr/rq.donor fields are accessed natively instead of
assuming flattened bindings when SCHED_PROXY_EXEC is disabled. Handwritten
naming-lint allowances are removed; generated code keeps only existing policy.

Default-off BROKEN and hard-error hooks retain original aggregate selection.
Configured layouts, nominal callback CFI, CPUSETS/SCHED_PROXY_EXEC alternatives,
syscall attributes, hardened usercopy/object-size provenance, instrumentation,
concurrency, caller attribution and runtime behavior remain unqualified. No
source review claim establishes a running-kernel vulnerability or acceptance.

Global load-average source checkpoint
------------------------------------

The existing load-average Rust implementation now uses native header-derived
types and constants, actual native atomic storage, and explicit primitives for
the original plain, READ_ONCE, WRITE_ONCE and barrier operations. Source review
preserves all eight entry points, fixed-point decay and rounding at native word
width, normal tick folding, NO_HZ double-buffer accounting and catch-up order.
Independent review corrects the kernel-specific C integer aliases.

The original build_utility.c owner remains selected. Default-off BROKEN and
selected-owner hard errors keep this proposal inactive. Native layout/ABI/CFI,
compiler and instrumentation policy, emitted barriers, 32-/64-bit arithmetic,
NO_HZ concurrency and original-test/runtime qualification remain open.

Autogroup source checkpoint
---------------------------

The existing autogroup Rust owner now uses native opaque objects, storage,
reference and lock primitives. Its source preserves allocation-failure warning
paths, RT-group redirection, signal-group migration, fork/exit references,
permission/rate checks and proc output. Independent review corrects the
PRINTK-disabled rate-limit declaration and checks native init adapters.

Default-off BROKEN and hard-error hooks keep original utility ownership.
Generated opaque ABI/CFI, init/instrumentation policy, allocation and diagnostic
attribution, reference/RCU ordering, configuration and runtime qualification
remain open. Native boundary leaves are still real runtime C work.

PELT source checkpoint
----------------------

The 13 existing PELT routines retain their source algorithms with corrected
native-width products, period/field types, eager signal updates and native
WRITE_ONCE boundaries. Independent review corrects kernel FFI aliases and the
non-PROXY_EXEC anonymous donor union access through a native plain field leaf.
Source inventory covers 24 native primitive leaves, not generated ABI evidence.

Default-off BROKEN and hard-error hooks keep the original policy owner active.
Trace declarations, generated field/layout/CFI, configured HW/IRQ alternatives,
compiler/instrumentation policy, numerical differential and concurrent runtime
qualification remain open. No native accounting acceptance is claimed.

Membarrier source checkpoint
----------------------------

The existing membarrier Rust algorithms retain command and registration-state
decisions while restoring native syscall width, mask cleanup, CPU-online checks
under the hotplug lock, configured callback selection and init registration.
Address-stable native mask storage remains borrowed until synchronous IPI
completion. Independent source review checks intent/READY publication, MM/RCU
lifetime, barriers and exact exit ordering, without native execution evidence.

Default-off BROKEN and hard-error hooks preserve the original utility owner.
Configured mask layout, nullable callback and enclosing-function CFI, native
compiler/instrumentation and init policy, allocation attribution, emitted
barriers, hotplug/MM lifetime and original-test behavior remain unqualified.

Scheduler debug source checkpoint
---------------------------------

The existing debug skeleton is continued with feature/control writes, domain
directory management, deadline-server validation, task reset, latency warnings
and task/runqueue/CPU/header output. Native headers replace invented layouts
and feature counts. Independent source review corrects bitfield/anonymous-union
access, signed flags, callback size types, UP iteration and output-call grouping.
The init algorithm retains explicit cold/init-section intent.

A concrete source parity gap remains: generic native formatting records
PRINTK_INDEX wrapper formats and locations instead of the original literal
format/callsite metadata. This instrumentation is not disabled or waived.
Default-off BROKEN and unconditional source errors retain the hold. Configured
ABI/CFI, variadic widths, exact instrumentation/protection and caller metadata,
locks/RCU, debugfs/VFS lifetime and runtime behavior also remain unqualified.

Inactive gates and remaining work
---------------------------------

Core keeps its BROKEN dependency and selection-time error. Fair keeps its
default-off hidden native-policy admission dependency. Neither selector is
enabled by this checkpoint. No original scheduler C implementation is
removed, renamed, included, or forwarded as a new Rust algorithm fallback.

The core source contains 408 original function/accessor names in recovered
Rust bodies. Fair's recovered halves contain their existing source bodies.
These are inventories, not semantic acceptance or native Rust-only ownership
claims. Native leaf code remains real runtime work and an explicit remaining
migration boundary.

Before admission, complete independent body review and configured native
type/layout/field/link checks. Caller return-PC capture, architecture context
switching, per-function tracing/instrumentation/stack/CFI/unwind policy,
allocation/diagnostic attribution and native field access all remain open.
Support interfaces and unqualified scheduler source remain active work.

Verification scope
------------------

Performed: recovered archive/file hashes, unchanged native C/header hashes,
manual source comparisons, exact scope-only FAIR readback, and text diff
whitespace checks. Original C/header/test files and compiler/security
settings are preserved.

Not run for this checkpoint: compiler, preprocessor, bindgen, configuration
generation, object or full builds, native probes, reproducers, guests,
original-C tests, or CI. No formatter/toolchain was installed. The requested
broad source phase remains active; integrated build-fix and original-test
qualification are later gates.
