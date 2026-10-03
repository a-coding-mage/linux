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
printf '{"provider":"%s","run_id":"%s","trigger_sha":"%s","run_attempt":1,"source_sha":"0008179a1ee0b082fa3ead118187fc6f3569a9f2","job_started_epoch":%s,"job_deadline_epoch":%s,"container_started_epoch":%s,"container_deadline_epoch":%s}\n' "$PILOT_PROVIDER" "$PILOT_RUN_ID" "$PILOT_TRIGGER_SHA" "$PILOT_JOB_STARTED_EPOCH" "$PILOT_JOB_DEADLINE_EPOCH" "$PILOT_CONTAINER_STARTED_EPOCH" "$PILOT_CONTAINER_DEADLINE_EPOCH" > /work/evidence/context.json
bash /pilot/guard.sh install 1200 bash /pilot/install-public-tools.sh
export PATH=/usr/lib/llvm-19/bin:$PATH
export RUSTC=/usr/bin/rustc HOSTRUSTC=/usr/bin/rustc RUSTFMT=/usr/bin/rustfmt BINDGEN=/usr/bin/bindgen
export RUST_LIB_SRC=/usr/src/rustc-1.85.1/library LIBCLANG_PATH=/usr/lib/llvm-19/lib
export RUSTC_BOOTSTRAP=1 LC_ALL=C TZ=UTC
export GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=safe.directory GIT_CONFIG_VALUE_0=/src
export KBUILD_BUILD_USER=lupos-integrated KBUILD_BUILD_HOST=public-ci KBUILD_BUILD_VERSION=1
export SOURCE_DATE_EPOCH="$(git -C /src show -s --format=%ct HEAD)"
export KBUILD_BUILD_TIMESTAMP="$(date -u -d "@$SOURCE_DATE_EPOCH" '+%a %b %d %T UTC %Y')"
cp /pilot/public-source.sha256 /work/evidence/source.lock
bash /pilot/verify-source.sh /src > /work/evidence/source.before.txt
# Bare-name resolution and the explicit make selection must name the same approved binary.
[[ $(command -v rustc) == /usr/bin/rustc ]]
{
  printf 'RUSTC_PATH_EQUIVALENCE bare=rustc resolved=/usr/bin/rustc requested=/usr/bin/rustc\n'
  clang --version; ld.lld --version; rustc -vV; bindgen --version
  rustfmt --version; pahole --version; qemu-system-x86_64 --version
  make --version; gcc --version
} > /work/evidence/tool-versions.txt
[[ $(clang -dumpversion) == 19.1.7 ]]
[[ $(rustc --version | awk '{print $2}') == 1.85.1 ]]
[[ $(bindgen --version) == 'bindgen 0.71.1' ]]
[[ $(pahole --version) == v1.30 ]]
printf 'b4e139165f4f075f9a3fb4b20e7e4898472f08304961706072752d4aa11522b1  /usr/bin/rustc\n' | sha256sum -c -
sha256sum /usr/bin/rustc /usr/bin/rustfmt /usr/bin/bindgen /usr/bin/pahole /usr/bin/qemu-system-x86_64 /usr/share/qemu/qboot.rom > /work/evidence/tool-binaries.sha256
printf '%s\n' "$(git -C /src rev-parse HEAD)" > /work/evidence/source.commit
cp /pilot/config-lock.json /work/evidence/config-lock.json
# Exactly six incremental providers differ. Original Kconfig resolves both.
unset KCONFIG_CONFIG KCONFIG_ALLCONFIG MAKEFLAGS MFLAGS MAKELEVEL MAKEOVERRIDES CARGO_MAKEFLAGS
gzip -dc /pilot/integrated-rust.config.gz > /work/O/.config
if [[ $PILOT_PROVIDER == c ]]; then
  for selector in RUST_BPF_LPM_TRIE RUST_BPF_TOKEN RUST_POWER_PROCESS RUST_BLK_CRYPTO_FALLBACK RUST_SELINUX_SERVICES RUST_SELINUX_POLICYDB; do
    /src/scripts/config --file /work/O/.config --disable "$selector"
  done
