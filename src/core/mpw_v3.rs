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

pub fn master_key(full_name: &str, master_password: &str, site_variant: &str)
                  -> Option<[u8; scrypt_settings::DK_LEN]> {
    let scope = common::scope_for_variant(&site_variant);

    if scope.is_some() {
        let key_scope = scope.unwrap();
        let scrypt_params = scrypt::ScryptParams::new(
            scrypt_settings::N.log(2.0) as u8, scrypt_settings::R, scrypt_settings::P);
        let mut digest: [u8; scrypt_settings::DK_LEN] = [0; scrypt_settings::DK_LEN];
        let mut master_key_salt = Vec::new();

        master_key_salt.extend_from_slice(&key_scope.as_bytes());
        master_key_salt.extend_from_slice(&common::u32_to_bytes(full_name.len() as u32));
        master_key_salt.extend_from_slice(&full_name.as_bytes());

        scrypt::scrypt(
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
                         site_type: &str, site_counter: &i32, site_variant: &str,
                         site_context: &str) -> Option<String> {
    let scope = common::scope_for_variant(&site_variant);

    if scope.is_some() {
        let site_scope = scope.unwrap();
        let mut site_password_seed: [u8; 32] = [0; 32];
        let mut mac = Box::new(
            Hmac::new(sha2::Sha256::new(), master_key)
        ) as Box<Mac>;

        mac.input(&site_scope.as_bytes());
        mac.input(&common::u32_to_bytes(site_name.len() as u32));
        mac.input(&site_name.as_bytes());
        mac.input(&common::u32_to_bytes(*site_counter as u32));
        mac.input(&common::u32_to_bytes(site_context.len() as u32));
        mac.input(&site_context.as_bytes());
        mac.raw_result(&mut site_password_seed);

        let template = common::template_for_type(site_type, &site_password_seed[0]);
        if template.is_some() {
            let template = template.unwrap();
            let template_bytes = template.as_bytes();
            let password = template_bytes
                .iter()
                .zip(1..template_bytes.len() + 1)
                .map(|pair| common::character_from_class(*pair.0, pair.1))
                .collect::<Vec<Option<u8>>>();

            match password.iter().any(|c| c.is_none()) {
                true => None,
                false => Some(String::from_utf8(password
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

    #[test]
    fn get_master_key_for_password() {
        let actual = master_key("test", "pass", "password").unwrap().to_vec();

        assert!(actual ==
            vec![
                51, 253, 82, 252, 68, 97, 191, 162, 127, 73, 153, 160, 52, 128, 204, 4, 183, 190,
                106, 180, 68, 126, 100, 94, 132, 141, 99, 143, 106, 211, 94, 245, 245, 255, 195, 72,
                28, 128, 197, 51, 99, 27, 125, 24, 54, 193, 223, 230, 118, 181, 225, 236, 171, 104,
                9, 158, 214, 23, 166, 89, 36, 174, 64, 112
            ]
        );
    }
}
