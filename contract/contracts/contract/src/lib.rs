#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, String, Vec};

// Structure for storing paper details
#[derive(Clone)]
#[contract]
pub struct AcademicPublishingContract;

#[contractimpl]
impl AcademicPublishingContract {

    // Submit a paper
    pub fn submit_paper(env: Env, author: String, title: String, content_hash: String) {
        let key = (symbol_short!("PAPER"), author.clone(), title.clone());

        env.storage().instance().set(&key, &content_hash);
    }

    // Get paper by author + title
    pub fn get_paper(env: Env, author: String, title: String) -> String {
        let key = (symbol_short!("PAPER"), author, title);

        env.storage()
            .instance()
            .get(&key)
            .unwrap_or(String::from_str(&env, "Not Found"))
    }

    // Simple peer review (stores approval)
    pub fn review_paper(env: Env, reviewer: String, title: String, approved: bool) {
        let key = (symbol_short!("REVIEW"), reviewer.clone(), title.clone());

        env.storage().instance().set(&key, &approved);
    }

    // Get review status
    pub fn get_review(env: Env, reviewer: String, title: String) -> bool {
        let key = (symbol_short!("REVIEW"), reviewer, title);

        env.storage()
            .instance()
            .get(&key)
            .unwrap_or(false)
    }
}