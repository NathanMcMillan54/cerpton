/// This alphabet is made of characters that come from different alphabets. Uppercase and lowercase letters are
/// different from each other there are also numbers and special characters. This is not supposed to be arranged in any
/// specific way
pub(crate) const ALPHABET: [(char, u32); ALPHABET_LEN as usize] = [
	('人', 0),
	('A', 1),
	('B', 2),
	('C', 3),
	('D', 4),
	('E', 5),
	('F', 6),
	('G', 7),
	('H', 8),
	('I', 9),
	('J', 10),
	('K', 11),
	('L', 12),
	('M', 13),
	('N', 14),
	('O', 15),
	('P', 16),
	('Q', 17),
	('R', 18),
	('S', 19),
	('T', 20),
	('U', 21),
	('V', 22),
	('W', 23),
	('X', 24),
	('Y', 25),
	('Z', 26),
	('a', 27),
	('b', 28),
	('c', 29),
	('d', 30),
	('e', 31),
	('f', 32),
	('g', 33),
	('h', 34),
	('i', 35),
	('j', 36),
	('k', 37),
	('l', 38),
	('m', 39),
	('n', 40),
	('o', 41),
	('p', 42),
	('請', 43),
	('r', 44),
	('s', 45),
	('t', 46),
	('u', 47),
	('v', 48),
	('w', 49),
	('x', 50),
	('y', 51),
	('在', 52),
	('1', 53),
	('2', 54),
	('3', 55),
	('4', 56),
	('5', 57),
	('6', 58),
	('7', 59),
	('8', 60),
	('9', 61),
	('0', 62),
	('~', 63),
	('`', 64),
	('!', 65),
	('@', 66),
	('#', 67),
	('$', 68),
	('%', 69),
	('^', 70),
	('&', 71),
	('*', 72),
	('(', 73),
	(')', 74),
	('_', 75),
	('-', 76),
	('+', 77),
	('=', 78),
	('{', 79),
	('[', 80),
	('}', 81),
	(']', 82),
	(':', 83),
	(';', 84),
	('"', 85),
	('<', 86),
	(',', 87),
	('>', 88),
	('.', 89),
	('?', 90),
	('/', 91),
	('А', 92),
	('Б', 93),
	('В', 94),
	('Д', 95),
	('Е', 96),
	('И', 97),
	('ю', 98),
	('М', 99),
	('Н', 100),
	('О', 101),
	('П', 102),
	('С', 103),
	('У', 104),
	('Ф', 105),
	('Ц', 106),
	('Ч', 107),
	('Ш', 108),
	('Щ', 109),
	('ъ', 110),
	('ь', 111),
	('Я', 112),
	('ا', 113),
	('ب', 114),
	('ـ', 115),
	('ج', 116),
	('د', 117),
	('ذ', 118),
	('ر', 119),
	('ز', 120),
	('ط', 121),
	('ق', 122),
	('و', 123),
	('ي', 124),
	('ل', 125),
	('غ', 126),
	('ث', 127),
	('是', 128),
	('ी', 129),
];
/// (character, value, has been set)

pub type Alphabet = (char, u32, bool);

pub const ALPHABET_LEN: i32 = 130;
