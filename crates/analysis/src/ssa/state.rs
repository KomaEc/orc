//! State for analysis steps

use common::data_structure::assoc::AssocExt;
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::mir::{Body, Local, Location};

use super::consume::Voidable;
use crate::ssa::{
    consume::{Consume, ConsumeChain, Definitions},
    dom::DominanceFrontier,
    join_points::{JoinPoints, PhiNode},
};

common::macros::newtype_index! {
    pub struct SSAIdx {
        DEBUG_FORMAT = "{}"
    }
}

impl Default for SSAIdx {
    fn default() -> Self {
        Self::INIT
    }
}

impl SSAIdx {
    pub const INIT: Self = SSAIdx::from_u32(0);
    // pub const INVALID: Self = SSAIdx::MAX;

    // #[inline]
    // pub fn is_invalid(&self) -> bool {
    //     *self == Self::INVALID
    // }
}

impl Voidable for SSAIdx {
    const VOID: Self = SSAIdx::MAX;

    #[inline]
    fn is_void(&self) -> bool {
        *self == Self::VOID
    }
}

impl std::ops::AddAssign<usize> for SSAIdx {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

pub struct SSAState {
    pub name_state: NameState,
    pub join_points: JoinPoints<PhiNode>,
    pub consume_chain: ConsumeChain,
}

impl SSAState {
    /// TODO: smarter initialisation.
    /// Do not generate entries for non-ptr locals
    /// Do not generate entries for locals at all.
    pub fn new<'tcx>(
        body: &Body<'tcx>,
        dominance_frontier: &DominanceFrontier,
        definitions: Definitions,
        // crate_ctxt: &CrateCtxt<'tcx>,
    ) -> Self {
        let name_state = NameState::new(body, &definitions.maybe_owning);
        let join_points =
            JoinPoints::new(body, dominance_frontier, &definitions.maybe_consume_sites);
        let consume_chain = ConsumeChain::new(body, definitions);
        SSAState {
            name_state,
            join_points,
            consume_chain,
        }
    }
}

impl SSAState {
    /// Try find valid consume at `location`
    #[inline]
    pub fn try_consume_at(&mut self, local: Local, location: Location) -> Option<Consume<SSAIdx>> {
        // tracing::debug!("consume chain before: {:?}", &self.consume_chain.consumes[location.block.index()]);
        let consume = self.consume_chain.consumes[location.block.index()][location.statement_index]
            .get_by_key_mut(&local)?;
        let old_ssa_idx = self.name_state.get_name(local);
        consume.r#use = old_ssa_idx;
        if consume.is_use() {
            return None;
        }
        let new_ssa_idx = self.name_state.generate_fresh_name(local);
        tracing::debug!(
            "consuming {:?} at {:?}, use: {:?}, def: {:?}",
            local,
            location,
            old_ssa_idx,
            new_ssa_idx
        );
        consume.def = new_ssa_idx;
        let consume = *consume;
        // tracing::debug!("consume chain before: {:?}", &self.consume_chain.consumes[location.block.index()]);
        assert_eq!(
            new_ssa_idx,
            self.consume_chain.locs[local].push(location.into())
        );
        Some(consume)
    }

    // #[inline]
    // pub fn consume_at(&mut self, local: Local, location: Location) -> Consume<SSAIdx> {
    //     self.try_consume_at(local, location)
    //         .unwrap_or_else(|| panic!("{:?} isn't defined at {:?}", local, location))
    // }

    // #[inline]
    // pub fn try_finalise(&mut self, local: Local) -> Option<SSAIdx> {
    //     self.name_state.try_get_name(local)
    // }
}

#[derive(Clone, Debug)]
pub struct NameState {
    count: IndexVec<Local, SSAIdx>,
    stack: IndexVec<Local, Vec<SSAIdx>>,
}

impl NameState {
    fn new<'tcx>(body: &Body<'tcx>, maybe_owned: &BitSet<Local>) -> Self {
        let count = IndexVec::from_elem(SSAIdx::INIT, &body.local_decls);

        // Notice: this has to be in accordance with ConsumeChain.locs
        let stack = body
            .local_decls
            .indices()
            .map(|local| {
                maybe_owned
                    .contains(local)
                    .then(|| vec![SSAIdx::INIT])
                    .unwrap_or_default()
                // .unwrap_or_else(Vec::new)
            })
            .collect();
        // let stack = IndexVec::from_elem(vec![SSAIdx::INIT], &body.local_decls);
        NameState { count, stack }
    }

    pub fn reset(&mut self) {
        self.count.raw.fill(SSAIdx::INIT);
        for stack in &mut self.stack {
            if !stack.is_empty() {
                stack.truncate(1);
            }
        }
    }

    #[inline]
    pub fn generate_fresh_name(&mut self, var: Local) -> SSAIdx {
        self.count[var] += 1;
        let idx = self.count[var];
        self.stack[var].push(idx);
        idx
    }

    #[inline]
    pub fn get_name(&self, var: Local) -> SSAIdx {
        self.try_get_name(var).unwrap_or_else(|| {
            panic!(
                "internal error: cannot find fresh name supply for {:?}",
                var
            )
        })
    }

    /// Get the newest version for a variable. If `None` is returned,
    /// this variable is uninitialised.
    #[inline]
    pub fn try_get_name(&self, var: Local) -> Option<SSAIdx> {
        self.stack[var].last().copied()
    }

    #[inline]
    pub fn pop(&mut self, var: Local) -> SSAIdx {
        // tracing::debug!("popping {:?}~{:?}", var, ssa_idx);
        self.stack[var]
            .pop()
            .unwrap_or_else(|| panic!("internal error: poping non existing version for {:?}", var))
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use smallvec::smallvec;

//     #[test]
//     fn test1() {
//         let mut state: NameState<u32> = NameState {
//             count: IndexVec::from_raw(vec![12u32.into()]),
//             stack: IndexVec::from_raw(vec![vec![4u32.into(), 5u32.into()]]),
//             n_defs: vec![
//                 smallvec![(0u32.into(), 1.try_into().unwrap())],
//                 smallvec![(0u32.into(), 1.try_into().unwrap())],
//             ],
//         };

//         assert_eq!(state.get_name(0), 5u32.into());
//         assert_eq!(state.generate_fresh_name(0), 13u32.into());
//         assert_eq!(state.get_name(0), 13u32.into());

//         assert_eq!(&state.n_defs[0][..], [(0u32.into(), 1.try_into().unwrap())]);
//         assert_eq!(&state.n_defs[1][..], [(0u32.into(), 2.try_into().unwrap())]);

//         state.remove_names(1);

//         assert_eq!(state.get_name(0), 4u32.into());
//         assert_eq!(state.n_defs.len(), 1);
//     }

//     #[test]
//     fn test2() {
//         let mut state: NameState<u32> = NameState {
//             count: IndexVec::from_raw(vec![0u32.into()]),
//             stack: IndexVec::from_raw(vec![vec![0u32.into()]]),
//             n_defs: vec![],
//         };

//         state.enter_new_block();
//         assert_eq!(state.get_name(0), SSAIdx::INIT);
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 1u32.into());
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 2u32.into());

//         state.enter_new_block();
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 3u32.into());

//         state.remove_names(1);
//         assert_eq!(state.get_name(0), 2u32.into());

//         state.enter_new_block();
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 4u32.into());
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 5u32.into());

//         state.remove_names(1);
//         assert_eq!(state.get_name(0), 2u32.into());
//     }
// }