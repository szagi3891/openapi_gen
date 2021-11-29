

fn first_letter_to_up(name: &str) -> String {
    let mut out = Vec::<char>::new();

    for (key, char) in name.chars().enumerate() {
        if key == 0 {
            for char2 in char.to_uppercase() {
                out.push(char2);
            }
        } else {
            out.push(char);
        }
    }

    out.into_iter().collect()
}

pub fn fix_to_camel_case(param_name: &String) -> String {
    let mut out = Vec::<String>::new();

    let chunks = param_name.split('-');

    for (key, item) in chunks.enumerate() {
        if key == 0 {
            out.push(item.to_string());
        } else {
            out.push(first_letter_to_up(item));
        }
    }

    out.join("")
}
