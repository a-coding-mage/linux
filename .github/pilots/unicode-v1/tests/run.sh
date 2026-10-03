#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
# Synthetic fixtures only. The git stub models reads; no repository/index/config
# is created or modified. All fixture writes remain inside this draft directory.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
work="$here/tests/generated"
mkdir -p "$work/bin" "$work/repo/.github/pilots"
cp "$here/../unicode-request.json" "$work/unarmed.json"
sed 's/"armed": false/"armed": true/' "$work/unarmed.json" > "$work/armed.json"
cat > "$work/bin/git" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
[[ $1 == -C ]]; shift 2
case "$*" in
  'rev-parse HEAD') echo aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;;
  'show -s --format=%P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa') echo "${STUB_PARENTS:-bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb}";;
  'show -s --format=%P bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb') echo "${STUB_SETUP_PARENT:-479ba34a322678167a144c7ebc69bc33d059842a}";;
  'cat-file -e bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb^{commit}') exit 0;;
  diff-tree*) printf '%b\n' "${STUB_CHANGES:-M\t.github/pilots/unicode-request.json}";;
  ls-tree*) printf '%s blob cccccccccccccccccccccccccccccccccccccccc\t.github/pilots/unicode-request.json\n' "${STUB_MODE:-100644}";;
  'show bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb:.github/pilots/unicode-request.json') cat "$STUB_WORK/unarmed.json";;
  *) printf 'unexpected fixture git command: %s\n' "$*" >&2; exit 2;;
esac
SH
chmod +x "$work/bin/git"
cat > "$work/base-event.json" <<'JSON'
{"repository":{"full_name":"a-coding-mage/linux","private":false},"ref":"refs/heads/feat/rust-translation-lupos","created":false,"deleted":false,"forced":false,"before":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb","after":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","commits":[{"id":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}],"head_commit":{"id":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}}
JSON
export PATH="$work/bin:$PATH" STUB_WORK="$work"
export GITHUB_EVENT_NAME=push GITHUB_RUN_ATTEMPT=1
export GITHUB_SHA=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
export GITHUB_REF=refs/heads/feat/rust-translation-lupos
cp "$work/armed.json" "$work/repo/.github/pilots/unicode-request.json"
run_gate() { bash "$here/gate.sh" "$work/repo" "$work/event.json" > "$work/last.stdout" 2> "$work/last.stderr"; }
cp "$work/base-event.json" "$work/event.json"
run_gate; echo 'PASS valid request/topology fixture'
reject_event() {
  jq "$1" "$work/base-event.json" > "$work/event.json"
  if run_gate; then echo "FAIL allowed $2"; exit 1; fi
  echo "PASS rejects $2"
}
reject_event '.forced=true' forced
reject_event '.created=true' new-branch
reject_event '.deleted=true' deleted-branch
reject_event '.ref="refs/heads/master"' default-branch
reject_event '.repository.private=true' private-repository
reject_event '.repository.full_name="other/linux"' wrong-repository
reject_event '.commits += .commits' multiple-commits
reject_event '.commits=[]' missing-commits
reject_event '.before="0000000000000000000000000000000000000000"' absent-before
reject_event '.head_commit.id="cccccccccccccccccccccccccccccccccccccccc"' mismatched-head
cp "$work/base-event.json" "$work/event.json"
export STUB_CHANGES=$'M\tREADME\nM\t.github/pilots/unicode-request.json'
if run_gate; then exit 1; fi; echo 'PASS rejects another changed file'; unset STUB_CHANGES
export STUB_CHANGES=$'M\tREADME'
if run_gate; then exit 1; fi; echo 'PASS rejects fail-open path filter'; unset STUB_CHANGES
export STUB_PARENTS='bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb cccccccccccccccccccccccccccccccccccccccc'
if run_gate; then exit 1; fi; echo 'PASS rejects merge topology'; unset STUB_PARENTS
export STUB_SETUP_PARENT=cccccccccccccccccccccccccccccccccccccccc
if run_gate; then exit 1; fi; echo 'PASS rejects repeated or wrong-base setup'; unset STUB_SETUP_PARENT
export STUB_MODE=120000
if run_gate; then exit 1; fi; echo 'PASS rejects symlink manifest'; unset STUB_MODE
GITHUB_RUN_ATTEMPT=2
if run_gate; then exit 1; fi; echo 'PASS rejects rerun'; GITHUB_RUN_ATTEMPT=1
cp "$work/unarmed.json" "$work/repo/.github/pilots/unicode-request.json"
if run_gate; then exit 1; fi; echo 'PASS rejects unarmed installation'
sed 's/unicode-public-62a3294-v3/unreviewed/' "$work/armed.json" > "$work/repo/.github/pilots/unicode-request.json"
if run_gate; then exit 1; fi; echo 'PASS rejects unreviewed request ID'
cp "$work/armed.json" "$work/repo/.github/pilots/unicode-request.json"
run_gate
set +e
bash "$here/admit-launch.sh" > "$work/admission.stdout" 2> "$work/admission.stderr"
result=$?
set -e
[[ $result == 78 ]]; echo 'PASS draft always blocks launch'
cat > "$work/serial.log" <<'LOG'
KTAP version 1
1..1
    KTAP version 1
    # Subtest: unicode_normalization
    1..4
    ok 1 check_supported_versions
    ok 2 check_utf8_comparisons
    ok 3 check_utf8_nfdicf
    ok 4 check_utf8_nfdi
ok 1 unicode_normalization
LOG
perl "$here/observe-original-cases.pl" "$work/serial.log"
sed 's/ok 4 /not ok 4 /' "$work/serial.log" > "$work/failure.log"
if perl "$here/observe-original-cases.pl" "$work/failure.log" >/dev/null 2>&1; then exit 1; fi
echo 'PASS serial observer rejects failed original case'
sed 's/ok 4 check_utf8_nfdi/ok 4 check_utf8_nfdi # SKIP/' "$work/serial.log" > "$work/skip.log"
if perl "$here/observe-original-cases.pl" "$work/skip.log" >/dev/null 2>&1; then exit 1; fi
echo 'PASS serial observer rejects skip'
cat "$work/serial.log" > "$work/extra-suite.log"
printf 'ok 2 unexpected_extra_suite\n' >> "$work/extra-suite.log"
if perl "$here/observe-original-cases.pl" "$work/extra-suite.log" >/dev/null 2>&1; then exit 1; fi
echo 'PASS serial observer rejects additional suite'
printf 'CONFIG_RUST_UNICODE_NORM=y\nCONFIG_DEBUG_INFO_NONE=y\n# CONFIG_RUST_UNICODE_CORE is not set\n' > "$work/preset"
cp "$work/preset" "$work/config"
perl "$here/compare-config.pl" "$work/preset" "$work/config" rust
sed 's/CONFIG_DEBUG_INFO_NONE=y/CONFIG_DEBUG_INFO_NONE=n/' "$work/preset" > "$work/config"
if perl "$here/compare-config.pl" "$work/preset" "$work/config" rust >/dev/null 2>&1; then exit 1; fi
echo 'PASS config observer rejects debug-feature drift'
perl "$here/tests/protocol.pl"
