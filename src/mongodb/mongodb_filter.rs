pub struct Filter;

const FILTER_OR: &str = "$or";

impl Filter {

    pub fn or<'a>() -> &'a str {
        FILTER_OR
    }

}