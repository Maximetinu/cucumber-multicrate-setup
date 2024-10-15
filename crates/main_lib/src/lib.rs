use sub_lib::Cat;

pub fn are_cats_hungry_by_default() -> bool {
    Cat::default().hungry
}
