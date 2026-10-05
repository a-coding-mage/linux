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
Configured trace ABI, generated field/layout/CFI, HW/IRQ alternatives,
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

The follow-up PRINTK_INDEX source proposal uses real native pi_entry records
with literal formats, original owner/line keys, configuration scopes, duplicate
group-path sites and native sizeof branches. All 165 Rust keys match its native
manifest by independent source review; seq-only calls have no index records.
The owner algorithms and output-call grouping remain unchanged.

Exact original __FILE__ spelling remains externally required with a hard error,
and macro-expanded provenance, object retention/emission and metadata parity
are unqualified. No path, layout or protection exception is fabricated.
Default-off BROKEN and unconditional source errors retain the hold. Configured
ABI/CFI, variadic widths, instrumentation/caller metadata, locks/RCU,
debugfs/VFS lifetime and runtime behavior also remain unqualified.

Cgroup CPU accounting source checkpoint
--------------------------------------

The 20 existing cpuacct Rust functions retain their accounting decisions with
native per-CPU root storage, 32-bit runqueue/IRQ lock pairs, allocation unwind,
eleven C-ABI callbacks and original cgroup descriptor metadata. Independent
source review checks user/system totals, reset fields, root/ancestor behavior,
wrapping arithmetic, native enum and formatting boundaries.

Default-off BROKEN and hard-error hooks retain the original utility owner.
Generated callback/enum ABI and CFI, native context/protection metadata,
cross-language shared-counter access, per-CPU concurrency, performance and
original-test behavior remain unqualified. The separate cputime owner is intact.

CPU isolation source checkpoint
-------------------------------

The existing 14 housekeeping bodies now preserve boot flag/CPU parsing,
NO_HZ_FULL rejection, present-CPU checks, domain updates and late memblock
reclamation. Native enum, static-key, RCU and ordinary/READ_ONCE/WRITE_ONCE
boundaries replace guessed Rust storage. Native scratch masks preserve both
configured cpumask_var_t representations without moving inline storage.

Independent caller review corrects the update safety contract: cpuset's top
mutex is held while CPU hotplug may proceed. The inherited unknown-final-flag
parser can advance past its terminating NUL; its buffer-domain safety remains
unproved and blocks admission. Boot static-key and partial-init allocation
caveats are likewise retained, without runtime or security-impact claims.
Default-off BROKEN and source errors remain, and native enum ABI/CFI, caller
metadata, RCU/lock lifetime, instrumentation and runtime qualification stay open.

Conditional runqueue task-field source repair
--------------------------------------------

FAIR and RT no longer assume that rq.curr and rq.donor are direct generated
Rust fields when SCHED_PROXY_EXEC is disabled. Five native field primitives
preserve 37 ordinary reads and one FAIR READ_ONCE site. Independent review
checks all 29 diff hunks, repeated evaluations and short-circuit placement.
Existing RCU and RT READ_ONCE sites and distinct cfs_rq.curr fields are intact.
This source correction does not qualify configured ABI/CFI, emitted load or
instrumentation behavior, concurrency, performance or native owner admission.

Scheduler statistics source checkpoint
-------------------------------------

The eight existing statistics bodies now use native types and fields, preserve
signed sleep/block accounting and latency narrowing, restore the version
newline and complete domain output, and retain native seq/proc/init callbacks.
Independent review checks seven format strings, domain ordering and iterator
bounds, and preserves native RCU acquire/release context annotations.

The original SCHEDSTATS utility owner and stats_header.rs consumers are intact.
Default-off BROKEN and selected-owner hard errors remain. Generated ABI/CFI,
native context and protection metadata, cross-language locking, configured
emission and original-test/runtime behavior remain unqualified.

Scheduler CPUFreq and schedutil source checkpoint
------------------------------------------------

