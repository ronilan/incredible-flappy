use crate::ui::theme;

#[derive(Clone, PartialEq, Debug, Default)]
pub enum Phase {
    #[default]
    Splash,
    Ready,
    Flying,
    Paused,
    Score,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct State {
    pub phase: Phase,
    pub kitty: bool,
    pub selected: SelectedGame,
    pub focus: Option<SelectedGame>,
    pub countdown: u8,
    pub countdown_at: f64,
    pub best_classic: [u32; 3],
    pub best_busy: [u32; 3],
    pub best_invaders: [u32; 3],
}

impl State {
    fn best_mut(&mut self, game: SelectedGame) -> &mut [u32; 3] {
        match game {
            SelectedGame::Classic => &mut self.best_classic,
            SelectedGame::Busy => &mut self.best_busy,
            SelectedGame::Invaders => &mut self.best_invaders,
        }
    }

    /// Best score for the given game.
    pub fn best_for(&self, game: SelectedGame) -> u32 {
        match game {
            SelectedGame::Classic => self.best_classic[0],
            SelectedGame::Busy => self.best_busy[0],
            SelectedGame::Invaders => self.best_invaders[0],
        }
    }

    /// Records a score into the top 3. Returns the medal rank
    /// (0 gold, 1 silver, 2 bronze) or None when it places outside.
    /// Scoreless runs earn nothing.
    pub fn record_score(&mut self, game: SelectedGame, score: u32) -> Option<usize> {
        if score == 0 {
            return None;
        }
        let top = self.best_mut(game);
        let rank = top.iter().filter(|s| **s > score).count();
        if rank >= top.len() {
            return None;
        }
        top[rank..].rotate_right(1);
        top[rank] = score;
        Some(rank)
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum SelectedGame {
    #[default]
    Classic,
    Busy,
    Invaders,
}

/// Palette per selected game.
pub(crate) fn palette_for(selected: &SelectedGame) -> theme::Palette {
    match selected {
        SelectedGame::Classic => theme::CLASSIC_PALETTE,
        SelectedGame::Busy => theme::BUSY_PALETTE,
        SelectedGame::Invaders => theme::INVADERS_PALETTE,
    }
}
