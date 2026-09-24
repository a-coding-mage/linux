# Reboot handoff — C-to-Rust migration

## Active again — 2026-09-24 15:24 UTC / September 25 JST

The user explicitly resumed: "we already restarted, just continue the work".
The goal is ACTIVE; historical pause instructions below no longer apply.
Private checkpoints and persistent toolchain survived; compatibility paths were
verified. Root is rebuilding MAIN x86/ARM64 donors from saved configurations;
workers are resuming list_sort isolation, UUID checker and parser runtime fixes.
Fresh job records: `/tmp/lupos-base64-uuid-runtime.I51l5PDI/resume-1528-main-{x86,arm64}-status.json`.
Both MAIN image/modules builds finished PASS/exit 0 at 15:32 UTC. Their fresh
bindings include parser/minmax/glob. MAIN is now read-only for worker fixtures.
Separate MODULAR builds are still running (`resume-1532-modular-*` job records).
Fresh strict Rust1.85 migration invariants: 3 pass, no skips, 46.860 seconds.
Fresh integer-math differential: 11 pass, no skips, 16.620 seconds, including
actual ELF32 C/Rust owner and independent consumer at O0/O2/Os. Use the complete
official i686 sysroot `/tmp/lupos-rust185.svY8Mh/toolchains/1.85.0-x86_64-unknown-linux-gnu`
for every `*_I686_SYSROOT`; the attempted custom-core restoration remains
incomplete and its two failed logs are retained. Recovery environment overrides
are saved in `/home/fenhir/.cache/lupos-migration/reboot-20260924/recovery-state.json`.
Both MODULAR builds subsequently passed. Integrated list_sort fixture isolation
(29 focused groups; combined boot/int_log/rational regression 126 pass), UUID
framework-derived callback/registration guards (30 pass), parser runtime checker
(19 pass including both final Rust artifact audits), and shared common-metadata
relative-cfg-path handling (11 focused groups). Fresh combined regression:
219 tests, zero skips, 56.625 seconds; `resume-1545-integrated-regression.log`.
This is a focused regression, not the complete host/kernel regression.

Parser, Base64 and UUID Rust-provider/C-and-Rust-caller matrices now pass on both
architectures: twelve fresh VMs, each with consumer unload/reload. Base64 and
UUID also load/reload their translated KUnit modules under modular KUnit. Job
prefixes: `resume-1543-parser-rust-*`, `resume-1545-base64-rust-modular-*`, and
`resume-1546-uuid-rust-modular-*`. The original Base64 modular audit failure is
retained under `resume-1536-*`; it exposed the fixed relative cfg path guard.
Current root jobs `resume-1547-three-c-providers-*` switch parser/Base64/UUID to
original C, rebuild, run both caller languages, then restore Rust and audit.
Inspect terminal status before assuming this C/restore phase completed.

Update 15:57 UTC: those first C jobs passed both parser callers, then failed
Base64 preflight because common metadata was incorrectly required to be newer
than global auto.conf after unrelated provider toggles. Root removed that false
dependency while retaining actual fixdep CONFIG-stamp freshness and all source,
generator/header, cfg/core and object checks; 14 focused tests pass, including
new changed/newly-created recorded-stamp negatives. Retry jobs
`resume-1553-c-matrix-retry-{x86,arm64}` both PASS, including Base64/UUID C-provider
VMs, restoring all three Rust providers, rebuilding and final selected audits.
This cycle has 24 fresh successful VMs across parser/Base64/UUID, each with both
consumer loads; the translated Base64/UUID suites also load twice per VM.
Parser and the strengthened Base64 suite now count as validated: 34/526 lib
units (6.46%, 492 remain); host tools stay 55/64 (85.94%, nine remain), with
three generators overlapping. This is not full-migration completion.

The reviewed memweight checker/test pair is integrated; root discovery passes
14/14 with both fresh MAIN donors (6.748 seconds). Private cumulative patch:
`memweight_runtime_revision2.3IKloDyz/integration.patch`, SHA256
`7bf7c7d6d5eb91b0f7747eacc8cdbaefcf9248df39b083cb5392e79ef5dc9168`.
Root's `resume-1557-memweight-rust-*` now runs both callers on both MODULAR Rust
providers. No memweight VM success is claimed until those job records finish.
The worker formerly on memweight now owns private cmdline revision directory
`cmdline_runtime_revision2.8URmYHjh`; its three original checker/test/fixture files
are in scope. MAIN remains read-only and MODULAR remains root-owned.

Update 16:07 UTC: `resume-1557-memweight-rust-*` and
`resume-1600-memweight-c-*` both finish PASS on both targets: eight noncrashing
VMs, 12,651 protected comparisons per consumer load, real 128 MiB allocation,
two loads per VM, Rust restored and audited. The separate deliberately crashing
BUG-threshold VM gate remains unrun; memweight is not yet added to the count.
Root additionally corrected `test_memweight.py` to accept genuine official
hashed i686 core filenames, isolate all compiler cwd/temp/response inputs,
watch read-only inputs and preserve private Kbuild's expanded flags in an
actual rustc response file (otherwise Make exceeds the shell argument limit).
Seven groups now pass without skips in 6.027 seconds, including real ELF32
O0/O2/Os execution and private C/Rust/C Kbuild/dependency/no-op tests on fresh
donors. Earlier cwd-guard and shell-size failures remain retained. Root also
corrected memweight strict-lint parsing: Rust 1.85 does NOT accept comma lists
as several lints; real unknown-lint probes are now negative controls.
Focused combined regression `resume-1604-integrated-regression-tests.log`:
243 pass, zero skips, 76.884 seconds, exact environment/argv in its JSON spec.
This still does not claim the complete migration regression.

