use bevy::prelude::*;

use super::spawn_puyo::spawn_puyo;

use super::CartesianState;

use super::CartesianBoard6x12;

use crate::puy_ass::PuyoAssets;

pub fn spawn_cartes_board(
    cmd: &mut Commands,
    puy_ass: &Res<PuyoAssets>,
    at: Transform,
    contents: &str,
) {
    let board = cmd
        .spawn((
            CartesianBoard6x12 {
                state: CartesianState::Physics,
                ..Default::default()
            },
            at,
            Visibility::Visible,
        ))
        .id();
    let lines = contents
        .lines()
        .filter(|x| !x.trim().is_empty())
        .take(12)
        .collect::<Vec<_>>();
    let start_line = lines.len() - 1;
    for (y_bottom, line) in lines.into_iter().enumerate() {
        let y = start_line - y_bottom;
        for (x, chr) in line.trim().chars().take(6).enumerate() {
            use super::PuyoType::*;
            let kind = match chr {
                ' ' | '_' => continue,
                'O' | 'o' | '0' => Nuisance,
                'R' | 'r' => Red,
                'G' | 'g' => Green,
                'B' | 'b' => Blue,
                'Y' | 'y' => Yellow,
                'P' | 'p' => Purple,
                _ => continue,
            };
            spawn_puyo(cmd, puy_ass, board, kind, (x, y));
        }
    }
}
