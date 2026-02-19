//! DTOs for the Blog module
//!
//! This module contains all Data Transfer Objects used for API communication.

mod post;

pub use post::{CreatePostInput, PostResponse, PostSummary, UpdatePostInput};
