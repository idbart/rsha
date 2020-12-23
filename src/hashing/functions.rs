
// functions used in the main loop of hashing
// all defined by the standard (i have no explanation)

pub fn Ch(x: u32, y: u32, z: u32) -> u32
{
	(x & y) ^ (!x & z)
}

pub fn Maj(x: u32, y: u32, z: u32) -> u32
{
	(x & y) ^ (x & z) ^ (y & z)
}

pub fn sigma_upper_0(x: u32) -> u32
{
	x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}
pub fn sigma_upper_1(x: u32) -> u32
{
	x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

pub fn sigma_lower_0(x: u32) -> u32
{
	x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}
pub fn sigma_lower_1(x: u32) -> u32
{
	x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}