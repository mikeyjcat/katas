extern crate regex;

pub fn get_demiliter(string_numbers: &str) -> (&str, usize) {
	let re_custom = regex::Regex::new("//(.)\n").unwrap();

	if re_custom.is_match(string_numbers) {
		return (re_custom.captures(string_numbers).unwrap().at(1).unwrap(), 4);
	} else {
		return (",|\n", 0);
	}
}

pub fn add_string(string_numbers: &str) -> i32 {
	
	let mut sum: i32 = 0;
	let mut list_negatives: Vec<i32> = vec![];

	if string_numbers == "" {
		sum = 0;
	} else {
		let (delimiter, string_start) = get_demiliter(string_numbers);
		let re = regex::Regex::new(delimiter).unwrap();
		
		for s in re.split(&string_numbers[string_start..]) {
			let x:i32 = s.trim().parse().unwrap();
			if x < 0 {
				list_negatives.push(x);
			} else if x <= 1000 {
				sum += x;
			}	
		}
	}
	if list_negatives.len() != 0 {
		println!("{:?}", list_negatives );
		panic!("Negatives found");
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
	fn should_return_number_when_given_single_number() {
		assert_eq!(0, add_string("0"));
		assert_eq!(5, add_string("5"));
		assert_eq!(23, add_string("23"));
	}
	#[test]
	fn should_return_sum_when_given_two_numbers_comma_del() {
		assert_eq!(3, add_string("1,2"));
		assert_eq!(23, add_string("10,13"));
	}
	#[test]
	fn should_return_sum_when_given_n_numbers_comma_del() {
		assert_eq!(21, add_string("1,2,3,4,5,6"));
	}
	#[test]
	fn should_return_sum_when_given_n_numbers_comma_or_nl_del() {
		assert_eq!(21, add_string("1,2,3\n4,5,6"));
	}
	#[test]
	fn should_return_sum_when_given_n_numbers_custom_del() {
		assert_eq!(21, add_string("//;\n1;2;3;4;5;6"));
	}
	#[test]
	#[should_panic]
	fn should_return_exception_when_given_any_negatives() {
		assert_eq!(21, add_string("1,-2,3,-4,5,-6"));
	}
	#[test]
	fn should_not_add_number_gt_1000() {
		assert_eq!(19, add_string("1,1001,3,4,5,6"));
	}
}