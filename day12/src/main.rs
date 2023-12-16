use cached::proc_macro::cached;

#[cached]
fn calc(p: String, check: Vec<u32>, idx: u32) -> u64 {
    if p.is_empty() {
        return if check.is_empty() { 1u64 } else { 0u64 };
    }

    let mut n: u64 = 0;

    let ch = p.chars().nth(0).unwrap();

    if ch == '#' || ch == '?' {
        n = n + calc(p[1..].to_string(), check.clone(), idx + 1)
    }

    if (ch == '.' || ch == '?') && (!check.is_empty() && check[0] == idx || idx == 0) {
        n = n + calc(
            p[1..].to_string(),
            if idx != 0 {
                check[1..].to_vec()
            } else {
                check.to_vec()
            },
            0,
        );
    }
    n
}

fn main() {
    let input: &str = include_str!("../input/12.txt");
    let lines: Vec<&str> = input.lines().collect();
    let mut sum: Vec<u64> = Vec::new();

    for prt in &lines {
        let parts: Vec<&str> = prt.split_whitespace().collect();

        if let (Some(string_part), Some(vector_part_str)) = (parts.get(0), parts.get(1)) {
            let vector_part: Vec<u32> = vector_part_str.split(',').flat_map(str::parse).collect();
            let repeated_vector: Vec<_> = vector_part
                .iter()
                .cycle()
                .take(vector_part.len() * 5)
                .cloned()
                .collect();
            let string_part = string_part.to_string();
            let mut multiplied_string: String = std::iter::repeat(string_part)
                .take(5)
                .collect::<Vec<_>>()
                .join("?");

            multiplied_string.push('.');

            let count: u64 = calc(multiplied_string.to_string(), repeated_vector, 0);
            sum.push(count);
        }
    }
    let sum: u64 = sum.iter().sum();

    println!("{}", sum);
}
