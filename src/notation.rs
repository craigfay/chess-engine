
// Get algebraic notation from index
pub fn index(square: usize) -> String {
    let rank = (square as u8 % 8 + 97) as char;
    let file = (square / 8) + 1;
    String::from(format!("{}{}", rank, file))
}

// Get index notation from algebraic
pub fn algebraic(s: &str) -> Option<usize> {
    let mut chars = s.chars();
    let file = chars.next();
    let rank = chars.next();
    let end = chars.next();

    if !rank.is_some() { return None }
    if !file.is_some() { return None }
    if end.is_some() { return None }

    if !file.unwrap().is_alphabetic() { return None }
    if !rank.unwrap().is_numeric() { return None }

    let file = file.unwrap() as usize - 97;
    let rank = (rank.unwrap().to_digit(10).unwrap() as usize - 1) * 8;

    Some(file + rank)
}

