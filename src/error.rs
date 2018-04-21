pub fn report(line: u64, w: &str, message: &str) {
    println!("[line {}] Error{}: {}", line, w, message);
}

pub fn error(line: u64, message: &str) {
    report(line, "", message);
}

