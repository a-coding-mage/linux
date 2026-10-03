#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
# Adapted strict framing observer; unchanged upstream kunit.py is required too.
use strict; use warnings; use Digest::SHA qw(sha256_hex); use FindBin;
@ARGV==4 or die "usage: observe-integrated-cases.pl SUITE CASE_NAMES SERIAL_LOG QEMU_EXIT\n";
my ($suite,$names,$logfile,$exitfile)=@ARGV;
my %spec=(crc=>['crc-cases.txt',16,'8daa4efcabeb1734087ffe7a6cf679fcb7115bfc1c92588900c7407e4c917a09'],chacha20poly1305=>['chacha-cases.txt',1,'31aef49c093babaead3476a63549164a549b00905c419c1bf8592ff017b5223f'],iov_iter=>['iov-cases.txt',17,'d5921f74a80a9f3a840e8d23ac3e10bce1c32298c3fc502ed4fe750af4cd774f']);
exists $spec{$suite} or die "unexpected suite\n";
sub readall { open my $f,'<:raw',$_[0] or die "$!: $_[0]\n"; local $/; return <$f>//'' }
my $reference=readall("$FindBin::Bin/$spec{$suite}[0]");
sha256_hex($reference) eq $spec{$suite}[2] or die "frozen original case list changed\n";
my $provided=readall($names);
$provided eq $reference or die "case manifest is not the frozen original list\n";
my @names=split /\n/,$provided; my $n=$spec{$suite}[1];
@names==$n && !grep(!/\A[A-Za-z_][A-Za-z_0-9]*\z/,@names) or die "invalid original names/count\n";
my %unique; !grep($unique{$_}++,@names) or die "duplicate original manifest names\n";
my $raw=readall($logfile); my $exit=readall($exitfile);
$exit =~ /\A0\n?\z/ or die "QEMU receipt must contain exactly zero\n";
my @want=('KTAP version 1','1..1','    KTAP version 1',"    # Subtest: $suite","    1..$n");
push @want,map {"    ok ".($_+1)." $names[$_]"} 0..$#names;
push @want,"ok 1 $suite";
my $state=0; my %totals;
for my $line(split /\n/,$raw) {
 $line =~ s/\r$//; $line =~ s/^\[\s*\d+\.\d+\]\s?//;
 die "nonpassing/fault serial output\n" if $line =~ /\bnot\s+ok\b|#\s*(?:SKIP|TODO)\b|Bail out!|Kernel panic|Oops:|BUG:|KASAN:|UBSAN:|KMSAN:|KCSAN:|KFENCE:|general protection fault|segfault|soft lockup|hard LOCKUP|watchdog:.*lockup|rcu:.*stall/i;
 if ($line =~ /^\s*(?:K?TAP version|\d+\.\.\d+|(?:not\s+)?ok\b|# Subtest:)/) {
  $state<@want && $line eq $want[$state] or die "unexpected/missing/skipped/repeated/out-of-order KTAP at $state: $line\n";
  $state++; next;
 }
 if ($line =~ /^\s*# (\Q$suite\E:|Totals:)\s+pass:(\d+)\s+fail:(\d+)\s+skip:(\d+)\s+total:(\d+)\s*$/) {
  !$totals{$1}++ && $2==$n && $3==0 && $4==0 && $5==$n or die "wrong/duplicate totals\n";
 } elsif ($line =~ /^\s*# .*\b(?:pass|fail|skip|total):/) { die "unrecognized totals\n"; }
}
$state==@want or die "incomplete KTAP\n";
print "ORIGINAL_C_CASES_PASS suite=$suite cases=$n ordered=1 skipped=0 failed=0 QEMU_exit=0\n";
print "case_names=",join(',',@names),"\nserial_sha256=",sha256_hex($raw),"\n";
print "upstream_original_parser=separately_required\n";
