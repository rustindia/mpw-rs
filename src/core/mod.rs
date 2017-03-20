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

mod mpw_v3;

pub fn master_key_for_user(full_name: String, master_password: String, algo: &str,
                           site_variant: String) -> Option<String> {
    match algo {
        "1" => unimplemented!(),
        "2" => unimplemented!(),
        "3" => Some(mpw_v3::master_key(full_name, master_password, site_variant)),
        _ => None
    }
}

pub fn password_for_site(master_key: String, site_name: String, site_type: String,
                         site_counter: i32, site_variant: String, site_context: String,
                         algo: &str) -> Option<String> {
    match algo {
        "1" => unimplemented!(),
        "2" => unimplemented!(),
        "3" => Some(mpw_v3::password_for_site(master_key, site_name, site_type, site_counter,
                                              site_variant, site_context)),
        _ => None
    }
}
