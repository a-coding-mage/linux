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
selector. Subsequent source-specific original-C checks are recorded below. Historical
test reports are not current results, and disabled domains remain unvalidated.

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

These private checks are supplemental. Native built-in and MIP6=n module
profiles at commit ``52764ae84c20231e5d8a12719da1d769861cebac`` passed the four
original targets and exact no-op checks. Actual compiler, archive, module,
callback, typed loader-hook and metadata ownership was verified. In the matched
module pair, the effective config differs only in RUST_INET6_AH; other selected
Rust providers stay enabled. Fourteen non-AH6 modules, including both original-C
kernel test modules, are byte-identical. Only ah6.ko is the expected difference.

Both providers pass four original-C nettest client/server flows (TCP/UDP, both
directions) in the same new IPv6 transport-mode topology. A separate
supplementary original-C UDP workload checks every byte, count and length of
ten 1300-byte datagrams per direction. Original source, Makefile/compiler-option
records, C binaries and adapter bytes match. Each fresh 4 GiB/eight-vCPU TCG
guest retained the original 45-second limit, complete logs, process exits,
positive AH counters, namespace cleanup and QEMU exit 0. The first incomplete
logging run is retained separately and is excluded from accepted results.

Paired runtime archive SHA256:
``54610ee5cacb55e58320b4f2282b1e39352f52b00ae8f1bb51d2b1b5e5bd5438``.
Common original-source manifest:
``7867d1197fa1fa5e6d47eea9f67bd48a9a19d384c71dc4f639f6cbf6797fc7f4``.
Common compiler manifest:
``bda33e24be680a7137aa95148b63a1b8a5282e7055b46e39f8c5871af357b66a``.

These are scoped original-C program checks, not complete upstream networking
or UDP GSO/GRO suites. They do not prove TCP payload integrity, explicit final
module unload, every async/error branch, ESN, MIP6/HAO or extension headers.
Selected original kmod validation is recorded separately as it completes.
Strict module DWARF/CFI/BTF and other architectures remain pending. Later GRE
and scatterlist source changes are not attributed acceptance of this earlier
frozen image; no historical AH6 execution is reused.

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
control. The fixed kernel has now passed the original four targets, exact no-op,
unchanged native table/ownership/import audits and the before-object negative
control. No original-C runtime pass is inferred from those build checks.

Reconstructed IPv6 GRE provider
-------------------------------

``net/ipv6/ip6_gre.rs`` is reconstructed from the complete retained C unit behind
``RUST_IPV6_GRE``. GRE, GRE TAP and IPv6 ERSPAN packet paths, tunnel lookup,
ioctl/netlink configuration, namespace lifetime and registration are implemented
in Rust. Generated declarations and narrow macro/inline, bitfield and compiler
metadata boundaries preserve the configured C ABI. Original C/tests remain intact.

Private canonical-command checks pass bindgen, strict C/Rust compilation and
relocatable linking for built-in and module modes. Original-C/Rust oracles match
192 size/alignment/offset values. Each mode's table audit matches 54 relocations
per C/Rust/combined object, complete scalar/null bytes, const/read-mostly placement
and module metadata. Actual module entry aliases call the Rust implementations.
No private-helper or panic imports remain; compiler-generated memmove is an
ordinary kernel dependency. Independent source review found no further mismatch.

These are supplemental object/ABI checks. Actual Kbuild/modpost/BTF, complete
feature configurations, runtime ownership and original C-suite acceptance remain
pending. The complete original BPF tunnel and tc_tunnel runners, compiled through
their original Makefiles with supported tools, are required with matched C
controls. No historical GRE execution result is reused by this reconstruction.

Scatterlist arithmetic boundary corrections
------------------------------------------

The existing Rust scatterlist owner now preserves C's signed-count wrapping
under the kernel's ``-fno-strict-overflow`` semantics. Its DMA entry counter
checks the nonzero divisor only when an entry is actually visited, so an empty
list returns zero as the C implementation does. Four extraction budgets now
preserve the C size_t subtraction before conversion back to ssize_t.

The actual saved Rust compiler command was replayed with only source/output
paths changed, keeping kernel overflow checks enabled. Both controls compile
without diagnostics. Original C AST and frontend records establish the
unsigned budget conversions and ``-fwrapv`` semantics. Scoped source/object
negative controls reject the old owner and accept the corrected owner. Six
arithmetic panic call sites disappear; 45 unrelated sites remain and are not
blanket-suppressed. Symbols, declarations, C sources and tests are unchanged.

These corrections cover representational and empty-input boundaries; no
ordinary production incident is claimed. Full native Kbuild and original-C
runtime acceptance of this change remain pending. The original C iov_iter
KUnit suite has 17 cases, including five scatterlist extraction cases, and is
the prepared kernel validation route. The separate 22-case host scatterlist
harness builds a C implementation and does not validate the Rust kernel owner.

Fresh selected kmod and host-build checks
----------------------------------------

The same frozen AH6 provider pair also passes original kmod cases 0005 and
0008 at their unchanged 45-second limits, with all 10 and 150 repetitions,
respectively. Original thread settings, trigger checks and test_result
assertions remain intact. No 180-second override was needed or used in these
new runs. Full serial output is retained for thread evidence when the bounded
post-test dmesg ring no longer contains every earlier repetition.

The genuine Debian kmod 34.2 version string exposed an integer-parsing problem
in the unchanged upstream shell guard. Signed official kmod 30 and its matching
libkmod2 satisfy that guard without test edits or output masking; identical
small dependency/depmod layers were applied to both guests. Module binaries and
original tests remain unchanged. The 34.2 negative control is preserved.
Selected-case evidence archive SHA256:
``67b62611fa36a026cd6c764e9a04e6ef2873856a3818eaa844b8707b98fd0732``.
These two cases do not stand for the complete thirteen-case suite or prove
explicit final AH6 unload, all optional configurations or other architectures.

The separate ``rust-host-tools`` aggregate and its exact no-op repeat now pass.
Actual command/executable ownership identifies 58 selected Rust utility entry
points and two original-C decoder test executables linked to the Rust decoder.
Linked C helpers and system/crypto/DWARF libraries are recorded; this is not a
claim of entirely C-free host binaries. The initial native link failure for
libdw/libelf is preserved. Signed matching Debian development libraries and a
gendwarfksyms-only library-search adaptation resolve it without changing
optimization, overflow checks, original sources or test assertions.
Host-build evidence archive SHA256:
``e919defdd15e64cc745e2742c5f68706ea9d6628f1f8c52913dfd104d7b9fea0``.

No host-test execution is inferred from successful compilation. The declared
``rust-host-tests`` discovery includes kernel/module/runtime and supplemental
Python checks, not only these utilities. The original C decoder invocations
and broader test side effects are being audited. The real historical baseline
``68f3e0875bdb9ebd86be04e912a58b1259d374f1`` was retrieved for unchanged
Git-based assertions; no fabricated replacement or test-reference edit is used.

The independently required ``tools/all`` target remains unrun. Its 29 immediate
prerequisites include source-local writers, C-selected leaf programs and
selftests that can report success with only partial collections. Native
invocation, per-collection outcomes and actual language ownership are required;
neither a filename census nor aggregate exit zero would close that contract.

Remaining work and priority
---------------------------

Virtio-9p reconstruction and the outstanding native/runtime validation of all
new providers remain separate work. Device-driver translation is deferred until core kernel,
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
