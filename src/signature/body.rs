// SPDX-License-Identifier: Apache-2.0

use crate::parameters::{Attributes, Masked, MiscSelect, Parameters};

impl Parameters {
    /// Creates a signature body
    ///
    /// This call creates a signature `Body` using the provided parameters and
    /// `mrenclave` value.
    ///
    /// Note that the `Masked` types in `Parameters` are interpreted as follows:
    ///   * `data`: contains the features the enclave author desires
    ///   * `mask`: contains the features the enclave author requires
    pub fn body(&self, mrenclave: [u8; 32]) -> Body {
        Body {
            misc: self.misc,
            reserved0: [0; 20],
            attr: self.attr,
            mrenclave,
            reserved1: [0; 32],
            isv_prod_id: self.isv_prod_id,
            isv_svn: self.isv_svn,
        }
    }
}

/// The enclave signature body
///
/// This structure encompasses the second block of fields from `SIGSTRUCT`
/// that is included in the signature. It is split out from `Signature`
/// in order to make it easy to hash the fields for the signature.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Body {
    misc: Masked<MiscSelect>,
    reserved0: [u8; 20],
    attr: Masked<Attributes>,
    mrenclave: [u8; 32],
    reserved1: [u8; 32],
    isv_prod_id: u16,
    isv_svn: u16,
}

impl core::fmt::Debug for Body {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Body")
            .field("misc", &self.misc)
            //.field("reserved0", &self.reserved0)
            .field("attr", &self.attr)
            .field("mrenclave", &self.mrenclave)
            //.field("reserved1", &self.reserved1)
            .field("isv_prod_id", &self.isv_prod_id)
            .field("isv_svn", &self.isv_svn)
            .finish()
    }
}

impl Body {
    /// Get the enclave measure hash
    pub fn mrenclave(&self) -> [u8; 32] {
        self.mrenclave
    }

    /// Get the enclave parameters
    pub fn parameters(&self) -> Parameters {
        Parameters {
            isv_prod_id: self.isv_prod_id,
            isv_svn: self.isv_svn,
            misc: self.misc,
            attr: self.attr,
        }
    }
}

#[cfg(test)]
testaso! {
    struct Body: 4, 128 => {
        misc: 0,
        reserved0: 8,
        attr: 28,
        mrenclave: 60,
        reserved1: 92,
        isv_prod_id: 124,
        isv_svn: 126
    }
}