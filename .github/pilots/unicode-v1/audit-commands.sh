#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
set -euo pipefail
provider=${1:?c or rust}; out=${2:?output tree}
[[ $provider == c || $provider == rust ]]
cd "$out"
extension=c; [[ $provider == c ]] || extension=rs
grep -Fx "source_fs/unicode/utf8-norm.o := /src/fs/unicode/utf8-norm.$extension" fs/unicode/.utf8-norm.o.cmd
grep -Fx 'source_fs/unicode/utf8-core.o := /src/fs/unicode/utf8-core.c' fs/unicode/.utf8-core.o.cmd
grep -Fx 'source_fs/unicode/tests/utf8_kunit.o := /src/fs/unicode/tests/utf8_kunit.c' fs/unicode/tests/.utf8_kunit.o.cmd
grep -Fx 'source_lib/crc/crc16.o := /src/lib/crc/crc16.c' lib/crc/.crc16.o.cmd
cmp fs/unicode/utf8data.c /src/fs/unicode/utf8data.c_shipped
grep -F -- '-fno-strict-overflow' fs/unicode/tests/.utf8_kunit.o.cmd
if [[ $provider == rust ]]; then
  grep -F -- '/usr/bin/rustc' fs/unicode/.utf8-norm.o.cmd
  grep -F -- '-Coverflow-checks=y' fs/unicode/.utf8-norm.o.cmd
  test -s rust/bindings/utf8_norm_generated.rs
  test -s rust/bindings/.utf8_norm_generated.rs.cmd
  cat rust/bindings/.utf8_norm_generated.rs.cmd
fi
cat .vmlinux.a.cmd .vmlinux.o.cmd .vmlinux.unstripped.cmd .vmlinux.cmd
grep -F -- '--whole-archive vmlinux.a' .vmlinux.o.cmd
llvm-ar t vmlinux.a > /work/evidence/archive-members.txt
cat /work/evidence/archive-members.txt
for object in fs/unicode/utf8-norm.o fs/unicode/utf8-core.o fs/unicode/utf8data.o fs/unicode/tests/utf8_kunit.o lib/crc/crc16.o; do
  [[ $(sed 's@^/work/O/@@' /work/evidence/archive-members.txt | grep -Fxc "$object") == 1 ]]
done
if grep -E '/\.rust-listing(-elf)?/|utf8-core-helpers\.o|utf8_kunit\.rs|utf8-norm\.rs\.o' /work/evidence/archive-members.txt; then exit 1; fi
for path in fs/unicode/tests/.utf8_kunit.o.cmd fs/unicode/.utf8-core.o.cmd fs/unicode/.utf8data.o.cmd lib/crc/.crc16.o.cmd; do
  cat "$path"
  # Identical absolute /src and /work/O paths mean the complete commands can be
  # compared without removing flags, dependencies or meaningful source text.
  sha256sum "$path"
done
sha256sum fs/unicode/tests/utf8_kunit.o fs/unicode/utf8-core.o fs/unicode/utf8data.o lib/crc/crc16.o
perl /pilot/audit-elf.pl "$out"
echo SELECTED_COMMAND_ARCHIVE_AND_FINAL_OWNER_PASS
