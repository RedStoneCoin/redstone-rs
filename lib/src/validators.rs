use log::warn;

use crate::{account::Account, block::Block, crypto::hash, crypto::Vrf, database::Database};

#[derive(Debug, Clone)]
pub struct Validator {
    pub address: String,
    pub public_key: String,
    pub age: u64,
    pub delegated_to: String,
    pub online: bool,
    pub stake: u64,
}

pub fn choose_next_proposer(block: Block) -> Result<Validator, Box<dyn std::error::Error>> {
    // take the VRF hash from the header
    let vrf = Vrf::from_proof(&block.header.vrf)?;
    let shuffle_number = u64::from_str_radix(
        &hash(vrf.numerical()?.0.to_string().as_bytes().to_vec())[..32].to_string(),
        16,
    )?; // use this to shuffle the node list
    let mut online = Validator::get_online()?;
    online.sort_by(|a, b| {
        hash(
            format!("{}{}", a.clone().public_key, &shuffle_number.to_string())
                .as_bytes()
                .to_vec(),
        )
        .cmp(&hash(
            format!("{}{}", b.clone().public_key, &shuffle_number.to_string())
                .as_bytes()
                .to_vec(),
        ))
    });
    // online is now shuffled, choose the proposer
    let mut coin_culmulative: u64 = 0;
    let total_stake: u64 = online.iter().map(|v| v.stake).sum();
    let target_coin = vrf.numerical()?.0 % total_stake;
    let mut trys = 0;
    loop {
        for validator in &online {
            coin_culmulative += validator.stake;
            if coin_culmulative >= target_coin {
                // we have found our proposer
                return Ok(validator.clone());
            }
        }
        log::warn!("Could not find a validator, trying again, total_stake: {}, target_coin: {}, coin_culmulative: {}, online_count {}", total_stake, target_coin, coin_culmulative, online.len());
        // we could not find a proposer, retry
        // This should not happen, but this is a saftey catch. WCS we return the first validator
        // TODO: Risk assess this, can this situation be triggered and then exploited?

        trys += 1;

        if trys >= 5 {
            // we have tried too many times, return the first validator
            log::error!("Unexpected error: failed to find proposer after 5 trys, total_stake: {}, target_coin: {}, coin_culmulative: {}, online_count {}", total_stake, target_coin, coin_culmulative, online.len());
            return Ok(online[0].clone());
        }
    }
}

pub fn form_validating_commitee(
    block: Block,
    proposer: Validator,
    size: u64,
) -> Result<Vec<Validator>, Box<dyn std::error::Error>> {
    if size % 2 != 0 {
        return Err("Size must be even".into());
    }
    // take the VRF hash from the header

    let vrf = Vrf::from_proof(&block.header.vrf)?;
    let shuffle_number = u64::from_str_radix(
        &hash(vrf.numerical()?.0.to_string().as_bytes().to_vec())[..32].to_string(),
        16,
    )?; // use this to shuffle the node list
    let mut online = Validator::get_online()?;
    online.sort_by(|a, b| {
        hash(
            format!("{}{}", a.clone().public_key, &shuffle_number.to_string())
                .as_bytes()
                .to_vec(),
        )
        .cmp(&hash(
            format!("{}{}", b.clone().public_key, &shuffle_number.to_string())
                .as_bytes()
                .to_vec(),
        ))
    });
    let mut validator_index: usize = 0;
    let mut set_validator_index = false;
    for (i, validator) in online.iter().enumerate() {
        if validator.public_key == proposer.public_key {
            validator_index = i;
            set_validator_index = true;
            break;
        }
    }
    if !set_validator_index {
        return Err("Failed to find proposer in online nodes set".into());
    }
    // choose the surrounding size / 2 validators
    let mut validators: Vec<Validator> = vec![];
    let step = size / 2;
    if (validator_index as i64 - step as i64) < 0 {
        // wrap around
        let overflow = (validator_index as i64 - step as i64).abs();
        for i in 0..validator_index {
            validators.push(online[i].clone())
        }
        // now the overflow
        for i in (online.len() as i64 - overflow)..online.len() as i64 {
            validators.push(online[i as usize].clone());
        }
    } else {
        for i in (validator_index - step as usize)..validator_index {
            validators.push(online[i as usize].clone());
        }
    }

    // reverse
    if (validator_index as i64 + step as i64) < online.len() as i64 {
        // wrap around
        let overflow = (online.len() as i128 - (validator_index as i128 + step as i128)).abs();
        for i in validator_index..online.len() {
            validators.push(online[i].clone())
        }
        // now the overflow
        for i in 0..overflow as i64 {
            validators.push(online[i as usize].clone());
        }
    } else {
        for i in (validator_index - step as usize)..validator_index {
            validators.push(online[i as usize].clone());
        }
    }
    if validators.len() != size as usize {
        return Err(format!(
            "Failed to form, selected size {}, formed size {}",
            size,
            validators.len()
        )
        .into());
    } else {
        return Ok(validators);
    }
}

impl Validator {
    pub fn from_string(as_string: String) -> Result<Validator, Box<dyn std::error::Error>> {
        let decoded_string = String::from_utf8(hex::decode(as_string)?)?;
        let split = decoded_string.split(".").collect::<Vec<&str>>();
        Ok(Validator {
            address: split[0].to_string(),
            public_key: split[1].to_string(),
            age: split[2].parse()?,
            delegated_to: split[3].to_string(),
            online: split[4].parse()?,
            stake: split[5].parse()?,
        })
    }
    pub fn to_string(&self) -> String {
        hex::encode(format!(
            "{}.{}.{}.{}.{}.{}",
            self.address, self.public_key, self.age, self.delegated_to, self.online, self.stake
        ))
    }
    pub fn get(address: &String) -> Result<Validator, Box<dyn std::error::Error>> {
        let mut db = Database::new();
        db.open(&String::from("validators"))?;
        if let Some(encoded) = db.get(&String::from("validators"), address)? {
            return Validator::from_string(encoded);
        } else {
            warn!("Validator {} does not exist in DB (key not found)", address);
            return Err("Validator not found in DB (key not found)".into());
        }
    }
    pub fn set(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = Database::new();
        db.open(&String::from("validators"))?;
        let encoded = self.to_string();
        db.set(&String::from("validators"), &self.address, &encoded)
    }
    pub fn get_online() -> Result<Vec<Validator>, Box<dyn std::error::Error>> {
        // TODO: Check our local "proof" cache db. This is a db that contains the last "action" of the validator that proves it is online (eg validating, proposing, voting)
        // USE STATE DB
        todo!()
    }
}
