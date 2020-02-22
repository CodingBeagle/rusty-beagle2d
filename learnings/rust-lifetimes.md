# Rust Lifetimes

## The Borrow Checker

The Rust compiler has a *borrow checker*.

The job of the borrow checker is to determine whether all borrows are valid. With the borrow checker, Rust can determine if, say, one variable refers to memory of another variable who's lifetime is *smaller*, meaning there might be a chance that it ends up referring to data that no longer exists, leading to *dangling references*.