Update 16:14 UTC: independent read-only boundary audit
`memweight_boundary_audit.SLvYRaIR/RESULT.md` confirmed no C/Rust discrepancy;
the dedicated crashing matrix was an extra assurance proposed in the review,
not an original self-test requirement or an identified defect. Preserve the
unrun native BUG-dispatch limitation explicitly; do not relabel core None or
host aborts as kernel BUG evidence. With the eight native ordinary-path VMs,
real ELF32/build tests, boundary mutants and restored audit complete, memweight
is now included: 35/526 lib units (6.65%; 491 remain). No extra crash runner was
implemented or executed. The audit's eight-console check confirms 202,416 native
comparisons across sixteen workload loads.

Reviewed win_minmax and glob checker packages are integrated. Root tests pass
17/17 (16.848 s) and 21/21 (67.048 s), respectively. Frozen patch hashes:
win_minmax `b41677a9e559191b556d008aef14c3ace10491f9018c6a241c4730455e616483`;
glob `0f51d7c1a96cf28fd6d5b6e7445e4e1bfa4adc4aa5b9ebb2184b995601c013b8`.
`resume-1610-win-minmax-rust-*` and `resume-1612-glob-rust-*` each PASS both
architectures and both callers with reload (four VMs per component). Glob's
original 64-parameter KUnit module also loads twice; its unique copied-build
console is inside the retained `glob-runtime-*` evidence directory, not the
original MODULAR rust-boot-test directory. Current root jobs
`resume-1613-minmax-glob-c-*` run C baselines then restore/audit Rust; inspect
their terminal status before claiming completion or counting these units.
Workers: cmdline revision still private; rbtree read-only review at
`review_rbtree_resume.CSwfPFsu` found missing recoverable checkpoint files and
unsafe old replay cwd, not yet a production defect conclusion. Third worker is
now diagnosing missing full-regression environment inputs, without rebuilding.

Update 16:17 UTC: `resume-1613-minmax-glob-c-*` both finished PASS, including
all C-provider VMs, Rust restoration/rebuild and both final audits. Min/max and
glob each now have eight successful provider/caller/architecture VMs with
reload. This resumed cycle totals 48 successful VMs across parser, Base64,
UUID, memweight, min/max and glob (eight per component). Lib inventory now
37/526 (7.03%, 489 remain), host55/64 unchanged; native panic-path limitations
above remain explicit. Expanded focused regression
`resume-1617-integrated-regression` is running, expected281 tests; inspect its
status instead of assuming success. Both MODULAR trees are currently restored
to all five Rust providers. No root build/VM is running at this update.

Cmdline's latest private review found two additional harness issues: comma-lint
parsing and binding text wrongly required newer than global .config/rustc_cfg.
Actual restored x86 bindings text is15:29 UTC while config/cfg are16:15 and
libbindings.rmeta16:15:41; Kbuild correctly reuses unchanged C bindings. Worker
is separating recorded bindgen dependencies from Rust metadata cfg dependencies,
with actual changed-header/stamp controls; do not alter timestamps.

Update 16:22 UTC: expanded focused regression finished PASS: 281 tests, zero
skips,170.935 seconds (`resume-1617-integrated-regression-tests.log`). Root also
fixed `test_hweight.py`'s literal `libcore.rlib` assumption to accept the official
hashed core, retaining missing/ambiguous-input rejection. Its genuine ELF32
matrix and two focused validation groups PASS3/3,3.305 seconds with Rust1.85
(`resume-1621-hweight-official-i686.log`). No hweight native replay was run;
its broader donor-isolation audit remains separate. This recovery fix adds no
migration unit. Cmdline's final18-test serial/concurrent private run is pending.

Full-regression recovery audit found the exact Unicode12.1 corpus retained at
`/tmp/lupos-parallel-work.TDJdWxsY/unicode_tests/frozen-final-gcc/unicode-oarpx63x/src/fs/unicode`
with all eight pinned hashes and PROVENANCE.json verified; no download required.
Current MAIN is not a substitute for all legacy gates: GENKSYMS is disabled,
x86 SYSTEM_EXTRA_CERTIFICATE is disabled, ARM lacks nVHE outputs, and x86
vmlinux lacks retained relocation sections. MAIN realmode.elf does have six
ELF32 relocation sections and may serve that separate input. Await the recovery
worker's final per-variable table before remapping or rebuilding absent trees.

Update 16:35 UTC: integrated cmdline checker/fixtures initially passed18 groups,
but its first native run rejected valid relative external-module saved paths.
Root corrected compiler-cwd resolution (M= for external consumers, O= for
in-tree bindings); retained strict dependency/freshness and wrong-path controls.
Discovery now passes19/19, zero skips,24.203 seconds. Rust-provider matrices
`resume-1632-cmdline-rust-{x86,arm64}` both PASS: four VMs, both caller languages,
original eight-case KUnit and consumer unload/reload. C baselines and Rust
restoration/audits are RUNNING under `resume-1634-cmdline-c-*`; do not count this
unit until their terminal records pass. Original failures `resume-1625-*` kept.

Recovery audit is final at `regression_recovery_audit.vaKsiPLC/RESULT.md`:
recovery-state.json remaps only verified Unicode corpus, x86 decoder and realmode
inputs; configuration-specific missing builds remain explicit. Full Unicode
regression is running as `resume-1635-unicode-corpus`; no result claimed yet.
Workers now own private rbtree revision3 (four concrete corrections from
`review_rbtree_resume.CSwfPFsu/RESULT.md`) and one-file hweight/glob test-transport
repairs. They may not write to MAIN/MODULAR or the repository. No production
Rust, SOURCE-COMMIT or Makefile edits were made during this runtime repair.

