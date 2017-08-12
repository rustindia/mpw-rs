extern crate clap;
extern crate rustyline;

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

use std::env;
use self::rustyline::error::ReadlineError;
use self::rustyline::Editor;

pub fn read_opt(matches: &clap::ArgMatches, name: &str, env_var: &str) -> Option<String> {
    match matches.value_of(name) {
        Some(value) => Some(value.to_string()),
        None => {
            match env::var(env_var) {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        }
    }
}

pub fn raw_input(prompt: &str) -> Option<String> {
    let mut rl = Editor::<()>::new();
    let read_line = rl.readline(prompt);

    match read_line {
        Ok(line) => Some(line),
        Err(ReadlineError::Interrupted) => {
            println!("Interrupted");
            None
        }
        Err(ReadlineError::Eof) => {
            println!("EOF Reached");
            None
        }
        Err(err) => {
            println!("Reading Error: {:?}", err);
            None
        }
    }
}
