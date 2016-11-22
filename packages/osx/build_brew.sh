#!/bin/sh
#
# Copyright 2016 LambdaStack All rights reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

REPO_BASE=$(git rev-parse --show-toplevel)

cargo build --release

APP=s3lsio
VERSION=0.1.17

tar -cvzf $APP-$VERSION.tar.gz $REPO_BASE/target/release/$APP

s3lsio cp $APP-$VERSION.tar.gz s3://s3lsio/mac/$APP-$VERSION.tar.gz
s3lsio acl set public-read s3://s3lsio/mac/$APP-$VERSION.tar.gz

# NB: This is not a good way but it creates the hash to change in s3lsio.rb.
# This needs to be changed soon to make it smooth...

# Do the brew create the first time. Then modify the formula for your install
# brew create https://s3.amazonaws.com/s3lsio/mac/$APP-$VERSION.tar.gz

# Generate the checksum
CHECKSUM=$(shasum -a 256 $APP-$VERSION.tar.gz | awk '{print $1}')
echo $CHECKSUM

rm $APP-$VERSION.tar.gz

# Copy the checksum to the s3lsio.rb before committing it to github

# :( Have to manually edit the s3lsio.rb (currently) with the correct hash key and then update the homebrew-tap repo
