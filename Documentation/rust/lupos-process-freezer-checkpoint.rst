System process freezer source checkpoint
========================================

Status and provenance
---------------------

This default-off source proposal reconstructs all five function bodies in the
233-line ``kernel/power/process.c`` at Linux commit
``84c953500b7e02473337375b4ffeb610a6e455fd``. The original C source and tests
are unchanged. No compiler, bindgen, Kconfig/make, kernel, guest, suspend,
sysfs or process-freezing operation was run for this checkpoint.

``CONFIG_RUST_POWER_PROCESS`` depends on ``RUST && FREEZER`` and defaults to
``n``. When selected it replaces the existing built-in ``process.o`` source,
adds ``process-rust.o`` for configured macro/static-inline boundaries, and
uses canonical rust/Makefile bindgen preparation. Disabled selection keeps
the original C owner. The shared rust/Makefile include is a separate proposed
hunk for composition with other translations.

This is an uncompiled, source-reviewed proposal, not runtime acceptance or
proof of native ABI/layout/section correctness. Rust/C parsing, configured
binding generation, actual Kbuild commands, linking and original-C integration
tests remain pending. Independent review of checkpoint 01 identified the two
source defects recorded below. Version 02 corrects both; its narrow-diff review
is pending when that corrected source snapshot is first sealed.

Semantic correspondence
-----------------------

* C lines 23-26: the public ``unsigned int freeze_timeout_msecs`` retains its
  value ``20 * MSEC_PER_SEC``. The final proposal defines it in generated Rust
  as an unmangled, public, mutable ``c_uint`` item. ``MSEC_PER_SEC`` is imported
  from configured C declarations. No extra export or registration is added.
  Reads remain at the original deadline and OOM-disable sites; the existing
  C sysfs accessors in ``kernel/power/main.c`` are unchanged.
* C lines 28-48: the same task-category strings and initial info message
  precede boottime sampling, real volatile jiffies sampling, and configured
  ``msecs_to_jiffies`` conversion. Deadline addition uses unsigned-long
  wrapping arithmetic. Only the kernel-task pass starts workqueue freezing.
* C lines 50-69: every iteration resets the unsigned task count, holds the
  tasklist read lock for the complete nested process/thread traversal, skips
  only the current task or a false ``freeze_task`` result, then unlocks before
  workqueue checks, timeout checks, wakeup checks or sleeping. The original
  ``freeze_task`` implementation continues to own per-task locking and state.
  Workqueue busy contributes one to the unsigned count only in the kernel pass.
  Zero remaining work short-circuits the jiffies read. Strict ``time_after``
  is retained via its real macro, including wraparound and equality behavior.
  A successful/expired pass does not check wakeup afterward.
* C lines 71-83: only a pending wakeup after the count/timeout checks sets
  ``wakeup``. Sleep remains the real ``usleep_range`` inline with unsigned-long
  arguments: 500-1000, 1000-2000, 2000-4000, then 4000-8000 microseconds.
  The initial 1000 is derived from ``USEC_PER_MSEC``; backoff stops at 8000.
* C lines 85-110: final boottime sampling, signed ktime subtraction and
  ``ktime_to_ms`` conversion precede the original narrowing to unsigned int.
  The Rust subtraction uses wrapping semantics consistent with the kernel's
  signed-arithmetic build policy. Failure keeps ``aborted`` versus ``failed``,
  seconds/milliseconds, refusing-task count excluding workqueue busy, and
  workqueue busy's boolean value. Busy workqueues are shown first. The second
  locked task walk occurs on non-wakeup failure or enabled PM debug messages;
  it short-circuits ``current``, ``freezing`` and ``frozen`` exactly before
  ``sched_show_task``. The success message and negative ``-EBUSY`` are restored.
* C lines 121-155: usermode-helper disable failure returns immediately with
  no freezer changes. The real ``PF_SUSPEND_TASK`` bit is set on current.
  The static key increments only if ``pm_freezing`` was false, wakeup state
  clears before ``pm_freezing = true``, and the user-only attempt follows.
  Success advances helper depth to ``UMH_DISABLED``. The original atomic BUG
  remains before OOM handling. Only successful freezing attempts disable the
  OOM killer, with a fresh timeout read and the real signed-long argument and
  bool return signature. Failure sets negative EBUSY and fully thaws before
  returning. ``PF_SUSPEND_TASK`` is 0x80000000 in this source; no handwritten
  copy of that value or of the task layout is used.
