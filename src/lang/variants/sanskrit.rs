use crate::lang::traits::LanguageConfig;

#[derive(Clone)]
pub struct Sanskrit;

impl LanguageConfig for Sanskrit {
  fn get_print_keyword(&self) -> &str {
      "likh"
  }
}
