use crate::lang::traits::LanguageConfig;

#[derive(Clone)]
pub struct Marathi;

impl LanguageConfig for Marathi {
  fn get_print_keyword(&self) -> &str {
      "liha"
  }
}