Update 16:40 UTC: cmdline's C-baseline jobs originally omitted the explicit
`--allow-c-baseline` opt-in and stopped before generating fixtures/booting;
that orchestration error remains logged at `resume-1634-*`. Corrected jobs
`resume-1636-cmdline-c-retry-{x86,arm64}` both PASS, including four C-provider
VMs, restoration/rebuild and final Rust binding/linkage audits. Together with
the four Rust-provider VMs this completes cmdline's dedicated matrix:33,411
comparisons per load, eight unchanged KUnit cases per suite load, all loaded
twice per VM. Inventory is now38/526 (7.22%,488 remain); host55/64 unchanged.
This resumed cycle has56 successful VMs. Both MODULAR trees are restored to
Rust and no root VM is running. Complete migration regression remains pending.

LLVM19.1.1 and official Rust1.85 PPC32/PPC64LE libraries restored privately at
`/home/fenhir/.cache/lupos-migration/reboot-20260924/cross-tools.VsTsQoQ7` (LLVM)
and the existing persistent1.85 sysroot (libraries). No global installation or
default-toolchain change. First targeted cross test run failed because root's
environment omitted BINDGEN; retain `resume-1640-restored-cross-tools` evidence
and rerun with the restored bindgen path. A fresh independent original-C x86
GENKSYMS/MODVERSIONS+certificate-space+relocatable build is running at
`/tmp/lupos-recovered-x86-genksyms.7TUlSeoX`; MAIN and MODULAR are untouched.

Update 16:55 UTC: cross-tools retry PASS7/7, zero skips,57.069 seconds. Hweight
one-file transport patch `7cd47decba81323030ede1b3cb63a703cc14e60aa77c349adf7447fdac2b8a76`
is integrated; root14/14PASS,zero skips,70.565s (`resume-1651-hweight-integrated`).
It preserves all original groups and adds normal-discovery concurrent C/Rust
replay with actual deleted-rcgu and full930-directory donor-write observation.
Glob one-file patch `abd2ce7a45d0fabf8aae762119f3298d2b66bbe0fff000a3958e923894e756ea`
is integrated byte-identical; root7-group replay is RUNNING (`resume-1654`).
Root additionally fixed native command/response path resolution and private
compiler cwd/TMP in `test_recordmcount.py`; all20 groups PASS,zero skips,10.048s,
including real MAIN compilation and an empty full-tree donor-write report.
No production Rust/C/provenance/Makefile changed; inventory stays38/526,host55/64.

Recovery builds: x86 GENKSYMS/certificate-space and ARM64 KVM/nVHE both PASS;
MIPS32 vDSO PASS. x86_64 RELOCATABLE alone does not retain relocations in this
tree: X86_NEED_RELOCS depends on RANDOMIZE_BASE (or32bit). The new isolated x86
build is therefore rebuilding with RANDOMIZE_BASE enabled (`resume-1653`).
MIPS64 hit Clang18 integrated-assembler errors on original genex.S forward
label differences, not a Rust migration failure. Private GNU binutils2.42
restoration succeeded; retry uses LLVM_IAS=0 and the actual GNU assembler.
Both MIPS retries explicitly set target RUSTC/RUSTDOC probes to1.85 as well as
HOSTRUSTC (initial MIPS had a1.98 config probe despite1.85 host compilations).
See `resume-1655-recovered-mips{32,64}-retry` terminal records before remapping.
Recovery paths are recorded individually in persistent recovery-state.json.

Integrated300-test regression (`resume-1642`) and full Unicode generator
regression (`resume-1635`) remain RUNNING. Unicode reached its last real-Kbuild
regeneration group; earlier official-corpus comparisons pass, but no overall
success is claimed yet. Workers now repair `test_cmdline.py`, finish private
rbtree revision3 and prepare a new private bsearch production candidate.

Update 17:04 UTC: combined focused regression PASS300/300, zero skips,
1080.694 seconds (`resume-1642-integrated-regression-tests.log`). Full Unicode
suite PASS9/9, zero skips,1440.121 seconds, including exact Unicode12.1 corpus
and private real-Kbuild regeneration (`resume-1635-unicode-corpus-tests.log`).
Integrated glob transport PASS7/7,zero skips,205.853 seconds (`resume-1654`).
Recovered ARM64 nVHE generator PASS11/11,zero skips,33.268 seconds (`resume-1656`);
MIPS32 vDSO generator PASS13/13,zero skips,26.078 seconds (`resume-1659`), after
explicit Rust1.85 config probes and genuine ELF32/o32/MIPS32r2 verification.
Only those verified input paths are remapped. The isolated x86 relocation and
MIPS64 GNU-assembler kernel builds remain running. Full host/kernel discovery
still awaits unsafe legacy fixture transport corrections; these PASS results
are not a full-regression claim or new migration units. Inventory remains
38/526 lib,55/64 host,56 successful fresh VMs in this resumed cycle.

Update 17:11 UTC: recovered x86 randomized-relocatable and MIPS64 GNU-assembler
builds both PASS. Verified actual x86 RELA sections/certificate symbols/GENKSYMS
records and retained unprocessed ELF64 MIPS64r2 `.tmp_vmlinux2` with relocations.
Fresh x86 tool regression PASS69/69,zero skips,107.401s; MIPS64 relocations
PASS18/18,zero skips,28.528s (`resume-1711-*-regression-tests.log`). Recursive
observations cover492/427 donor directories respectively, with zero writes.
The corresponding legacy input variables now point to these verified builds.

