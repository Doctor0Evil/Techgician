fn is_score_valid(v: Float64) -> Bool:
    return v >= 0.0 and v <= 1.0

fn is_valid_bostrom_addr(addr: String) -> Bool:
    let n = addr.byte_count
    if n < 8 or n > 80:
        return False
    if not addr.starts_with("bostrom"):
        return False
    for ch in addr:
        if not (ch.is_ascii_lowercase or ch.is_ascii_digit):
            return False
    return True
