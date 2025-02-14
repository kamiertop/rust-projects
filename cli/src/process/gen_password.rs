use rand::rngs::ThreadRng;
use rand::seq::{IndexedRandom, SliceRandom};
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHGKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBERS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*_";
pub fn process_gen_password(
	length: u8, upper: bool, lower: bool, numbers: bool, symbol: bool, show_strength: bool
) -> anyhow::Result<String> {
	let mut rng = rand::rng();
	let mut password = Vec::new();
	let mut chars = Vec::new();
	if length <= 0 {
		return Err(anyhow::anyhow!("密码长度必须大于0"));
	}
	// 移除I和l, 容易混淆
	if lower {
		password.push(choose_char(&mut chars,LOWER, &mut rng));
	}
	if numbers {
		// 数字0和字母o容易给人造成错觉, 所以都去掉
		password.push(choose_char(&mut chars, NUMBERS, &mut rng))
	}
	if upper {
		// chars.extend_from_slice(UPPER);
		password.push(choose_char(&mut chars, UPPER, &mut rng))
	}

	if symbol {
		chars.extend_from_slice(SYMBOLS);
		password.push(choose_char(&mut chars, SYMBOLS, &mut rng))
	}

	for _ in 0..(length- password.len() as u8) {
		let c = chars.choose(&mut rng).expect("chars won't be empty in this context");
		password.push(*c as char)
	}
	// 打乱
	password.shuffle(&mut rng);

	if show_strength {
		let result = zxcvbn(&String::from_iter(password.clone()), &[]);
		eprintln!("Password strength: {}", result.score());
	}

	Ok(String::from_iter(password))
}

fn choose_char(chars: &mut Vec<u8>,slice: &[u8], rng: &mut ThreadRng) -> char {
	chars.extend_from_slice(slice);
	*slice.choose(rng).expect("slice won't be empty in this context") as char
}