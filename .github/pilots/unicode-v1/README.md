# Current cycle: incremental Phase 2 comparison (unarmed)

This section describes the current proposed cycle. The older Unicode contract
below is retained as historical reference; its experiment-specific source,
configuration, case counts and ownership assertions do not govern this cycle.
This README grants no authority; the request stays unarmed until its separately
reviewed manifest-only activation. launch.json records approval of one cycle.

Kernel source: `0008179a1ee0b082fa3ead118187fc6f3569a9f2`. The complete frozen
configuration hashes are in `config-lock.json`; `integrated-rust.config.gz`
contains the exact reviewed R configuration. Both members preserve the proven
mixed Rust baseline. Only the six selectors in `provider-delta.config` switch
from original C to Rust. This is an incremental comparison, not full all-C parity.

The finite cycle performs the original full build targets and identical no-op,
24 bounded owner checks, actual Rust `start_kernel` and Rust-init KUnit call
proof, unchanged Bash PID1 with all 18 checks, and the original C CRC (16),
ChaCha20Poly1305 (1) and iov_iter (17) suites. Each suite keeps its original
source, flags, assertions, filter and timeout. The boot-only guest explicitly
sets `kunit.enable=0`; the three suite guests explicitly enable KUnit.

Important open gates: native `.s/.ll/.rsi/.lst` inspection remains static-only.
The audit covers `init/version.o` and `linux_proc_banner`, but does not establish
the final `init/version-timestamp.o` command/object or strong `linux_banner` and
`init_uts_ns` ownership. Complete startup/version-metadata qualification is not
claimed. Functional BPF/freezer/block-fallback/SELinux policy tests, Phase 3
FS/network/zcrx, broader drivers and ARM64 remain separate, unrun scopes.

Existing standard-runner supervision, immutable toolchain, cleanup and text
transport remain in use. The planned maximum stays 10+10+120+120=260 runner
minutes; no artifacts, caches, paid runner, secrets, settings changes or automatic
retry are added. The regenerated public guest stays inside the measured output
cap and is compared by complete manifests/digests. New integrated build, guest
and ELF execution remain unrun. Actual source/setup publication and activation
require their separately reviewed exact inputs and the existing manifest gate.

## Historical reference begins here

# Corrected public Unicode pilot: request v2

This is a review candidate. No publication or run is authorized by these files.
The source remains `62a3294181556f17db863c9016524fa3068a374b`; original C tests,
config values, provider selectors, flags and guest settings retain their reviewed
profile. The setup-base gate is pinned to 298abda59725d491c10be678618ac76300d1aba1.
The new request ID is unicode-public-62a3294-v2. Do not change the experiment pin.

Both matrix jobs reject `github.run_attempt != 1` before checkout, and reject
reruns again in the host, container, installer and stage shell boundaries. A
rerun requires a new separately reviewed request. The initial launch record is
still unapproved and the request manifest is still unarmed.

The installer uses strict `apt-get --error-on=any update` and requires exactly
one matching InRelease for each of the three reviewed suites. All three exact
hashes are checked after apt's signature and default validity/time checks. No
insecure/trusted/freshness/TLS bypass is added. The earliest expiry is updates
metadata on 2026-10-09 at 14:11:18 UTC. Expiry must fail and return for review.

Docker CID bytes are preserved as Docker writes them: exactly 64 lowercase hex
characters with no newline. The incorrect older newline fixture is retained as
a negative control. No whitespace normalization changes the recorded file.

Image pull and the full container lifetime are supervised. Installation,
configuration, build, no-op, ownership audit, guest and parser have individual
admission/live/terminal samples and explicit native/supervisor/cleanup/interruption
statuses. Cleanup targets the owned process group even when its leader exits
first. Docker cleanup is bounded, records errors and fails on uncertainty. The
host separately checks complete output/evidence caps at termination. The output
cap stays 1.25 GiB, evidence cap 128 MiB, minimum free disk 1 GiB and minimum
available memory 512 MiB. Admission retains 4 GiB disk and 6 GiB available memory;
the initial host admission requires 10 GiB disk. A small evidence reserve covers
final status writes. Cap measurements use allocated filesystem bytes. Every
measurement, parse, clock read and record write checks its own return status;
partial numeric output from a failed command is rejected even when Bash errexit
is disabled. No evidence is truncated to produce a pass.

## Proposed run and time commitment

The corrected setup updates the existing helpers and resets the new request to unarmed. It can trigger
one request-only run, which is expected to reject the setup push and start no
member. After that expected outcome is verified, a separate manifest-only
activation commit can trigger one request job and the sequential C/R pair.
There is no skip directive or hidden trigger bypass. Approval must cover both
commits, both possible workflow runs and the specified public logs.

The request job has a 10-minute ceiling in each run. Each member has a proposed
120-minute GitHub job ceiling. Thus the complete planned setup-plus-activation
maximum is 260 standard-runner minutes: 10 + 10 + 120 + 120. It is not a measured
duration. The verified base contains the previously published pilot workflow; the head and
workflow inventory must be rechecked before publication.

The internal soft job deadline is 115 minutes from the first workflow step,
leaving a five-minute tail relative to that step's 120-minute window. Platform
setup or cancellation is outside this internal timer; a hard runner cancellation
remains a failed attempt. Pull is allowed at most 10 minutes. The container
aggregate is at most 100 minutes and must end at least three minutes before the
soft job deadline. Native stage allowances are installation 20, configuration 10,
build 30, no-op 10, ownership 5, guest supervision 17 and parser 5 minutes.

