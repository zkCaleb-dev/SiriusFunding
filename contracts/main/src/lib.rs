#![no_std]
#[warn(unused_imports)]
use soroban_sdk::{
    contract, 
    contractimpl, 
    vec, 
    Env, 
    String, 
    Vec, 
    Address, 
    contracttype, 
    contracterror, 
    Error, 
    BytesN,
    Symbol, 
    Val,
};

use soroban_sdk::token::{
    Client,
    TokenClient
};

mod events;

use crate::events::projects_by_project_id;
use crate::storage_types::{DataKey, Project, ProjectStatus};

#[contract]
pub struct CrowdfundingContract;

#[contracttype]
pub struct DonationData {
    pub donor: Address,
    pub amount: u128,
    pub timestamp: u64,
}


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[contracterror]
pub enum ContractError {
    AmountCannotBeZero = 1,
    ProjectAlreadyExists = 2,
    ProjectNotFound = 3,
    ProjectFundingEnded = 4,
    GoalNotReached = 5,
    AlreadyClaimed = 6,
    ProjectStillActive = 7,
    ProjectSuccessful = 8,
    NoFundsToRefund = 9,
    NotProjectCreator = 10,
    ProjectExpired = 11,
    AmountTooLarge = 12,
    NotAuthorized = 13,
    InvalidProjectStatus = 14,
    ClaimConditionsNotMet = 15,
}

#[contractimpl]
impl CrowdfundingContract {

    // pub fn init(env: Env, token: Address) {
    //     env.storage().instance().set(DataKey::TokenContract, &token);
    // }

    pub fn deploy(
        env: Env,
        deployer: Address,
        wasm_hash: BytesN<32>,
        salt: BytesN<32>,
        init_fn: Symbol,
        init_args: Vec<Val>,
    ) -> (Address, Val) {
        if deployer != env.current_contract_address() {
            deployer.require_auth();
        }

        let deployed_address = env
            .deployer()
            .with_address(deployer, salt)
            .deploy(wasm_hash);

        let res: Val = env.invoke_contract(&deployed_address, &init_fn, init_args);
        (deployed_address, res)
    }

    pub fn create_project(
        env: Env,
        project_id: String,
        creator: Address,
        goal: u128,
        deadline: u64,
        description: String,
    ) -> Result<String, ContractError> {


        creator.require_auth();

        if goal == 0 {
            return Err(ContractError::AmountCannotBeZero);
        }

        let key = DataKey::Project(project_id.clone());
        if env.storage().instance().has(&key) {
            return Err(ContractError::ProjectAlreadyExists);
        }

        // Add token_address to project struct
        let project = Project {
            creator: creator.clone(),
            goal,
            raised: 0,
            deadline,
            claimed: false,
            description,
            // You might want to add a status field to track if project is active/failed/successful
            status: ProjectStatus::Active
        };

        env.storage().instance().set(&key, &project);
        Ok(project_id)
    }

    pub fn fund_project(
        env: Env, 
        project_id: String, 
        donor: Address, 
        amount: u128,
    ) -> Result<(), ContractError> {
        donor.require_auth();
        if amount == 0 {
            return Err(ContractError::AmountCannotBeZero);
        }
        let project_key = DataKey::Project(project_id.clone());
        let mut project_result = Self::get_project_by_id(env.clone(), project_id.clone());

        let mut project = match project_result {
            Ok(pro) => pro,
            Err(err) => return Err(err),
        };
        let current_time = env.ledger().timestamp();
        // if current_time > project.deadline {
        //     return Err(ContractError::ProjectExpired);
        // }

        let amount_i128 = u128_to_i128(amount);

        let xlm_address_str = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC";
        let xlm_address_sdk_string = soroban_sdk::String::from_str(&env, xlm_address_str);
        let xlm_token = Address::from_str(&env, "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC");
        
        let token_client = TokenClient::new(&env, &xlm_token);

        if project.raised <= project.goal && project.raised + amount >= project.goal {
            // The goal is reached with this contribution
            project.status = ProjectStatus::GoalReached;
        }

        token_client.transfer(
            &donor, 
            &env.current_contract_address(), 
            &amount_i128
        );
        project.raised += amount;
        env.storage().instance().set(&project_key, &project);

        let donation_key = DataKey::Donation(project_id.clone(), donor.clone());
        let current_donation = env.storage().instance().get(&donation_key).unwrap_or(0u128);

        env.storage().instance().set(&donation_key, &(current_donation + amount));

        if current_donation == 0 {
            let donors_key = DataKey::ProjectDonors(project_id.clone());
            let mut donors = env.storage().instance().get(&donors_key)
                .unwrap_or(Vec::new(&env));
            donors.push_back(donor.clone());
            env.storage().instance().set(&donors_key, &donors);
        }
        
        Ok(())
    }

    pub fn claim_funds(
        env: Env, 
        project_id: String, 
        creator: Address
    ) -> Result<(), ContractError> {
        creator.require_auth();
        let project_key = DataKey::Project(project_id.clone());
        let mut project_result = Self::get_project_by_id(env.clone(), project_id.clone());
        let mut project = match project_result {
            Ok(pro) => pro,
            Err(err) => return Err(err),
        };

        if project.creator != creator {
            return Err(ContractError::NotAuthorized);
        }

        if project.status != ProjectStatus::GoalReached {
            return Err(ContractError::InvalidProjectStatus);
        }

        let current_time = env.ledger().timestamp();
        let goal_reached = project.raised >= project.goal;

        if !goal_reached {
            return Err(ContractError::ClaimConditionsNotMet);
        }

        let xlm_address_str = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC";
        let xlm_address_sdk_string = soroban_sdk::String::from_str(&env, xlm_address_str);
        let xlm_token = Address::from_str(&env, "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC");
        let token_client = TokenClient::new(&env, &xlm_token);

        let project_raised_i128 = u128_to_i128(project.raised);
        
        token_client.transfer(
            &env.current_contract_address(),
            &creator,
            &project_raised_i128
        );

        project.status = ProjectStatus::Completed;

        env.storage().instance().set(&project_id, &project);
        
        env.events().publish(
            (Symbol::new(&env, "claim"), project_id),
            project.raised
        );
        Ok(())
    }

    pub fn refund(env: Env, project_id: u32, backer: Address) {
        // Verifica que el proyecto haya fallado
        // Devuelve el aporte al backer
    }

    pub fn get_project_by_id(e: Env, project_id: String) -> Result<Project, ContractError> {
        let project_key = DataKey::Project(project_id.clone());
        if let Some(project) = e.storage().instance().get::<DataKey, Project>(&project_key) {
            projects_by_project_id(&e, project_id.clone(), project.clone());
            Ok(project)
        } else {
            return Err(ContractError::ProjectNotFound)
        }
    }
    
    
}

fn u128_to_i128(value: u128) -> i128 {
    // Check if the value exceeds the maximum i128 value
    if value > i128::MAX as u128 {
        return 42
    }
    
    // Safe to convert now
    value as i128
}

mod test;
mod storage_types;