extern crate bcrypt;
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
use self::bcrypt::hash;
use common::SiteType;
use common::SiteVariant;
use core::master_key_for_user;
use core::password_for_site;

fn calc_speed(elapsed: time::Duration, iterations: u32) -> f64 {
    let seconds = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    let speed = iterations as f64 / seconds;
    println!(" * Completed {} iterations in {:0.2} seconds at {:0.2} ops/s.",
             iterations,
             seconds,
             speed);
    speed
}

#[allow(unused_must_use)]
pub fn mpw_bench() {
    let full_name = "Robert Lee Mitchel";
    let master_password = "banana colored duckling";
    let site_name = "masterpasswordapp.com";
    let site_counter = 1_i32;
    let site_type = SiteType::Long;
    let site_variant = SiteVariant::Password;
    let site_context = "";
    let algo = "3";

    println!("\n## Benchmarking Master Password\n");

    let master_key = master_key_for_user(&full_name, &master_password, &algo, &site_variant)
        .unwrap();
    let iterations = 3_000_000;
    println!("### Performing {} iterations of {}:",
             iterations,
             "hmac-sha-256");
    let start = time::Instant::now();
    for _ in 1..iterations {
        hmac::sign(&hmac::SigningKey::new(&digest::SHA256, &master_key),
                   "".as_bytes());
    }
    let hmac_sha256_speed = calc_speed(start.elapsed(), iterations);

    let bcrypt_cost = 9;
    let iterations = 1000;
    println!("### Performing {} iterations of {}:", iterations, "bcrypt9");
    let start = time::Instant::now();
    for _ in 1..iterations {
        hash(master_password, bcrypt_cost);
    }
    let bcrypt_9_speed = calc_speed(start.elapsed(), iterations);

    let iterations = 50;
    println!("### Performing {} iterations of {}:",
             iterations,
             "scrypt_mpw");
    let start = time::Instant::now();
    for _ in 1..iterations {
        master_key_for_user(full_name, master_password, algo, &site_variant);
    }
    let scrypt_speed = calc_speed(start.elapsed(), iterations);

    let iterations = 50;
    println!("### Performing {} iterations of {}:", iterations, "mpw");
    let start = time::Instant::now();
    for _ in 1..iterations {
        let key = master_key_for_user(full_name, master_password, algo, &site_variant).unwrap();
        password_for_site(&key,
                          site_name,
                          &site_type,
                          &site_counter,
                          &site_variant,
                          &site_context,
                          algo);
    }
    let mpw_speed = calc_speed(start.elapsed(), iterations);

    println!("\n### Summary for this machine:");
    println!(" * mpw is {:0.2} times slower than hmac-sha-256.",
             hmac_sha256_speed / mpw_speed);
    println!(" * mpw is {:0.2} times slower than bcrypt (cost 9).",
             bcrypt_9_speed / mpw_speed);
    println!(" * scrypt is {:0.2} times slower than bcrypt (cost 9).",
             bcrypt_9_speed / scrypt_speed);

    println!("\n## Benchmark complete");
}
