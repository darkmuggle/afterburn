// Copyright 2020 Red Hat, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This is a stub provider. Right now Noop does absolutely nothing
// interesting.

//! azurestack/azurestack metadata fetcher
use slog_scope::crit;
use crate::providers::MetadataProvider;

#[derive(Clone, Copy, Debug)]
pub struct Noop;

impl Noop {
    pub fn new() -> Self {
       crit!(concat!(
           "This platform is using the development 'noop' provider ",
           "and is NOT a supported Afterburn target."));
        Self
    }
}

impl MetadataProvider for Noop{}