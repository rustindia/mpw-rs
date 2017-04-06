extern crate clap;

// This file is part of Master Password.
//
// Master Password is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Master Password is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Master Password. If not, see <http://www.gnu.org/licenses/>.

use std::env;
use std::io::{self, Write};

pub fn read_opt(matches: &clap::ArgMatches, name: &str, env_var: &str) -> Option<String> {
    match matches.value_of(name) {
        Some(value)
        => Some(value.to_string()),
        None
        => match env::var(env_var) {
            Ok(val)
            => Some(val),
            Err(_)
            => None
        }
    }
}

pub fn raw_input(prompt: &str) -> Option<String> {
    let mut buffer = String::new();

    print!("{}", prompt);
    let _ = io::stdout().flush();

    match io::stdin().read_line(&mut buffer) {
        Ok(_)
        => Some(buffer.trim().to_string()),
        Err(_)
        => None
    }
}