Integrated frozen rbtree revision3 patch (25 files), SHA256
`4d97d3a8776a74976555043c441db0d50df86ab2858fc0b35009ad44b4d02d13`;
every integrated file compares byte-identical to its candidate manifest.
Private final discovery PASS17/17,zero skips,407.560s and zero donor writes.
Root integrated discovery is running (`resume-1709-rbtree-integrated`). The
four original SOURCE-COMMIT values are preserved and added to the invariant
manifest (193 paths; focused marker test PASS). C remains the default provider;
CONFIG_RUST_RBTREE opts into Rust. Both MODULAR trees are rebuilding with
RUST_RBTREE=y and RBTREE_TEST=m (`resume-1710-rbtree-selected-*`). Root has NOT
yet verified final selected linkage or booted these kernels, and rbtree is NOT
added to the38/526 count. MAIN stays read-only. The new runtime checker is
private at `rbtree_runtime.h60xFkop`; original rbtree_test deliberately returns
-EAGAIN after its checks and must not be treated as a normal successful preload.

Integrated one-file cmdline transport patch SHA256
`977816f67098a61eac862deab6cead43124c60f45fb9087a649584b9bf0f9a5e`;
root file SHA256 `2aeb40af9839bcbb4b0a54133c072325fc80a06454112c79a55f30b11654def7`.
Private final serial11/11 and both concurrent11/11 suites PASS; root integrated
replay running (`resume-1711-cmdline-host-integrated`). Worker now owns only a
private test_base64.py transport correction; another continues bsearch with
shared binding hunks rebased on integrated rbtree. Root is repairing the static
key fixture transport. No source/default-C removal, staging or commit occurred.

Update 17:22 UTC: both selected Rust-rbtree full image/modules builds PASS;
integrated rbtree17/17PASS,zero skips,251.760s. Original RBTREE_TEST=m module is
built on both targets, but selected read-only preflight/runtime remain pending
in the private worker. No new VM or inventory increment. Cmdline host transport
root11/11PASS,zero skips,23.382s.

Root repaired three remaining fixture transports using the shared audited
response/path/private-output helpers: static-key11/11PASS,zero skips,2.220s
(`resume-1714-static-key-isolation-retry`), GCD/LCM17/17PASS,zero skips,130.159s
(`resume-1719-gcd-lcm-isolation`), hweight kernel checker16/16PASS,zero skips,
14.653s (`resume-1722-hweight-kernel-isolation`). Native donor directories are
watched for deleted/transient writes. Original assertions and C algorithm/header
oracles remain; C compiler flags now come from always-C scatterlist where an
unselected gcd C command is absent. The initial static-key missing-gcd-command
failure is retained. Static-key's fresh run covers JUMP_LABEL=y, not a new
JUMP_LABEL=n build. GCD/LCM uses freshly rebuilt MODULAR x86; other donors remain
MAIN read-only. Full regression still awaits Base64/UUID/list-sort/rational
fixture follow-ups and complete discovery, so do not call these full coverage.

An external actor committed the preceding integrated work as `bb52e04891b23f564d1c3df1747afc5e0cc4596a`
at17:10:55 UTC, and staged `test_cmdline.py`. Root did not perform those git
mutations and preserves them. Current root-owned unstaged work additionally
includes this note and the three fixture repairs above. Do not reset or unstage.

Update 17:46 UTC: root fixture follow-ups now PASS without skips: rational
11/11 in102.711s (`resume-1725-rational-isolation-tests.log`), list-sort KUnit
6/6 in32.942s (`resume-1732-list-sort-kunit-isolation-retry2-tests.log`), UUID
KUnit8/8 in64.430s (`resume-1731-uuid-kunit-isolation-tests.log`). Compiler cwd,
outputs and TMP paths remain private; the Kbuild fixtures carry the same Rust
arguments in response files to avoid the shell's argument-size limit. Earlier
failed attempts are retained, not counted as passing.

Cmdline now accepts the four exact sibling runtime environment variables while
still rejecting unknown CMDLINE inputs; combined-environment12/12PASS,zero skips,
17.540s (`resume-1734-cmdline-combined-environment-tests.log`). This correction
is unstaged on top of the external actor's staged transport patch; preserve both.
Base64's analogous sibling-environment correction is still private, with final
serial/concurrent evidence pending handoff. No production Base64 change.

Bsearch production candidate is frozen at
`/tmp/lupos-parallel-work.TDJdWxsY/bsearch_resume.nQoGdqOn`; patch SHA256
`5a6eefa98ae2584b638868e2d3342c520f1c2dc83806ac88bc426ba12134d1e9`.
Worker full10/10PASS,zero skips,453.426s includes actual original-C differential,
full fresh native bindings and private Kbuild C/R/C/R selection. Root reviewed
the production patch, but it is NOT integrated or counted yet. Shared bindings
and original modfile identity must be merged before root tests/builds. Runtime
checker work is separate at `bsearch_runtime.vZnUgHzQ`; no selected build/VM yet.
Rbtree private selected preflights pass both architectures, but its new runtime
checker and private caller-module pre-VM builds are still being finalized.
Hold shared source mutations until both active workers declare a safe boundary;
all MAIN/MODULAR donors are read-only during those runs. Inventory remains
38/526 lib,55/64 host,56 successful resumed VMs; complete discovery is pending.

Update 17:51 UTC: Base64 one-file host transport is integrated exactly at SHA256
`8d063cd40a1aa06d5d2fb42751a57b1e47821370e8e1bcc8997ee395872668f1`;
root21/21PASS,zero skips,121.928s (`resume-1744-base64-integrated-tests.log`).
Worker composition-safe full runs21/21 each also pass192.026/189.625/187.682s;
all930 donor directories remain untouched. Original16 groups/74 assertion ASTs
and the original algorithm/corpus/provenance checks are preserved.

