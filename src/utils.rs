// Copyright 2019-2020 Liebi Technologies.
// This file is part of Bifrost.

// Bifrost is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Bifrost is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Bifrost.  If not, see <http://www.gnu.org/licenses/>.

use std::{
	path::Path, 
	fs::{File, OpenOptions}, 
	error::Error, 
	io::{Read, Write}
};

pub fn read_json_from_file(json_path: &str) -> Result<String, Box<dyn Error>> {
	let path = Path::new(json_path);
	let mut file = File::open(path)?;
	let mut json_str = String::new();
	file.read_to_string(&mut json_str)?;

	Ok(json_str)
}

pub fn write_json_to_file(json_str: &str, json_path: &str) -> Result<(), Box<dyn Error>> {
	let path = Path::new(json_path);
	let mut file = OpenOptions::new()
					.read(true)
					.write(true)
					.create(true)
					.open(path)?;
	file.write(json_str.as_bytes())?;

	Ok(())
}
