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

pub mod scrypt_settings {
    pub const N: f64 = 32768_f64;
    pub const R: u32 = 8_u32;
    pub const P: u32 = 2_u32;
    pub const DK_LEN: usize = 64_usize;
}

pub fn scope_for_variant(site_variant: &str) -> Option<String> {
    match site_variant {
        "password" => Some(String::from("com.lyndir.masterpassword")),
        "login" => Some(String::from("com.lyndir.masterpassword.login")),
        "answer" => Some(String::from("com.lyndir.masterpassword.answer")),
        _ => None
    }
}

pub fn template_for_type(site_type: &str, seed_byte: &u8) -> Option<String> {
    let choice = match site_type {
        "x" | "max" | "maximum" => Some(vec!["anoxxxxxxxxxxxxxxxxx", "axxxxxxxxxxxxxxxxxno"]),
        "l" | "long" => Some(vec!["CvcvnoCvcvCvcv", "CvcvCvcvnoCvcv", "CvcvCvcvCvcvno",
                                  "CvccnoCvcvCvcv", "CvccCvcvnoCvcv", "CvccCvcvCvcvno",
                                  "CvcvnoCvccCvcv", "CvcvCvccnoCvcv", "CvcvCvccCvcvno",
                                  "CvcvnoCvcvCvcc", "CvcvCvcvnoCvcc", "CvcvCvcvCvccno",
                                  "CvccnoCvccCvcv", "CvccCvccnoCvcv", "CvccCvccCvcvno",
                                  "CvcvnoCvccCvcc", "CvcvCvccnoCvcc", "CvcvCvccCvccno",
                                  "CvccnoCvcvCvcc", "CvccCvcvnoCvcc", "CvccCvcvCvccno"]),
        "m" | "med" | "medium" => Some(vec!["CvcnoCvc", "CvcCvcno"]),
        "b" | "basic" => Some(vec!["aaanaaan", "aannaaan", "aaannaaa"]),
        "s" | "short" => Some(vec!["Cvcn"]),
        "i" | "pin" => Some(vec!["nnnn"]),
        "n" | "name" => Some(vec!["cvccvcvcv"]),
        "p" | "phrase" => Some(vec!["cvcc cvc cvccvcv cvc", "cvc cvccvcvcv cvcv",
                                    "cv cvccv cvc cvcvccv"]),
        _ => None
    };

    match choice {
        Some(val) => Some(String::from(val[*seed_byte as usize % val.len()])),
        None => None
    }
}

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
        _ => None
    };

    match choice {
        Some(val) => Some(val.as_bytes()[seed_byte % val.len()]),
        None => None
    }
}

pub fn u32_to_bytes(u: u32) -> [u8; 4] {
    [
        ((u >> 24) & 0xff) as u8,
        ((u >> 16) & 0xff) as u8,
        ((u >> 8) & 0xff) as u8,
        (u & 0xff) as u8
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_scope_for_valid_variant() {
        assert!(scope_for_variant("password") == Some(String::from("com.lyndir.masterpassword")));
    }

    #[test]
    fn get_scope_for_invalid_variant() {
        assert!(scope_for_variant("invalid") == None);
    }

    #[test]
    fn get_template_for_valid_type() {
        assert!(template_for_type("long", &(11 as u8)) == Some(String::from("CvcvCvcvCvccno")));
    }

    #[test]
    fn get_template_for_invalid_type() {
        assert!(template_for_type("invalid", &(11 as u8)) == None);
    }

    #[test]
    fn get_character_from_valid_class() {
        assert!(character_from_class(b'v', 11 as usize) == Some(b'e'));
    }

    #[test]
    fn get_character_from_invalid_class() {
        assert!(character_from_class(b'z', 11 as usize) == None);
    }

    #[test]
    fn get_bytes_from_u32() {
        assert!(u32_to_bytes(2) == [0, 0, 0, 2]);
    }
}
