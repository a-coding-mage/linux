# Unarmed public Unicode pilot: revision v4 of the unreleased evidence protocol

This is a review candidate. No publication or run is authorized by these files.
The source remains `62a3294181556f17db863c9016524fa3068a374b`; original C tests,
config values, provider selectors, flags and guest settings retain their reviewed
profile. The setup-base gate is deliberately stale and must be refreshed to the
then-verified feature-branch tip after review. Do not change the experiment pin.

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

The setup commit adds both this workflow and the unarmed manifest. It can trigger
one request-only run, which is expected to reject the setup push and start no
member. After that expected outcome is verified, a separate manifest-only
activation commit can trigger one request job and the sequential C/R pair.
There is no skip directive or hidden trigger bypass. Approval must cover both
commits, both possible workflow runs and the specified public logs.

The request job has a 10-minute ceiling in each run. Each member has a proposed
120-minute GitHub job ceiling. Thus the complete planned setup-plus-activation
maximum is 260 standard-runner minutes: 10 + 10 + 120 + 120. It is not a measured
duration. The verified prospective base had no pre-existing workflow files; the
head and workflow inventory must be rechecked before publication.

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
