#[macro_export]
macro_rules! load_input {
    () => {
        include_str!("./input").split("\n").collect::<Vec<&str>>()
    };
}