Root also repaired Base64 KUnit's private native path/response transport and
Kbuild command-size issue, reusing the audited helpers without changing its
allocator/callback/original-C oracles. Native3/3PASS,zero skips,20.191s
(`resume-1747-base64-kunit-native-isolation-tests.log`); complete12/12PASS,zero
skips,180.280s (`resume-1748-base64-kunit-isolation-tests.log`). Tests now watch
all native donor directories and reject fixture output beneath either donor.
No root test process remains active at this checkpoint. Parser host fixture
transport is the next private worker repair; no production Parser change.

Update 17:54 UTC: bsearch frozen nine-file patch is integrated; every candidate
file matched its recorded SHA before root's explicit shared merge. Added the
manual public binding/blocklist/helper include, retained original modfile
`lib/bsearch`, added a real saved-command modfile assertion, and added two exact
SOURCE-COMMIT manifest entries (195 paths). No original C source was changed.
`resume-1753-bsearch-integrated` runs the full10 groups plus provenance marker
test against read-only MAIN donors. `resume-1753-bsearch-selected-{x86,arm64}`
select RUST_BSEARCH=y and rebuild MODULAR images/modules; RUST_RBTREE=y remains.
All three jobs are running. MAIN rebuild waits active private readers.

Rbtree worker released the source-mutation boundary. Private full13/13PASS,
zero skips,113.479s (`logs/discovery-second.log`) predates the strengthened CPIO
checks. The final bounded replay validates all four retained genuine archives,
module/PID1 bytes, original regular-file-only checksums, symlink and hardlink
semantics plus corrupted/truncated controls (`logs/archive-final-replay.log`,
1PASS0.246s). Earlier wrong-format and unconditional-symlink-checksum failures
remain preserved. Root must replay final13 after fresh bindings, then run VMs.
The original generator uses CRC-format headers for all entry kinds with -c,
but stores zero checksums for nonregular entries; do not misdescribe this as
a production generator defect. Inventory38/526,host55/64,56VM unchanged.

Update 17:57 UTC: both `resume-1753-bsearch-selected-*` full image/modules builds
PASS, exit0. Fresh MODULAR artifacts select both Rust bsearch and Rust RBTree.
Root integrated bsearch11-group job remains running (session19591), MAIN
read-only. Rbtree final runtime patch is frozen but not yet integrated:
`rbtree_runtime.h60xFkop/integration.patch`, SHA256
`4b6650aa09783ab98d350712d429e4c325d2745f5078279b06ebd32b6f8cb56f`.
Final checker/test/fixture hashes and honest final-byte replay requirements are
in its RESULT.md. No new VM or inventory increment.

The user reported desktop slowdown and requested unused VMs be stopped. Two
process inspections found NO running VM processes; nothing was killed. Both
kernel builds had finished. Root lowered the remaining verified bsearch test
runner/descendants to nice10 and idle I/O (ionice class3); workers must use the
same priority for future runs. Keep broad test/build batches serialized while
the desktop is under pressure; run later VMs one at a time and terminate each
when its test ends. Do not stop unrelated processes or interpret this as a
request to pause the goal. Parser worker owns a private one-file repair; bsearch
runtime worker may now read the fresh MODULAR outputs. Coordinate all rebuilds.

Historical pre-integration glob review (superseded by the PASS records above)
is in private `review_runtime_batch.tea1792r/glob/RESULT.md`:
compiler isolation, actual KUnit registration/type proof, effective strict flags
and early MAKE validation require correction before its native tests/integration.
Workers are now correcting `glob_runtime_revision2`, continuing
`memweight_runtime_revision2.3IKloDyz`, and continuing
`win_minmax_runtime_revision2.Dvhx7DXN`; all private, with MAIN read-only and no
MODULAR access. Follow each STATUS rather than stale historical worker labels.

## Latest checkpoint — second reboot request, 2026-09-24 13:33 UTC

The user explicitly requested another stop for a computer reset. **Pause the goal;
do not resume until the user asks.** This section supersedes conflicting state
descriptions in the earlier checkpoint below; older test results are historical.

Current observed repository HEAD: `1d46d1d72` (Add documentation and tests for
Base64 KUnit integration). The worktree was clean at this stop request. No new
production/checker changes were integrated during the short post-reboot recovery;
only this handoff is being updated. No commits, staging or resets were performed.
Preserve exact SOURCE-COMMIT markers and keep C selectable alongside Rust.

### Recovery completed since the previous checkpoint

- The first reboot really cleared `/tmp`, including all four native build trees.
  The original 82 MiB durable archive was SHA256-verified and restored, without
  overwriting existing paths. Final rbtree reports/patch were restored separately
  from `final-rbtree/`. Restored logs are NOT fresh test executions.
- Rust 1.85.0, rustdoc, cargo, rust-src, x86/i686/AArch64-musl standard libraries,
  bindgen 0.71.1, matching elfutils headers, and QEMU 8.2.2 for x86/ARM64 are now
  stored persistently under:
  `/home/fenhir/.cache/lupos-migration/reboot-20260924/toolchain`.
  Fresh tool checks passed (actual bindgen/Rust compilation, original C
  gendwarfksyms oracle compilation, QEMU version/dependency checks). No kernel or
  VM was built/run. User-default Rust and system packages were not changed.
  Read `RESULT.md` and `REBOOT.md` there. After another `/tmp` clear, run
  `bash /home/fenhir/.cache/lupos-migration/reboot-20260924/toolchain/recreate-compatibility.sh`
  and source its `env.sh`. The script refuses conflicting paths.
- All four native output paths listed below are **currently absent**, not merely
  stale. Restore saved configurations and rebuild before native tests. Saved
  MODULAR configurations are actually the last builtin-KUnit checkpoint.
  The custom `/tmp/lupos-math-i686-core.67Nfmg` core also remains absent; installed
  i686 standard libraries do not establish that custom-core proof.

### Newly stopped private work — not integrated

