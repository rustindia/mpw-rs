/*
 * This file is part of Master Password.
 *
 * Master Password is free software: you can redistribute it and/or modify
 * Mit under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Master Password is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Master Password. If not, see <http://www.gnu.org/licenses/>.
 */

// use std::time;
// use ring::{digest, hmac};
// use bcrypt::hash;
// use crate::common::SiteType;
// use crate::common::SiteVariant;
// use crate::core::master_key_for_user;
// use crate::core::password_for_site;
//
// fn calc_speed(elapsed: time::Duration, iterations: u32) -> f64 {
//     let seconds = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
//     let speed = iterations as f64 / seconds;
//     println!(" * Completed {} iterations in {:0.2} seconds at {:0.2} ops/s.",
//              iterations,
//              seconds,
//              speed);
//     speed
// }

use crate::common::SiteType;
use crate::common::SiteVariant;
use crate::core::master_key_for_user;
use crate::core::password_for_site;
use bcrypt::hash;
use ring::{digest, hmac};
extern crate test as TEST;
use TEST::Bencher;

#[bench]
fn hmac_sha256(b: &mut Bencher) {
    let full_name = "Robert Lee Mitchel";
    let master_password = "banana colored duckling";
    let site_variant = SiteVariant::Password;
    let algo = "3";
    let master_key: [u8; 64] =
        master_key_for_user(&full_name, &master_password, &algo, &site_variant).unwrap();
    b.iter(|| {
        hmac::sign(
            &hmac::SigningKey::new(&digest::SHA256, &master_key),
            "".as_bytes(),
        );
    })
}

#[bench]
fn bcrypt9(b: &mut Bencher) {
    let master_password = "banana colored duckling";
    let bcrypt_cost = 9;
    b.iter(|| hash(master_password, bcrypt_cost))
}

#[bench]
fn scrypt_mpw(b: &mut Bencher) {
    let full_name = "Robert Lee Mitchel";
    let master_password = "banana colored duckling";
    let site_variant = SiteVariant::Password;
    let algo = "3";

    b.iter(|| master_key_for_user(full_name, master_password, algo, &site_variant));
}

#[bench]
fn mpw(b: &mut Bencher) {
    let full_name = "Robert Lee Mitchel";
    let master_password = "banana colored duckling";
    let site_name = "masterpasswordapp.com";
    let site_counter = 1_i32;
    let site_type = SiteType::Long;
    let site_variant = SiteVariant::Password;
    let site_context = "";
    let algo = "3";

    b.iter(|| {
        let key = master_key_for_user(full_name, master_password, algo, &site_variant).unwrap();
        password_for_site(
            &key,
            site_name,
            &site_type,
            &site_counter,
            &site_variant,
            &site_context,
            algo,
        );
    });
}
