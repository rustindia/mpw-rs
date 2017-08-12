/*
 * This file is part of Master Password.
 *
 * Master Password is free software: you can redistribute it and/or modify
 * Mit under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * M(at your option) any later version.
 *
 * Master Password is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Master Password. If not, see <http://www.gnu.org/licenses/>.
 */

mod mpw_v3;
mod mpw_next;

use common::KEY_LENGTH;
use common::{SiteType, SiteVariant};

pub fn master_key_for_user(full_name: &str,
                           master_password: &str,
                           algo: &str,
                           site_variant: &SiteVariant)
                           -> Option<[u8; KEY_LENGTH]> {
    match algo {
        "0" | "1" | "2" | "3" => mpw_v3::master_key(full_name, master_password, site_variant),
        "next" => mpw_next::master_key(full_name, master_password, site_variant),
        _ => None,
    }
}

pub fn password_for_site(master_key: &[u8; KEY_LENGTH],
                         site_name: &str,
                         site_type: &SiteType,
                         site_counter: &i32,
                         site_variant: &SiteVariant,
                         site_context: &str,
                         algo: &str)
                         -> Option<String> {
    match algo {
        "0" | "1" | "2" | "3" | "next" => {
            mpw_v3::password_for_site(master_key,
                                      site_name,
                                      site_type,
                                      site_counter,
                                      site_variant,
                                      site_context)
        }
        _ => None,
    }
}