The existing hook and governor owners now test the stored per-CPU hook,
preserve nullable callback validation through a native ABI adapter, restore
worker scheduling attributes/affinity, tunable references and failure unwind,
select all three update hooks, and retain cancellation and slow-limit ordering.
Native header types replace invented policy/storage layouts. Independent review
corrects CPU iteration to the original find_next_bit/small_cpumask_bits contract.

Two default-off BROKEN selectors and hard errors preserve original utility
ownership. Generated layout/callback CFI, worker/kobject/RCU lifetime, native
allocation/caller and protection metadata, configured emission, concurrency
and runtime qualification remain open. Generic CPUFreq drivers are unchanged.

Core-cookie and stop-task source checkpoint
------------------------------------------

The eleven existing core-cookie functions retain reference/permission ordering,
raw uninitialized lock-state storage, two-pass PID/thread traversal, native
usercopy and corrected force-idle masks and arithmetic. Source review preserves
four distinct RCU diagnostic sites and the native unsigned CPU-mask comparison.
The force-idle fallback reads core_pick once and accesses rq.curr through a
native plain-field leaf across both proxy-execution layouts.
The thirteen stop-task callbacks retain C ABI and native DEFINE_SCHED_CLASS
registration, including const storage, linker section and the shared affinity
callback. Neither owner duplicates the separately reviewed core.c family.

Separate default-off BROKEN selectors and hard errors retain original utility
ownership. Generated enum/callback ABI and CFI, diagnostics/caller attribution,
refcount/lock/usercopy and compiler protection policy, symbol ownership and
original-test/runtime qualification remain open.

Core task-group RCU callback parameters now name the native callback_head
instead of presuming a Rust type for the C-only rcu_head macro. Source readback
preserves both callback bodies, registrations and their two grace periods.

Scheduler topology source checkpoint
-----------------------------------

The existing topology implementation is continued in eight Rust families:
flags/debug, energy scheduling, root-domain lifetime, cache scheduling, groups,
domain initialization, NUMA and allocation/build/partition. Native headers own
layouts, flag-table expansion, per-CPU storage and metadata. Source inventories
cover 34 original public functions and 23 original public storage names;
337 native interface definitions remain real, unqualified C runtime work.

Independent family review corrects anonymous unions and callback type names,
configured static-inline boundaries, native CPU/node iterator and integer
semantics, constant-NULL RCU clearing and bitmap cleanup. Caller-specific
unsafe contracts preserve ordinary diagnostics without allowances. Original
partial-allocation and unchecked boot/fallback limitations remain explicit
qualification risks rather than newly invented recovery behavior.

Default-off BROKEN and root/native hard errors retain original utility
ownership. Configured binding/layout/CFI, init/weak-alias/registration metadata,
allocation/refcount/RCU/IRQ lifetime, protection/caller attribution, all-config
emission and original-test/runtime behavior remain unqualified.

Sched_ext CID source checkpoint
-------------------------------

The existing CID owner is continued with topology construction, sparse CPU
slots, shard ranges, allocation/unwind and embedded-node RCU reclamation. Arena
override validation uses immutable snapshots before mutation; mask/reference
operations and four native BPF adapters retain their source contracts.
Independent review corrects callback_head naming, exact generic/possible-CPU
iteration and original unsigned DIV_ROUND_UP arithmetic. Source inventory
contains all 23 original public entry points, without runtime acceptance.

Default-off BROKEN and unconditional source errors preserve original ownership.
CID BTF ID sets depend on the shared ext/build_policy translation unit; no
standalone native object is selected. Shared ext algorithms/storage, configured
binding/CFI, BPF arena rebasing/fault recovery, metadata/protection, allocation
and RCU lifetime, concurrency and original-test behavior remain unqualified.

Sched_ext arena source checkpoint
---------------------------------

The six existing arena bodies now use native gen_pool/chunk layouts and the
actual inline flexible bitmap, exact bit-range iteration, native page geometry,
callback ABI and kernel-address rebasing. Independent source review preserves
growth, registration-failure rollback and pool teardown ordering.

