#![no_std]
#[warn(unused_imports)]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec, Address, contracttype, contracterror, Error};

#[contract]
pub struct CrowdfundingContract;


#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Project {
    pub creator: Address,
    pub goal: u128,
    pub raised: u128,
    pub deadline: u64,
    pub claimed: bool,
}

#[contracttype]
pub enum DataKey {
    Project(String),
    Contribution(String, Address),
    TokenContract,
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
}

#[contractimpl]
impl CrowdfundingContract {

    // pub fn init(env: Env, token: Address) {
    //     env.storage().set(DataKey::TokenContract, &token);
    // }

    pub fn create_project(
        env: Env,
        project_id: String,
        creator: Address,
        goal: u128,
        deadline: u64
    ) -> Result<String, ContractError> {


        creator.require_auth();

        if goal == 0 {
            return Err(ContractError::AmountCannotBeZero);
        }

        let key = DataKey::Project(project_id.clone());
        if env.storage().instance().has(&key) {
            return Err(ContractError::ProjectAlreadyExists);
        }

        let project = Project {
            creator: creator.clone(),
            goal,
            raised: 0,
            deadline,
            claimed: false,
        };

        env.storage().instance().set(&key, &project);
        Ok(project_id)
    }

    pub fn fund_project(env: Env, project_id: u32, from: Address, amount: i128) {
        // Transfiere tokens desde el usuario al contrato
        // Suma el aporte al total recaudado del proyecto
        // Guarda la contribución del usuario por si necesita reembolso
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

mod test;
