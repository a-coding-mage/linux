Lupos Rust recovery checkpoint
=============================

This branch restores the twelve-provider source checkpoint after the prior
execution workspace became unavailable on 2026-10-02. It is an incomplete
engineering checkpoint, not a release or a claim that all Linux tests pass.

Recovered source
----------------

The base is ``81bb5d6483714efb4cee248d4fdb703c592723d2``. The saved patch is
``lupos-twelve-provider-source-20261001T1015Z.patch`` (912,558 bytes), with SHA256
``dbbfdebe23e115df74a00cc84b4e8c554bc45e15fe8531bdc4a0d1e9aa80510e``.
It changes 167 files, with 14,981 insertions and 2,514 deletions. This status
file is additional recovery documentation.

The complete saved text was recovered and matched the original byte hash.
Application and reverse-application checks passed against the exact clean
base. All 167 target blob hashes match, and the complete regenerated per-file
diff blocks, including modes, match the saved patch. These are source recovery
checks only; no complete kernel build or original-test run has yet been validated in this
replacement workspace. Historical reports remain historical evidence; the prior raw build
artifacts and execution logs are currently unavailable.

Remaining work
--------------

Later coalesce, AH6 and IPv6 GRE source and integration changes are outside this
saved checkpoint and require exact recovery or clearly identified reconstruction,
review and fresh validation. Separately prepared ACPI, IPv4 tunnel, NFSv4 and
virtio-9p candidates were not part of this saved patch either.

Acceptance still requires the applicable original C-source tests, built with
their original recipes, to execute verified Rust providers, with matched C
controls and configuration, ABI, symbol and artifact provenance. Supplemental
checks and translated test suites do not replace those tests. The recorded
memory-accounting failure, strict CFI-symbol gate and broader architecture,
configuration and uncovered behavior gaps remain open.

Future coherent source changes will be checkpointed separately. The complete
migration and the subsequently requested Lubuntu/Firefox A/B workflow are not
complete at this checkpoint.

Scheduler deadline correction
-----------------------------

The rebuilt configuration exposed a real constant-binding discrepancy:
``CONFIG_HZ=1000`` evaluates to 1000 in the original C kernel headers, while
bindgen's bare ``HZ`` macro constant was 100 from the userspace ABI headers.
The Rust blk-mq scheduler consequently computed a 100-tick dispatch deadline
where the C implementation used 1000 ticks.

The scheduler now uses a Clang-evaluated ``LUPOS_SCHED_HZ`` enum derived from the
explicit kernel jiffies header. The bare macro is no longer allowlisted.
Actual configured C assertions, generated bindings, C adapter compilation and
full Rust scheduler object compilation pass. Original compiler flags, including
Rust overflow checks, remain unchanged. A stale-value negative control fails
as intended. Additional preprocessor-only HZ variants are supplemental checks,
not alternate kernel builds or original-C runtime acceptance.

The complete corrected kernel rebuild and affected original C tests remain
pending. The initial baseline build stays pinned to its original source until
its terminal boundary; this commit does not relabel that build as corrected.
