use rand::Rng;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub solutio_balance: u64,
    pub mtoshi_balance: u64,
    pub activity_points: u64,
    pub level: u32,
    pub experience: u64,
    pub inventory: Vec<Item>,
    pub strength: u32,
}

impl Player {
    pub fn new(id: String) -> Self {
        Player {
            id,
            solutio_balance: 0,
            mtoshi_balance: 0,
            activity_points: 0,
            level: 1,
            experience: 0,
            inventory: vec![],
            strength: 10,
        }
    }

    pub fn gain_experience(&mut self, xp: u64) {
        self.experience += xp;
        while self.experience >= Self::xp_for_next_level(self.level) {
            self.experience -= Self::xp_for_next_level(self.level);
            self.level += 1;
            self.strength += 2;
            println!("{} leveled up to level {}! Strength: {}", self.id, self.level, self.strength);
        }
    }

    fn xp_for_next_level(level: u32) -> u64 {
        (level as u64) * 100
    }

    pub fn add_item(&mut self, item: Item) {
        println!("{} obtained a {} (value: {})", self.id, item.name, item.value);
        self.inventory.push(item);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct GameFiRuntime {
    pub players: HashMap<String, Player>,
    pool_balance: u64,
    mining_pool: HashMap<String, u64>,
}

impl GameFiRuntime {
    pub fn new() -> Self {
        GameFiRuntime {
            players: HashMap::new(),
            pool_balance: 1_000_000,
            mining_pool: HashMap::new(),
        }
    }

    pub fn mine_block(&mut self, previous_hash: &str, miner: String, index: u64) -> Option<catena_node::Block> {
        let nonce = rand::thread_rng().gen::<u64>();
        let computed_hash: u64 = rand::thread_rng().gen();

        if computed_hash % 1000 < 100 {
            let base_reward = 50;
            let mut transactions = vec![];

            *self.mining_pool.entry(miner.clone()).or_insert(0) += base_reward;
            let total_contribution: u64 = self.mining_pool.values().sum();

            for (player_id, contribution) in self.mining_pool.iter() {
                let player_reward = (contribution * base_reward) / total_contribution;
                if let Some(player) = self.players.get_mut(player_id) {
                    player.solutio_balance += player_reward;
                    player.gain_experience(player_reward * 2);
                    if rand::thread_rng().gen_bool(0.3) {
                        player.add_item(Item {
                            name: "Mining Gem".to_string(),
                            value: 10,
                        });
                    }
                    transactions.push(catena_node::Transaction {
                        sender: "system".to_string(),
                        receiver: player_id.clone(),
                        amount: player_reward.into(),
                        token: "Solutio".to_string(),
                    });
                }
            }

            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let hash = format!("{:x}", computed_hash);

            println!("Mining pool distribution: {:?}", transactions);
            Some(catena_node::Block {
                index,
                timestamp,
                transactions,
                previous_hash: previous_hash.to_string(),
                hash,
                nonce,
            })
        } else {
            None
        }
    }

    pub fn record_activity(&mut self, player_id: &str, activity_type: u32) {
        if let Some(player) = self.players.get_mut(player_id) {
            let (points, xp, item_chance) = match activity_type {
                1 => (10, 20, 0.2),
                2 => (15, 30, 0.3),
                3 => (5, 10, 0.1),
                4 => (20, 40, 0.4),
                _ => (0, 0, 0.0),
            };
            player.activity_points += points;
            player.gain_experience(xp);
            if rand::thread_rng().gen_bool(item_chance) {
                player.add_item(Item {
                    name: format!("Activity {} Reward", activity_type),
                    value: xp / 2,
                });
            }
            println!("{} performed activity {}: +{} points, +{} XP", player_id, activity_type, points, xp);
        }
    }

    pub fn battle(&mut self, player1_id: &str, player2_id: &str) {
        let mut player1 = self.players.remove(player1_id).unwrap_or_else(|| {
            println!("Player {} not found!", player1_id);
            Player::new(player1_id.to_string())
        });
        let mut player2 = self.players.remove(player2_id).unwrap_or_else(|| {
            self.players.insert(player1_id.to_string(), player1.clone());
            println!("Player {} not found!", player2_id);
            Player::new(player2_id.to_string())
        });

        let player1_roll = player1.strength + rand::thread_rng().gen_range(0..10);
        let player2_roll = player2.strength + rand::thread_rng().gen_range(0..10);
        println!("Battle: {} (strength: {}) vs {} (strength: {})", player1_id, player1_roll, player2_id, player2_roll);
        if player1_roll >= player2_roll {
            player1.gain_experience(30);
            player1.activity_points += 15;
            println!("{} wins the battle!", player1_id);
            if rand::thread_rng().gen_bool(0.5) {
                player1.add_item(Item {
                    name: "Battle Trophy".to_string(),
                    value: 20,
                });
            }
        } else {
            player2.gain_experience(30);
            player2.activity_points += 15;
            println!("{} wins the battle!", player2_id);
            if rand::thread_rng().gen_bool(0.5) {
                player2.add_item(Item {
                    name: "Battle Trophy".to_string(),
                    value: 20,
                });
            }
        }

        self.players.insert(player1_id.to_string(), player1);
        self.players.insert(player2_id.to_string(), player2);
    }

    pub fn trade_items(&mut self, player1_id: &str, player1_item_index: usize, player2_id: &str, player2_item_index: usize) -> bool {
        let mut player1 = self.players.remove(player1_id).unwrap_or_else(|| {
            println!("Player {} not found!", player1_id);
            Player::new(player1_id.to_string())
        });
        let mut player2 = self.players.remove(player2_id).unwrap_or_else(|| {
            self.players.insert(player1_id.to_string(), player1.clone());
            println!("Player {} not found!", player2_id);
            Player::new(player2_id.to_string())
        });

        if player1_item_index >= player1.inventory.len() || player2_item_index >= player2.inventory.len() {
            println!("Invalid item indices!");
            self.players.insert(player1_id.to_string(), player1);
            self.players.insert(player2_id.to_string(), player2);
            return false;
        }

        let item1 = player1.inventory.remove(player1_item_index);
        let item2 = player2.inventory.remove(player2_item_index);
        player1.inventory.insert(player1_item_index, item2.clone());
        player2.inventory.insert(player2_item_index, item1.clone());
        println!("{} traded {} (value: {}) with {} for {} (value: {})",
            player1_id, item1.name, item1.value, player2_id, item2.name, item2.value);

        self.players.insert(player1_id.to_string(), player1);
        self.players.insert(player2_id.to_string(), player2);
        self.record_activity(player1_id, 3);
        self.record_activity(player2_id, 3);

        true
    }

    pub fn distribute_mtoshi_rewards(&mut self) {
        let total_points: u64 = self.players.values().map(|p| p.activity_points).sum();
        if total_points == 0 {
            return;
        }

        let total_reward = 500;
        if total_reward > self.pool_balance {
            return;
        }

        for player in self.players.values_mut() {
            let player_reward = (player.activity_points * total_reward) / total_points;
            player.mtoshi_balance += player_reward;
            player.activity_points = 0;
            println!("{} received {} MTOSHI", player.id, player_reward);
        }
        self.pool_balance -= total_reward;
    }

    pub fn show_leaderboard(&self) {
        let mut leaderboard: Vec<(&String, &Player)> = self.players.iter().collect();
        leaderboard.sort_by(|a, b| b.1.level.cmp(&a.1.level).then(b.1.experience.cmp(&a.1.experience)));
        println!("\nLeaderboard:");
        for (rank, (id, player)) in leaderboard.iter().enumerate() {
            println!("{}. {} - Level: {}, XP: {}, Solutio: {}, MTOSHI: {}", 
                rank + 1, id, player.level, player.experience, player.solutio_balance, player.mtoshi_balance);
        }
    }

    pub fn get_mtoshi_balance(&self, player_id: &str) -> Option<u64> {
        self.players.get(player_id).map(|player| player.mtoshi_balance)
    }

    pub fn simulate_player_activity(&mut self) {
        let player_ids: Vec<String> = self.players.keys().cloned().collect();
        for player_id in player_ids {
            let activity = rand::thread_rng().gen_range(0..4);
            match activity {
                0 => self.record_activity(&player_id, 1),
                1 => self.record_activity(&player_id, 2),
                2 => self.record_activity(&player_id, 3),
                3 => self.record_activity(&player_id, 4),
                _ => {}
            }
        }
    }
}

pub mod catena_node {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Transaction {
        pub sender: String,
        pub receiver: String,
        pub amount: u128,
        pub token: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Block {
        pub index: u64,
        pub timestamp: u64,
        pub transactions: Vec<Transaction>,
        pub previous_hash: String,
        pub hash: String,
        pub nonce: u64,
    }
}
