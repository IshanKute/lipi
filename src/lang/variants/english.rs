use crate::lang::traits::LanguageConfig;

#[derive(Clone)]
pub struct English;

impl LanguageConfig for English {
  fn get_print_keyword(&self) -> &str {
      "print"
  }
}
