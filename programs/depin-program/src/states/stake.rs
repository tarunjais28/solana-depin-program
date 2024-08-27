use super::*;

/// The struct containing instructions for staking
#[account]
#[derive(Default)] // Automatically provides a default value for the struct's fields
pub struct StakeState {
    /// Initial staked amount by the user
    pub staked_amount: u64,

    /// Timestamp of when the tokens were staked
    pub staked_at: i64,

    /// Rewards earned based on staking duration
    pub rewards: u64,

    /// Penalty applied for early withdrawal
    pub penality: u64,
}

impl StakeState {
    // Function to update the stake with a new amount
    pub fn stake(&mut self, staked_amount: u64) -> Result<()> {
        // Add the new staked amount to the existing amount
        self.staked_amount += staked_amount;
        // Record the current timestamp when the tokens are staked
        self.staked_at = Clock::get()
            .expect("Error while getting staked duration.")
            .unix_timestamp;

        Ok(())
    }

    // Function to calculate rewards and penalties based on staking duration
    fn calc_rewards_and_penality(&mut self) {
        // Get the current timestamp
        let now = Clock::get()
            .expect("Error while getting current timestamp.")
            .unix_timestamp;

        // Calculate the number of days the tokens have been staked
        let num_of_days = ((now - self.staked_at) as f64 / SECONDS_PER_DAY as f64) as u64;

        // Determine penalty and rewards based on the number of days staked
        match num_of_days {
            0..=1 => {
                // High penalty for very short staking period
                self.penality = 30;
                self.rewards = 0;
            }
            2..=15 => {
                // Medium penalty for moderate staking period
                self.penality = 15;
                self.rewards = 0;
            }
            16..=29 => {
                // Minimal penalty for longer staking period
                self.penality = 1;
                self.rewards = 0;
            }
            _ => {
                // No penalty and rewards are earned for staking over 29 days
                self.penality = 0;
                self.rewards = 1;
            }
        }
    }

    // Function to calculate the amount to be withdrawn by the user
    pub fn withdraw(&mut self, global_state: &mut GlobalState) -> u64 {
        // Calculate rewards and penalties based on the staking duration
        self.calc_rewards_and_penality();

        // Decrease the total number of stakers in the global state
        global_state.total_stakers -= 1;

        // If there is a penalty, calculate the penalty amount
        if self.penality > 0 {
            let penality = self.penality * self.staked_amount / 100;
            global_state.amount_after_penality += penality; // Add the penalty to the global state
            self.staked_amount - penality // Return the staked amount minus the penalty
        } else {
            // Distribute the penalty share among remaining stakers
            let penality_share = global_state.amount_after_penality / global_state.total_stakers;
            global_state.amount_after_penality -= penality_share; // Deduct the share from global state
            let rewards = self.rewards * self.staked_amount / 100; // Calculate the rewards
            self.staked_amount + rewards + penality_share // Return the staked amount plus rewards and penalty share
        }
    }
}
