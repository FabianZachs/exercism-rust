pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::from("");
    }
    let mut proverb = String::new();

    for i in 0..(list.len() - 1) {
        proverb.push_str(&get_line(list[i], list[i + 1]));
    }

    proverb.push_str(&format!("And all for the want of a {}.", list[0]));
    proverb
}

fn get_line(w1: &str, w2: &str) -> String {
    format!("For want of a {} the {} was lost.\n", w1, w2)
}