fi
make_cmd=(make -C /src O=/work/O ARCH=x86_64 LLVM=1 HOST_TOOLS_LANG=rust RUSTC=/usr/bin/rustc HOSTRUSTC=/usr/bin/rustc BINDGEN=/usr/bin/bindgen -j2)
bash /pilot/guard.sh configure 600 "${make_cmd[@]}" olddefconfig
perl /pilot/compare-config.pl /pilot/integrated-rust.config.gz /work/O/.config "$PILOT_PROVIDER" /pilot/config-lock.json > /work/evidence/config-check.txt
cp /work/O/.config /work/evidence/config.full
bash /pilot/guard.sh build 3600 "${make_cmd[@]}" vmlinux modules bzImage usr_gen_init_cpio
cmp /work/O/.config /work/evidence/config.full
# The fixture lives inside the original measured O cap and is sealed before
# the no-op inventory. Original kernel build output and source stay intact.
bash /pilot/guard.sh fixture 360 bash /pilot/prepare-boot.sh
hash_output() { (cd /work/O && find . -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum); }
link_output() { printf 'path\ttarget\n'; (cd /work/O && find . -type l -printf '%p\t%l\n' | LC_ALL=C sort); }
hash_output > /work/evidence/O-before-noop.sha256; link_output > /work/evidence/O-links-before-noop.tsv
bash /pilot/guard.sh noop 600 "${make_cmd[@]}" vmlinux modules bzImage usr_gen_init_cpio
hash_output > /work/evidence/O-after-noop.sha256; link_output > /work/evidence/O-links-after-noop.tsv
cmp /work/evidence/O-before-noop.sha256 /work/evidence/O-after-noop.sha256
cmp /work/evidence/O-links-before-noop.tsv /work/evidence/O-links-after-noop.tsv
bash /pilot/guard.sh ownership 600 bash /pilot/audit-integrated.sh "$PILOT_PROVIDER" /work/O
sha256sum /work/O/arch/x86/boot/bzImage /work/O/vmlinux /work/O/vmlinux.unstripped /work/O/vmlinux.o /work/O/vmlinux.a /work/O/.config /work/O/usr/gen_init_cpio /work/O/Module.symvers /work/O/modules.order /work/O/include/config/kernel.release > /work/evidence/image-identities.sha256
# Each run binds the actual new immutable image, payload and firmware identities.
cat /work/O/boot-inputs/metadata/fixture.sha256 /work/O/boot-inputs/metadata/payload-files.sha256 /work/evidence/boot-firmware.sha256 > /work/evidence/boot-inputs.sha256
sha256sum /work/O/boot-inputs/metadata/fixture.sha256 /pilot/boot-init /work/O/arch/x86/boot/bzImage /work/O/.config /work/O/usr/gen_init_cpio /work/O/include/config/kernel.release /work/evidence/source.commit /usr/bin/qemu-system-x86_64 >> /work/evidence/boot-inputs.sha256
sha256sum -c /work/evidence/boot-inputs.sha256 > /work/evidence/boot-inputs.before.txt
perl -MJSON::PP -MDigest::SHA=sha256_hex -e '
 sub raw{open my$f,"<:raw",$_[0]or die$!;local$/;return<$f>}
 my$source=raw("/work/evidence/source.commit");chomp$source;
 my$release=raw("/work/O/include/config/kernel.release");chomp$release;
 print JSON::PP->new->canonical->encode({source_sha=>$source,kernel_release=>$release,immutable_inputs_sha256=>sha256_hex(raw("/work/evidence/boot-inputs.sha256"))}),"\n";
