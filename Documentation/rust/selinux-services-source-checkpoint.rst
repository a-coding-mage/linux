SELinux services source checkpoint
=================================

The default-off ``CONFIG_RUST_SELINUX_SERVICES`` provider reconstructs the
87 function bodies in ``security/selinux/ss/services.c`` at source base
``298abda59725d491c10be678618ac76300d1aba1``. The retained C implementation,
original headers and original tests are unchanged.

``services.rs`` includes four Rust fragments: access decisions and
constraints (22 bodies), SID/context conversion (21), policy lifecycle and
object labeling (20), and queries/audit/NetLabel/serialization (24).
Configured bindgen and ``kernel::ffi`` replace handwritten ABI layouts.
Three C adapters contain only existing header macros/inlines, scalar
projections and fixed diagnostics. No original services algorithm is
called through a same-unit C fallback.

The proposal retains original authorization/enforcement conditions,
constraint guards, kernel versus userspace class mapping, stale-SID
retries, ownership/unwind paths, policy freeze/grace periods, and optional
``CONFIG_NETLABEL`` branches. Separate C macro sites preserve all 27
ordinary and 6 protected RCU policy reads. C-derived layout assertions are
included as build obligations, not executed evidence. The local audit
permission-name scratch array initializes unclaimed slots to null so the
existing ``????`` diagnostic fallback is deterministic; populated entries
and access decisions retain their original behavior.

Kbuild keeps the original ``ss/services.o`` composite slot and selects
Rust only under the explicit option; otherwise it uses untouched C.
The binding fragment invokes the original SELinux generated-header recipe
before configured bindgen. Object, assembly, LLVM IR and expanded-Rust
recipes are explicit. No Rust ``.lst`` recipe is supplied in this source
checkpoint; a future listing request requires separate integration.

Validation remains incomplete
-----------------------------

Source inventory matches all 87 definitions and cross-review corrected two
binding/type issues. No configured bindgen, C/Rust compilation, layout
assertion execution, object/symbol ownership check, kernel build, original
C tests or runtime comparison has run for this provider. None of those
stages is claimed passing. Do not treat this checkpoint as a validated
replacement or enable it on a production system.

The unchanged original ``tools/testing/selftests/lsm`` C programs and their
Makefile/lib.mk recipes offer surrounding LSM inventory and attribute
checks. They do not prove policy reload, constraints, MLS, audit, NetLabel,
labeling or serialization parity. Dedicated acceptance still requires
unchanged maintained original C SELinux tests and an explicitly authorized
isolated policy environment. No translated tests were substituted, and no
security settings, policy, host/guest or network operation was performed.
