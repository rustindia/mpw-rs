/*
 * This file is part of Master Password.
 *
 * Master Password is free software: you can redistribute it and/or modify
 * Mit under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Master Password is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Master Password. If not, see <http://www.gnu.org/licenses/>.
 */

use scrypt::{scrypt, ScryptParams};
use argon2rs::{Variant, Argon2};

pub const KEY_LENGTH: usize = 64_usize;

#[derive(PartialEq, Eq, Debug)]
pub enum SiteVariant {
    Password,
    Login,
    Answer,
}

impl SiteVariant {
    pub fn from(s: &str) -> Option<SiteVariant> {
        match s {
            "p" | "password" => Some(SiteVariant::Password),
            "l" | "login" => Some(SiteVariant::Login),
            "a" | "answer" => Some(SiteVariant::Answer),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum SiteType {
    Maximum,
    Long,
    Medium,
    Basic,
    Short,
    PIN,
    Name,
    Phrase,
}

impl SiteType {
    pub fn from(s: &str) -> Option<SiteType> {
        match s {
            "x" | "max" | "maximum" => Some(SiteType::Maximum),
            "l" | "long" => Some(SiteType::Long),
            "m" | "med" | "medium" => Some(SiteType::Medium),
            "b" | "basic" => Some(SiteType::Basic),
            "s" | "short" => Some(SiteType::Short),
            "i" | "pin" => Some(SiteType::PIN),
            "n" | "name" => Some(SiteType::Name),
            "p" | "phrase" => Some(SiteType::Phrase),
            _ => None,
        }
    }
}

pub fn scope_for_variant(site_variant: &SiteVariant) -> Option<String> {
    match *site_variant {
        SiteVariant::Password => Some(String::from("com.lyndir.masterpassword")),
        SiteVariant::Login => Some(String::from("com.lyndir.masterpassword.login")),
        SiteVariant::Answer => Some(String::from("com.lyndir.masterpassword.answer")),
    }
}

pub fn template_for_type(site_type: &SiteType, seed_byte: &u8) -> Option<String> {
    let choice = match *site_type {
        SiteType::Maximum => Some(vec!["anoxxxxxxxxxxxxxxxxx", "axxxxxxxxxxxxxxxxxno"]),
        SiteType::Long => {
            Some(vec!["CvcvnoCvcvCvcv",
                      "CvcvCvcvnoCvcv",
                      "CvcvCvcvCvcvno",
                      "CvccnoCvcvCvcv",
                      "CvccCvcvnoCvcv",
                      "CvccCvcvCvcvno",
                      "CvcvnoCvccCvcv",
                      "CvcvCvccnoCvcv",
                      "CvcvCvccCvcvno",
                      "CvcvnoCvcvCvcc",
                      "CvcvCvcvnoCvcc",
                      "CvcvCvcvCvccno",
                      "CvccnoCvccCvcv",
                      "CvccCvccnoCvcv",
                      "CvccCvccCvcvno",
                      "CvcvnoCvccCvcc",
                      "CvcvCvccnoCvcc",
                      "CvcvCvccCvccno",
                      "CvccnoCvcvCvcc",
                      "CvccCvcvnoCvcc",
                      "CvccCvcvCvccno"])
        }
        SiteType::Medium => Some(vec!["CvcnoCvc", "CvcCvcno"]),
        SiteType::Basic => Some(vec!["aaanaaan", "aannaaan", "aaannaaa"]),
        SiteType::Short => Some(vec!["Cvcn"]),
        SiteType::PIN => Some(vec!["nnnn"]),
        SiteType::Name => Some(vec!["cvccvcvcv"]),
        SiteType::Phrase => {
            Some(vec!["cvcc cvc cvccvcv cvc", "cvc cvccvcvcv cvcv", "cv cvccv cvc cvcvccv"])
        }
    };

    match choice {
        Some(val) => Some(String::from(val[*seed_byte as usize % val.len()])),
        None => None,
    }
}

#[inline(always)]
pub fn character_from_class(class: u8, seed_byte: usize) -> Option<u8> {
    let choice = match class {
        b'V' => Some("AEIOU"),
        b'C' => Some("BCDFGHJKLMNPQRSTVWXYZ"),
        b'v' => Some("aeiou"),
        b'c' => Some("bcdfghjklmnpqrstvwxyz"),
        b'A' => Some("AEIOUBCDFGHJKLMNPQRSTVWXYZ"),
        b'a' => Some("AEIOUaeiouBCDFGHJKLMNPQRSTVWXYZbcdfghjklmnpqrstvwxyz"),
        b'n' => Some("0123456789"),
        b'o' => Some("@&%?,=[]_:-+*$#!'^~;()/."),
        b'x' => Some("AEIOUaeiouBCDFGHJKLMNPQRSTVWXYZbcdfghjklmnpqrstvwxyz0123456789!@#$%^&*()"),
        b' ' => Some(" "),
        _ => None,
    };

    match choice {
        Some(val) => Some(val.as_bytes()[seed_byte % val.len()]),
        None => None,
    }
}

#[inline(always)]
pub fn u32_to_bytes(u: u32) -> [u8; 4] {
    [((u >> 24) & 0xff) as u8, ((u >> 16) & 0xff) as u8, ((u >> 8) & 0xff) as u8, (u & 0xff) as u8]
}

pub fn derive_key(password: &[u8], salt: &[u8]) -> [u8; KEY_LENGTH] {
    let scrypt_params = ScryptParams::new(32768_f64.log(2.0) as u8, 8_u32, 2_u32).unwrap();
    let mut digest: [u8; KEY_LENGTH] = [0; KEY_LENGTH];

    scrypt(password, salt, &scrypt_params, &mut digest).unwrap();

    digest
}

pub fn derive_key_argon(password: &[u8], salt: &[u8]) -> [u8; KEY_LENGTH] {
    let inst = Argon2::new(1, 4, 131072, Variant::Argon2i);
    let mut digest: [u8; KEY_LENGTH] = [0; KEY_LENGTH];

    inst.unwrap().hash(&mut digest, password, salt, &[], &[]);

    digest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_valid_site_variant() {
        assert_eq!(SiteVariant::from("password"), Some(SiteVariant::Password));
    }

    #[test]
    fn get_invalid_site_variant() {
        assert_eq!(SiteVariant::from("invalid"), None);
    }

    #[test]
    fn get_valid_site_type() {
        assert_eq!(SiteType::from("basic"), Some(SiteType::Basic));
    }

    #[test]
    fn get_invalid_site_type() {
        assert_eq!(SiteType::from("invalid"), None);
    }

    #[test]
    fn get_scope_for_valid_variant() {
        assert_eq!(scope_for_variant(&SiteVariant::Password),
                   Some(String::from("com.lyndir.masterpassword")));
    }

    #[test]
    fn get_template_for_valid_type() {
        assert_eq!(template_for_type(&SiteType::Long, &(11 as u8)),
                   Some(String::from("CvcvCvcvCvccno")));
    }

    #[test]
    fn get_character_from_valid_class() {
        assert_eq!(character_from_class(b'v', 11 as usize), Some(b'e'));
    }

    #[test]
    fn get_character_from_invalid_class() {
        assert_eq!(character_from_class(b'z', 11 as usize), None);
    }

    #[test]
    fn get_bytes_from_u32() {
        assert_eq!(u32_to_bytes(2), [0, 0, 0, 2]);
    }
}


