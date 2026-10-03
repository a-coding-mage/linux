#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
use strict;use warnings;use FindBin;use lib $FindBin::Bin;use Evidence;use Digest::SHA qw(sha256_hex);use Encode qw(decode FB_CROAK);
my($runtime,$host,$provider,$run,$trigger)=@ARGV;
die "bad evidence context\n" unless defined($trigger) && $provider=~/^(c|rust)$/ && $run=~/^\d+$/ && $trigger=~/^[0-9a-f]{40}$/;
my %header=(schema=>3,provider=>$provider,run_id=>"$run",trigger_sha=>$trigger,run_attempt=>1,source_sha=>$Evidence::SOURCE);
print 'UEV3 BEGIN ',$Evidence::JSON->encode(\%header),"\n";
my($count,$bad,$printed)=(0,0,0);my @manifest;
for my $entry(Evidence::roles()){
 my($role,$logical)=@$entry;my $physical=$logical;$physical=~s!^runtime/!$runtime/!;$physical=~s!^host/!$host/!;
 if(!-f $physical || -l $physical){print "UEV3 MISSING $role\n";$bad++;next}
 my $raw=Evidence::slurp($physical);my $len=length($raw);my $hash=sha256_hex($raw);
 if((!$len && !Evidence::may_be_empty($role)) || $len>134217728){print "UEV3 INVALID $role\n";$bad++;next}
 my $text=eval{decode('UTF-8',$raw,FB_CROAK)};
 if($@){print "UEV3 INVALID_UTF8 $role\n";$bad++;next}
 my $head=$Evidence::JSON->encode({role=>$role,path=>$logical,bytes=>$len,sha256=>$hash});
 print "UEV3 FILE $head\n";my $chunks=0;
 while(length($text)){my $piece=substr($text,0,4096,'');my $line='UEV3 DATA '.$Evidence::JSON->encode({index=>$chunks,text=>$piece})."\n";$printed+=length($line);die "evidence text emission cap exceeded\n" if $printed>134217728;print $line;$chunks++}
 print "UEV3 END_FILE $role $chunks\n";push @manifest,join("\t",$role,$logical,$len,$hash,$chunks);$count++;
}
print 'UEV3 END ',$Evidence::JSON->encode({files=>$count,manifest_sha256=>sha256_hex(join("\n",@manifest)."\n"),complete=>$bad?JSON::PP::false:JSON::PP::true}),"\n";
exit($bad?78:0);
