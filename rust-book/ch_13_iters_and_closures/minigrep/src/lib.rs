pub fn search<'a>(query: &'a str, contents: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    Box::new(contents.lines().filter(move |line| line.contains(query)))
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    let query = query.to_lowercase();
    Box::new(
        contents
            .lines()
            .filter(move |line| line.to_lowercase().contains(&query)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let expected = vec!["safe, fast, productive."];
        let actual: Vec<&str> = search(query, contents).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_result() {
        let query = ".";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let expected = vec!["safe, fast, productive.", "Pick three."];
        let actual: Vec<&str> = search(query, contents).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
