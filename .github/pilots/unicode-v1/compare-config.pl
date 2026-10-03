#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
# Compare every recorded symbol; do not silently normalize away feature drift.
use strict; use warnings;
my ($preset, $actual, $provider) = @ARGV;
die "usage: compare-config.pl preset actual c|rust\n" unless defined($provider) && $provider =~ /\A(?:c|rust)\z/;
sub read_config {
    my ($path) = @_; open my $fh, '<', $path or die "$path: $!";
    my %map;
    while (<$fh>) {
        my ($key, $value);
        if (/\A(CONFIG_[A-Za-z0-9_]+)=(.*)\n?\z/) { ($key, $value) = ($1, $2); }
        elsif (/\A# (CONFIG_[A-Za-z0-9_]+) is not set\s*\z/) { ($key, $value) = ($1, 'n'); }
        else { next; }
        die "duplicate $key in $path\n" if exists $map{$key};
        $map{$key} = $value;
    }
    return \%map;
}
my $want = read_config($preset); my $got = read_config($actual);
$want->{CONFIG_RUST_UNICODE_NORM} = $provider eq 'rust' ? 'y' : 'n';
my %new_off = map { $_ => 1 } qw(CONFIG_RUST_AFS_ADDR_PREFS CONFIG_RUST_BLK_CRYPTO_FALLBACK CONFIG_RUST_BPF_TOKEN);
for my $key (sort keys %$want) {
    die "missing/changed $key: expected $want->{$key}\n" if !exists($got->{$key}) || $got->{$key} ne $want->{$key};
}
for my $key (sort keys %$got) {
    next if exists $want->{$key};
    die "unexpected added/active $key=$got->{$key}\n" unless $new_off{$key} && $got->{$key} eq 'n';
}
print "CONFIG_ALL_VALUES_PRESERVED provider=$provider symbols=", scalar(keys %$got), "\n";
