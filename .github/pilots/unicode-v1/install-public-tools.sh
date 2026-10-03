#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
# Newly authored installer using the official immutable Debian archive only.
set -euo pipefail
[[ ${PILOT_RUN_ATTEMPT:-} == 1 ]]
[[ $(dpkg --print-architecture) == amd64 ]]
[[ -r /usr/share/keyrings/debian-archive-keyring.pgp ]]
mkdir -p /work/evidence /work/apt-lists/partial /work/apt-cache/archives/partial
# HTTP bootstrap is authenticated by Debian's archive signature and the exact
# InRelease hash; the pinned slim image does not contain a TLS CA bundle yet.
cat > /work/snapshot.list <<'EOF'
deb [arch=amd64 signed-by=/usr/share/keyrings/debian-archive-keyring.pgp] http://snapshot.debian.org/archive/debian/20261002T142810Z/ trixie main
deb [arch=amd64 signed-by=/usr/share/keyrings/debian-archive-keyring.pgp] http://snapshot.debian.org/archive/debian/20261002T142810Z/ trixie-updates main
deb [arch=amd64 signed-by=/usr/share/keyrings/debian-archive-keyring.pgp] http://snapshot.debian.org/archive/debian-security/20261002T180123Z/ trixie-security main
EOF
apt_options=(-o Dir::Etc::sourcelist=/work/snapshot.list -o Dir::Etc::sourceparts=-
  -o Dir::State::lists=/work/apt-lists -o Dir::Cache=/work/apt-cache
  -o Acquire::Retries=0 -o APT::Install-Recommends=false -o APT::Install-Suggests=false)
export DEBIAN_FRONTEND=noninteractive
apt-get "${apt_options[@]}" --error-on=any update
: > /work/evidence/apt-suites.tsv
while read -r suite expected; do
  mapfile -t releases < <(find /work/apt-lists -maxdepth 1 -name "*_dists_${suite}_InRelease" -type f)
  [[ ${#releases[@]} == 1 ]]
  printf '%s  %s\n' "$expected" "${releases[0]}" | sha256sum -c -
  printf '%s\t%s\n' "$suite" "$expected" >> /work/evidence/apt-suites.tsv
done <<'LOCK'
trixie 0584fba32e13e0ab8285fb16c27adea1ec03a73669c18702821094fd6ca86675
trixie-updates bde606f5b1303e2c864d2118fca1a07c8aca59e097d10180f4e1ea1e7f53c99c
trixie-security f44452462d1d78526274ee79a391cd8e8df9dc327717dabc39a9d868eb7e4114
LOCK
dpkg-query -W -f='${binary:Package}\t${Version}\t${Architecture}\n' | LC_ALL=C sort > /work/evidence/base-packages.tsv
mapfile -t pins < <(awk -F '\t' 'NR>1 {print $1 "=" $2}' /pilot/public-packages-candidate.tsv)
# These build/observer dependencies come from the same immutable snapshot.
# Their complete selected versions and .deb hashes are emitted below.
deps=(git jq ca-certificates gnupg xz-utils curl make gcc pkg-config python3 perl
  binutils libc6-dev libssl-dev libelf-dev libdw-dev zlib1g-dev libzstd-dev
  time util-linux coreutils findutils diffutils gzip tar)
apt-get "${apt_options[@]}" --download-only -y install "${deps[@]}" "${pins[@]}"
# Keep all downloaded closure hashes, including automatically resolved packages.
for package in /work/apt-cache/archives/*.deb; do
  [[ -f $package ]] || continue
  name=$(dpkg-deb -f "$package" Package)
  version=$(dpkg-deb -f "$package" Version)
  arch=$(dpkg-deb -f "$package" Architecture)
  hash=$(sha256sum "$package" | cut -d' ' -f1)
  printf '%s\t%s\t%s\t%s\n' "$name" "$version" "$arch" "$hash"
  expected=$(awk -F '\t' -v n="$name" -v v="$version" -v a="$arch" 'NR>1 && $1==n && $2==v && $3==a {print $6}' /pilot/public-packages-candidate.tsv)
  [[ -z $expected || $expected == "$hash" ]]
done | LC_ALL=C sort > /work/evidence/downloaded-packages.tsv
apt-get "${apt_options[@]}" --no-download -y install "${deps[@]}" "${pins[@]}"
while IFS=$'\t' read -r name version arch bytes installed hash file; do
  [[ $name == Package ]] && continue
  [[ $(dpkg-query -W -f='${Version}' "$name") == "$version" ]]
done < /pilot/public-packages-candidate.tsv
dpkg-query -W -f='${binary:Package}\t${Version}\t${Architecture}\n' | LC_ALL=C sort > /work/evidence/installed-packages.tsv
cat /work/evidence/base-packages.tsv /work/evidence/downloaded-packages.tsv /work/evidence/installed-packages.tsv
sha256sum /work/evidence/*packages.tsv
