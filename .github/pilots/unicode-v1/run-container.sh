#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
set -euo pipefail
[[ ${PILOT_RUN_ATTEMPT:-} == 1 ]] || { echo RERUN_REJECTED; exit 78; }
[[ ${PILOT_PROVIDER:-} == c || ${PILOT_PROVIDER:-} == rust ]]
[[ ${PILOT_RUN_ID:-} =~ ^[0-9]+$ && ${PILOT_TRIGGER_SHA:-} =~ ^[0-9a-f]{40}$ ]]
for key in PILOT_JOB_STARTED_EPOCH PILOT_JOB_DEADLINE_EPOCH PILOT_CONTAINER_STARTED_EPOCH PILOT_CONTAINER_DEADLINE_EPOCH; do
  [[ ${!key:-} =~ ^[1-9][0-9]{9,11}$ ]] || exit 78
done
mkdir -p /work/evidence /work/O
printf '{"provider":"%s","run_id":"%s","trigger_sha":"%s","run_attempt":1,"source_sha":"62a3294181556f17db863c9016524fa3068a374b","job_started_epoch":%s,"job_deadline_epoch":%s,"container_started_epoch":%s,"container_deadline_epoch":%s}\n' "$PILOT_PROVIDER" "$PILOT_RUN_ID" "$PILOT_TRIGGER_SHA" "$PILOT_JOB_STARTED_EPOCH" "$PILOT_JOB_DEADLINE_EPOCH" "$PILOT_CONTAINER_STARTED_EPOCH" "$PILOT_CONTAINER_DEADLINE_EPOCH" > /work/evidence/context.json
bash /pilot/guard.sh install 1200 bash /pilot/install-public-tools.sh
export PATH=/usr/lib/llvm-19/bin:$PATH
export RUSTC=/usr/bin/rustc RUSTFMT=/usr/bin/rustfmt BINDGEN=/usr/bin/bindgen
export RUST_LIB_SRC=/usr/src/rustc-1.85.1/library LIBCLANG_PATH=/usr/lib/llvm-19/lib
export RUSTC_BOOTSTRAP=1 LC_ALL=C TZ=UTC
export GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=safe.directory GIT_CONFIG_VALUE_0=/src
export KBUILD_BUILD_USER=unicode-pilot KBUILD_BUILD_HOST=public-ci KBUILD_BUILD_VERSION=1
export SOURCE_DATE_EPOCH="$(git -C /src show -s --format=%ct HEAD)"
export KBUILD_BUILD_TIMESTAMP="$(date -u -d "@$SOURCE_DATE_EPOCH" '+%a %b %d %T UTC %Y')"
cp /pilot/public-source.sha256 /work/evidence/source.lock
bash /pilot/verify-source.sh /src > /work/evidence/source.before.txt
{
  clang --version; ld.lld --version; rustc -vV; bindgen --version
  rustfmt --version; pahole --version; qemu-system-x86_64 --version
  make --version; gcc --version
} > /work/evidence/tool-versions.txt
[[ $(clang -dumpversion) == 19.1.7 ]]
[[ $(rustc --version | awk '{print $2}') == 1.85.1 ]]
[[ $(bindgen --version) == 'bindgen 0.71.1' ]]
[[ $(pahole --version) == v1.30 ]]
sha256sum /usr/bin/rustc /usr/bin/rustfmt /usr/bin/bindgen /usr/bin/pahole /usr/bin/qemu-system-x86_64 /usr/share/qemu/qboot.rom > /work/evidence/tool-binaries.sha256
cp /pilot/config.symbols /work/O/.config
if [[ $PILOT_PROVIDER == c ]]; then sed -i 's/^CONFIG_RUST_UNICODE_NORM=y$/# CONFIG_RUST_UNICODE_NORM is not set/' /work/O/.config; fi
make_cmd=(make -C /src O=/work/O ARCH=x86_64 LLVM=1 HOST_TOOLS_LANG=rust -j2)
bash /pilot/guard.sh configure 600 "${make_cmd[@]}" olddefconfig
perl /pilot/compare-config.pl /pilot/config.symbols /work/O/.config "$PILOT_PROVIDER" > /work/evidence/config-check.txt
cp /work/O/.config /work/evidence/config.full
bash /pilot/guard.sh build 1800 "${make_cmd[@]}" bzImage
cmp /work/O/.config /work/evidence/config.full
hash_output() { (cd /work/O && find . -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum); }
link_output() { printf 'path\ttarget\n'; (cd /work/O && find . -type l -printf '%p\t%l\n' | LC_ALL=C sort); }
hash_output > /work/evidence/O-before-noop.sha256; link_output > /work/evidence/O-links-before-noop.tsv
bash /pilot/guard.sh noop 600 "${make_cmd[@]}" bzImage
hash_output > /work/evidence/O-after-noop.sha256; link_output > /work/evidence/O-links-after-noop.tsv
cmp /work/evidence/O-before-noop.sha256 /work/evidence/O-after-noop.sha256
cmp /work/evidence/O-links-before-noop.tsv /work/evidence/O-links-after-noop.tsv
bash /pilot/guard.sh ownership 300 bash /pilot/audit-commands.sh "$PILOT_PROVIDER" /work/O
sha256sum /work/O/arch/x86/boot/bzImage /work/O/vmlinux /work/O/vmlinux.unstripped /work/O/vmlinux.o /work/O/.config > /work/evidence/image-identities.sha256
sha256sum /work/O/fs/unicode/tests/utf8_kunit.o /work/O/fs/unicode/utf8-core.o /work/O/fs/unicode/utf8data.o /work/O/lib/crc/crc16.o > /work/evidence/object-identities.sha256
cp /work/O/fs/unicode/tests/.utf8_kunit.o.cmd /work/evidence/test.cmd
cp /work/O/fs/unicode/.utf8-core.o.cmd /work/evidence/core.cmd
cp /work/O/fs/unicode/.utf8data.o.cmd /work/evidence/data.cmd
cp /work/O/lib/crc/.crc16.o.cmd /work/evidence/crc.cmd
cp /work/O/fs/unicode/.utf8-norm.o.cmd /work/evidence/norm.cmd
hash_output > /work/evidence/O-before-guest.sha256; link_output > /work/evidence/O-links-before-guest.tsv
cmp /work/evidence/O-after-noop.sha256 /work/evidence/O-before-guest.sha256
cmp /work/evidence/O-links-after-noop.tsv /work/evidence/O-links-before-guest.tsv
guest=(qemu-system-x86_64 -L /usr/share/qemu -nodefaults -m 2048
  -kernel /work/O/arch/x86/boot/bzImage
  -append 'console=ttyS0 kunit.enable=1 kunit.autorun=1 kunit.filter_glob=unicode_normalization kunit_shutdown=reboot panic=-1 oops=panic nokaslr'
  -no-reboot -nographic -accel tcg,thread=single -cpu max -smp 1 -serial stdio -nic none -bios qboot.rom)
bash /pilot/admit-guest.sh > /work/evidence/guest-admission.json
set +e
bash /pilot/guard.sh guest 1020 timeout --signal=TERM --kill-after=10 900 "${guest[@]}"; guest_status=$?
bash /pilot/guard.sh parse 300 bash /pilot/parse-guest.sh; parse_status=$?
set -e
hash_output > /work/evidence/O-after-guest.sha256; link_output > /work/evidence/O-links-after-guest.tsv
cmp /work/evidence/O-before-guest.sha256 /work/evidence/O-after-guest.sha256
cmp /work/evidence/O-links-before-guest.tsv /work/evidence/O-links-after-guest.tsv
cmp /work/O/.config /work/evidence/config.full
bash /pilot/verify-source.sh /src > /work/evidence/source.after.txt
(( guest_status == 0 && parse_status == 0 ))
printf 'provider=%s source=62a3294181556f17db863c9016524fa3068a374b config=pass noop=pass ownership=pass guest=pass parser=pass invariance=pass\n' "$PILOT_PROVIDER" > /work/evidence/member.status
