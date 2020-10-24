use crate::traits::{Api, Querier, Storage};
use crate::QuerierWrapper;

/// Holds all external dependencies of the contract.
/// Designed to allow easy dependency injection at runtime.
/// This cannot be copied or cloned since it would behave differently
/// for mock storages and a bridge storage in the VM.
pub struct Deps<S: Storage, A: Api, Q: Querier> {
    pub storage: S,
    pub api: A,
    pub querier: Q,
}

impl<S: Storage, A: Api, Q: Querier> Deps<S, A, Q> {
    /// change_querier is a helper mainly for test code when swapping out the Querier
    /// from the auto-generated one from mock_dependencies. This changes the type of
    /// Deps so replaces requires some boilerplate.
    pub fn change_querier<T: Querier, F: Fn(Q) -> T>(self, transform: F) -> Deps<S, A, T> {
        Deps {
            storage: self.storage,
            api: self.api,
            querier: transform(self.querier),
        }
    }
}

pub struct DepsMut<'a> {
    pub storage: &'a mut dyn Storage,
    pub api: &'a dyn Api,
    pub querier: QuerierWrapper<'a>,
}

pub struct DepsRef<'a> {
    pub storage: &'a dyn Storage,
    pub api: &'a dyn Api,
    pub querier: QuerierWrapper<'a>,
}

impl<S: Storage, A: Api, Q: Querier> Deps<S, A, Q> {
    pub fn as_ref(&'_ self) -> DepsRef<'_> {
        DepsRef {
            storage: &self.storage,
            api: &self.api,
            querier: QuerierWrapper {
                querier: &self.querier,
            },
        }
    }

    pub fn as_mut(&'_ mut self) -> DepsMut<'_> {
        DepsMut {
            storage: &mut self.storage,
            api: &self.api,
            querier: QuerierWrapper {
                querier: &self.querier,
            },
        }
    }
}

impl<'a> DepsMut<'a> {
    pub fn as_ref(&'_ self) -> DepsRef<'_> {
        DepsRef {
            storage: self.storage,
            api: self.api,
            querier: self.querier,
        }
    }
}