The build allowance is over four times the recorded 6m54s native Rust build,
while package/network/CI performance remains unmeasured. Stage ceilings share
the aggregate budget; they are not promises that every stage can consume its
maximum. A stage overrun fails the pilot instead of changing its work.

The original KUnit timeout remains 300 seconds. The original QEMU outer timeout
remains exactly 900 seconds with a 10-second kill grace. Its separate supervisor
allowance is 1,020 seconds, so a 900-second stage wrapper cannot preempt the grace
or postchecks. Before starting the guest, a recorded admission requires at least
1,560 seconds remaining in both the container and soft job budgets: 1,020 guest
supervision + 300 parser + 60 cleanup + 180 evidence seconds. Insufficient budget
stops before guest launch. No guest clock, workload or original timeout is tuned.

The unchanged original Python KUnit parser and the new strict observer both run.
Their outputs are written directly, with both statuses and nonempty required
results checked. There is no tee pipeline whose failure can be lost.

## Complete text evidence

`Evidence.pm` is the exact ordered role catalog. `emit-evidence.pl` emits only
those text files using `UEV3` framing. It records file identity, UTF-8 byte count,
SHA256, ordered text chunks and an ordered manifest digest. No binary image,
object, archive or encoded binary payload is emitted.

The envelope includes full raw stage logs, exact command arrays, resource samples,
status/cleanup records, source/config/tool/apt evidence, complete before/after
output-tree hash and symlink inventories, original C command files and object
identities, the complete selected normalizer command sidecar, final ELF-owner
observations, raw serial, strict observer output and
unchanged upstream parser JSON. The selected normalizer command's source/required
flags are checked and its exact bytes are bound to the output-tree hash inventory;
C and Rust normalizer command files are not required to be identical.
Empty diagnostic files are allowed only for
explicitly designated stderr/cleanup roles. Any missing file, cap overrun or
encoding/emission failure prevents success. Failures remain visible in the logs.

## Read-only download and comparison

After an authorized run, first verify its repository, event, feature branch,
approved activation commit, attempt 1, workflow identity and the two unique jobs
`unicode-c` and `unicode-rust` through authenticated read-only GitHub metadata.
Both jobs must be terminal successes. Preserve the fetched job text exactly as
UTF-8 and hash those saved files before comparison. Populate the example receipt
from that verified metadata and those local file hashes; never populate it from
success text in the logs themselves.

When a connector returns decoded text, the preserved bytes are the UTF-8 encoding
of that decoded text. They are not claimed to be original HTTP response bytes.
If the connector omits or truncates text, obtain a complete supported read or
stop; do not reconstruct or assume missing evidence. Retain the download receipt
and original saved text with the existing approved backup process.

Run:

    perl compare-logs.pl trusted-download-receipt.json C-job.log Rust-job.log

The comparator checks the download hashes before recognizing optional GitHub
timestamp prefixes. It then requires one uniquely ordered complete envelope and
one successful terminal record, verifies every payload/manifest digest and exact
named role, and checks the substantive stage, config, source, tool, command,
ownership, original-case and upstream-JSON evidence. C/R original C objects,
complete commands, package/tool closures and config values must match, except
the one normalizer selector. Full output and symlink inventories must remain
identical through each member's no-op, audit and guest.

This validates the evidence contract for logs obtained from the verified run.
An offline parser cannot authenticate an invented download receipt or distinguish
a fully fabricated, internally consistent report from its purported origin.
The GitHub metadata/download verification is therefore a required trust boundary.
The positive synthetic fixtures intentionally exercise the protocol; they are
never evidence of a kernel build or runtime pass.

Even a complete passing envelope retains observations and hashes, not final ELF
bytes. It remains a provisional logs-only C/R result. Independent later binary
reinspection requires separately approved bounded artifacts; no artifact upload
or binary-through-logs substitute is included here.

## Corrected harness and unchanged experiment

The prior attempt 37081883721 failed before any original C test ran. Its failure
and logs are retained. This request is one new bounded attempt, never a rerun.

The allocation sampler retries only exact disappearing-entry ENOENT failures,
at most three attempts inside one shared five-second budget. It accepts a
metric only after a complete successful rescan. Invalid, mixed, permission,
I/O, truncated or NUL-tainted output fails closed; resource caps do not change.

Configure/build/no-op now pass RUSTC=/usr/bin/rustc as an explicit make variable.
This is a build-integration change. Bare rustc must first resolve to that exact
path and executable SHA256 b4e139165f4f075f9a3fb4b20e7e4898472f08304961706072752d4aa11522b1.
No compiler flags change. The five original command files are retained before
the fail-closed ownership audit. Exact compiler/source and enabled overflow
checks remain required; any conflicting or disabled overflow setting fails.

Cleanup uses bounded command capture, monotonic deadlines, process-group
termination/reaping and an exact-CID absence receipt. It retains the existing
60-second reserve, 20/25-second operation limits and shared 20-second removal
and verification interval. Timeout, malformed or NUL-tainted output, failure
to reap, unknown daemon diagnostics and unproved absence all fail.

Static/synthetic controls do not establish real Docker compatibility or a
kernel/guest result. The prior missing normalizer command and final container
absence are not reconstructed. Authentic complete fresh C/R logs and unchanged
final-ELF ownership checks are still required for a provisional result.