New durable delta directory:
`/home/fenhir/lupos-migration-checkpoint-20260924.ieh2isUq/reboot2-20260924-1333.LIQeP2hd`.
Read its `README.md` for archive hashes and final worker outcomes. Restore the
original archive first, then this delta only where its paths are absent or after
reviewing conflicts. Keep the original archive/configs/resume-state.json.

- `list_sort_isolation_resume`: private candidate for
  `scripts/tests/test_list_sort_kernel_check.py`; isolates compiler cwd/output/
  temporary directories, resolves relative inputs and response files, adds
  transient-write/concurrency controls. First normal-discovery run: 27 tests,
  22 passed and 5 explicit native skips. No genuine donor compile occurred.
  Review final STATUS/RESULT and integration.patch; native verification pending.
- `uuid_checker_resume`: private checkpoint for the actual KUnit-framework-
  derived callback type guard, registration/metadata checks and isolated
  compiler transport. Two-file draft patch exists, SHA256
  `6f17aee0c59245d8c3a6dbcbe0fe2a2a55ca055958c9a911ed5569d65cef965b`.
  No syntax, unit, native or patch-apply checks ran; transport review and
  negative/concurrent controls remain incomplete. No integration. Read RESULT.
- `parser_runtime_revision2.RXNiI9XO/integration.patch`: reviewed private patch
  still unapplied; `git apply --check` passed against current HEAD. Packaging has
  no final RESULT. Historical serial/concurrent runs are not fresh validation.
  A new review issue was found before stopping: `check_parser_kernel.py` passes
  the entire explicit QEMU command to `shutil.which`, rejecting the required
  x86 command containing BIOS arguments. Before integration, parse with
  `shlex.split`, validate the executable token, and test quoted paths/arguments,
  missing executables and malformed/empty commands before any side effects.

### Resume order

1. Obtain user resume; inspect current git state and this note. Read the private
   `AGENT_PROTOCOL.md` fully before restarting workers. Old process/session IDs
   are invalid. Do not automatically restart workers from a stale RUNNING label.
2. Restore private delta and tool compatibility paths; inspect tool reports.
   Native command arrays/environment remain in the original `resume-state.json`.
3. Rebuild both MAIN donor trees from saved configs using actual Rust 1.85 and
   strict flags. Only then release read-only donors to fixture workers.
4. Review/finish list_sort isolation and UUID type-guard patches; execute fresh
   native positives/negatives. Complete parser preflight repair and validation.
5. Restore root-owned modular configurations, build image/modules, and rerun the
   strengthened Base64 modular guard and outstanding provider/caller matrices.
   Continue the remaining private candidates and full regression from below.
6. Reconcile translation-progress.rst with verified evidence. Conservative
   counts remain 55/64 host tools and 32/526 lib units, with five additional
   integrated providers awaiting full validation. Do not count tool restoration,
   skipped tests, or private patches as completed migration units.

Final shutdown verification: all three managed workers stopped and reported no
active subprocesses; process inspection found no native jobs, compiler, build or
QEMU process. Both new private directories, including final reports and patches,
were archived and the archive listed successfully. Delta SHA256:
`2697b385958612d7bf80fe85b699ce9a15690a1f71018b1e353b38d941f86c98`.
This updated handoff is also copied into the durable delta directory.

## Earlier checkpoint — historical evidence and remaining scope

Saved 2026-09-24, approximately 12:35 UTC (21:35 JST), at the user's explicit request to stop for a computer reset. The migration goal must remain **paused**, not complete. Resume only when the user asks.

## Goal and constraints

Integrate the existing C-to-Rust translations throughout build scripts, host tools and target kernel; repair imports, definitions, ABI and behavior so building, booting and testing match the original C. Do not redefine success as the currently working subset.

Preserve every exact SOURCE-COMMIT. Keep original C selectable; add Rust choices to existing Makefiles or separate Rust rules. No replacement production C. Temporary original-C oracle/test fixtures are permitted. Minimum Rust 1.85 with strict warnings; do not fake CFI, DWARF types, CRCs or unsupported runtime behavior. Apply source edits with apply_patch; do not stage/commit/reset. External auto-commits have occurred, so inspect the actual tree.

Repository: /home/fenhir/projects/Lupos/vendor/linux.
Last observed HEAD before this note: 787709871 (Add tests for replay handling in boot architecture).
At that point only scripts/tests/check_base64_kernel.py and test_base64_kernel_check.py were modified; other changes below had been auto-committed. Recheck on resume.

## Durable backup

Directory: /home/fenhir/lupos-migration-checkpoint-20260924.ieh2isUq

- private-work.tar.gz: filtered private source, patches, status/review reports, command/test logs, JSON job specifications and KMSAN probe IR from the workspaces below. Does NOT preserve every compiled binary, native output tree or toolchain.
- resume-state.json: environment variables, exact native make/runtime command arrays and historical quantification. Some historical fields are stale; this note and current artifacts take precedence.
- configs/: actual current .config for both MAIN and both MODULAR output paths.

Check the archive before relying on it. If /tmp was cleared, inspect the archive listing and restore its relative directories into /tmp; do not blindly overwrite newer work. Native outputs/toolchains may need reconstruction. Existing source edits are saved in the repository, independently of /tmp.

Private root: /tmp/lupos-parallel-work.TDJdWxsY
Native job logs/specifications/driver: /tmp/lupos-base64-uuid-runtime.I51l5PDI
Older inventory audit: /tmp/lupos-parallel-audit.wb0Yxd8e
Read AGENT_PROTOCOL.md in the private root before resuming workers.

## Confirmed integrated checkpoint

Conservative inventory before the latest Base64 suite reconciliation: 55/64 canonical C-origin host tools, 32/526 translated lib units; the counts overlap through three host generators. 189 source paths now in scripts/tests/translated_sources.txt. No trustworthy whole-kernel completion percentage. Most kernel/drivers/architecture/selftest translations remain unintegrated.

