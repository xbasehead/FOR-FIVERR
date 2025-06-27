use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::runtime::{GameFiRuntime as FullGameFiRuntime, catena_node::Block};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    RequestTokens { player: String, token: String },
    GetBalance { player: String, token: String },
    GetPlayer { player: String },
    GetContentStats { player: String },
    SimulateActivity,
    MineBlock { miner: String, index: u64 },
    RecordActivity { player: String, activity_type: u32 },
    Battle { player1: String, player2: String },
    TradeItems { player1: String, player1_item_index: u32, player2: String, player2_item_index: u32 },
    DistributeMtoshiRewards,
    ShowLeaderboard,
    GetMtoshiBalance { player: String },
    MintNft { player: String, name: String, value: u64 },
    BuyNft { buyer: String, seller: String, nft_index: u32 },
    SellNft { seller: String, nft_index: u32, price: u64 },
    UploadContent { player: String, content_id: String, title: String },
    LikeContent { player: String, content_id: String },
    CommentContent { player: String, content_id: String, comment: String },
    ViewContent { player: String, content_id: String },
    PostContent { player: String, post_id: String, caption: String },
    LikePost { player: String, post_id: String, paid: bool },
    CommentPost { player: String, post_id: String, comment: String },
    BoostPost { player: String, post_id: String, tokens: u64 },
    GetPosts,
    GetVideos,
    GetNftListings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionResult {
    TokensRequested { player: String, token: String, amount: u64 },
    Balance { player: String, token: String, balance: u64 },
    Player { player: String, data: HashMap<String, String> },
    ContentStats { player: String, stats: HashMap<String, u64> },
    Error { message: String },
    ActivitySimulated,
    BlockMined { miner: String, block: Option<Block> },
    ActivityRecorded { player: String, activity_type: u32, points: u64 },
    BattleResult { winner: String, loser: String },
    ItemsTraded { player1: String, player2: String, success: bool },
    MtoshiDistributed { total_reward: u64 },
    Leaderboard { players: Vec<(String, u32, u64, u64, u64)> },
    MtoshiBalance { player: String, balance: u64 },
    NftMinted { player: String, name: String, value: u64 },
    NftBought { buyer: String, seller: String, nft_index: u32 },
    NftListed { seller: String, nft_index: u32, price: u64 },
    ContentUploaded { player: String, content_id: String },
    ContentLiked { player: String, content_id: String },
    ContentCommented { player: String, content_id: String },
    ContentViewed { player: String, content_id: String },
    ContentPosted { player: String, post_id: String },
    PostLiked { player: String, post_id: String, paid: bool },
    PostCommented { player: String, post_id: String },
    PostBoosted { player: String, post_id: String, tokens: u64 },
    Posts { posts: Vec<(String, String, String, u64, u64)> }, // (id, creator, caption, likes, boosts)
    Videos { videos: Vec<(String, String, String, u64)> }, // (id, creator, title, views)
    NftListings { listings: Vec<(String, u32, String, u64)> }, // (seller, index, name, price)
}

#[derive(Debug, Default)]
pub struct GamefiRuntime {
    players: HashMap<String, HashMap<String, u64>>,
    content_stats: HashMap<String, HashMap<String, u64>>,
}

