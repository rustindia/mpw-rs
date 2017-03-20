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

pub fn scope_for_variant(site_variant: &str) -> Option<String> {
    match site_variant {
        "password" => Some(String::from("com.lyndir.masterpassword")),
        "login" => Some(String::from("com.lyndir.masterpassword.login")),
        "answer" => Some(String::from("com.lyndir.masterpassword.answer")),
        _ => None
    }
}
