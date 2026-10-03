#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
# Source-only adapter of guest-userspace/setup-debian-tools.sh fetch/extract
# and guest-runs/boot-smoke-v1/prepare.sh. Called inside the existing CI guard;
# it neither installs packages nor launches/supervises a guest.
set -Eeuo pipefail
export LC_ALL=C LANG=C TZ=UTC
umask 022
unset TAR_OPTIONS GZIP BZIP2 XZ_OPT XZ_DEFAULTS
PILOT=/pilot
WORK=/work/O/boot-inputs
ROOT=$WORK/root
EVIDENCE=/work/evidence
GEN=/work/O/usr/gen_init_cpio
CAP=268435456
[[ $# == 0 && -d /src && -d /work/O && -d $EVIDENCE ]]
[[ ! -e $WORK && ! -L $WORK && -x $GEN && ! -L $GEN ]]
for tool in curl dpkg-deb sha256sum stat du df awk find sort xargs gzip cpio readlink cmp; do
  command -v "$tool" >/dev/null
done
printf '%s  %s\n' \
  03d8ea3e7b8eb73c36a0825f91eed8c9cc0767447b41654a58e0a70bcf842860 "$PILOT/boot-packages.tsv" \
  a80b17c42c08706a15528d23a58cecd36829599be2ac3c874bb0a2d0325d25c2 "$PILOT/boot-init" | sha256sum -c -
mkdir -p "$WORK"/{downloads,root,tmp,metadata,receipts}
export TMPDIR=$WORK/tmp
cp "$PILOT/boot-packages.tsv" "$WORK/metadata/packages.tsv"
component_budget() {
  local extra=${1:-0} allocated free
  allocated=$(du -s -B1 -- "$WORK" | awk '{print $1}')
  free=$(df -PB1 -- "$WORK" | awk 'NR==2 {print $4}')
  [[ $allocated =~ ^[0-9]+$ && $free =~ ^[0-9]+$ && $extra =~ ^[0-9]+$ ]]
  (( allocated + extra <= CAP && free >= 1073741824 + extra )) || {
    printf 'BOOT_COMPONENT_BUDGET_EXCEEDED allocated=%s reserve=%s cap=%s free=%s\n' "$allocated" "$extra" "$CAP" "$free" >&2
    exit 78
  }
}
# Exact historical seven-root dependency closure, with no mutable index lookup.
awk -F '\t' '
  NR==1 {if ($0!="Package\tVersion\tArchitecture\tSize\tInstalled-Size-KiB\tSHA256\tFilename") exit 1; next}
  NF!=7 || seen[$1]++ || $1!~/^[a-z0-9][a-z0-9+.-]*$/ || $2!~/^[A-Za-z0-9.+:~_-]+$/ ||
    $3!~/^(amd64|all)$/ || $4!~/^[1-9][0-9]*$/ || $5!~/^[1-9][0-9]*$/ ||
    $6!~/^[0-9a-f]+$/ || length($6)!=64 || $7!~/^pool\/[A-Za-z0-9+._~\/-]+\.deb$/ || index($7,"..") {bad=1}
  {n++; downloads+=$4; installed+=$5*1024}
  END {if (bad || n!=46 || downloads!=20125876 || installed!=77764608 ||
    !seen["bash"] || !seen["busybox"] || !seen["mount"] || !seen["util-linux"] ||
    !seen["coreutils"] || !seen["cpio"] || !seen["grep"]) exit 1}
' "$WORK/metadata/packages.tsv"
component_budget 20125876
printf 'Package\tVersion\tArchitecture\tSize\tSHA256\n' > "$WORK/metadata/package-identities.tsv"
# Retain the original fetch/extract operations, with fixed rows instead of
# metadata/plan and tighter per-download limits inside the existing stage guard.
while IFS=$'\t' read -r package version arch bytes installed digest path; do
  [[ $package == Package ]] && continue
  filename=${path##*/}
  deb=$WORK/downloads/$filename
  component_budget "$((bytes + 4096))"
  curl --disable --fail --location --proto '=https' --proto-redir '=https' \
    --retry 0 --connect-timeout 20 --max-time 90 --max-filesize "$bytes" \
    --output "$deb.partial" "https://deb.debian.org/debian/$path"
  [[ $(stat -c %s -- "$deb.partial") == "$bytes" ]]
  printf '%s  %s\n' "$digest" "$deb.partial" | sha256sum -c -
  mv -- "$deb.partial" "$deb"
  [[ $(dpkg-deb -f "$deb" Package) == "$package" ]]
  [[ $(dpkg-deb -f "$deb" Version) == "$version" ]]
  [[ $(dpkg-deb -f "$deb" Architecture) == "$arch" ]]
  printf '%s\t%s\t%s\t%s\t%s\n' "$package" "$version" "$arch" "$bytes" "$digest" >> "$WORK/metadata/package-identities.tsv"
  component_budget "$((installed * 1024 + 1048576))"
  dpkg-deb --extract "$deb" "$ROOT"
  component_budget
done < "$WORK/metadata/packages.tsv"

# Same merged-/usr layout and Bash-as-sh choice as the historical fixture.
for alias in bin sbin lib lib64; do
  if [[ ! -e $ROOT/$alias && ! -L $ROOT/$alias ]]; then
    ln -s "usr/$alias" "$ROOT/$alias"
  fi
  [[ -L $ROOT/$alias && $(readlink "$ROOT/$alias") == "usr/$alias" ]]
done
[[ ! -e $ROOT/usr/bin/sh && ! -L $ROOT/usr/bin/sh ]]
ln -s bash "$ROOT/usr/bin/sh"
for mount in proc sys dev run tmp; do
  [[ ! -L $ROOT/$mount ]]
  mkdir -p "$ROOT/$mount"
  chmod 0755 "$ROOT/$mount"
done
[[ ! -e $ROOT/init && ! -L $ROOT/init ]]
cp "$PILOT/boot-init" "$ROOT/init"
chmod 0755 "$ROOT/init"
cmp "$PILOT/boot-init" "$ROOT/init"
bash -n "$ROOT/init"
for tool in bash busybox mount mountpoint uname cat cpio grep sleep; do
  [[ -f $ROOT/usr/bin/$tool && -x $ROOT/usr/bin/$tool ]]
done
[[ $(readlink "$ROOT/usr/bin/sh") == bash ]]
[[ ! -e $ROOT/dev/console && ! -L $ROOT/dev/console && ! -e $ROOT/dev/null && ! -L $ROOT/dev/null ]]

# Reject names the original gen_init_cpio list format cannot represent before
# serializing metadata. No device nodes are ever created on the host.
find "$ROOT" -print0 > "$WORK/tmp/payload-paths"
while IFS= read -r -d '' entry; do
  [[ $entry != *[$' \t\r\n\v\f']* && $entry != *$'\\'* ]]
  if [[ -L $entry ]]; then
    target=$(readlink "$entry")
    [[ -n $target && $target != *[$' \t\r\n\v\f']* && $target != *$'\\'* ]]
  fi
done < "$WORK/tmp/payload-paths"
find "$ROOT" -printf '%y\t%m\t%P\t%l\n' | sort > "$WORK/metadata/payload-layout.tsv"
while IFS=$'\t' read -r kind mode relative target; do
  [[ -n $relative ]] || continue
  case $kind in
    d) printf 'dir /%s %s 0 0\n' "$relative" "$mode" ;;
    f) printf 'file /%s %s/%s %s 0 0\n' "$relative" "$ROOT" "$relative" "$mode" ;;
    l) printf 'slink /%s %s %s 0 0\n' "$relative" "$target" "$mode" ;;
    *) printf 'Unexpected payload type: %s\n' "$kind" >&2; exit 1 ;;
  esac