Caller quiescence remains essential: native pool destruction supplies no RCU
grace period. BPF rollback can defer cleanup or retain pages until map teardown,
and scratch-fault recovery needs an applicable BPF program on the stack. Pool
bookkeeping does not grant Rust reference validity for shared arena payload.
Default-off BROKEN and hard errors remain; shared translation-unit ownership,
ABI/CFI, native protection, lifetime and original-test behavior are unqualified.

Sched_ext idle source checkpoint
-------------------------------

The existing idle owner now contains selection, topology, allocation/reset,
notification/renotify and validation behavior with all 21 original public
entries represented. Native BPF adapters preserve fourteen continuations and
five ordered registrations. Source review checks exact CPU/node iteration,
unsigned node counts and lazy native debug argument evaluation.

An inherited compatibility-selection path can call lock-requiring task-scheduler
resolution during child rejection before taking pi_lock. This unresolved
caller/lockdep obligation is retained, not waived or claimed safe. Default-off
BROKEN and source errors remain; shared BTF/type/linkage, callback ABI/CFI,
native protection, pinning/IRQ/RCU lifetime and runtime qualification stay open.

Sched_ext sub-scheduler source checkpoint
----------------------------------------

The existing sub-scheduler owner is continued through tree/shard allocation,
rescue and rejection, capability propagation and BPF access, and enable/disable
transactions with task/cgroup migration. All 68 inventoried original identities
have source bodies; the four original disabled fallbacks remain byte-identical.
Two independent source reviews cover all families, including all sixteen
lifecycle functions and their failure, reference and IRQ/RCU ordering.

Review restores the native RCU-list entry diagnostic and separates original
lockdep, error and cgroup-op callsites. Same-unit BTF/context visibility, shared
canonical binding types and storage, callback/CFI and stack behavior, native
metadata/protection attribution and arena/concurrency semantics remain open.
Native leaves remain explicit runtime C boundaries, and ext-owned dispatch
helpers remain ext dependencies. No original owner is replaced by these
default-off BROKEN proposals; unconditional source and selection errors remain.

Sched_ext core foundation source checkpoint
------------------------------------------

This partial continuation preserves the existing unsigned-order helper and
repairs CPU validation and protected task-slice behavior. It supplies common
state/parameter boundaries, cursor traversal, slice-OOB accounting and current
task accounting. Native storage and private layouts retain their original
definitions. Independent review covers these two bounded source families.
The subsequent task, cgroup, object, exit/dump, enqueue/consume and bypass
increments are separately reviewed; nine core families remain outside these checkpoints.

Two current-task reads now use an exact plain native accessor for configured
rq union access. The native source permits competing slice/vtime writers in
some cases; that contract remains an unresolved Rust memory-model obligation,
not an assumption of exclusive access. Native varargs formatting, remaining
owner algorithms, canonical cross-family types, shared BTF/CFI/initialization,
source attribution, compiler protections and runtime behavior remain open.
Default-off BROKEN and unconditional selection/source holds remain unchanged.

Sched_ext task-lifetime source checkpoint
----------------------------------------

The task-lifetime increment supplies all 35 original function identities,
including the two original no-op class callbacks. State transitions, iteration,
init/enable/exit, fork/death, TID allocation and scheduling-class transitions
retain their source ordering and configuration alternatives.

Independent review corrects two native scoped IRQ guards to preserve this
tree's reference-counted interrupt semantics. Integration review restores
task_rq expression evaluation inside the normal and cancelled-exit callback
envelopes. Three supported raw concurrent-read cases remain explicit Rust
memory-model blockers; ABI/CFI, stack/context analysis, native diagnostics and
all runtime qualification remain open. The held enqueue-family dependency is
not supplied by a C fallback. All selection/source errors remain.

Sched_ext cgroup source checkpoint
---------------------------------

All seventeen cgroup identities now have independently reviewed source bodies,
including their native configuration alternatives, migration preparation and
rollback, parent-directed controls and root group initialization/exit.
Native rwsem and shared flag storage retain one owner each.

