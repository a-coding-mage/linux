# Reboot handoff — C-to-Rust migration

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
