// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct CpReturnedDate {
    posix: i64,
    iso_8601: String, // consider iso8601 crate
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct CpLoginResponse {
    sid: String,
    url: Url,
    session_timeout: usize,
    last_login_was_at: CpReturnedDate,
    read_only: bool,
    api_server_version: String, // can this be a different type? (not SEMVER, but ... something?)
}
