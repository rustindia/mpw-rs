extern crate ring;
extern crate ring_pwhash;

// This file is part of Master Password.
//
// Master Password is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Master Password is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Master Password. If not, see <http://www.gnu.org/licenses/>.

use self::ring::{digest, hmac};
use self::ring_pwhash::scrypt::{scrypt, ScryptParams};
use common;
use common::scrypt_settings;
use common::{SiteVariant, SiteType};

pub fn master_key(full_name: &str, master_password: &str, site_variant: &SiteVariant)
                  -> Option<[u8; scrypt_settings::DK_LEN]> {
    let scope = common::scope_for_variant(site_variant);

    if scope.is_some() {
        let key_scope = scope.unwrap();
        let scrypt_params = ScryptParams::new(
            scrypt_settings::N.log(2.0) as u8, scrypt_settings::R, scrypt_settings::P);
        let mut digest: [u8; scrypt_settings::DK_LEN] = [0; scrypt_settings::DK_LEN];
        let mut master_key_salt = Vec::new();

        master_key_salt.extend_from_slice(key_scope.as_bytes());
        master_key_salt.extend_from_slice(&common::u32_to_bytes(full_name.len() as u32));
        master_key_salt.extend_from_slice(full_name.as_bytes());

        scrypt(
            master_password.as_bytes(),
            master_key_salt.as_slice(),
            &scrypt_params,
            &mut digest
        );

        Some(digest)
    } else {
        None
    }
}

pub fn password_for_site(master_key: &[u8; scrypt_settings::DK_LEN], site_name: &str,
                         site_type: &SiteType, site_counter: &i32, site_variant: &SiteVariant,
                         site_context: &str) -> Option<String> {
    let scope = common::scope_for_variant(site_variant);

    if scope.is_some() {
        let site_scope = scope.unwrap();
        let mut input = Vec::new();

        input.extend_from_slice(site_scope.as_bytes());
        input.extend_from_slice(&common::u32_to_bytes(site_name.len() as u32));
        input.extend_from_slice(site_name.as_bytes());
        input.extend_from_slice(&common::u32_to_bytes(*site_counter as u32));
        if !site_context.is_empty() {
            input.extend_from_slice(&common::u32_to_bytes(site_context.len() as u32));
            input.extend_from_slice(site_context.as_bytes());
        }

        let signing_key = hmac::SigningKey::new(&digest::SHA256, master_key);
        let output = hmac::sign(&signing_key, input.as_slice());
        let site_password_seed = output.as_ref();

        let template = common::template_for_type(site_type, &site_password_seed[0]);
        if template.is_some() {
            let template = template.unwrap();
            let template_bytes = template.as_bytes();
            let password = template_bytes
                .iter()
                .zip(1..template_bytes.len() + 1)
                .map(|pair| common::character_from_class(
                    *pair.0, site_password_seed[pair.1] as usize))
                .collect::<Vec<Option<u8>>>();

            if password.iter().any(|c| c.is_none()) { None } else {
                Some(String::from_utf8(password
                    .iter().map(|c| c.unwrap()).collect::<Vec<u8>>()).unwrap())
            }
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{SiteType, SiteVariant};

    #[test]
    fn get_master_key_for_password() {
        let actual = master_key("test", "pass", &SiteVariant::Password).unwrap().to_vec();

        assert!(actual ==
            vec![
                51, 253, 82, 252, 68, 97, 191, 162, 127, 73, 153, 160, 52, 128, 204, 4, 183, 190,
                106, 180, 68, 126, 100, 94, 132, 141, 99, 143, 106, 211, 94, 245, 245, 255, 195, 72,
                28, 128, 197, 51, 99, 27, 125, 24, 54, 193, 223, 230, 118, 181, 225, 236, 171, 104,
                9, 158, 214, 23, 166, 89, 36, 174, 64, 112
            ]
        );
    }

    #[test]
    fn get_master_password() {
        let master_key = master_key("test", "pass", &SiteVariant::Password).unwrap();
        let actual = password_for_site(&master_key, "site", &SiteType::Maximum, &(1 as i32),
                                       &SiteVariant::Password, "");

        assert!(actual == Some(String::from("QsKBWAYdT9dh^AOGVA0.")));
    }
}
