extern crate crypto;

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

use crypto::scrypt;
use crypto::sha2;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use common;
use common::scrypt_settings;
use common::MpwCharPair;

pub fn master_key(full_name: &str, master_password: &str, site_variant: &str)
                  -> Option<[u8; scrypt_settings::DK_LEN]> {
    let scope = common::scope_for_variant(&site_variant);

    if scope.is_some() {
        let key_scope = scope.unwrap();
        let master_key_salt = format!("{}{}{}", key_scope, full_name.len(), master_password);
        let scrypt_params = scrypt::ScryptParams::new(
            scrypt_settings::N.log(2.0) as u8, scrypt_settings::R, scrypt_settings::P);
        let mut digest: [u8; scrypt_settings::DK_LEN] = [0; scrypt_settings::DK_LEN];

        scrypt::scrypt(
            master_password.as_bytes(),
            master_key_salt.as_bytes(),
            &scrypt_params,
            &mut digest
        );

        Some(digest)
    } else {
        None
    }
}

pub fn password_for_site(master_key: &[u8; scrypt_settings::DK_LEN], site_name: &str,
                         site_type: &str, site_counter: &i32, site_variant: &str,
                         site_context: &str) -> Option<String> {
    let scope = common::scope_for_variant(&site_variant);

    if scope.is_some() {
        let site_scope = scope.unwrap();
        let mut site_password_seed: [u8; 32] = [0; 32];
        let mut mac = Box::new(
            Hmac::new(sha2::Sha256::new(), master_key)
        ) as Box<Mac>;

        let input = format!(
            "{}{}{}{}{}{}",
            site_scope,
            site_name.len(),
            site_name,
            site_counter,
            site_context.len(),
            site_context
        );

        mac.input(input.as_bytes());
        mac.raw_result(&mut site_password_seed);

        let template = common::template_for_type(site_type, &site_password_seed[0]);
        if template.is_some() {
            let template = template.unwrap();
            let template_bytes = template.as_bytes();
            let password = template_bytes
                .iter()
                .zip(1..template_bytes.len() + 1)
                .map(|pair| MpwCharPair { class: *pair.0, seed_byte: pair.1 })
                .map(common::character_from_class)
                .map(|c| c.unwrap_or(b'\0'))
                .filter(|&c| c != b'\0')
                .collect::<Vec<u8>>();

            if password.len() == template_bytes.len() {
                Some(String::from_utf8(password).unwrap())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
