use std::collections::HashSet;
use std::str::CharIndices;

pub struct StringSlicer<'a> {
	string: &'a String,
	char_iter: CharIndices<'a>,
	last: usize,
	curr: usize,
}

impl<'a> StringSlicer<'a> {
	pub fn new(string: &String) -> StringSlicer {
		StringSlicer {
			string: string,
			char_iter: string.char_indices(),
			last: 0,
			curr: 0,
		}
	}

	pub fn take_part(&mut self) -> &str {
		let s = unsafe { self.string.get_unchecked(self.last..self.curr) };
		self.last = self.curr;

		s
	}

	pub fn take_last_part(&mut self) -> &str {
		let s = unsafe { self.string.get_unchecked(self.last..) };
		self.last = self.curr;

		s
	}
}

impl<'a> Iterator for StringSlicer<'a> {
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
    fn test_string_slicer_take_part() {
      	let s = "merhaba dünya".to_string();
        let mut w = StringSlicer::new(&s);

        let t = ["merh", "ab", "a düny", "a"];
        let mut j = 0;
        // seet the bug at: https://github.com/rust-lang/rust/issues/8372
        while let Some(i) = w.next() {
        	if i == 'a' {
        		assert_eq!(t[j], w.take_part());
        		j+=1;
        	}
        }
        assert_eq!(t[j], w.take_last_part());
    }
}
