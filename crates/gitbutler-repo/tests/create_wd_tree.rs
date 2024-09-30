use std::path::Path;

use gitbutler_repo::RepositoryExt as _;
use gitbutler_testsupport::testing_repository::{assert_tree_matches, TestingRepository};

/// These tests exercise the truth table that we use to update the HEAD
/// tree to match the worktree.
///
/// Truth table for upsert/remove:
/// | HEAD Tree -> Index | Index -> Worktree | Action |
/// | add                | delete            | no-action |
/// | modify             | delete            | remove |
/// |                    | delete            | remove |
/// | delete             |                   | remove |
/// | delete             | add               | upsert |
/// | add                |                   | upsert |
/// |                    | add               | upsert |
/// | add                | modify            | upsert |
/// | modify             | modify            | upsert |
#[cfg(test)]
mod head_upsert_truthtable {

    use super::*;

    // | add                | delete            | no-action |
    #[test]
    fn index_new_worktree_delete() {
        let test_repository = TestingRepository::open();

        let commit = test_repository.commit_tree(None, &[]);
        test_repository
            .repository
            .branch("master", &commit, true)
            .unwrap();

        std::fs::write(test_repository.tempdir.path().join("file1.txt"), "content1").unwrap();

        let mut index = test_repository.repository.index().unwrap();
        index.add_path(Path::new("file1.txt")).unwrap();
        index.write().unwrap();

        std::fs::remove_file(test_repository.tempdir.path().join("file1.txt")).unwrap();

        let tree: git2::Tree = test_repository.repository.create_wd_tree().unwrap();

        assert_eq!(tree.len(), 0, "Tree should end up empty");
    }

    // | modify             | delete            | remove    |
    #[test]
    fn index_modify_worktree_delete() {
        let test_repository = TestingRepository::open();

        let commit = test_repository.commit_tree(None, &[("file1.txt", "content1")]);
        test_repository
            .repository
            .branch("master", &commit, true)
            .unwrap();

        std::fs::write(test_repository.tempdir.path().join("file1.txt"), "content2").unwrap();

        let mut index = test_repository.repository.index().unwrap();
        index.add_path(Path::new("file1.txt")).unwrap();
        index.write().unwrap();

        std::fs::remove_file(test_repository.tempdir.path().join("file1.txt")).unwrap();

        let tree: git2::Tree = test_repository.repository.create_wd_tree().unwrap();

        assert_eq!(tree.len(), 0, "Tree should end up empty");
    }

    // |                    | delete            | remove    |
    #[test]
    fn worktree_delete() {
        let test_repository = TestingRepository::open();

        let commit = test_repository.commit_tree(None, &[("file1.txt", "content1")]);
        test_repository
            .repository
            .branch("master", &commit, true)
            .unwrap();

        std::fs::remove_file(test_repository.tempdir.path().join("file1.txt")).unwrap();

        let tree: git2::Tree = test_repository.repository.create_wd_tree().unwrap();

        assert_eq!(tree.len(), 0, "Tree should end up empty");
    }

    // | delete             |                   | remove    |
    #[test]
    fn index_delete() {
        let test_repository = TestingRepository::open();

        let commit = test_repository.commit_tree(None, &[("file1.txt", "content1")]);
        test_repository
            .repository
            .branch("master", &commit, true)
            .unwrap();

        let mut index = test_repository.repository.index().unwrap();
        index.remove_all(["*"], None).unwrap();
        index.write().unwrap();

        let tree: git2::Tree = test_repository.repository.create_wd_tree().unwrap();

        // We should ignore whatever happens to the index
        assert_tree_matches(
            &test_repository.repository,
            &tree,
            &[("file1.txt", b"content1")],
        );
    }

    // | delete             | add               | upsert    |
    #[test]
    fn index_delete_worktree_add() {
        let test_repository = TestingRepository::open();

        let commit = test_repository.commit_tree(None, &[("file1.txt", "content1")]);
        test_repository
            .repository
            .branch("master", &commit, true)
            .unwrap();

        let mut index = test_repository.repository.index().unwrap();
        index.remove_all(["*"], None).unwrap();
        index.write().unwrap();

        std::fs::write(test_repository.tempdir.path().join("file1.txt"), "content2").unwrap();

        let tree: git2::Tree = test_repository.repository.create_wd_tree().unwrap();

        // Tree should match whatever is written on disk
        assert_tree_matches(
            &test_repository.repository,
            &tree,
            &[("file1.txt", b"content2")],
        );
    }

