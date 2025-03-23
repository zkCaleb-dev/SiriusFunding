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

use crate::events::escrows_by_engagement_id;
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
        let mut project: Project = env.storage().instance().get(&project_key).ok_or(ContractError::ProjectNotFound)?;

        let current_time = env.ledger().timestamp();
        if current_time > project.deadline {
            return Err(ContractError::ProjectExpired);
        }

        let amount_i128 = u128_to_i128(amount);

        // Get the network's XLM address
        // Create a proper soroban_sdk::String from a string literal
        let xlm_address_str = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC";
        let xlm_address_sdk_string = soroban_sdk::String::from_str(&env, xlm_address_str);

        // Now use the properly created String to create the Address
        let xlm_token = Address::from_string(&xlm_address_sdk_string);
        // For futurenet use: CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT
        // For testnet use: CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
        // For public network use: CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA
        
        let token_client = TokenClient::new(&env, &xlm_token);

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

    pub fn claim_funds(env: Env, project_id: u32, creator: Address) {
        // Verifica que se alcanzó la meta y que quien llama es el creador
        // Transfiere los fondos acumulados al creador
    }

    pub fn refund(env: Env, project_id: u32, backer: Address) {
        // Verifica que el proyecto haya fallado
        // Devuelve el aporte al backer
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