* C lines 165-177: the kernel pass sets only ``pm_nosig_freezing``, tries the
  remaining tasks/workqueues, keeps its own atomic BUG site, and on failure
  thaws only kernel threads. It leaves user-space thaw responsibility with
  the caller, as documented by the original API.
* C lines 179-213: current is captured before the begin trace. Static-key
  decrement is conditional on the previous ``pm_freezing``; both PM flags
  clear before enabling the OOM killer. Restart logging, helper depth
  ``UMH_FREEZING`` and workqueue thaw retain their order. Under the tasklist
  read lock, every task gets the original other-task flag warning followed
  unconditionally by ``__thaw_task``. After unlocking, the current-task flag
  warning precedes clearing its bit. Helper enabling, schedule, final log
  and end trace follow in the original order.
* C lines 215-233: only ``pm_nosig_freezing`` clears; the original kernel
  restart message precedes workqueue thaw. The locked full task walk thaws
  only tasks with configured ``PF_KTHREAD``. Schedule and final log remain
  after unlocking. This path does not modify ``pm_freezing``, the static key,
  OOM state, helper depth or current's suspend flag.

Task lifetime, macros and diagnostics
------------------------------------

The private Rust traversal starts from ``next_task(&init_task)``, excludes the
init-task sentinel, and walks each process's ``signal->thread_head`` through
the last thread before advancing the process. A read-lock guard lasts through
the whole double loop and every visit; no visited task pointer escapes a
caller's walk. It does not acquire extra references or allocate. Only real
thread nodes are converted to task pointers, avoiding a Rust pointer fabricated
from the list-head sentinel. Configured bindgen layouts and C-derived size,
alignment and member-offset assertions cover the task/signal fields used.

The C companion exposes ``next_task``, ``READ_ONCE`` next-link loads,
``list_entry`` conversion and the exact ``__list_check_rcu`` condition used by
``for_each_thread``. Four distinct diagnostic adapters preserve the four
original macro sites and their independent ``RCU_LOCKDEP_WARN``/``__warned``
states. A private Rust walk-kind argument selects the corresponding adapter
for freezing, failure diagnostics, full thaw or kernel-only thaw. The shared
Rust traversal invokes that adapter once at the start of each process's
thread list, just as the original macro does. Checkpoint 01 incorrectly merged
these four diagnostic states; the separate version 02 correction restores
them without moving any traversal or sentinel decisions into C. This is
equivalent under the retained tasklist read lock, which protects list and
signal lifetimes. There is no C visitor loop or original-C algorithm fallback.

``freezing``, static-key operations, ``current``, time conversion, delay,
usermode-helper enabling and configuration-dependent PM operations use their
real headers. This includes ``FREEZER=y, PM_SLEEP=n`` from cgroup1 freezer,
where PM wakeup APIs are inlines, and PM debug disabled, where the debug state
is a macro. No external symbol is invented for those cases.

Both BUG macro sites, both WARN expressions, seven original printk literal
formats/levels and both ``TPS("thaw_processes")`` trace call sites remain in
the C macro adapter. Rust preserves call order and predicate short-circuiting.
Warnings/trace metadata retain their function but refer to the new adapter
source locations; identical file/line addresses or stack frames are not
claimed. No new asynchronous callback or callback registration is introduced.
The four public entry points retain unmangled C ABI names and int/void shapes;
native symbol/CFI/BTF checks remain required.

Separate writable-state ownership proposal
-----------------------------------------

The base body patch preserves the data definition in C solely to use actual
architecture-defined ``__read_mostly`` placement. That intermediate patch has
a disclosed writable-state ownership gap. It must not be described as a fully
Rust-owned provider. The separate state-owner patch removes that definition.
The supplied final source tree includes both patches.

In the state-owner proposal, ``process-state.h`` includes the same configured
header input and emits a marked expansion of ``__read_mostly``. A local
kernel/power Kbuild rule preprocesses it with ``$(CC) $(c_flags)`` and the C
adapter's built-in target identity, then runs ``process-state.awk``. The decoder
accepts exactly an empty annotation or a single canonical section attribute
with a simple section name. These are the forms in the inspected cache.h
sources. It does not contain an architecture list, assume that every target
uses the read-mostly section, or drop unknown annotation tokens. Explicit
alignment and all other attributes are unsupported and fail generation.

