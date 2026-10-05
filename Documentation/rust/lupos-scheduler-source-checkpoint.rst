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

The RT bodies archive was recovered separately, but does not contain the
later native interface layer. The deadline early archive contains only
Kconfig and review/oracle context, not later Rust bodies. Neither missing
work nor later private policy fixes are claimed as byte-identical recovery.
Their continuing implementation is separate from this core/fair checkpoint.

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

Existing cpupri and cpudeadline Rust source receives bounded compatibility
repairs: allocation-failure unwind bounds, raw-pointer indexing and shared
field addresses, boolean mask checks, native IRQ flag types, allocation
pointer casts, callback extraction, and ten public C ABI symbol definitions.
The original heap, fallback and barrier algorithms are retained. These
support fragments still require configured native binding/macro and
single-owner integration.

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
Support fragments and missing RT/deadline source remain active work.

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
