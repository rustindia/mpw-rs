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
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Master Password.  If not, see <http://www.gnu.org/licenses/>.

mod helpers;

use clap::{Arg, App};

pub struct MpwOptions {
    pub site: String,
    pub user: String,
    pub variant: String,
    pub template: String,
    pub counter: String,
    pub algo: String,
    pub context: String
}

pub fn get_opts() -> MpwOptions {
    let matches = App::new("Master Password")
        .version("2.4.0")
        .author("Rahul De <rahul080327@gmail.com>, Maarten Billemont <lhunath@lyndir.com>")
        .about("The rusty, stateless password manager")
        .arg(Arg::with_name("site")
            .index(1)
            .value_name("SITE")
            .help("The name of the website."))
        .arg(Arg::with_name("user")
            .short("u")
            .long("user")
            .value_name("USER")
            .help("Specify the full name of the user.\n\
                                    Defaults to MP_FULLNAME in env")
            .takes_value(true))
        .arg(Arg::with_name("template")
            .short("t")
            .long("template")
            .value_name("TEMPLATE")
            .help("Specify the template of the password.\n\
                                    Defaults to MP_SITETYPE in env or 'long' for password, 'name' for login.\n\
                                        x, max, maximum | 20 characters, contains symbols.\n\
                                        l, long         | Copy-friendly, 14 characters, contains symbols.\n\
                                        m, med, medium  | Copy-friendly, 8 characters, contains symbols.\n\
                                        b, basic        | 8 characters, no symbols.\n\
                                        s, short        | Copy-friendly, 4 characters, no symbols.\n\
                                        i, pin          | 4 numbers.\n\
                                        n, name         | 9 letter name.\n\
                                        p, phrase       | 20 character sentence.")
            .takes_value(true))
        .arg(Arg::with_name("counter")
            .short("c")
            .long("counter")
            .value_name("COUNTER")
            .help("The value of the counter.\n\
                                    Defaults to MP_SITECOUNTER in env or 1.")
            .takes_value(true))
        .arg(Arg::with_name("algo")
            .short("a")
            .long("algo")
            .value_name("ALGO")
            .help("The algorithm version to use.\n\
                                    Defaults to MP_ALGORITHM in env or 3.")
            .takes_value(true))
        .arg(Arg::with_name("variant")
            .short("v")
            .long("variant")
            .value_name("VARIANT")
            .help("The kind of content to generate.\n\
                                    Defaults to 'password'.\n\
                                        p, password | The password to log in with.\n\
                                        l, login    | The username to log in as.\n\
                                        a, answer   | The answer to a security question.")
            .takes_value(true))
        .arg(Arg::with_name("context")
            .short("C")
            .long("context")
            .value_name("CONTEXT")
            .help("A variant-specific context.\n\
                                    Defaults to empty.\n\
                                        -v p, password | Doesn't currently use a context.\n\
                                        -v l, login    | Doesn't currently use a context.\n\
                                        -v a, answer   | Empty for a universal site answer or\n\
                                                       | the most significant word(s) of the question."))
        .get_matches();

    let site = match helpers::read_opt(&matches, "site", "") {
        Some(val) => val.to_string(),
        None => helpers::raw_input("Site Name: ")
    };

    let user = match helpers::read_opt(&matches, "user", "MP_FULLNAME") {
        Some(val) => val.to_string(),
        None => helpers::raw_input("Your full name: ")
    };

    let variant = match helpers::read_opt(&matches, "variant", "") {
        Some(val) => val.to_string(),
        None => "password".to_string()
    };

    let template = match helpers::read_opt(&matches, "template", "MP_SITETYPE") {
        Some(val) => val.to_string(),
        None => if variant == "password" {
            "long".to_string()
        } else if variant == "login" {
            "name".to_string()
        } else {
            unimplemented!()
        }
    };

    let counter = match helpers::read_opt(&matches, "counter", "MP_SITECOUNTER") {
        Some(val) => val.to_string(),
        None => "1".to_string()
    };

    let algo = match helpers::read_opt(&matches, "algo", "MP_ALGORITHM") {
        Some(val) => val.to_string(),
        None => "3".to_string()
    };

    let context = match helpers::read_opt(&matches, "context", "") {
        Some(val) => val.to_string(),
        None => String::new()
    };

    MpwOptions {
        site: site,
        user: user,
        variant: variant,
        template: template,
        counter: counter,
        algo: algo,
        context: context
    }
}