done < "$WORK/metadata/payload-layout.tsv" > "$WORK/initramfs.list"
printf 'nod /dev/console 0600 0 0 c 5 1\nnod /dev/null 0666 0 0 c 1 3\n' >> "$WORK/initramfs.list"
find "$ROOT" -type f -print0 | sort -z | xargs -0 sha256sum > "$WORK/metadata/payload-files.sha256"
find "$ROOT" -depth ! -type l -exec chmod a-w -- {} +
find "$ROOT" -printf '%y\t%m\t%P\t%l\n' | sort > "$WORK/metadata/payload-host-layout.tsv"
component_budget 83886080
"$GEN" -t 0 -o "$WORK/initramfs.cpio" "$WORK/initramfs.list"
component_budget "$(( $(stat -c %s "$WORK/initramfs.cpio") + 1048576 ))"
gzip -n -6 -c "$WORK/initramfs.cpio" > "$WORK/initramfs.cpio.gz"
gzip -t "$WORK/initramfs.cpio.gz"
cpio -itv < "$WORK/initramfs.cpio" > "$WORK/receipts/cpio-listing.txt" 2> "$WORK/receipts/cpio-listing.stderr"
component_budget
# Regenerate through pipes: prove both newc and gzip bytes without a second
# full-sized archive or an unmeasured temporary tree.
"$GEN" -t 0 "$WORK/initramfs.list" | sha256sum > "$WORK/metadata/repeat-initramfs.sha256"
"$GEN" -t 0 "$WORK/initramfs.list" | gzip -n -6 -c | sha256sum > "$WORK/metadata/repeat-gzip.sha256"
first_cpio=$(sha256sum "$WORK/initramfs.cpio" | cut -d' ' -f1)
first_gzip=$(sha256sum "$WORK/initramfs.cpio.gz" | cut -d' ' -f1)
[[ $first_cpio == "$(cut -d' ' -f1 "$WORK/metadata/repeat-initramfs.sha256")" ]]
[[ $first_gzip == "$(cut -d' ' -f1 "$WORK/metadata/repeat-gzip.sha256")" ]]
printf 'artifact\tsize_bytes\tfirst_sha256\trepeated_sha256\ninitramfs.cpio\t%s\t%s\t%s\ninitramfs.cpio.gz\t%s\t%s\t%s\n' \
  "$(stat -c %s "$WORK/initramfs.cpio")" "$first_cpio" "$first_cpio" \
  "$(stat -c %s "$WORK/initramfs.cpio.gz")" "$first_gzip" "$first_gzip" > "$WORK/metadata/determinism.tsv"

