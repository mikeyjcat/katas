extern crate regex;

fn get_delimiter(string_numbers: &str) -> (&str, usize) {
	
	let re_custom_short = regex::Regex::new(r"//(.)\n").unwrap();

	if re_custom_short.is_match(string_numbers) {
		let delimiter = re_custom_short.captures(string_numbers)
							.unwrap()
							.at(1)
							.unwrap();
		return (delimiter, 4); 
	} else {
		return (",|\n", 0);
	}
}


pub fn add_string(string_numbers: &str) -> i32 {

	let mut sum: i32 = 0;
	let mut list_negatives: Vec<i32> = vec![];
	let (delimiter, start_position) = get_delimiter(string_numbers);
	println!("{:?}", delimiter);
	println!("{:?}", start_position);

	let re = regex::Regex::new(delimiter).unwrap();

	if string_numbers != "" {
		for s in re.split(&string_numbers[start_position..]) {
			let x: i32 = s.trim().parse().unwrap();
			if x < 0 {
				list_negatives.push(x);
			} else {
				if x < 1001 {
					sum += x; 
				}
			}
		}
	}

	if list_negatives.len() != 0 {
		panic!("negatives not allowed {:?}", list_negatives);
	}

	return sum;	
}

#[cfg(test)]
mod tests {
	use super::add_string;

	#[test]
	fn should_return_zero_when_given_empty_string() {
		assert_eq!(0, add_string(""));
	}
	#[test]
	fn should_return_number_when_given_one_number() {
		assert_eq!(0, add_string("0"));
		assert_eq!(5, add_string("5"));
		assert_eq!(23, add_string("23"));
	}
	#[test]
	fn should_return_sum_when_given_two_numbers_comma_delim() {
		assert_eq!(3, add_string("1,2"));
		assert_eq!(23, add_string("10,13"));
	}
	#[test]
	fn should_return_sum_when_given_any_numbers_comma_delim() {
		assert_eq!(21, add_string("1,2,3,4,5,6"));
	}
	#[test]
	fn should_return_sum_when_given_any_numbers_comma_or_nl_delim() {
		assert_eq!(21, add_string("1,2,3\n4,5,6"));
	}
	#[test]
	fn should_return_sum_when_given_any_numbers_custom_delim() {
		assert_eq!(21, add_string("//;\n1;2;3;4;5;6"));
	}
	#[test]
	#[should_panic]
	fn should_panic_when_given_any_negatives() {
		assert_eq!(-3, add_string("1,-2,3,-4,5,-6"));
	}
	#[test]
	fn should_ignore_numbers_gt_1000() {
		assert_eq!(1001, add_string("1,1000,1001"));
	}
	// #[test]
	// fn should_ignore_numbers_gt_1000() {
	// 	assert_eq!(1001, add_string("1,1000,1001"));
	// }
}
