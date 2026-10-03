#!/usr/bin/perl
# SPDX-License-Identifier: GPL-2.0-only
# All selected header calls are initialization-only. Preserve configured
# bindgen signatures verbatim when adding lifetime and forward declarations.
use strict; use warnings;
my ($path) = @ARGV;
open my $in, '<', $path or die "$path: $!";
local $/; my $text = <$in>; close $in;
my $count = $text =~ s/^(\S[^\n]*?)(\b\w+__rust_setup\s*\()/$1__init $2/gm;
die "no static wrapper definitions found in $path\n" unless $count;
# Bindgen emits each full wrapper definition on one line. Copy the exact
# return type, attributes, name and configured parameter types into a
# declaration; retain every byte of the original body.
my $declared = $text =~ s/^(\S[^\n]*?\b\w+__rust_setup\s*\([^\n]*?\))(\s*\{)/$1;\n$1$2/gm;
die "wrapper/declaration count differs ($count != $declared)\n" unless $count == $declared;
open my $out, '>', $path or die "$path: $!"; print $out $text; close $out;
