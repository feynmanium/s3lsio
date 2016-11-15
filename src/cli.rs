// Copyright 2016 LambdaStack All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::{App, Arg, SubCommand};

pub fn build_cli<'a>(app: &str, home: &'a str, version: &'a str) -> App<'a, 'a> {
  App::new(app)
    .about("S3 Client and Benchmarking Utility that can access AWS S3, Ceph or any third party S3 enable environment.")
    .author("Chris Jones")
    .version(version)
    .after_help("For more information about a specific command, try `s3lsio <command> --help`\nSource code for s3lsio available at: https://github.com/lambdastackio/s3lsio")
    .arg(Arg::with_name("generate-bash-completions")
      .short("g")
      .long("generate-bash-completions")
      .help("Outputs bash completions"))
    .arg(Arg::with_name("admin")
      .short("a")
      .long("admin")
      .help("Ceph RGW Admin"))
    .arg(Arg::with_name("config")
      .short("c")
      .long("config")
      .value_name("FILE")
      .default_value(home)
      .help("Sets a custom config file.")
      .takes_value(true))
   .arg(Arg::with_name("endpoint")
      .short("e")
      .long("endpoint")
      .value_name("URL:<port>")
      .help("Sets a custom endpoint URL:<port> (port is optional). Default is AWS default endpoints based on Region")
      .takes_value(true))
  .arg(Arg::with_name("bucket_virtual_host")
     .short("i")
     .long("bucket_virtual_host")
     .value_name("true or false")
     .help("Overrides the default of TRUE for virtual buckets. Useful for non AWS environments")
     .default_value("true")
     .takes_value(true))
   .arg(Arg::with_name("output-color")
      .short("l")
      .long("output-color")
      .default_value("green")
      .value_name("green or red or blue or yellow or white or normal")
      .help("Specifies the output color.")
      .takes_value(true))
   .arg(Arg::with_name("output-format")
      .short("f")
      .long("output-format")
      .default_value("pretty-json")
      .value_name("pretty-json or json or plain or serialize")
      .help("Specifies the output to stdout (and disk in some cases). Options are json, none, noneall, pretty-json, plain, serialize")
      .takes_value(true))
   .arg(Arg::with_name("output-bench-format")
      .short("o")
      .long("output-bench-format")
      .default_value("pretty-json")
      .value_name("pretty-json or json or plain or serialize")
      .help("Specifies the output to stdout (and disk in some cases). Options are json, none, noneall, pretty-json, plain, serialize")
      .takes_value(true))
   .arg(Arg::with_name("proxy")
      .short("p")
      .long("proxy")
      .value_name("URL:<port>")
      .help("Sets a custom proxy URL:<port>. Default is to use http(s)_proxy and no_proxy")
      .takes_value(true))
   .arg(Arg::with_name("quiet")
      .short("q")
      .long("quiet")
      .help("No output is produced"))
   .arg(Arg::with_name("region")
      .short("r")
      .long("region")
      .value_name("Region")
      .default_value("UsEast1")
      .help("Sets S3 Region.")
      .takes_value(true))
   .arg(Arg::with_name("signature")
      .short("s")
      .long("signature")
      .value_name("V2 or V4")
      .default_value("V4")
      .help("Sets an API Signature version.")
      .takes_value(true))
   .arg(Arg::with_name("time")
      .short("t")
      .long("time")
      .help("Track time duration of operation(s)"))
   .arg(Arg::with_name("bench")
      .short("b")
      .long("bench")
      .value_name("Format: N:N:N:N:N:A (N - Number, A - Alpha)")
      .help("Benchmarking command: AAA:BBB:CCC:DDD:EEE:F AAA - Duration in seconds, BBB - Iterations (must be 0 if using duration), CCC - Virtual Users (threads), DDD - Hosts (only 1 for now), EEE - Ramp up time (0 - Thundering Heard, anythig else spread out), F - Summary or Detail (must be S or D). MUST set `-f noneall` when using this option.")
      .takes_value(true))
   .arg(Arg::with_name("yes")
      .short("y")
      .long("yes")
      .help("Answer yes automatically"))
   .subcommand(SubCommand::with_name("abort")
      .about("Abort multipart upload: s3lsio abort <upload_id> s3://<bucket>/<object>")
      .arg_from_usage("[upload_id] 'Multipart Upload ID'")
      .arg_from_usage("[bucket] 'Bucket name'"))
   .subcommand(SubCommand::with_name("acl")
      .about("Get Bucket ACLs: s3lsio acl s3://<bucket>")
      .arg_from_usage("[bucket] 'Bucket name'"))
   .subcommand(SubCommand::with_name("head")
      .about("Head Bucket or Object: s3lsio head s3://<bucket> or s3lsio head s3://<bucket>/<object>")
      .arg_from_usage("[bucket] 'Bucket name'"))
   .subcommand(SubCommand::with_name("ls")
      .about("List Buckets or Objects in bucket with optional version tag: s3lsio ls OR s3lsio ls s3://<bucket>/<prefix> <option>")
      .arg_from_usage("[bucket] 'Bucket name'")
      .arg_from_usage("[option] 'ver or multi'")
      .arg_from_usage("[upload_id] 'multipart upload ID option'"))
   .subcommand(SubCommand::with_name("mb")
      .about("Make Bucket: s3lsio mb s3://<bucket>")
      .arg_from_usage("[bucket] 'Bucket name'"))
   .subcommand(SubCommand::with_name("rb")
      .about("Remove Bucket: s3lsio rb s3://<bucket>")
      .arg_from_usage("[bucket] 'Bucket name'"))
   .subcommand(SubCommand::with_name("rm")
      .about("Remove Object and/or Object version: s3lsio rm s3://<bucket>/<object> <version>")
      .arg_from_usage("[bucket] 'Bucket name'")
      .arg_from_usage("[version] 'Version'"))
   .subcommand(SubCommand::with_name("get")
      .about("Get Object (use `cp`): s3lsio get s3://<bucket>/<object> <path>")
      .arg_from_usage("[bucket] 'Bucket name'")
      .arg_from_usage("[path] 'Path'"))
   .subcommand(SubCommand::with_name("gen")
      .about("Gen files: s3lsio gen <path> s3://<bucket>/<object> <size>. Generates synthetic files of a given size.")
      .arg_from_usage("[bucket] 'Bucket name'")
      .arg_from_usage("[path] 'Path'")
      .arg_from_usage("[size] 'Size of file'"))
   .subcommand(SubCommand::with_name("cp")
      .about("Copy Object: s3lsio cp s3://<bucket>/<object> <path> OR s3lsio cp <path> s3://<bucket>/<object> <size of parts>")
      .arg_from_usage("[bucket] 'Bucket name'")
      .arg_from_usage("[path] 'Path'")
      .arg_from_usage("[size] 'Size of parts'"))
   .subcommand(SubCommand::with_name("put")
      .about("Put Object (use `cp`) <size of parts> is optional: s3lsio put <path> s3://<bucket>/<object> <size of parts>")
      .arg_from_usage("[path] 'Path (for benchmarking use `.`. Example: s3lsio put . s3://<bucket>/<object> <size of parts>)'")
      .arg_from_usage("[bucket] 'Bucket name'")
      .arg_from_usage("[size] 'Size of parts (for benchmarking this value is the size of the object to generate in bytes)'"))
   .subcommand(SubCommand::with_name("range")
      .about("Byte-Range request of Object: s3lsio range <offset> <len> s3://<bucket>/<object> <path>")
      .arg_from_usage("[offset] 'Range begin offset'")
      .arg_from_usage("[len] 'Range len'")
      .arg_from_usage("[bucket] 'Bucket name'")
      .arg_from_usage("[path] 'Path'"))
   .subcommand(SubCommand::with_name("setacl")
      .about("Set Bucket ACLs: s3lsio setacl <acl> s3://<bucket>")
      .arg_from_usage("[acl] 'ACL - public-read, public-readwrite, private'")
      .arg_from_usage("[bucket] 'Bucket name'"))
   .subcommand(SubCommand::with_name("setver")
      .about("Enables Bucket Versioning: s3lsio setver on|off s3://<bucket>")
      .arg_from_usage("[ver] 'On or Off'")
      .arg_from_usage("[bucket] 'Bucket name'"))
   .subcommand(SubCommand::with_name("ver")
      .about("Shows Bucket Versioning: s3lsio ver s3://<bucket>")
      .arg_from_usage("[bucket] 'Bucket name'"))
    // Ceph RGW Admin Section...
   .subcommand(SubCommand::with_name("bucket")
      .about("Admin Bucket Options: s3lsio -a bucket <command> s3://<bucket> <user>")
      .arg_from_usage("[command] 'Commands: create - Create, delete - Delete, ls - List, stats - stats, link - Link, unlink - Unlink, index - Index'")
      .arg_from_usage("[bucket] 'Bucket name or Bucket/Object name'")
      .arg_from_usage("[user] 'User ID (uid)'")
      .arg_from_usage("[stats] '(Optional) - true or false. Default is false. Only used with stats command'")
      .arg_from_usage("[fix] '(Optional) - true or false. Default is false. Only used with index command'")
      .arg_from_usage("[check] '(Optional) - true or false. Default is false. Only used with index command'"))
   .subcommand(SubCommand::with_name("object")
      .about("Admin Object Options: s3lsio -a object <command> s3://<bucket>/<object> <user>")
      .arg_from_usage("[command] 'Commands: delete - Delete object for given user'")
      .arg_from_usage("[bucket] 'Bucket name or Bucket/Object name'")
      .arg_from_usage("[user] 'User ID (uid)'"))
   .subcommand(SubCommand::with_name("quota")
      .about("Admin Quota Options: s3lsio -a quota <user> <size> <command> <action>")
      .arg_from_usage("[user] 'User ID (uid)'")
      .arg_from_usage("[size] 'Quota size - max size in MB for buckets (defaults to -1)'")
      .arg_from_usage("[objects] 'Quota object - max number of objects (defaults to -1)'")
      .arg_from_usage("[command] 'Commands: bucket - Bucket, user - User'")
      .arg_from_usage("[action] 'Action: get - Get, set - Set'"))
   .subcommand(SubCommand::with_name("user")
      .about("Admin User Options: s3lsio -a user <command> <user> <display_name> <email> <access_key> <secret_key> <caps>")
      .arg_from_usage("[command] 'Commands: ls - List users, create - Create new user, delete - Delete user, get - Get user'")
      .arg_from_usage("[user] 'User ID (uid)'")
      .arg_from_usage("[display_name] 'Display name - multi-word names should be inside of quotation marks'")
      .arg_from_usage("[email] 'User email (optional)'")
      .arg_from_usage("[access_key] 'Access Key ID (optional)'")
      .arg_from_usage("[secret_key] 'Secret Key ID (optional)'")
      .arg_from_usage("[caps] 'Capability of user (optional)'")) // This is to allow for creating another admin like user
   .subcommand(SubCommand::with_name("usage")
      .about("Admin Usage Options: s3lsio -a usage <command> <user> <display_name> <email> <access_key> <secret_key> <caps>")
      .arg_from_usage("[command] 'Commands: ls - List usage, trim - Trim usage'")
      .arg_from_usage("[user] 'User ID (uid)'")
      .arg_from_usage("[start] 'Start DateTime [yyyy-mm-dd hh:mm:ss] (optional)'")
      .arg_from_usage("[end] 'End DateTime [yyyy-mm-dd hh:mm:ss] (optional)'")
      .arg_from_usage("[remove_all] 'Remove all object [true or false] (optional)'"))
   .subcommand(SubCommand::with_name("keys")
      .about("Admin Keys Options: s3lsio -a keys <command> <user> <access_key> <secret_key>")
      .arg_from_usage("[command] 'Commands: add - Add keys to user, delete - Delete keys from user, gen - Generate keys but do not assign'")
      .arg_from_usage("[user] 'User ID (uid) (optional for gen command)'")
      .arg_from_usage("[access_key] 'Access Key ID (optional)'")
      .arg_from_usage("[secret_key] 'Secret Key ID (optional)'"))
}
