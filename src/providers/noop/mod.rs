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
use std::collections::HashMap;

use crate::errors::*;
use crate::network;
use crate::providers::MetadataProvider;

use openssh_keys::PublicKey;
use slog_scope::{crit, warn};

#[derive(Clone, Copy, Debug)]
pub struct Noop;

impl Noop {
    pub fn new() -> Self {
       crit!(concat!(
           "This platform is using the development 'noop' provider ",
           "and is NOT a supported Afterburn target."));
        Self
    }

    /// Get the hostname from local system settings.
    fn system_hostname() -> Result<String> {
        let hostname = hostname::get()
            .chain_err(|| "unable to get hostname")?
            .to_string_lossy()
            .into_owned();
        Ok(hostname)
    }
}

impl MetadataProvider for Noop {
    fn attributes(&self) -> Result<HashMap<String, String>> {
        let hostname = Self::system_hostname()?;

        let attributes = maplit::hashmap! {
            "NOOP_HOSTNAME".to_string() => hostname,
        };
        Ok(attributes)
    }

    fn hostname(&self) -> Result<Option<String>> {
        warn!("hostname requested, but not supported on this platform");
        Ok(None)
    }

    fn ssh_keys(&self) -> Result<Vec<PublicKey>> {
        warn!("ssh-keys requested, but not supported on this platform");
        Ok(vec![])
    }

    fn networks(&self) -> Result<Vec<network::Interface>> {
        Ok(vec![])
    }

    fn virtual_network_devices(&self) -> Result<Vec<network::VirtualNetDev>> {
        Ok(vec![])
    }

    fn boot_checkin(&self) -> Result<()> {
        warn!("boot check-in requested, but not supported on this platform");
        Ok(())
    }
}
