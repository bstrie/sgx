// SPDX-License-Identifier: Apache-2.0

//! Page SecInfo (Section 38.11)
//! These structs specify metadata about en enclave page.

mod class;
mod perms;
mod sinfo;

pub use class::Class;
pub use perms::Perms;
pub use sinfo::SecInfo;
