#[derive(strum_macros::Display)]
pub enum AttackType {
    Psychic(String, i8),
    Verbal(String, i8),
    Physical(String, i8),
    Whimsical(String, i8),
}