For the accepted forms, C has the natural unsigned-int alignment. Rust's
``c_uint`` size and alignment are asserted against configured C declarations.
When a section is present, its exact string becomes ``#[link_section]``;
otherwise no section attribute is added. The emitted item is ``#[no_mangle]``
and initializes the same public mutable scalar to ``20 * MSEC_PER_SEC``.
Generation rejects missing/duplicate marker records and empty preprocessing.
The recipe replaces the target only after successful preprocessing/decoding.
Version 02 uses sequential commands under Kbuild's ``set -e`` so failure cannot
be masked by the ``fixdep``/saved-command tail appended by ``if_changed_dep``.
Checkpoint 01's AND-list could preserve an old target yet incorrectly update
the saved command; the version 02 correction removes that failure mode.

``process.rs`` includes ``OBJTREE/kernel/power/process-state-generated.rs``.
Each Rust object/assembly/IR/expanded-source/private-listing rule explicitly
depends on it, preserving ``process.rs`` as that rule's first prerequisite.
The generator lists its header, adapter header, AWK decoder and Makefile as
prerequisites and uses ``if_changed_dep`` for configured header dependencies.
Generated/temporary outputs are cleanable after deselection. Bindings remain
under canonical rust/Makefile preparation. Actual dependency/no-op/provider
transition behavior is unexecuted and must be checked natively.

Text-only generator controls pass: 3 accepted records, 11 rejected records,
and preservation of an existing output when decoding fails. They verify exact
Rust text for empty and section attributes and rejection of absent/truncated/
duplicate records, unexpanded macros, unknown or combined attributes, explicit
alignment, injected trailing declarations, whitespace inside a section string,
and split identifiers. Supplemental version 02 shell controls model the
appended Kbuild tail with an existing depfile and saved command. Preprocessing
failure and real decoder rejection both exit nonzero, preserve the old target,
and leave the old saved command intact; a success control reaches the tail.
These are not configured-preprocessor, actual make or Rust tests.

Original-C qualification route and limitations
----------------------------------------------

No maintained in-tree direct C unit suite for these five functions was found
in the inspected tools/testing, kernel/power or lib sources. Keep the existing
C callers and tests, with independent C-provider and Rust-provider outputs
using otherwise identical source, configurations and tools.

The maintained original-C integration entry point is
``tools/testing/selftests/breakpoints/step_after_suspend_test.c``, built by its
unchanged Makefile and selftests/lib.mk C rule. Its default execution performs
a suspend/resume and checks that suspend's success counter increased, then
checks ptrace single stepping on every allowed CPU. ``-n`` omits suspend and
therefore does not exercise this provider. Use a separately authorized isolated
guest with ``RUST``, ``PM``, ``SUSPEND``, ``SUSPEND_FREEZER``, resulting
``PM_SLEEP``/``FREEZER``, ``SYSFS``, a supported suspend state, alarm-clock
timerfd/RTC wakeup, the necessary capabilities and ptrace support. Count skips
and unsupported suspend as gaps, never passes.

The retained ``tools/testing/selftests/timers/alarmtimer-suspend.c`` provides
additional RTC/alarmtimer suspend integration. Its original Makefile marks it
as a destructive extended test. Build it through that C rule, not its adjacent
Rust translation. Its exit code alone is insufficient: unsupported clocks and
suspend command failures may break loops without setting its final failure
variable. Require the full intended clock/alarm/suspend iterations, actual
increased suspend counters and console evidence in both provider runs. Do not
change the original test merely to obtain a passing result.

The maintained original-C PM test mode documented in
``Documentation/power/basic-pm-debugging.rst`` offers a freezer-stage integration
route with ``PM_DEBUG``/``PM_SLEEP_DEBUG`` and suitable suspend/hibernation
configuration. It is an invasive guest-only future qualification route, not
an action run or authorized by this source checkpoint. The cgroup2
``tools/testing/selftests/cgroup/test_freezer.c`` suite exercises its separate
job-control freezer and cannot establish PM-freezer acceptance.

Success-path integration does not cover helper-disable failure, blocked
workqueues, refusal timeout, wrap/equality at jiffies expiry, wakeup-abort
ordering, OOM-disable failure, diagnostic predicates or every failure unwind.
Those still require separately approved original-C differential/fault coverage
and concurrency/lifetime checks. Build lanes must include PM_SLEEP disabled
where FREEZER remains enabled, PM debug on/off, lockdep/PROVE_RCU_LIST on/off,
JUMP_LABEL alternatives and supported preemption modes. State-owner acceptance
also needs a native lane for each accepted annotation form, with ELF/object
proof of unique symbol owner, exact initialized value, unsigned-int size,
alignment and section against original C. Kconfig defaults/dependencies,
C-to-Rust-to-C selection, prepare from missing bindings, changed-header and
generator rebuilds, genuine no-ops, inspection products and all native
ABI/CFI/BTF/link checks remain pending.
