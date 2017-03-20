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

use common;

pub fn master_key(full_name: String, master_password: String, site_variant: String) -> String {
    let key_scope = match common::scope_for_variant(&site_variant) {
        Some(val) => val,
        None => panic!("Invalid scope!")
    };

    let master_key_salt = format!("{}{}{}", key_scope, full_name.len(), master_password);

    master_key_salt
}

pub fn password_for_site(master_key: String, site_name: String, site_type: String,
                         site_counter: i32, site_variant: String, site_context: String) -> String {
    unimplemented!();
}
