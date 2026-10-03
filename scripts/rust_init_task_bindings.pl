#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
# Narrow bindgen representation adjustment, not a hand-maintained C layout.
# C stores a linker address in unsigned long thread.sp on x86_64. Rust const
# evaluation requires a pointer type to carry that relocation. Every byte of
# the generated structure remains in place; the Rust owner checks C-derived
# size, alignment, containing-field offset, and the adjusted field offset.
use strict;
use warnings;
local $/;
my $source = <>;
die "missing task layout constants\n"
    unless $source =~ /pub const RUST_INIT_TASK_SIZE\s*:/;
if ($source =~ /pub const RUST_INIT_TASK_SP_OFFSET\s*:/) {
    my $changed = 0;
    my $rewrite = sub {
        my ($start, $body, $end) = @_;
        $changed += ($body =~ s{\bpub sp\s*:\s*ffi\s*::\s*c_ulong\s*,}{pub sp: *mut ffi::c_ulong,}g);
        return $start . $body . $end;
    };
    my $structures = ($source =~ s~(pub struct thread_struct\s*\{)([^}]*)(\})~$rewrite->($1, $2, $3)~ge);
    die "thread.sp binding shape changed; inspect native layout\n"
        unless $structures == 1 && $changed == 1;
}
print $source;
