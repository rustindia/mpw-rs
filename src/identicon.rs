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

use ring::{digest, hmac};

pub fn generate(full_name: &str, master_password: &str) -> String {
    let left_arm = vec!['╔', '╚', '╰', '═'];
    let right_arm = vec!['╗', '╝', '╯', '═'];
    let body = ['█', '░', '▒', '▓', '☺', '☻'];
    let accessory = vec!['◈', '◎', '◐', '◑', '◒', '◓', '☀', '☁', '☂', '☃',
                         '☄', '★', '☆', '☎', '☏', '⎈', '⌂', '☘', '☢', '☣',
                         '☕', '⌚', '⌛', '⏰', '⚡', '⛄', '⛅', '☔', '♔', '♕',
                         '♖', '♗', '♘', '♙', '♚', '♛', '♜', '♝', '♞', '♟',
                         '♨', '♩', '♪', '♫', '⚐', '⚑', '⚔', '⚖', '⚙', '⚠',
                         '⌘', '⏎', '✄', '✆', '✈', '✉', '✌'];

    let signing_key = hmac::SigningKey::new(&digest::SHA256, master_password.as_bytes());
    let output = hmac::sign(&signing_key, full_name.as_bytes());
    let identicon_seed = output.as_ref();

    format!("{}{}{}{}",
            left_arm[identicon_seed[0] as usize % left_arm.len()],
            body[identicon_seed[1] as usize % body.len()],
            right_arm[identicon_seed[2] as usize % right_arm.len()],
            accessory[identicon_seed[3] as usize % accessory.len()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_identicon_for_full_name_and_master_password() {
        assert_eq!(generate("test", "test"), "╔░╝☂");
    }
}
