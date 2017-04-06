// This file is part of Master Password.
//
// Master Password is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Master Password is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Master Password. If not, see <http://www.gnu.org/licenses/>.

use common;
use common::SiteVariant;

pub fn master_key(full_name: &str,
                  master_password: &str,
                  site_variant: &SiteVariant)
                  -> Option<[u8; common::KEY_LENGTH]> {
    let scope = common::scope_for_variant(site_variant);

    if scope.is_some() {
        let key_scope = scope.unwrap();
        let mut master_key_salt = Vec::new();

        master_key_salt.extend_from_slice(key_scope.as_bytes());
        master_key_salt.extend_from_slice(&common::u32_to_bytes(full_name.len() as u32));
        master_key_salt.extend_from_slice(full_name.as_bytes());

        Some(common::derive_key_argon(master_password.as_bytes(), master_key_salt.as_slice()))
    } else {
        None
    }
}
