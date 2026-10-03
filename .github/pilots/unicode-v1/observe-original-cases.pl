#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
# Newly authored second observer; the unchanged upstream parser is also required.
use strict; use warnings;
my @expected = qw(check_supported_versions check_utf8_comparisons check_utf8_nfdicf check_utf8_nfdi);
my (@seen, $suite, $plan, $summary, $failed, $top_plan);
while (<>) {
    s/\r\n/\n/g;
    $failed++ if /\bnot ok\b|#\s*SKIP\b|Bail out!|Kernel panic|Oops:|BUG:|KASAN:|UBSAN:/i;
    if (/^1\.\.1\s*$/) { $top_plan++; next; }
    if (/^\s*# Subtest: unicode_normalization\s*$/) { $suite++; next; }
    if ($suite && !defined($summary) && /^\s+1\.\.4\s*$/) { $plan++; next; }
    if (/^\s+ok\s+([1-4])\s+(check_[A-Za-z0-9_]+)\s*$/) {
        die "case outside suite\n" unless $suite && !defined($summary);
        push @seen, "$1:$2";
        next;
    }
    if (/^ok\s+1\s+unicode_normalization\s*$/) { $summary++; next; }
    $failed++ if /^\s*(?:(?:not\s+)?ok\b|\d+\.\.\d+\b|# Subtest:)/;
}
die "nonpassing or incomplete serial result\n" if $failed || ($suite // 0) != 1 || ($plan // 0) != 1 || ($summary // 0) != 1 || ($top_plan // 0) != 1;
my @want = map { ($_+1) . ':' . $expected[$_] } 0..$#expected;
die "original cases missing, duplicated or out of order\n" unless join(',', @seen) eq join(',', @want);
print "ORIGINAL_C_CASES_PASS ", join(',', @expected), "\n";
