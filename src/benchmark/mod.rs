extern crate ring;

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

use std::time;
use self::ring::{digest, hmac};
use common::SiteType;
use common::SiteVariant;
use core::master_key_for_user;
use core::password_for_site;

fn show_speed(start: time::Instant, elapsed: time::Duration, iterations: u32, operation: &str) {
    let seconds = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    let speed = iterations as f64 / seconds;
    println!("* {} {} iterations in {} seconds at {:0.2} ops/s",
             iterations, operation, seconds, speed);
}

pub fn mpw_bench() {
    let full_name = "Robert Lee Mitchel";
    let master_password = "banana colored duckling";
    let site_name = "masterpasswordapp.com";
    let site_counter = 1_i32;
    let site_type = SiteType::Long;
    let site_variant = SiteVariant::Password;
    let site_context = "";
    let algo = "3";

    println!("<<< Benchmarking Master Password >>>\n");

    let master_key = master_key_for_user(
        full_name,
        master_password,
        algo,
        &site_variant
    ).unwrap();

    let master_key = master_key_for_user(
        &full_name, &master_password, &algo, &site_variant).unwrap();
    let iterations = 3_000_000;
    let start = time::Instant::now();
    for _ in 1..iterations {
        hmac::sign(
            &hmac::SigningKey::new(&digest::SHA256, &master_key),
            "".as_bytes()
        );
    }
    show_speed(start, start.elapsed(), iterations, "hmac-sha-256");

    println!("\n<<< Benchmark complete >>>");
}
