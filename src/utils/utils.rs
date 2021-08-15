use regex::Regex;

pub fn convert_to_id(mention_or_id: String) -> anyhow::Result<u64> {
    let re = Regex::new("[<!@>]")?;
    let replaced = re.replace_all(&mention_or_id, "");
    // println!("rep: {}", replaced);
    // println!("rep: {}", replaced);

    Ok(replaced.parse::<u64>()?)
}