    // | add                |                   | upsert    |
    #[test]
    fn index_add() {
        let test_repository = TestingRepository::open();

        let commit = test_repository.commit_tree(None, &[]);
        test_repository
            .repository
            .branch("master", &commit, true)
            .unwrap();

        std::fs::write(test_repository.tempdir.path().join("file1.txt"), "content2").unwrap();

        let mut index = test_repository.repository.index().unwrap();
        index.add_path(Path::new("file1.txt")).unwrap();
        index.write().unwrap();

        let tree: git2::Tree = test_repository.repository.create_wd_tree().unwrap();

        // Tree should match whatever is written on disk
        assert_tree_matches(
            &test_repository.repository,
            &tree,
            &[("file1.txt", b"content2")],
        );
    }

    // |                    | add               | upsert    |
    #[test]
    fn worktree_add() {
        let test_repository = TestingRepository::open();

        let commit = test_repository.commit_tree(None, &[]);
        test_repository
            .repository
            .branch("master", &commit, true)
            .unwrap();

        std::fs::write(test_repository.tempdir.path().join("file1.txt"), "content2").unwrap();

        let tree: git2::Tree = test_repository.repository.create_wd_tree().unwrap();

        // Tree should match whatever is written on disk
        assert_tree_matches(
            &test_repository.repository,
            &tree,
            &[("file1.txt", b"content2")],
        );
    }

    // | add                | modify            | upsert    |
    #[test]
    fn index_add_worktree_modify() {
        let test_repository = TestingRepository::open();

        let commit = test_repository.commit_tree(None, &[]);
        test_repository
            .repository
            .branch("master", &commit, true)
            .unwrap();

        std::fs::write(test_repository.tempdir.path().join("file1.txt"), "content1").unwrap();

        let mut index = test_repository.repository.index().unwrap();
        index.add_path(Path::new("file1.txt")).unwrap();
        index.write().unwrap();

        std::fs::write(test_repository.tempdir.path().join("file1.txt"), "content2").unwrap();

        let tree: git2::Tree = test_repository.repository.create_wd_tree().unwrap();

        // Tree should match whatever is written on disk
        assert_tree_matches(
            &test_repository.repository,
            &tree,
            &[("file1.txt", b"content2")],
        );
    }

    // | modify             | modify            | upsert    |
    #[test]
    fn index_modify_worktree_modify() {
        let test_repository = TestingRepository::open();

        let commit = test_repository.commit_tree(None, &[("file1.txt", "content1")]);
        test_repository
            .repository
            .branch("master", &commit, true)
            .unwrap();

        std::fs::write(test_repository.tempdir.path().join("file1.txt"), "content2").unwrap();

        let mut index = test_repository.repository.index().unwrap();
        index.add_path(Path::new("file1.txt")).unwrap();
        index.write().unwrap();

        std::fs::write(test_repository.tempdir.path().join("file1.txt"), "content3").unwrap();

        let tree: git2::Tree = test_repository.repository.create_wd_tree().unwrap();

        // Tree should match whatever is written on disk
        assert_tree_matches(
            &test_repository.repository,
            &tree,
            &[("file1.txt", b"content3")],
        );
    }
}

#[test]
fn lists_uncommited_changes() {
    let test_repository = TestingRepository::open();

    // Initial commit
    // Create wd tree requires the HEAD branch to exist and for there
    // to be at least one commit on that branch.
    let commit = test_repository.commit_tree(None, &[]);
    test_repository
        .repository
        .branch("master", &commit, true)
        .unwrap();

    std::fs::write(test_repository.tempdir.path().join("file1.txt"), "content1").unwrap();
    std::fs::write(test_repository.tempdir.path().join("file2.txt"), "content2").unwrap();

    let tree = test_repository.repository.create_wd_tree().unwrap();

    assert_tree_matches(
        &test_repository.repository,
        &tree,
        &[("file1.txt", b"content1"), ("file2.txt", b"content2")],
    );
}

