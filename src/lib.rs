use std::collections::HashSet;
use std::str::CharIndices;

pub struct WordController<'a> {
	string: &'a String,
	char_iter: CharIndices<'a>,
	last: usize,
	curr: usize,
}

impl<'a> WordController<'a> {
	pub fn new(string: &String) -> WordController {
		WordController {
			string: string,
			char_iter: string.char_indices(),
			last: 0,
			curr: 0,
		}
	}

	pub fn take_part(&mut self) -> &str {
		let s = unsafe {
			self.string.get_unchecked(self.last..self.curr)
		};
		self.last = self.curr;

		s
	}
}

impl<'a> Iterator for WordController<'a> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some((i, j)) = self.char_iter.next() {
			self.curr = i;
			Some(j)
		} else {
			None
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn check_take_part() {

    }
}
