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

pub struct MpwCharPair {
    pub class: u8,
    pub seed_byte: usize
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

pub fn character_from_class(pair: MpwCharPair) -> Option<u8> {
    let choice = match pair.class {
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
        Some(val) => Some(val.as_bytes()[pair.seed_byte % val.len()]),
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