' > /work/evidence/boot-context.json
hash_output > /work/evidence/O-before-guest.sha256; link_output > /work/evidence/O-links-before-guest.tsv
cmp /work/evidence/O-after-noop.sha256 /work/evidence/O-before-guest.sha256
cmp /work/evidence/O-links-after-noop.tsv /work/evidence/O-links-before-guest.tsv
# KUnit is disabled only for the historical boot-only fixture. Original C suites
# run next in their own unchanged, explicitly enabled guest commands.
boot=(qemu-system-x86_64 -L /work/O/boot-inputs/qemu-data -machine q35
  -accel tcg,thread=single -cpu max -smp 1 -m 2048 -nic none -nodefaults
  -display none -serial stdio -monitor none -no-reboot
  -kernel /work/O/arch/x86/boot/bzImage -initrd /work/O/boot-inputs/initramfs.cpio.gz
  -append 'console=ttyS0,115200 rdinit=/init panic=-1 oops=panic nokaslr kunit.enable=0')
bash /pilot/admit-guest.sh boot > /work/evidence/boot.admission.json
set +e
bash /pilot/guard.sh boot 360 timeout --verbose --signal=TERM --kill-after=10 240 "${boot[@]}"; boot_status=$?
set -e
sha256sum /work/evidence/boot.log > /work/evidence/boot-serial.sha256
set +e
bash /pilot/guard.sh boot-parse 120 perl /pilot/verify-boot.pl /work/evidence/boot.log /work/evidence/boot.status /work/evidence/source.commit /work/O/include/config/kernel.release /work/evidence/boot-inputs.sha256; boot_parse=$?
set -e
sha256sum -c /work/evidence/boot-inputs.sha256 > /work/evidence/boot-inputs.after.txt
(( boot_status == 0 && boot_parse == 0 ))
for suite in crc chacha20poly1305 iov_iter; do
  # guard stage names deliberately retain the original suite name below.
  case $suite in chacha20poly1305) stage=chacha;; iov_iter) stage=iov;; *) stage=crc;; esac
  guest=(qemu-system-x86_64 -L /usr/share/qemu -nodefaults -m 2048
    -kernel /work/O/arch/x86/boot/bzImage
    -append "console=ttyS0 kunit.enable=1 kunit.autorun=1 kunit.filter_glob=$suite kunit_shutdown=reboot panic=-1 oops=panic nokaslr"
    -no-reboot -nographic -accel tcg,thread=single -cpu max -smp 1 -serial stdio -nic none -bios qboot.rom)
  bash /pilot/admit-guest.sh "$stage" > "/work/evidence/$stage.admission.json"
  set +e
  bash /pilot/guard.sh "$stage" 1020 timeout --signal=TERM --kill-after=10 900 "${guest[@]}"; guest_status=$?
  set -e
  # Retain the native receipt from the existing guard; never infer QEMU success
  # from serial text or from a supervisor's unrelated return status.
  native=$(sed -n 's/.*native_exit=\([0-9]*\).*/\1/p' "/work/evidence/$stage.status")
  [[ $native =~ ^[0-9]+$ ]]
  printf '%s\n' "$native" > "/work/evidence/$stage.qemu-exit"
  set +e
  bash /pilot/guard.sh "$stage-parse" 300 bash /pilot/parse-guest.sh "$suite" "$stage"; parse_status=$?
  set -e
  (( guest_status == 0 && parse_status == 0 ))
done
hash_output > /work/evidence/O-after-guest.sha256; link_output > /work/evidence/O-links-after-guest.tsv
cmp /work/evidence/O-before-guest.sha256 /work/evidence/O-after-guest.sha256
cmp /work/evidence/O-links-before-guest.tsv /work/evidence/O-links-after-guest.tsv
cmp /work/O/.config /work/evidence/config.full
bash /pilot/verify-source.sh /src > /work/evidence/source.after.txt
printf 'provider=%s source=%s scope=incremental-six-core config=pass noop=pass ownership=pass boot=pass original_C_cases=34 original_C_suites=3 parser=pass invariance=pass new_provider_functional_suites=UNRUN\n' "$PILOT_PROVIDER" "$(cat /work/evidence/source.commit)" > /work/evidence/member.status
