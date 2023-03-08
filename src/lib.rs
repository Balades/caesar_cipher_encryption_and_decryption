pub fn shifter(character: char, shift: i32) -> char {
    let character_shift = character as i32 + shift;
    if character_shift >= 97 && character_shift <= 122 && character.is_lowercase() {
        character_shift as u8 as char
    } else if character_shift >= 65 && character_shift <= 90 && character.is_uppercase() {
        character_shift as u8 as char
    } else if character_shift < 97 && character.is_lowercase() {
        let resulting_shift = 123 - (97 - character_shift);
        resulting_shift as u8 as char
    } else if character_shift > 122 && character.is_lowercase() {
        let resulting_shift = (character_shift - 122) + 96;
        resulting_shift as u8 as char
    } else if character_shift < 65 && character.is_uppercase() {
        let resulting_shift = 91 - (65 - character_shift);
        resulting_shift as u8 as char
    } else if character_shift > 90 && character.is_uppercase() {
        let resulting_shift = 64 + (character_shift - 90);
        resulting_shift as u8 as char
    } else {
        character
    }
}

pub fn encrypt(message: &String, shift: i32) -> String {
    let mut encrypted_message = String::new();
    if shift > -27 && shift < 27{
        for character in message.chars() {
            encrypted_message.push(shifter(character, shift));
        }
        encrypted_message
    } 
    else {
        "Shift Error, make sure the range is -27 < shift < 27".to_string()
    }
}

pub fn decrypt(message: &String, shift: i32) -> String {
    let mut decrypted_message = String::new();
    if shift > -27 && shift < 27 {
        for character in message.chars() {
        let decrypted_shift = shifter(character, shift * -1);
        decrypted_message.push(decrypted_shift);
    } 
        decrypted_message
    } else {
        "Shift Error, make sure the range is -27 < shift < 27".to_string()
    }
}
