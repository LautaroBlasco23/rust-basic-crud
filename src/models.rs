use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Stars {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReviewCreation {
    book_name: String,
    book_author: String,
    review_comment: String,
    stars: Stars,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Review {
    id: Uuid,
    book_name: String,
    book_author: String,
    review_comment: String,
    stars: Stars,
    created_at: DateTime<Utc>,
}

impl Review {
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn new(review: &ReviewCreation) -> Review {
        Review {
            id: Uuid::new_v4(),
            book_name: review.book_name.clone(),
            book_author: review.book_author.clone(),
            review_comment: review.review_comment.clone(),
            stars: review.stars,
            created_at: Utc::now(),
        }
    }

    pub fn update_review(&mut self, review: ReviewCreation) -> Review {
        // Setting all updated values:
        self.book_name = review.book_name;
        self.book_author = review.book_author;
        self.review_comment = review.review_comment;
        self.stars = review.stars;

        self.clone()
    }
    
    pub fn get_stars(&self) -> Stars {
        self.stars
    }
}
