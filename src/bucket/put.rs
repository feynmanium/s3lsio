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

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use clap::ArgMatches;
use aws_sdk_rust::aws::errors::s3::S3Error;
use aws_sdk_rust::aws::common::credentials::AwsCredentialsProvider;
use aws_sdk_rust::aws::common::request::DispatchSignedRequest;
use aws_sdk_rust::aws::s3::acl::*;
use aws_sdk_rust::aws::s3::bucket::*;

use term;
use Client;
use bucket::get::get_bucket_acl;

/// All PUT requests pass through this function.
pub fn commands<P, D>(matches: &ArgMatches,
                      client: &mut Client<P, D>)
                      -> Result<(), S3Error>
                      where P: AwsCredentialsProvider,
                            D: DispatchSignedRequest {
    let bucket = matches.value_of("name").unwrap_or("");

    match matches.subcommand() {
        /// set acl on bucket
        ("acl", Some(sub_matches)) => {
            let result = put_bucket_acl(sub_matches, bucket, client);
        },
        /// set versioning on bucket
        ("versioning", Some(sub_matches)) => {
            let result = put_bucket_versioning(sub_matches, bucket, client);
        },
        (e, _) => {
            if e.is_empty() && bucket.is_empty() {

                println!("what?");

            } else {
                let error = format!("incorrect or missing request {}", e);
                println_color_quiet!(client.is_quiet, term::color::RED, "{}", error);
            }
        },
    }

    Ok(())
}

fn put_bucket_versioning<P, D>(sub_matches: &ArgMatches,
                               bucket: &str,
                               client: &Client<P, D>)
                               -> Result<(), S3Error>
                               where P: AwsCredentialsProvider,
                                     D: DispatchSignedRequest {
    let request = PutBucketVersioningRequest{
        bucket: bucket.to_string(),
        versioning_configuration: VersioningConfiguration {
            status: "Enabled".to_string(),
            mfa_delete: "".to_string(),
        },
        mfa: None,
        content_md5: None,
    };

    match client.s3client.put_bucket_versioning(&request) {
        Ok(()) => {
          println_color_quiet!(client.is_quiet, client.output.color, "Success");
          Ok(())
        },
        Err(e) => {
          println_color_quiet!(client.is_quiet, client.error.color, "{:#?}", e);
          Err(e)
        },
    }
}

fn put_bucket_acl<P, D>(sub_matches: &ArgMatches,
                        bucket: &str,
                        client: &Client<P, D>)
                        -> Result<(), S3Error>
                        where P: AwsCredentialsProvider,
                              D: DispatchSignedRequest {

    let acl: CannedAcl;

    match sub_matches.subcommand() {
      ("public-read", _) => acl = CannedAcl::PublicRead,
      ("public-rw", _) => acl = CannedAcl::PublicReadWrite,
      ("public-readwrite", _) => acl = CannedAcl::PublicReadWrite,
      ("private", _) => acl = CannedAcl::Private,
      (e, _) => {
        println_color_quiet!(client.is_quiet, client.error.color, "missing acl: public-read, public-rw, public-readwrite or private");
        return Err(S3Error::new("missing acl: public-read, public-rw, public-readwrite or private"));
      },
    }

    let mut request = PutBucketAclRequest::default();
    request.bucket = bucket.to_string();

    // get acl option...
    request.acl = Some(acl);

    match client.s3client.put_bucket_acl(&request) {
      Ok(val) => {
        let acl = get_bucket_acl(bucket, client);
        if let Ok(acl) = acl {
            println_color_quiet!(client.is_quiet, client.output.color, "{:#?}", acl);
        }
      },
      Err(e) => {
        let error = format!("{:#?}", e);
        println_color_quiet!(client.is_quiet, client.error.color, "{}", error);
        return Err(S3Error::new(error));
      },
    }

    Ok(())
}
