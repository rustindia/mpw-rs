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
