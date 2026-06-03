pub mod tree;
pub mod fenwick;
pub mod naive;
pub mod optimized;

// RE-EXPORT for external use (main.rs, tests, benches)
pub use optimized::OptimizedTree;