Review restores callback field/helper evaluation inside all original native
macro sites, including repeated task_rq evaluation during task movement. A typed
adapter reaches the single Rust task-group helper at the original callback
argument point. Cross-frame context, canonical native types, ABI/CFI/BTF,
diagnostic attribution and runtime behavior remain unqualified. Default-off
BROKEN and all selection/source hard errors remain.

Sched_ext object-lifetime source checkpoint
------------------------------------------

The reviewed object-lifetime increment covers DSQ creation/destruction, arena
scratch, sysfs, exit-info allocation, hierarchy publication and complete
scheduler allocation/unwind and RCU-work teardown. Native storage and callback
metadata remain single-owned, with explicit cross-family dependencies.

Review corrects lockdep initializer spelling, exact possible-CPU iteration,
cleanup predicates and native context annotations. Event output retains the
original native formatting expansion and integer accumulator, without the
candidate's synthetic BUG fallback; that formatting remains native runtime
work. The subsequent exit/dump increment supplies its callback declarations;
the subsequent bypass increment supplies its timer callback. Watchdog callbacks
and event aggregation remain pending. Native ABI,
CFI/BTF, allocation attribution, lifetime/RCU and protection qualification are
still open, with all default-off BROKEN and hard source/selection holds intact.

Sched_ext exit/dump source checkpoint
------------------------------------

All twenty-nine mapped exit/dump functions and event-format expansion have
independently reviewed source continuations. Claim-before-format, independent
native argument lists, propagation/disable ordering, CPU/task dumps and packed
BPF formatting retain their original owner boundaries and error paths.

Review repairs warning expressions, private format annotations, native guard
scope, single-CPU iteration and separate checked-CPU sites. Seven printk index
records now retain original source-site keys with native metadata types and
runtime formatting targets. The true build-dependent original __FILE__ value
is still required behind an additional PRINTK_INDEX hard error; no guessed
path, emitted-record result or protection equivalence is claimed.

Root teardown/event aggregation, off-context dump-state concurrent reads,
native varargs/CFI/NMI/stack behavior, source attribution and runtime semantics
remain unqualified. All default-off BROKEN and source/selection holds remain.

Sched_ext enqueue/consume source checkpoint
------------------------------------------

The enqueue and consume increments supply twenty-six and twelve original
function bodies, respectively. Reviews cover DSQ/state transitions, direct
dispatch, rq transfer and custody rechecks, migration and dispatch-buffer
ordering. The task-lifetime clear_direct_dispatch dependency is now supplied
by its actual Rust owner; deferred-work callbacks remain pending.

Review restores original warning expressions and native operand evaluation,
and routes set_task_cpu through its configured SMP/UP native declaration.
The original build_policy branch-profile-disable policy is retained; no repair
of suppressed profile records is claimed, and unnecessary branch-only callback
layers were removed. Native ABI/CFI, memory ordering, concurrency, allocation
lifetime, optimizer hints and source/protection attribution remain unqualified.
All source/selection errors and default-off BROKEN gating remain.

Sched_ext bypass source checkpoint
---------------------------------

All nine bypass owners now have independently reviewed source bodies, including
load balancing, timer work and nested scheduler bypass/unbypass transitions.
The native cursor frame stays address-stable across rq lock drops, and the
original IRQ APIs, empty sched_change scope and repeated knob/time expression
evaluation remain explicit.

Review repairs native-word slice arithmetic and reverse-list predecessor fetch
ordering. The recovered pinned trace header confirms parameter order and native
argument evaluation, closing a source-availability gap without claiming emitted
trace equivalence. The object-lifetime timer callback dependency is now supplied
by its actual owner. All native ABI/CFI, concurrency/lifetime, stack/context,
instrumentation and runtime gates remain open, with unchanged source/selection
errors and default-off BROKEN admission.

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
