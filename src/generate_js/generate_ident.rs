pub fn generate_ident(ident: u32) -> String {
    let mut out: Vec<String> = Vec::new();

    for _ in 0..ident {
        out.push(" ".into());
    }

    return out.join("");
}
