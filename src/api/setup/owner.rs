#[path = "owner/owner-post.rs"]
mod owner_post;
pub use owner_post::post;

#[path = "owner/owner-get.rs"]
mod owner_get;
pub use owner_get::get;