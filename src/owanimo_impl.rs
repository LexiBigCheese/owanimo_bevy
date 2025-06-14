use std::collections::HashMap;

use bevy::prelude::*;
use owanimo::{
    Board,
    standard::{ColorBoard, NuisanceBoard},
};

use crate::puy_components::{Puyo, PuyoType, puyo_component::PuyoState};

pub struct CartBoart<'world, 'state, 'a> {
    pub board: Entity,
    pub puyos: Query<'world, 'state, (&'a mut Puyo, Entity)>,
}

impl<'world, 'state, 'a> CartBoart<'world, 'state, 'a> {
    fn get_at_coord(&self, pos: (u32, u32)) -> Option<(&Puyo, Entity)> {
        self.puyos
            .iter()
            .filter(|(puyo, _)| puyo.grid_pos == pos && &puyo.board == &self.board)
            .next()
    }
    fn get_at_icoord(&self, pos: (i32, i32)) -> Option<(&Puyo, Entity)> {
        if pos.0 < 0 || pos.1 < 0 {
            None
        } else {
            self.get_at_coord((pos.0 as u32, pos.1 as u32))
        }
    }
    fn get_puyo(&self, entity: Entity) -> Option<&Puyo> {
        self.puyos
            .iter()
            .filter(|(puyo, cmp)| cmp == &entity && &puyo.board == &self.board)
            .map(|(puyo, _)| puyo)
            .next()
    }
}

impl<'world, 'state, 'a> Board for CartBoart<'world, 'state, 'a> {
    type Handle = Option<Entity>;

    fn tiles(&self) -> impl Iterator<Item = Self::Handle> {
        self.puyos
            .iter()
            .filter(|(puyo, _)| puyo.board == self.board)
            .map(|(_, handle)| Some(handle))
    }

    fn neighbors(&self, handle: &Self::Handle) -> impl Iterator<Item = Self::Handle> {
        let Some(entity) = handle else {
            let arr: [Option<Entity>; 4] = [None; 4];
            return arr.into_iter();
        };
        let Some(puy) = self.get_puyo(entity.clone()) else {
            let arr: [Option<Entity>; 4] = [None; 4];
            return arr.into_iter();
        };
        let (x, y) = (puy.grid_pos.0 as i32, puy.grid_pos.1 as i32);
        let arr = [
            self.get_at_icoord((x - 1, y)).map(|(_, b)| b),
            self.get_at_icoord((x + 1, y)).map(|(_, b)| b),
            self.get_at_icoord((x, y - 1)).map(|(_, b)| b),
            self.get_at_icoord((x, y + 1)).map(|(_, b)| b),
        ];
        arr.into_iter()
    }

    fn connects(&self, a: &Self::Handle, b: &Self::Handle) -> bool {
        let Some(a) = a else { return false };
        let Some(b) = b else { return false };
        let (Some(a), Some(b)) = (self.get_puyo(a.clone()), self.get_puyo(b.clone())) else {
            return false;
        };

        match (a.ty, b.ty) {
            (PuyoType::Nuisance, _) | (_, PuyoType::Nuisance) => false,
            (x, y) => x == y,
        }
    }
}

impl<'world, 'state, 'a> NuisanceBoard for CartBoart<'world, 'state, 'a> {
    fn nuisance(&self, handle: &Self::Handle) -> bool {
        let Some(handle) = handle else { return false };
        let Some(puyo) = self.get_puyo(handle.clone()) else {
            return false;
        };
        puyo.ty == PuyoType::Nuisance
    }
}
impl<'world, 'state, 'a> ColorBoard for CartBoart<'world, 'state, 'a> {
    type Color = PuyoType;

    fn color(&self, handle: &Self::Handle) -> Option<Self::Color> {
        let puyo = self.get_puyo(handle.clone()?)?;
        if puyo.ty != PuyoType::Nuisance {
            Some(puyo.ty)
        } else {
            None
        }
    }
}

pub struct GravityCartBoart<'world, 'state, 'a, 'b> {
    pub board: Entity,
    pub puyos: Query<'world, 'state, (&'a mut Puyo, Entity)>,
    pub states: Query<'world, 'state, &'b mut PuyoState>,
}
impl<'world, 'state, 'a, 'b> GravityCartBoart<'world, 'state, 'a, 'b> {
    pub fn fall(&mut self) -> bool {
        let mut did_fall = false;
        let mut cols: HashMap<u32, Vec<(u32, Entity)>> = Default::default();
        for (puy, ent) in self
            .puyos
            .iter()
            .filter(|(puyo, _)| puyo.board == self.board)
        {
            let (x, y) = puy.grid_pos;
            cols.entry(x)
                .and_modify(|v| v.push((y, ent)))
                .or_insert_with(|| vec![(y, ent)]);
        }
        for mut col in cols.into_values() {
            col.sort_by_key(|(y, _)| *y);
            for (y, (_, ent)) in col.into_iter().enumerate() {
                let y = y as u32;
                let Ok((mut puy, _)) = self.puyos.get_mut(ent) else {
                    continue;
                };
                let Ok(mut state) = self.states.get_mut(ent) else {
                    continue;
                };
                if puy.grid_pos.1 != y {
                    did_fall = true;
                    state.start_falling();
                    puy.grid_pos.1 = y;
                }
            }
        }
        if did_fall {
            println!("We did fall!");
        }
        did_fall
    }
}
