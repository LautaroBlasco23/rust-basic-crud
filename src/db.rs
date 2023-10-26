use crate::models::{Review, ReviewCreation};
use uuid::Uuid;

pub struct MyDatabase {
    my_data: Vec<Review>,
}

impl MyDatabase {
    pub fn new() -> Self {
        MyDatabase { 
            my_data: vec![],
        }
    }

    pub fn get_all(&self) -> &Vec<Review> {
        &self.my_data
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<&Review> {
        for review in &self.my_data {
            if (*review).get_id() == (id) {
                return Some(&review)
            }
        } 
        None
    }

    pub fn insert_review(&mut self, review: &ReviewCreation) {
        let new_review: Review = Review::new(review);
        self.my_data.push(new_review);
    }

    pub fn modify_review(&mut self, id: Uuid, review_update_data: ReviewCreation) -> Option<Review>{
        let mut index: i32 = -1;
        
        for i in 0..self.my_data.len() {
            if self.my_data[i].get_id() == id {
                index = i as i32;
                break;
            }
        }
        
        if index > -1 {
            return Some(self.my_data[index as usize].update_review(review_update_data));
        }
        None
    }

    pub fn delete_review(&mut self, id: Uuid) -> Option<Uuid>{
        let mut index: i32 = -1;
        
        for i in 0..self.my_data.len() {
            if self.my_data[i].get_id() == id {
                index = i as i32;
                break;
            }
        }
        
        if index > -1 {
            let id = self.my_data[index as usize].get_id();
            self.my_data.remove(index as usize);
            return Some(id);
        }
        None
    }
}
