#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
use strict;use warnings;use FindBin;use lib $FindBin::Bin;use CompareEvidence;
die "usage: compare-logs.pl trusted-download-receipt.json C-job.log Rust-job.log\n" unless @ARGV==3;
my $receipt=$Evidence::JSON->decode(Evidence::slurp($ARGV[0]));
CompareEvidence::compare($receipt,Evidence::slurp($ARGV[1]),Evidence::slurp($ARGV[2]),$FindBin::Bin);
print "LOGS_ONLY_PAIR_PASS source=$Evidence::SOURCE transport=connector-decoded-text-utf8 raw_http_bytes=unavailable durable_binary_review=false\n";