impl GamefiRuntime {
    pub fn simulate_player_activity(&mut self) {
        for (player, tokens) in self.players.iter_mut() {
            for (token, balance) in tokens.iter_mut() {
                *balance += 10;
                self.content_stats
                    .entry(player.clone())
                    .or_insert_with(HashMap::new)
                    .entry(token.clone())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    pub fn show_leaderboard(&self) {
        for (player, tokens) in &self.players {
            let total: u64 = tokens.values().sum();
            println!("Player: {}, Total Tokens: {}", player, total);
        }
    }
}

#[derive(Debug)]
pub struct Blockchain {
    gamefi_runtime: GamefiRuntime,
    full_gamefi_runtime: FullGameFiRuntime,
    nft_marketplace: HashMap<String, Vec<(u32, u64)>>,
    content: HashMap<String, (String, String, u64, Vec<String>, Vec<String>)>, // (player, title, views, likes, comments)
    posts: HashMap<String, (String, String, Vec<String>, Vec<String>, u64, u64)>, // (player, caption, likes, comments, boosts, reputation_points)
    reputations: HashMap<String, u64>,
    daily_rewards: HashMap<String, u64>,
    total_users: u64,
    platform_pool: u64,
    community_pool: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            gamefi_runtime: GamefiRuntime::default(),
            full_gamefi_runtime: FullGameFiRuntime::new(),
            nft_marketplace: HashMap::new(),
            content: HashMap::new(),
            posts: HashMap::new(),
            reputations: HashMap::new(),
            daily_rewards: HashMap::new(),
            total_users: 1000, // Simulated user base
            platform_pool: 0,
            community_pool: 0,
        }
    }

    fn get_mining_reward(&self, player: &str) -> u64 {
        let base_rate = match self.total_users {
            0..=1_000_000 => 0.1,
            1_000_001..=10_000_000 => 0.05,
            10_000_001..=50_000_000 => 0.01,
            _ => 0.001,
        };
        let reputation = self.reputations.get(player).unwrap_or(&0);
        let multiplier = match reputation {
            0..=100 => 1.0,
            101..=200 => 1.2,
            201..=300 => 1.5,
            _ => 2.0,
        };
        (base_rate * multiplier * 1_000_000_000.0) as u64 // Convert to integer (scaled)
    }

    fn check_daily_cap(&mut self, player: &str, reward: u64) -> u64 {
        let current = self.daily_rewards.get(player).unwrap_or(&0);
        let max_daily: u64 = 500 * 1_000_000_000; // 500 DTOSHI (scaled)
        let remaining = max_daily.saturating_sub(*current);
        let allowed = reward.min(remaining);
        self.daily_rewards
            .entry(player.to_string())
            .and_modify(|e| *e += allowed)
            .or_insert(allowed);
        allowed
    }

    fn burn_tokens(&mut self, amount: u64) -> u64 {
        let burn = (amount as f64 * 0.01) as u64;
        burn
    }

    pub fn process_action(&mut self, action: Action) -> ActionResult {
        match action {
            Action::RequestTokens { player, token } => {
                let amount = 1000;
                self.gamefi_runtime
                    .players
                    .entry(player.clone())
                    .or_insert_with(HashMap::new)
                    .entry(token.clone())
                    .and_modify(|e| *e += amount)
                    .or_insert(amount);
                if !self.full_gamefi_runtime.players.contains_key(&player) {
                    self.full_gamefi_runtime.players.insert(player.clone(), crate::runtime::Player::new(player.clone()));
                }
                ActionResult::TokensRequested { player, token, amount }
            }
            Action::GetBalance { player, token } => {
                let balance = self.gamefi_runtime
                    .players
                    .get(&player)
                    .and_then(|tokens| tokens.get(&token))
                    .copied()
                    .unwrap_or(0);
                ActionResult::Balance { player, token, balance }
            }
            Action::GetPlayer { player } => {
                let data = self.gamefi_runtime
                    .players
                    .get(&player)
                    .map(|tokens| {
                        tokens
                            .iter()
                            .map(|(k, v)| (k.clone(), v.to_string()))
                            .collect::<HashMap<_, _>>()
                    })
                    .unwrap_or_default();
                ActionResult::Player { player, data }
            }
            Action::GetContentStats { player } => {
                let stats = self.gamefi_runtime
                    .content_stats
                    .get(&player)
                    .cloned()
                    .unwrap_or_default();
                ActionResult::ContentStats { player, stats }
            }
            Action::SimulateActivity => {
                self.full_gamefi_runtime.simulate_player_activity();
                ActionResult::ActivitySimulated
            }
            Action::MineBlock { miner, index } => {
                let block = self.full_gamefi_runtime.mine_block("previous_hash", miner.clone(), index);
                ActionResult::BlockMined { miner, block }
            }
            Action::RecordActivity { player, activity_type } => {
                self.full_gamefi_runtime.record_activity(&player, activity_type);
                let points = match activity_type {
                    1 => 10,
                    2 => 15,
                    3 => 5,
                    4 => 20,
                    _ => 0,
                };
                ActionResult::ActivityRecorded { player, activity_type, points }
            }
            Action::Battle { player1, player2 } => {
                self.full_gamefi_runtime.battle(&player1, &player2);
                ActionResult::BattleResult { winner: player1, loser: player2 }
            }
            Action::TradeItems { player1, player1_item_index, player2, player2_item_index } => {
                let success = self.full_gamefi_runtime.trade_items(&player1, player1_item_index as usize, &player2, player2_item_index as usize);
                ActionResult::ItemsTraded { player1, player2, success }
            }
            Action::DistributeMtoshiRewards => {
                self.full_gamefi_runtime.distribute_mtoshi_rewards();
                let total_reward = 500;
                ActionResult::MtoshiDistributed { total_reward }
            }
            Action::ShowLeaderboard => {
                self.full_gamefi_runtime.show_leaderboard();
                let players = self.full_gamefi_runtime.players.iter()
                    .map(|(id, p)| (id.clone(), p.level, p.experience, p.solutio_balance, p.mtoshi_balance))
                    .collect();
                ActionResult::Leaderboard { players }
            }
            Action::GetMtoshiBalance { player } => {
                let balance = self.full_gamefi_runtime.get_mtoshi_balance(&player).unwrap_or(0);
                ActionResult::MtoshiBalance { player, balance }
            }
            Action::MintNft { player, name, value } => {
                let cost = 10 * 1_000_000_000; // 10 DTOSHI
                if self.full_gamefi_runtime.players.get_mut(&player).map(|p| p.mtoshi_balance >= cost).unwrap_or(false) {
                    let player_ref = self.full_gamefi_runtime.players.get_mut(&player).unwrap();
                    player_ref.mtoshi_balance -= cost;
                    player_ref.add_item(crate::runtime::Item { name: name.clone(), value });
                    self.burn_tokens(cost);
                    println!("{} minted NFT {} (value: {}) for {} DTOSHI", player, name, value, cost / 1_000_000_000);
                    ActionResult::NftMinted { player, name, value }
                } else {
                    ActionResult::Error { message: "Insufficient DTOSHI".to_string() }
                }
            }
            Action::BuyNft { buyer, seller, nft_index } => {
                let price = self.nft_marketplace.get(&seller).and_then(|listings| listings.iter().find(|(i, _)| *i == nft_index)).map(|(_, p)| *p);
                if let Some(price) = price {
                    if self.full_gamefi_runtime.players.get(&buyer).map(|p| p.mtoshi_balance >= price).unwrap_or(false) {
                        if self.full_gamefi_runtime.players.contains_key(&seller) && (nft_index as usize) < self.full_gamefi_runtime.players[&seller].inventory.len() {
                            let nft = self.full_gamefi_runtime.players.get_mut(&seller).unwrap().inventory.remove(nft_index as usize);
                            self.full_gamefi_runtime.players.get_mut(&buyer).unwrap().inventory.push(nft.clone());
                            self.full_gamefi_runtime.players.get_mut(&buyer).unwrap().mtoshi_balance -= price;
                            self.full_gamefi_runtime.players.get_mut(&seller).unwrap().mtoshi_balance += price;
                            self.nft_marketplace.get_mut(&seller).unwrap().retain(|(i, _)| *i != nft_index);
                            self.burn_tokens(price);
                            println!("{} bought NFT {} from {} for {} DTOSHI", buyer, nft.name, seller, price / 1_000_000_000);
                            ActionResult::NftBought { buyer, seller, nft_index }
                        } else {
                            ActionResult::Error { message: "Seller or NFT not found".to_string() }
                        }
                    } else {
                        ActionResult::Error { message: "Insufficient DTOSHI".to_string() }
                    }
                } else {
                    ActionResult::Error { message: "NFT not listed".to_string() }
                }
            }
            Action::SellNft { seller, nft_index, price } => {
                if let Some(player) = self.full_gamefi_runtime.players.get(&seller) {
                    if (nft_index as usize) < player.inventory.len() {
                        self.nft_marketplace
                            .entry(seller.clone())
                            .or_insert_with(Vec::new)
                            .push((nft_index, price));
                        println!("NFT {} listed for sale by {} at {} DTOSHI", nft_index, seller, price / 1_000_000_000);
                        ActionResult::NftListed { seller, nft_index, price }
                    } else {
                        ActionResult::Error { message: "Invalid NFT index".to_string() }
                    }
                } else {
                    ActionResult::Error { message: "Seller not found".to_string() }
                }
            }
            Action::UploadContent { player, content_id, title } => {
                let cost = 5 * 1_000_000_000; // 5 DTOSHI
                if self.full_gamefi_runtime.players.get_mut(&player).map(|p| p.mtoshi_balance >= cost).unwrap_or(false) {
                    self.full_gamefi_runtime.players.get_mut(&player).unwrap().mtoshi_balance -= cost;
                    self.content.insert(content_id.clone(), (player.clone(), title.clone(), 0, vec![], vec![]));
                    self.burn_tokens(cost);
                    self.reputations
                        .entry(player.clone())
                        .and_modify(|e| *e += 5)
                        .or_insert(5);
                    println!("{} uploaded content {}: {}", player, content_id, title);
                    ActionResult::ContentUploaded { player, content_id }
                } else {
                    ActionResult::Error { message: "Insufficient DTOSHI".to_string() }
                }
            }
            Action::LikeContent { player, content_id } => {
                if let Some((creator, _, _, likes, _)) = self.content.get(&content_id) {
                    if !likes.contains(&player) {
                        let creator = creator.clone();
                        let reward = self.check_daily_cap(&creator, 1 * 1_000_000_000);
                        let content_id = content_id.clone();
                        let player = player.clone();
                        if let Some((_, _, _, likes, _)) = self.content.get_mut(&content_id) {
                            likes.push(player.clone());
                        }
                        if let Some(creator_player) = self.full_gamefi_runtime.players.get_mut(&creator) {
                            creator_player.mtoshi_balance += reward;
                            self.burn_tokens(reward);
                        }
                        self.reputations
                            .entry(player.clone())
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                        println!("{} liked content {}", player, content_id);
                        ActionResult::ContentLiked { player, content_id }
                    } else {
                        ActionResult::Error { message: "Already liked".to_string() }
                    }
                } else {
                    ActionResult::Error { message: "Content not found".to_string() }
                }
            }
            Action::CommentContent { player, comment, content_id } => {
                if let Some((creator, _, _, _, _comments)) = self.content.get(&content_id) {
                    let creator = creator.clone();
                    let reward = self.check_daily_cap(&creator, 2 * 1_000_000_000);
                    let content_id = content_id.clone();
                    let player = player.clone();
                    if let Some((_, _, _, _, comments)) = self.content.get_mut(&content_id) {
                        comments.push(comment.clone());
                    }
                    if let Some(creator_player) = self.full_gamefi_runtime.players.get_mut(&creator) {
                        creator_player.mtoshi_balance += reward;
                        self.burn_tokens(reward);
                    }
                    self.reputations
                        .entry(player.clone())
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                    println!("{} commented on content {}: {}", player, content_id, comment);
                    ActionResult::ContentCommented { player, content_id }
                } else {
                    ActionResult::Error { message: "Content not found".to_string() }
                }
            }
            Action::ViewContent { player, content_id } => {
                if let Some((creator, _, _views, _, _)) = self.content.get(&content_id) {
                    let creator = creator.clone();
                    let reward = self.check_daily_cap(&creator, self.get_mining_reward(&player));
                    let content_id = content_id.clone();
                    let player = player.clone();
                    if let Some((_, _, views, _, _)) = self.content.get_mut(&content_id) {
                        *views += 1;
                    }
                    if let Some(creator_player) = self.full_gamefi_runtime.players.get_mut(&creator) {
                        creator_player.mtoshi_balance += reward;
                        self.burn_tokens(reward);
                    }
                    println!("{} viewed content {}", player, content_id);
                    ActionResult::ContentViewed { player, content_id }
                } else {
                    ActionResult::Error { message: "Content not found".to_string() }
                }
            }
            Action::PostContent { player, post_id, caption } => {
                let cost = 3 * 1_000_000_000; // 3 DTOSHI
                if self.full_gamefi_runtime.players.get_mut(&player).map(|p| p.mtoshi_balance >= cost).unwrap_or(false) {
                    self.full_gamefi_runtime.players.get_mut(&player).unwrap().mtoshi_balance -= cost;
                    self.posts.insert(post_id.clone(), (player.clone(), caption.clone(), vec![], vec![], 0, 0));
                    self.burn_tokens(cost);
                    self.reputations
                        .entry(player.clone())
                        .and_modify(|e| *e += 5)
                        .or_insert(5);
                    println!("{} posted content {}: {}", player, post_id, caption);
                    ActionResult::ContentPosted { player, post_id }
                } else {
                    ActionResult::Error { message: "Insufficient DTOSHI".to_string() }
                }
            }
            Action::LikePost { player, post_id, paid } => {
                if let Some((creator, _, likes, _, _, _reputation_points)) = self.posts.get(&post_id) {
                    if !likes.contains(&player) {
                        let creator = creator.clone();
                        let post_id = post_id.clone();
                        let player = player.clone();
                        let cost = if paid { 0.05 * 1_000_000_000 as f64 } else { 0.0 }; // 0.05 DTOSHI
                        let creator_share = if paid { (cost * 0.8) as u64 } else { 0 };
                        let platform_share = if paid { (cost * 0.1) as u64 } else { 0 };
                        let community_share = if paid { (cost * 0.1) as u64 } else { 0 };
                        let reward = if paid { self.check_daily_cap(&creator, creator_share) } else { self.check_daily_cap(&creator, 1 * 1_000_000_000) };
                        let burn = if paid { self.burn_tokens(cost as u64) } else { 0 };
                        let reputation_increase = if paid { 2 } else { 1 };
                        if paid && !self.full_gamefi_runtime.players.get(&player).map(|p| p.mtoshi_balance >= cost as u64).unwrap_or(false) {
                            return ActionResult::Error { message: "Insufficient DTOSHI".to_string() };
                        }
                        if let Some((_, _, likes, _, _, reputation_points)) = self.posts.get_mut(&post_id) {
                            likes.push(player.clone());
                            *reputation_points += 1;
                        }
                        if paid {
                            self.full_gamefi_runtime.players.get_mut(&player).unwrap().mtoshi_balance -= cost as u64;
                            self.platform_pool += platform_share;
                            self.community_pool += community_share;
                        }
                        if let Some(creator_player) = self.full_gamefi_runtime.players.get_mut(&creator) {
                            creator_player.mtoshi_balance += reward;
                        }
                        if burn > 0 {
                            self.burn_tokens(burn);
                        }
                        self.reputations
                            .entry(player.clone())
                            .and_modify(|e| *e += reputation_increase)
                            .or_insert(reputation_increase);
                        println!("{} liked post {} (paid: {})", player, post_id, paid);
                        ActionResult::PostLiked { player, post_id, paid }
                    } else {
                        ActionResult::Error { message: "Already liked".to_string() }
                    }
                } else {
                    ActionResult::Error { message: "Post not found".to_string() }
                }
            }
            Action::CommentPost { player, post_id, comment } => {
                if let Some((creator, _, _, _comments, _, _reputation_points)) = self.posts.get(&post_id) {
                    let creator = creator.clone();
                    let reward = self.check_daily_cap(&creator, 1 * 1_000_000_000);
                    let post_id = post_id.clone();
                    let player = player.clone();
                    if let Some((_, _, _, comments, _, reputation_points)) = self.posts.get_mut(&post_id) {
                        comments.push(comment.clone());
                        *reputation_points += 1;
                    }
                    if let Some(creator_player) = self.full_gamefi_runtime.players.get_mut(&creator) {
                        creator_player.mtoshi_balance += reward;
                        self.burn_tokens(reward);
                    }
                    self.reputations
                        .entry(player.clone())
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                    println!("{} commented on post {}: {}", player, post_id, comment);
                    ActionResult::PostCommented { player, post_id }
                } else {
                    ActionResult::Error { message: "Post not found".to_string() }
                }
            }
            Action::BoostPost { player, post_id, tokens } => {
                let cost = tokens * 1_000_000_000; // tokens DTOSHI
                if cost < 1_000_000_000 || cost > 2_000_000_000 {
                    return ActionResult::Error { message: "Boost cost must be 1-2 DTOSHI".to_string() };
                }
                if self.full_gamefi_runtime.players.get(&player).map(|p| p.mtoshi_balance >= cost).unwrap_or(false) {
                    if let Some((creator, _, _, _, _boosts, _reputation_points)) = self.posts.get(&post_id) {
                        if creator != &player {
                            return ActionResult::Error { message: "Only post creator can boost".to_string() };
                        }
                        let creator = creator.clone();
                        let creator_share = (cost as f64 * 0.8) as u64;
                        let platform_share = (cost as f64 * 0.1) as u64;
                        let community_share = (cost as f64 * 0.1) as u64;
                        let reward = self.check_daily_cap(&creator, creator_share);
                        let post_id = post_id.clone();
                        self.full_gamefi_runtime.players.get_mut(&player).unwrap().mtoshi_balance -= cost;
                        if let Some((_, _, _, _, boosts, reputation_points)) = self.posts.get_mut(&post_id) {
                            *boosts += tokens;
                            *reputation_points += tokens;
                        }
                        if let Some(creator_player) = self.full_gamefi_runtime.players.get_mut(&creator) {
                            creator_player.mtoshi_balance += reward;
                        }
                        self.platform_pool += platform_share;
                        self.community_pool += community_share;
                        self.burn_tokens(cost);
                        println!("{} boosted post {} for {} DTOSHI", player, post_id, tokens);
                        ActionResult::PostBoosted { player, post_id, tokens }
                    } else {
                        ActionResult::Error { message: "Post not found".to_string() }
                    }
                } else {
                    ActionResult::Error { message: "Insufficient DTOSHI".to_string() }
                }
            }
            Action::GetPosts => {
                let posts = self.posts.iter()
                    .map(|(id, (creator, caption, likes, _, boosts, _))| (id.clone(), creator.clone(), caption.clone(), likes.len() as u64, *boosts))
                    .collect();
                ActionResult::Posts { posts }
            }
            Action::GetVideos => {
                let videos = self.content.iter()
                    .map(|(id, (creator, title, views, _, _))| (id.clone(), creator.clone(), title.clone(), *views))
                    .collect();
                ActionResult::Videos { videos }
            }
            Action::GetNftListings => {
                let listings = self.nft_marketplace.iter()
                    .flat_map(|(seller, listings)| {
                        listings.iter().filter_map(|(index, price)| {
                            self.full_gamefi_runtime.players.get(seller)
                                .and_then(|p| p.inventory.get(*index as usize))
                                .map(|nft| (seller.clone(), *index, nft.name.clone(), *price))
                        })
                    })
                    .collect();
                ActionResult::NftListings { listings }
            }
        }
    }

    pub fn simulate_player_activity(&mut self) {
        self.full_gamefi_runtime.simulate_player_activity();
    }

    pub fn show_leaderboard(&self) {
        self.full_gamefi_runtime.show_leaderboard();
    }

    pub fn distribute_time_rewards(&mut self) {
        let rewards: Vec<(String, String, u64)> = self.posts.iter()
            .map(|(post_id, (creator, _, _, _, _, _))| {
                let reward = self.get_mining_reward(creator);
                (creator.clone(), post_id.clone(), reward)
            })
            .chain(self.content.iter()
                .map(|(content_id, (creator, _, _, _, _))| {
                    let reward = self.get_mining_reward(creator);
                    (creator.clone(), content_id.clone(), reward)
                }))
            .collect();

        for (creator, id, reward) in rewards {
            let capped_reward = self.check_daily_cap(&creator, reward);
            if let Some(creator_player) = self.full_gamefi_runtime.players.get_mut(&creator) {
                creator_player.mtoshi_balance += capped_reward;
                self.burn_tokens(capped_reward);
                println!("Distributed {} DTOSHI to {} for {}", capped_reward / 1_000_000_000, creator, id);
            }
        }
    }
}