#[test]
fn does_not_include_staged_but_deleted_files() {
    let test_repository = TestingRepository::open();

    // Initial commit
    // Create wd tree requires the HEAD branch to exist and for there
    // to be at least one commit on that branch.
    let commit = test_repository.commit_tree(None, &[]);
    test_repository
        .repository
        .branch("master", &commit, true)
        .unwrap();

    std::fs::write(test_repository.tempdir.path().join("file1.txt"), "content1").unwrap();
    std::fs::write(test_repository.tempdir.path().join("file2.txt"), "content2").unwrap();

    std::fs::write(test_repository.tempdir.path().join("file3.txt"), "content2").unwrap();
    let mut index: git2::Index = test_repository.repository.index().unwrap();
    index.add_path(Path::new("file3.txt")).unwrap();
    index.write().unwrap();
    std::fs::remove_file(test_repository.tempdir.path().join("file3.txt")).unwrap();

    let tree: git2::Tree = test_repository.repository.create_wd_tree().unwrap();

    assert_tree_matches(
        &test_repository.repository,
        &tree,
        &[("file1.txt", b"content1"), ("file2.txt", b"content2")],
    );
    assert!(tree.get_name("file3.txt").is_none());
}

#[test]
fn should_be_empty_after_checking_out_empty_tree() {
    let test_repository = TestingRepository::open();

    let commit = test_repository.commit_tree(
        None,
        &[("file1.txt", "content1"), ("file2.txt", "content2")],
    );
    test_repository
        .repository
        .branch("master", &commit, true)
        .unwrap();

    // Checkout an empty tree
    {
        let tree_oid = test_repository
            .repository
            .treebuilder(None)
            .unwrap()
            .write()
            .unwrap();
        let tree = test_repository.repository.find_tree(tree_oid).unwrap();
        test_repository
            .repository
            .checkout_tree_builder(&tree)
            .force()
            .remove_untracked()
            .checkout()
            .unwrap();
    }

    assert!(!test_repository.tempdir.path().join("file1.txt").exists());
    assert!(!test_repository.tempdir.path().join("file2.txt").exists());

    let tree: git2::Tree = test_repository.repository.create_wd_tree().unwrap();

    // Fails because `create_wd_tree` uses the head commit as the base,
    // and then performs modifications to the tree
    assert_eq!(tree.len(), 0);
}

#[test]
fn should_track_deleted_files() {
    let test_repository = TestingRepository::open();

    let commit = test_repository.commit_tree(
        None,
        &[("file1.txt", "content1"), ("file2.txt", "content2")],
    );
    test_repository
        .repository
        .branch("master", &commit, true)
        .unwrap();

    // Make sure the index is empty, perhaps the user did this action
    let mut index: git2::Index = test_repository.repository.index().unwrap();
    index.remove_all(["*"], None).unwrap();
    index.write().unwrap();

    std::fs::remove_file(test_repository.tempdir.path().join("file1.txt")).unwrap();

    assert!(!test_repository.tempdir.path().join("file1.txt").exists());
    assert!(test_repository.tempdir.path().join("file2.txt").exists());

    let tree: git2::Tree = test_repository.repository.create_wd_tree().unwrap();

    // Fails because `create_wd_tree` uses the head commit as the base,
    // and then performs modifications to the tree
    assert!(tree.get_name("file1.txt").is_none());
    assert!(tree.get_name("file2.txt").is_some());
}

#[test]
fn should_not_change_index() {
    let test_repository = TestingRepository::open();

    let commit = test_repository.commit_tree(None, &[("file1.txt", "content1")]);
    test_repository
        .repository
        .branch("master", &commit, true)
        .unwrap();

    let mut index = test_repository.repository.index().unwrap();
    index.remove_all(["*"], None).unwrap();
    index.write().unwrap();

    let index_tree = index.write_tree().unwrap();
    let index_tree = test_repository.repository.find_tree(index_tree).unwrap();
    assert_eq!(index_tree.len(), 0);

    let tree: git2::Tree = test_repository.repository.create_wd_tree().unwrap();

    let mut index = test_repository.repository.index().unwrap();
    let index_tree = index.write_tree().unwrap();
    let index_tree = test_repository.repository.find_tree(index_tree).unwrap();
    assert_eq!(index_tree.len(), 0);

    // Tree should match whatever is written on disk
    assert_tree_matches(
        &test_repository.repository,
        &tree,
        &[("file1.txt", b"content1")],
    );
}
