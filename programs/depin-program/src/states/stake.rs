use super::*;

/// The struct containing instructions for staking
#[account]
#[derive(Default)]
pub struct StakeState {
    /// Initial staked amount
    pub staked_amount: u64,

    /// Timestamp when token is going to staked
    pub staked_at: i64,

    /// Rewards earned
    pub rewards: u64,

    /// Penality earned
    pub penality: u64,
}

impl StakeState {
    // Function to stake data
    pub fn stake(&mut self, staked_amount: u64) -> Result<()> {
        self.staked_amount += staked_amount;
        self.staked_at = Clock::get()
            .expect("Error while getting staked duration.")
            .unix_timestamp;

        Ok(())
    }

    // Function to calculate rewards and penality
    pub fn calc_rewards_and_penality(&mut self) {
        let now = Clock::get()
            .expect("Error while getting current timestamp.")
            .unix_timestamp;

        let num_of_days = ((now - self.staked_at) as f64 / SECONDS_PER_DAY as f64) as u64;

        match num_of_days {
            0..=1 => {
                self.penality = 30;
                self.rewards = 0;
            }
            2..=15 => {
                self.penality = 15;
                self.rewards = 0;
            }
            16..=29 => {
                self.penality = 1;
                self.rewards = 0;
            }
            _ => {
                self.penality = 0;
                self.rewards = 1;
            }
        }
    }

    // Function to calculate payable amount
    pub fn withdraw(&mut self, mut amount_after_penality: u64, total_stakers: u64) -> (u64, u64) {
        self.calc_rewards_and_penality();

        let withdrawal_amount = if self.penality > 0 {
            let penality = self.penality * self.staked_amount / 100;
            amount_after_penality += penality;
            self.staked_amount - penality
        } else {
            let penality_share = amount_after_penality / total_stakers;
            amount_after_penality -= penality_share;
            let rewards = self.rewards * self.staked_amount / 100;
            self.staked_amount + rewards + penality_share
        };

        (withdrawal_amount, amount_after_penality)
    }
}
