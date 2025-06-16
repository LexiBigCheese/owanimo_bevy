use owanimo::{
    Board,
    standard::{ColorBoard, GroupFromColorBoard, NuisanceBoard},
};

use super::{SBoard, SPuyo};

use crate::puyo_chara::PuyoType;

impl SBoard {
    pub fn get_at(&self, (col_n, row_n): (usize, usize)) -> Option<&SPuyo> {
        self.columns.get(col_n)?.get(row_n)
    }
    pub fn get_mut_at(&mut self, (col_n, row_n): (usize, usize)) -> Option<&mut SPuyo> {
        self.columns.get_mut(col_n)?.get_mut(row_n)
    }
}

impl Board for SBoard {
    type Handle = (usize, usize);
    fn tiles(&self) -> impl Iterator<Item = Self::Handle> {
        let lens = self.columns.iter().map(|col| col.len()).collect::<Vec<_>>();
        lens.into_iter()
            .enumerate()
            .flat_map(|(col_n, col_len)| (0..col_len).map(move |row_n| (col_n, row_n)))
    }
    fn connects(&self, a: &Self::Handle, b: &Self::Handle) -> bool {
        use PuyoType::*;
        let (Some(a), Some(b)) = (self.get_at(*a), self.get_at(*b)) else {
            return false;
        };
        let (a, b) = (a.kind, b.kind);
        match (a, b) {
            (Nuisance | NuisanceBL | NuisanceBR | NuisanceTL | NuisanceTR, _)
            | (_, Nuisance | NuisanceBL | NuisanceBR | NuisanceTL | NuisanceTR) => false,
            (a, b) => a == b,
        }
    }
    fn neighbors(&self, handle: &Self::Handle) -> impl Iterator<Item = Self::Handle> {
        let have = |x: isize, y: isize| {
            if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
                self.get_at((x, y)).map(move |_| (x, y))
            } else {
                None
            }
        };
        let (x, y) = *handle;
        let (x, y) = (x as isize, y as isize);
        let arr = [
            have(x - 1, y),
            have(x + 1, y),
            have(x, y - 1),
            have(x, y + 1),
        ];
        arr.into_iter().flatten()
    }
}

impl NuisanceBoard for SBoard {
    fn nuisance(&self, handle: &Self::Handle) -> bool {
        matches!(
            self.get_at(*handle),
            Some(SPuyo {
                kind: PuyoType::Nuisance
                    | PuyoType::NuisanceBL
                    | PuyoType::NuisanceBR
                    | PuyoType::NuisanceTL
                    | PuyoType::NuisanceTR,
                ..
            })
        )
    }
}

impl ColorBoard for SBoard {
    type Color = PuyoType;
    fn color(&self, handle: &Self::Handle) -> Option<Self::Color> {
        self.get_at(*handle).and_then(|puy| match puy.kind {
            PuyoType::Nuisance
            | PuyoType::NuisanceBL
            | PuyoType::NuisanceTL
            | PuyoType::NuisanceBR
            | PuyoType::NuisanceTR => None,
            x => Some(x),
        })
    }
}

impl GroupFromColorBoard for SBoard {}
