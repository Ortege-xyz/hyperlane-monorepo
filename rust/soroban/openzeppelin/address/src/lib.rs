use soroban_sdk::{Address, xdr::{ScVal, ScAddress}};

pub struct AddressLib;

impl AddressLib {
    pub fn is_contract(account: Address) -> Result<bool, core::fmt::Error>{
        let sc_val = ScVal::try_from(account).map_err(|_| core::fmt::Error)?;
        if let ScVal::Address(addr) = sc_val {
            match addr {
                ScAddress::Account(_account_id) => {
                    return Ok(false);
                }
                ScAddress::Contract(_contract_id) => {
                    return Ok(true);
                }
            }
        } else {
            return Ok(false);
        }
    }
}

#[cfg(test)]
mod tests;
