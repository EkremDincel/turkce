use turkce::*;

fn main() {
    	let s = "merhaba dünya".to_string();
        let mut w = WordController::new(&s);
        while let Some(i) = w.next() {
        	if i == 'a' {
        		println!("{}", w.take_part());
        	}
        }
}