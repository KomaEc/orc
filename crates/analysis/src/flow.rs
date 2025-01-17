use std::ops::{Index, IndexMut};

use rustc_index::IndexSlice;
use rustc_middle::mir::{BasicBlock, Local, Location};
use utils::data_structure::vec_vec::VecVec;

pub mod def_use;
mod dom;
mod join_points;
pub mod ownership;
mod state;

utils::macros::newtype_index! {
    #[debug_format = "{}"]
    pub struct SSAIdx {
    }
}

impl Default for SSAIdx {
    fn default() -> Self {
        Self::INIT
    }
}

impl SSAIdx {
    pub const INIT: Self = SSAIdx::from_u32(0);
}

impl std::ops::AddAssign<usize> for SSAIdx {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RichLocation {
    Entry,
    Phi(BasicBlock),
    Mir(Location),
}

impl From<Location> for RichLocation {
    fn from(location: Location) -> Self {
        RichLocation::Mir(location)
    }
}

pub struct LocationMap<T> {
    map: VecVec<T>,
}

impl<T> Index<BasicBlock> for LocationMap<T> {
    type Output = [T];

    fn index(&self, block: BasicBlock) -> &Self::Output {
        &self.map[block.index()]
    }
}

impl<T> IndexMut<BasicBlock> for LocationMap<T> {
    fn index_mut(&mut self, block: BasicBlock) -> &mut Self::Output {
        &mut self.map[block.index()]
    }
}

impl<T> Index<Location> for LocationMap<T> {
    type Output = T;

    fn index(&self, location: Location) -> &Self::Output {
        let Location {
            block,
            statement_index,
        } = location;
        &self.map[block.index()][statement_index]
    }
}

impl<T> IndexMut<Location> for LocationMap<T> {
    fn index_mut(&mut self, location: Location) -> &mut Self::Output {
        let Location {
            block,
            statement_index,
        } = location;
        &mut self.map[block.index()][statement_index]
    }
}

impl<T> LocationMap<T> {
    pub fn iter_enumerated(&self) -> impl Iterator<Item = (Location, &T)> {
        self.map.iter().enumerate().flat_map(|(bb, bb_data)| {
            bb_data.iter().enumerate().map(move |(index, data)| {
                let location = Location {
                    block: bb.into(),
                    statement_index: index,
                };
                (location, data)
            })
        })
    }
}

pub struct LocalMap<T> {
    map: VecVec<T>,
}

impl<T> Index<Local> for LocalMap<T> {
    type Output = IndexSlice<SSAIdx, T>;

    fn index(&self, index: Local) -> &Self::Output {
        IndexSlice::from_raw(&self.map[index.index()])
    }
}

impl<T> IndexMut<Local> for LocalMap<T> {
    fn index_mut(&mut self, index: Local) -> &mut Self::Output {
        IndexSlice::from_raw_mut(&mut self.map[index.index()])
    }
}

impl<T> LocalMap<T> {
    pub fn iter_enumerated(&self) -> impl Iterator<Item = (Local, SSAIdx, &T)> {
        self.map.iter().enumerate().flat_map(|(local, bb_data)| {
            bb_data
                .iter()
                .enumerate()
                .map(move |(ssa_idx, data)| (local.into(), ssa_idx.into(), data))
        })
    }
}
