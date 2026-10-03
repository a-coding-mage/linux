package OverflowPolicy;
# SPDX-License-Identifier: GPL-2.0-only
use strict; use warnings; use Text::ParseWords qw(shellwords);
sub check {
 my($text)=@_;
 my @lines=$text=~/^savedcmd_fs\/unicode\/utf8-norm\.o := ([^\n]+)$/mg;
 die "overflow policy: missing/duplicate compiler command\n" unless @lines==1;
 my @args=shellwords($lines[0]); my $seen=0;
 die "overflow policy: malformed compiler command\n" unless @args;
 for(my $i=0;$i<@args;$i++){
  my $option;
  if($args[$i] eq '-C' || $args[$i] eq '--codegen'){
   die "overflow policy: missing codegen argument\n" unless ++$i<@args;
   $option=$args[$i];
  }elsif($args[$i]=~/^-C(.+)$/ || $args[$i]=~/^--codegen=(.+)$/){$option=$1}
  next unless defined($option) && $option=~/^overflow[-_]checks(?:=(.*))?$/;
  my $value=$1; $seen++;
  die "overflow policy: conflicting/disabled setting\n" if defined($value) && $value!~/^(?:y|yes|on|true|1)$/;
 }
 die "overflow policy: missing enabled setting\n" unless $seen;
 return 1;
}
unless(caller){local $/;my $text=<>;check($text);print "RUST_OVERFLOW_POLICY_PASS\n"}
1;
