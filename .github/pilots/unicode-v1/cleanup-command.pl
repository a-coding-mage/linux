#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
# Core-only bounded capture, process-group termination and child reaping.
use strict; use warnings; use POSIX qw(setsid WNOHANG);
use Time::HiRes qw(clock_gettime CLOCK_MONOTONIC);
use IO::Select; use Errno qw(EAGAIN EINTR ESRCH);
sub now { clock_gettime(CLOCK_MONOTONIC) }
my($cap,$deadline,$capture,$receipt,@command)=@ARGV;
die "bad supervisor arguments\n" unless @command && $cap=~/^\d+$/ && $cap>=2 && $cap<=25 && $deadline=~/^\d+\.\d+$/;
open my $out,'>:raw',$capture or die "$capture: $!";
open my $record,'>',$receipt or die "$receipt: $!";
my($native,$captured,$supervisor,$reaped,$absent)=(125,0,0,0,0);
my $hard=now()+$cap; $hard=$deadline if $deadline<$hard;
sub finish {
 print {$record} "command_exit=$native capture_exit=$captured supervisor_exit=$supervisor reaped=$reaped group_absent=$absent\n" or exit 79;
 close $record or exit 79; close $out or exit 79;
 exit(($captured || $supervisor || !$reaped || !$absent)?79:0);
}
if($hard-now()<1.5){$supervisor=124;print {$out} "CLEANUP_DEADLINE_EXHAUSTED\n";finish()}
pipe(my $reader,my $writer) or die "pipe: $!";
my $pid=fork(); die "fork: $!" unless defined $pid;
if(!$pid){
 close $reader; setsid()>=0 or die "setsid: $!";
 open STDOUT,'>&',$writer or die "stdout: $!";
 open STDERR,'>&',\*STDOUT or die "stderr: $!";
 close $writer; exec @command; die "exec: $!";
}
close $writer; my $select=IO::Select->new($reader);
my($bytes,$closed,$term,$killed)=(0,0,0,0);
my $kill_at=$hard-.25; my $term_at=$hard-1.25;
sub signal_group {
 my($signal)=@_;
 if(!kill($signal,-$pid) && !$!{ESRCH}){$supervisor=79}
}
while(1){
 my $time=now();
 if(!$reaped){my $wait=waitpid($pid,WNOHANG);if($wait==$pid){$native=($?&127)?128+($?&127):$?>>8;$reaped=1}elsif($wait<0){$supervisor=79}}
 $absent=kill(0,-$pid)?0:($!{ESRCH}?1:0);
 last if $reaped && $closed && $absent;
 if($time>=$hard){$supervisor=124;signal_group('KILL');last}
 if(!$term && ($captured || $time>=$term_at || ($reaped && !$absent))){
  $supervisor=124 if !$captured && $time>=$term_at;
  $supervisor=79 if $reaped && !$absent;
  signal_group('TERM');$term=1;
  $kill_at=$time+1 if $time+1<$kill_at;
 }
 if($term && !$killed && $time>=$kill_at){signal_group('KILL');$killed=1}
 if(!$closed && $select->can_read(.025)){
  my $count=sysread($reader,my $chunk,512);
  if(!defined $count){next if $!{EINTR} || $!{EAGAIN};$captured=79;$count=0}
  if(!$count){close $reader;$closed=1;next}
  my $keep=4096-$bytes; $keep=$count if $count<$keep;
  if($keep){print {$out} substr($chunk,0,$keep) or $captured=79;$bytes+=$keep}
  if(index($chunk,"\0")>=0){$captured=79;print {$out} "\nCLEANUP_OUTPUT_NUL_REJECTED\n";close $reader;$closed=1;next}
  if($count>$keep){$captured=79;print {$out} "\nCLEANUP_OUTPUT_LIMIT_EXCEEDED limit=4096\n";close $reader;$closed=1}
 }else{select undef,undef,undef,.01}
}
if(now()>=$hard){$supervisor=124}
if(!$reaped){my $wait=waitpid($pid,WNOHANG);if($wait==$pid){$native=($?&127)?128+($?&127):$?>>8;$reaped=1}}
$absent=kill(0,-$pid)?0:($!{ESRCH}?1:0);
print {$out} "\nCLEANUP_SUPERVISION_FAILED code=$supervisor\n" if $supervisor;
finish();
