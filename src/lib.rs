pub fn generate(message: &str) -> String {
    let byte_array = message.as_bytes();

    let mut result = String::new();

    let mut prev = 0u8;
    let loop_length = 10u8;

    for current in byte_array {
        let diff: i16 = *current as i16 - prev as i16;

        let sign = if diff >= 0 {"+"} else {"-"};


        let loop_count = diff / loop_length as i16;
        let remainder = diff % loop_length as i16;

        if diff.abs() >= 10 {
            result.push_str(&"+".repeat(loop_count.unsigned_abs() as usize));
            result.push_str("[>");
            result.push_str(&sign.repeat(loop_length as usize));
            result.push_str("<-]");
        }
        result.push('>');
        result.push_str(&sign.repeat(remainder.unsigned_abs() as usize));
        result.push_str(".<");

        prev = *current;
    }

    result
}