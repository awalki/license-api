pub fn generate_license_key() -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let chars: Vec<char> = charset.chars().collect();

    let mut key = String::new();

    for segment in 0..5 {
        if segment > 0 {
            key.push('-');
        }

        for _ in 0..5 {
            let idx = fastrand::usize(0..chars.len());
            key.push(chars[idx]);
        }
    }

    key
}
