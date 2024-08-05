//! GitButler internal library containing functionality related to branches, i.e. the virtual branches implementation
mod actions;
pub use actions::VirtualBranchActions;

mod r#virtual;
pub use r#virtual::*;

mod branch_manager;
pub use branch_manager::{BranchManager, BranchManagerExt};

mod base;
pub use base::BaseBranch;

mod integration;
pub use integration::{update_gitbutler_integration, verify_branch};

mod file;
pub use file::{Get, RemoteBranchFile};

mod remote;
pub use remote::{list_remote_branches, RemoteBranch, RemoteBranchData, RemoteCommit};

pub mod conflicts;

mod author;
mod status;
use gitbutler_branch::VirtualBranchesHandle;
pub use status::get_applied_status;
trait VirtualBranchesExt {
    fn virtual_branches(&self) -> VirtualBranchesHandle;
}

impl VirtualBranchesExt for gitbutler_project::Project {
    fn virtual_branches(&self) -> VirtualBranchesHandle {
        VirtualBranchesHandle::new(self.gb_dir())
    }
}

mod branch;
mod commit;
mod hunk;

pub use branch::{
    get_branch_listing_details, list_branches, Author, BranchIdentity, BranchListing,
    BranchListingDetails, BranchListingFilter,
};
