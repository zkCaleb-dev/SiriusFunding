use soroban_sdk::{contracttype, Address, String};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Project {
    pub creator: Address,
    pub goal: u128,
    pub raised: u128,
    pub deadline: u64,
    pub claimed: bool,
    pub description: String,
    pub status: ProjectStatus    // Added this field
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    Project(String),              // project_id -> Project
    Donation(String, Address),    // (project_id, donor) -> amount
    ProjectDonors(String),
    AllProjects                   // List of all project IDs
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum ProjectStatus {
    Active,
    Completed,
    GoalReached,
    Failed
}