Lupos Rust recovery and validation checkpoint
===========================================

This branch contains an incomplete migration checkpoint, not a release or a
claim that all Linux tests pass. Earlier raw build/test artifacts became
unavailable when the execution workspace was replaced on 2026-10-02.

Recovered source
----------------

Commit ``e44de5c4cfd292667d42c66446fdda615699fca8`` restored the exact saved
twelve-provider patch on base ``81bb5d6483714efb4cee248d4fdb703c592723d2``.
The patch ``lupos-twelve-provider-source-20261001T1015Z.patch`` has 912,558 bytes
and SHA256 ``dbbfdebe23e115df74a00cc84b4e8c554bc45e15fe8531bdc4a0d1e9aa80510e``.
Its 167 target source blobs, modes and complete diff blocks were verified.
The original C implementations and tests are preserved.

Fresh build verification
------------------------

The restored e44de5c feature configuration passed the original ``vmlinux``,
``modules``, ``bzImage`` and ``usr_gen_init_cpio`` targets. An exact repeat
performed no compile, link or generation work and retained artifact hashes.
All twelve restored providers have actual Rust compiler command records,
matching source hashes, unique archive membership and representative final
symbols. This is a mixed Rust/C/assembly feature profile. The separate stock
x86_64 LLVM defconfig reference has Rust disabled.

The configuration retains stock debug-info, BTF and module-version choices.
It does not validate disabled features, other architectures or every Rust
selector. No original-C runtime acceptance has yet been established in the
replacement environment; historical test reports are not current results.

A boot-only smoke check of commit 84a15dcc reached Bash PID1, mounted the guest
filesystems, identified genuine runtime tools and powered down with QEMU exit 0.
The pinned image and complete serial/terminal evidence were recorded. This
establishes startup/shutdown only, not original-C runtime-suite acceptance.

Scheduler deadline correction
-----------------------------

Commit ``aedd84a1decd32a6097c6878cdb1ed9123b6ffdd`` corrects a configured
constant discrepancy. ``CONFIG_HZ=1000`` evaluates to 1000 in the original C
kernel headers, while the old bindgen bare ``HZ`` constant was 100 from
userspace ABI headers. Rust now uses a Clang-evaluated ``LUPOS_SCHED_HZ`` enum
derived from the explicit kernel jiffies header.

Configured C/Rust compile checks and a stale-value negative control pass.
The separate corrected kernel passed the same four original targets and an
exact no-op repeat. The real Rust object and final linked kernel contain the
1000-tick addition after the jiffies call. Source ownership and the unchanged
``CONFIG_HZ=1000`` are recorded. Original compiler warnings and overflow checks
remain enabled. Original C-source runtime tests and matched controls remain
pending; supplemental macro probes are not alternate kernel configurations.

Reconstructed AH6 provider
--------------------------

``net/ipv6/ah6.rs`` is a new reconstruction from the retained original C unit,
not byte-identical recovery of the unavailable later implementation. The
opt-in ``RUST_INET6_AH`` selector preserves the original ah6.o/ah6.ko module
identity. Packet processing, callbacks, state lifetime and registration policy
are in Rust; dedicated wrappers expose configured C declarations and header
macros/inlines. The original C provider remains available as the control.

Private configured bindgen, C-helper/Rust compilation and relocatable-link
checks pass. The audit compares 103 C/Rust layout values and 24 callback-table
relocations from real ELF objects, including const/mutable placement and
builtin metadata/init/exit linkage. Explicit wrapping arithmetic preserves C
semantics while Rust overflow checks remain enabled. No unresolved AH6 helper
or panic/unwind references remain in the combined object.

These private checks are supplemental. A separate native built-in build passed
the four original targets, exact no-op and actual final callback/init ownership.
MIP6 configurations, modular loading, CFI/BTF, original C-source runtime tests, matched controls and
other architectures remain pending. No historical AH6 runtime pass applies to
this reconstructed source.

Reconstructed NFSv4 file operations
----------------------------------

``fs/nfs/nfs4file.rs`` is a new reconstruction from the unchanged original C
unit, selected by ``RUST_NFS4_FILE``. File open/flush/lease, conditional NFSv4.2
COPY/seek/allocation/clone and SSC lifetime/registration are implemented in Rust.
Configured declarations and narrow macro/inline wrappers preserve the C ABI.

