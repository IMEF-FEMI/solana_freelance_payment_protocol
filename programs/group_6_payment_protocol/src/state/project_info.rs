use crate::errors::ErrorCode;
use anchor_lang::prelude::*;

/// state
#[account]
pub struct ProjectInfo {
    pub total_project_funds: u64,
    pub milestones: u8,
    pub milestones_reached: u8,
    pub milestone_funds_withdrawn: u8,
    pub client: Pubkey,
    pub freelancer: Pubkey,
    pub multisig: Pubkey,
    pub status: u8,
}

impl ProjectInfo {
    pub const MAX_SIZE: usize = 8 //    total_project_funds
    + 1 //milestones
    + 1 //milestones_reached
    + 1 //milestone_funds_withdrawn
    + 32 //client
    + 32 //freelancer
    + 32 //multisig
    + 1; //status
         //see more at: https://book.anchor-lang.com/anchor_references/space.html
}
#[derive(PartialEq, Eq)]
pub enum ProjectStatus {
    //client can still cancel the project
    //and withdraw funds
    Pending,
    //ongoing stage
    Running,
    //Completed
    Completed,
    //collective cancellation
    Cancelled,
}

impl ProjectStatus {
    pub fn to_code(&self) -> u8 {
        match self {
            ProjectStatus::Pending => 0,
            ProjectStatus::Running => 1,
            ProjectStatus::Completed => 2,
            ProjectStatus::Cancelled => 3,
        }
    }

    pub fn from(val: u8) -> std::result::Result<ProjectStatus, ErrorCode> {
        match val {
            0 => Ok(ProjectStatus::Pending),
            1 => Ok(ProjectStatus::Running),
            2 => Ok(ProjectStatus::Completed),
            3 => Ok(ProjectStatus::Cancelled),
            _ => Err(ErrorCode::InvalidStatus.into()),
        }
    }
}
