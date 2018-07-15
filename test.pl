#!/usr/bin/env perl
# coding: utf-8



use strict;
use utf8;
use LWP::UserAgent;
use JSON;

sub _get_root {

	my $url = "http://localhost:6767/";
	chomp($url);
	my $ua = LWP::UserAgent->new();
	my $response = $ua->get($url, Content_Type => 'text/plain; charset=UTF-8');
	if(!$response->is_success) {
		print($response->status_line, "\n");
		return;
	}
	print($response->decoded_content, "\n");
}

sub _post_login {

	print("### LOGIN ###\n");
	my $content = {
		"email" => "jimm\@gmail.com",
		"password" => "p\@ssw0rd",
		"submit1" => "ログイン",
		"test1" => "あああ"};
	my $ua = LWP::UserAgent->new();
	my $response = $ua->post("http://localhost:6767/login", Content => $content);
	if(!$response->is_success) {
		print($response->status_line, "\n");
		return;
	}
	print($response->decoded_content, "\n");
	print("\n");
}

sub _post_test1 {

	my $values = {text => 'てすてす'};
	my $content = JSON::to_json($values, {utf8 => 1});

	my $url = "http://localhost:6767/test1";
	chomp($url);

	my $ua = LWP::UserAgent->new();
	my $response = $ua->post(
		$url,
		Content_Type => 'application/json; charset=UTF-8',
		Content => $content);
	if(!$response->is_success) {
		print($response->status_line, "\n");
		return;
	}

	print($response->decoded_content, "\n");
}

sub _main {

	binmode(STDIN,  ':utf8');
	binmode(STDOUT, ':utf8');
	binmode(STDERR, ':utf8');

	_get_root();
	_post_login();
	_post_test1();
}

_main();