Private configured probes compile both the original NFSv4.2-off profile and a
separate on overlay. Bindgen, strict C/Rust compilation, relocatable linking,
read-only operation-table bytes and all table callback relocations match the
original C comparison (14 off, 17 on). Canonical compiler flags remain intact;
the probes do not replace full Kconfig/Kbuild or runtime tests.

The unchanged original ``guard-regions.c`` builds through its own Makefile with
no flag changes; only its test listing has run. Applicable tests can execute on
a guest NFS mount. Native NFSv4.2-off Kbuild, no-op and final operation-table
ownership checks have passed. Configured variants, CFI/FSCACHE, actual mounts, original-C runtime tests and matched controls remain pending. COPY,
SSC, clone and seek require distinct coverage; the selected guard tests do not
prove those operations. No old unavailable NFS candidate result is reused.

Reconstructed ethtool coalesce provider
--------------------------------------

``net/ethtool/coalesce.rs`` is reconstructed from the retained C original behind
``RUST_ETHTOOL_COALESCE``. Policy tables, request callbacks, nested DIM profiles,
RCU updates and driver callback/error ordering remain Rust-owned. Binding
constants come from Clang-evaluated kernel declarations. Canonical Kbuild flags
are retained; no per-object function-section override is introduced.

Private configured binding/C-helper/Rust object and link checks passed, along
with the original C unit's compile-time support-bit assertions. Independent
source review found no additional discrepancy. Native Kbuild and exact no-op
passed, with actual Rust ownership and all constant-table bytes/final pointers
verified. A subsequent panic-import audit failed; its repair is described below.
Feature configurations and runtime behavior remain pending.

The original netdevsim coalesce test has 20 scalar and two adaptive-mode shell
assertions. Its unchanged sources, Makefile and runner are prepared, but the
test has not run. It must demonstrate real generic-netlink dispatch to the Rust
provider rather than ioctl fallback. No applicable original C userspace
coalesce assertion suite was found; these shell checks cannot be relabeled as
one. DIM profiles, error paths and other uncovered behavior remain explicit
coverage gaps. The original C netdevsim driver is only a test fixture.

Coalesce checked-arithmetic correction
-------------------------------------

The native build exposed eight add/multiply overflow-panic relocations in
``coalesce_reply_size``. This was emitted-object evidence, not an observed
runtime panic. The earlier private v2 harness omitted the kernel's
``-Coverflow-checks=y`` flag; adding only that flag reproduces all eight sites.
Those earlier probes must not be treated as full compiler-policy coverage.

The original C AST evaluates the formula as signed int after sizeof-to-int
conversions at ``nla_total_size``. Its canonical ``-fno-strict-overflow`` driver
flag produces frontend ``-fwrapv``. Valid primitive sizes are 8, 8 and 4, and
the actual compiled C function returns 520. Rust now widens intermediates to
i64, whose bounds cover the complete i32 primitive domain with the actual
profile count, then returns the valid original int result.

Replaying the exact saved Kbuild command with only private source/output path
substitutions reproduces the old gate failure and accepts the fixed object with
zero panic imports/relocations. Overflow checks, warning policy, original C
sources and tests are unchanged. The before object is retained as a negative
control. A complete fixed-kernel rebuild and final audit remain pending; no
runtime or original-C test pass is inferred from these checks.

Remaining work and priority
---------------------------

IPv6 GRE and virtio-9p reconstructions and their acceptance
remain separate work. Device-driver translation is deferred until core kernel,
memory, filesystems and networking. Existing driver candidates are preserved;
unchanged C drivers may still supply build dependencies or test fixtures.

Acceptance requires applicable original C-source tests built through their
original recipes to exercise verified Rust providers, with matched C controls
and recorded configuration, ABI, symbols and artifacts. Supplemental checks and
translated test suites do not replace that requirement. Memory-accounting,
strict CFI-symbol and broader configuration/architecture gaps remain open.

Coherent source changes are committed incrementally. The complete migration
and the subsequent Lubuntu/Firefox A/B workflow remain unfinished. Module work
and its bounded validation are tracked in the parent Lupos repository issues.
