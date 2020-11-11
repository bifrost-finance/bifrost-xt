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
	fmt::{self, Display},
	error,
};

#[derive(Clone, Debug)]
pub enum Error {
	WrongSudoSeed,
	SubxtError(&'static str),
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::WrongSudoSeed => write!(f, "Wrong sudo seed, failed to sign transaction."),
			Self::SubxtError(e) => write!(f, "Error from subxt crate: {}", e),
		}
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Self::WrongSudoSeed => "Wrong sudo seed, failed to sign transaction.",
			Self::SubxtError(e) => e,
		}
	}
}