Five more providers are already integrated but not yet counted fully validated: parser, win_minmax, memweight, cmdline, glob. Their 33 focused groups pass without skips, both x86-64/ARM64 native builds pass, and original C cmdline/glob KUnit preflight passed twice per VM (8 cases / 64 parameters). Dedicated full C/Rust consumer/provider matrices remain outstanding.

### Work completed immediately before reboot

1. Integrated repaired lib/tests/base64_kunit.rs, independent default-off CONFIG_RUST_BASE64_KUNIT, original object/module identity and C selection, Makefile .o/.s/.ll rules, test_base64_kunit.py and exact provenance row. Four original cases, allocation/fatal-assert behavior and both real timing loops retained.
   - Integrated GCC: 11 tests, zero skips, 72.737 s.
   - Integrated Clang: 11 tests, zero skips, 76.549 s.
   - Logs: base64KunitIntegrated-{gcc,clang}.log in native job directory.
   - Provenance/language invariants: 3 pass, zero skips, 21.893 s; manifest189Invariants.log.

2. Base64 translated modular suite: C/Rust providers × C/Rust callers × x86/ARM64 all pass, including unload/reload: four native builds and eight VMs. Logs/specifications start base64-c-provider-rust-suite-* and base64-rust-provider-rust-suite-*.

3. Closed the previously untested builtin Rust-suite path for **Base64, UUID and list_sort**: both native builds and twelve VMs pass, with Rust providers/suites, C/Rust callers and consumer unload/reload.
   - three-rust-suites-builtin-{x86,arm64}-status.json: all jobs PASS, exit 0.
   - Per-run console logs copied to the native job directory.
   - These outputs are NOW builtin KUnit, not their previous modular configuration.

4. Integrated stronger Base64 checker/test delta from module_build/base64-suite-runtime.
   - Actual callback KCFI is derived from selected lib/kunit/test.o:kunit_cleanup and original header prototype; an arbitrary common nonzero ID is no longer enough.
   - Actual C/Rust y/m objects, wrong-nominal/disabled-CFI negatives, metadata/common guards, allocator dependencies and compiler-output isolation are tested.
   - Final integrated run: 33 pass, zero skips, 13.726 s; base64SuiteRuntimeIntegratedTests.log.
   - Read-only strengthened builtin selected-artifact audit passed on both current outputs.
   - IMPORTANT: the modular eight-VM matrix predates the strengthened checker. Rerun its new modular metadata/common guard on freshly restored modular outputs, and preferably rerun relevant native consumers. Do not claim that new guard has already checked the old modular artifacts.
   - Frozen input patch SHA256 eea6d834b68e12cd1fb0183c5054aca10a84e1a813f30d539e93a1b9277dc546.

5. Fixed shared boot event handling for genuine printk replay:
   - Added scripts/tests/kernel_console.py by moving the established strict normalizer unchanged out of check_int_log_kernel.py; old import interface remains available.
   - boot_kernel.py normalizes explicit matching replays before module/failslab event validation.
   - Added full/prefix replay, malformed/ordinary duplicate and failslab ordering tests.
   - Combined boot/int_log/rational/list_sort regression: 118 pass, zero skips, 11.234 s; sharedConsoleTransportRegression.log.
   - This regression exposed an inherited unsafe compiler cwd in test_list_sort_kernel_check.py (see urgent follow-ups). Do not characterize that historical run as strictly read-only merely because transient files disappeared.

Documentation/rust/translation-progress.rst was updated during integration but still has provisional text saying Base64 runtime / some builtin suites are pending. Reconcile it with this new evidence on resume. Full regression remains pending; the old 1,512-pass checkpoint predates these changes.

## Native output state — critical

MAIN read-only donor trees, intentionally not rebuilt:
- x86 /tmp/lupos-native-x86-kcfi.l06fMN
- ARM64 /tmp/lupos-native-arm64-cordic-cfi.amoe5a

Their generated bindings predate parser/minmax/glob additions. Private forwarding fixtures are NOT final-kernel proof. All workers must leave these donors untouched.

Root-owned MODULAR path names (currently BUILTIN KUNIT configuration):
- x86 /tmp/lupos-rational-module-config.LL5WgfM7
- ARM64 /tmp/lupos-arm64-kunit-module.wTVt8l6q

Both currently have KUNIT=y, BASE64_KUNIT=y, UUID_KUNIT_TEST=y, TEST_LIST_SORT=y; all three Rust-suite selectors and providers enabled. Five new Rust providers also enabled. CMDLINE_KUNIT_TEST/GLOB_KUNIT_TEST remain m. Common/per-module Rust metadata remains selected. Native tests above finished successfully. No restoration to KUNIT=m was started after the reboot request.

When restoring modular coverage, set KUNIT, BASE64_KUNIT, UUID_KUNIT_TEST and TEST_LIST_SORT to m, retain Rust selectors, run olddefconfig then actual image/modules build sequentially per architecture. Refresh external modules after configuration/provider/CRC changes. Existing prime callers may now be stale.

## Private work and next actions

All paths below are relative to the private root. Private candidates are NOT integrated.