# Match the historical combined QEMU/SeaBIOS search directory. Record every
# resolved installed firmware file, including distribution symlinks. These
# links are outside the guest root, so no host firmware is embedded in newc.
mkdir "$WORK/qemu-data"
for firmware_root in /usr/share/qemu /usr/share/seabios; do
  [[ -d $firmware_root ]]
  find -L "$firmware_root" -type f -print0 | sort -z > "$WORK/tmp/firmware-paths"
  while IFS= read -r -d '' firmware; do
    relative=${firmware#"$firmware_root/"}
    [[ -n $relative && $relative != *[$' \t\r\n\v\f']* && $relative != *$'\\'* ]]
    destination=$WORK/qemu-data/$relative
    if [[ -e $destination || -L $destination ]]; then
      [[ -L $destination ]]
      cmp "$firmware" "$destination"
    else
      mkdir -p "${destination%/*}"
      ln -s "$firmware" "$destination"
    fi
  done < "$WORK/tmp/firmware-paths"
done
[[ -s $WORK/qemu-data/bios-256k.bin ]]
find "$WORK/qemu-data" -type l -printf '%p\t%l\n' | sort > "$WORK/metadata/firmware-links.tsv"
find -L "$WORK/qemu-data" -type f -print0 | sort -z | xargs -0 -r sha256sum > "$WORK/metadata/firmware.sha256"
[[ -s $WORK/metadata/firmware.sha256 && -s $WORK/metadata/firmware-links.tsv ]]
sha256sum "$WORK"/initramfs.{list,cpio,cpio.gz} "$WORK/metadata/"{packages.tsv,package-identities.tsv,payload-layout.tsv,payload-host-layout.tsv,payload-files.sha256,repeat-initramfs.sha256,repeat-gzip.sha256,determinism.tsv,firmware-links.tsv,firmware.sha256} > "$WORK/metadata/fixture.sha256"
find "$WORK" -type f -exec chmod a-w -- {} +
component_budget

# Complete text-only manifests. Compare actual archive sizes and digests across
# the independent members; no binary or binary encoding enters public evidence.
# /work/evidence is subject to the existing guard's 128 MiB allocation cap.
cp "$WORK/metadata/packages.tsv" "$EVIDENCE/boot-packages.tsv"
cp "$WORK/metadata/package-identities.tsv" "$EVIDENCE/boot-package-identities.tsv"
cp "$WORK/metadata/payload-layout.tsv" "$EVIDENCE/boot-layout.tsv"
cp "$WORK/metadata/payload-host-layout.tsv" "$EVIDENCE/boot-host-layout.tsv"
cp "$WORK/metadata/payload-files.sha256" "$EVIDENCE/boot-files.sha256"
cp "$WORK/metadata/determinism.tsv" "$EVIDENCE/boot-determinism.tsv"
cp "$WORK/metadata/fixture.sha256" "$EVIDENCE/boot-fixture.sha256"
cp "$WORK/metadata/firmware.sha256" "$EVIDENCE/boot-firmware.sha256"
cp "$WORK/metadata/firmware-links.tsv" "$EVIDENCE/boot-firmware-links.tsv"
cp "$WORK/initramfs.list" "$EVIDENCE/boot-initramfs.list"
printf 'BOOT_FIXTURE_PREPARED packages=46 roots=bash,busybox,mount,util-linux,coreutils,cpio,grep deterministic=pass component_cap_bytes=%s guest_started=no\n' "$CAP"
