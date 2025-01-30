use crate::lang::traits::LanguageConfig;

#[derive(Clone)]
pub struct Hindi;

impl LanguageConfig for Hindi {
  fn get_print_keyword(&self) -> &str {
      "likho"
  }
}