- string_revision2: frozen cumulative patch df34203624c95a5bd13d54420bef2dd8beba2afa17e793a273317b8fd60327eb. 13/13 tests in both concurrent runs, zero skips; MaybeUninit copy validity, exact detailed-warning condition metadata and compiler isolation repaired. Do not integrate as fully faithful yet: concrete KMSAN instrumentation gap, other architectures, exception/MTE/early-boot/sanitizer/native runtime remain.
- rust_kmsan_probe: managed worker saved a reboot checkpoint. Actual Rust1.85 accepts memory sanitizer + LLVM msan-kernel on a private capability-extended genuine target with rebuilt real core/compiler_builtins, also with KCFI. IR/ABI NOT yet audited. Start with target-memory-kernel/probe.ll vs clang-kernel-eager-0/probe.ll; validate actual context layout, shadow/origin/param-return ABI and absence of user-MSan TLS/init. -Cpasses=msan alone is a confirmed invalid shortcut (zeroes load/copy shadow). No native support claimed.
- rbtree_revision2: actual full regenerated bindings compile on both targets; 150,000-operation differential and 11 export CFI matches; actual LKMM kernel atomics replace core atomics/volatile mixing. C/R/C archive/noop/header gates pass. Root executed retained genuine i686 binary exit 0, SHA256 3ff0b5a98e6c65b0faf019c7f93c385ce0d0330e53577552491910402f3e3972. Latest callback matrix: 2 groups PASS, 168 x86 outcomes including 36 expected SIGILL controls; real native atomic/build_error handling, no fake helpers. Final all-group run/format/package review still pending. Read final STATUS/RESULT saved for reboot.
- nullable_callback_binding: frozen exact-context proof for bsearch/sort; transparent genuine nullable callback wrappers plus compiler cfi_encoding, NOT universal typedefs. sort proof is only a bounded ABI probe, not the algorithm.
- bsearch_native.S2FgpTzq: unfinished private full bsearch/header/selector/bindings. Native strict x86/ARM O0/O2/Os passes; further metadata/Kbuild/32-bit checks pending.
- sort_native.EAUItkiy: unfinished full four-API algorithm, MaybeUninit swaps, internal enum selectors and exact sort_r grammar. ELF64 differential passes; ELF32 managed execution SIGSYS needs root replay. Real cond_resched integration remains unresolved; no production stub permitted.
- tools_fixdep.EL4bFIq0: unfinished tools/build fixdep (distinct from scripts/basic/fixdep). Last report 9 groups: 7 pass, 2 explicit skips (ptrace fstat injection and unavailable i686 std); actual tools bootstrap/switching/raw-byte output tests. Review final files, do not treat as integrated or add blindly to 64-tool denominator.

Runtime checker candidates:
- parser_runtime, win_minmax_runtime, memweight_runtime, cmdline_runtime, glob_runtime: frozen first versions, unintegrated.
- Independent reviews in review_runtime_batch.tea1792r/{parser,win_minmax,memweight,cmdline}/RESULT.md.
- parser_runtime_revision2.RXNiI9XO: compiler isolation and durable Rust-provider allocation proof corrections underway; last report serial + two concurrent full runs pass, zero writes observed over 980 donor dirs. Check whether final patch packaging completed before stop.
- memweight_runtime_revision2.3IKloDyz: fixes private cwd, effective strict-lint options and invalid QEMU/MAKE preflight; serial correction underway.
- win_minmax_runtime_revision2.Dvhx7DXN: fixes private cwd, ordered complete DWARF ABI and consistent replay transport; first private validation underway.
- cmdline has NOT received correction worker yet. Review found: wrong lib/lib.a adjacency (Kbuild sorts lib-y), missing actual KUnit relocation/CFI graph, unexercised fifth nonexported API parse_option_str, invalid/private-log destination acceptance, missing string.h binding freshness, and optional summaries. Full report is authoritative.
- glob completed 17/17 private tests (30.547 s); root has NOT reviewed full final package. MAIN source/native hashes recorded; source changes during worker were root Base64 changes. Needs independent review before integration.
- Check every stopped worker's actual files/status; a RUNNING label can be stale. Old tool/session IDs are not valid evidence after reboot. No automatic restart merely because a log stopped.

Urgent harness follow-ups:
1. test_list_sort_kernel_check.py: private_compile runs rustc cwd=build and strips --out-dir (approximately lines 238,265–269); inherited unsafe transient rcgu.o/ll writes into MAIN. Not fixed yet. Fix cwd, output/temp dirs, all relative input paths, add transient/concurrent control, then rerun. Similar issue repaired in Base64 and pending revisions above; audit before executing new native fixture helpers.
2. check_uuid_kernel.py approximately 498–499 still checks only common nonzero callback ID; port actual-framework-derived type proof plus compiled wrong-tag negative. Do not weaken CFI or infer ABI from the common value.
3. Strong Base64 checker now has explicit native metadata/common checks; verify modular restoration under it.
4. Full regression only after stale native dependencies/outputs and isolated compiler transport are properly handled. Preserve previous failures and honest skipped/unproven cases.

## Worker shutdown

At reboot request root finished/polled all active native job and test handles: all terminal exit 0. No QEMU or native_jobs.py process remained. Six independent CLI process groups for unfinished new implementations/revisions were sent TERM; their handles subsequently reported terminal and process groups were absent. Managed KMSAN worker explicitly idled with no subprocess; rbtree worker is saving final status only. No new build was started after the request.

Final shutdown verification: all three managed agents reported completed with no subprocesses. Rbtree's final reboot-only patch, integration-unreviewed.patch (SHA256 fb1f8606787e886c9f0a89fdda0a7326103891861ac2cfe9691df91c2c52e014), is NOT merge-approved. Its final STATUS/RESULT/patch/hash files are separately saved in the backup's final-rbtree/ directory. Current uncommitted Base64 checker files are also copied to latest-checker/.

The 82 MiB private-work.tar.gz archive was listed successfully and contains the protocol, final KMSAN checkpoint, string patch and both successful builtin native job records. SHA256: dbdb0814e879cfe29874fe7114c60d2fdae479648836a2768c7759eb0afd202b.

Next session: inspect this note, git status/log, durable backup and actual worker reports; obtain user resume, then continue the original full goal. Do NOT mark complete based on this checkpoint.
