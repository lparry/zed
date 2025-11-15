use criterion::{Criterion, criterion_group, criterion_main};
use project::{Entry, EntryKind, GitEntry, ProjectEntryId};
use project_panel::par_sort_worktree_entries;
use std::sync::Arc;
use util::{paths::DirectorySortOrder, rel_path::RelPath};

const LINUX_REPO_SNAPSHOT: &str = include_str!("linux_repo_snapshot.txt");

fn load_linux_repo_snapshot() -> Vec<GitEntry> {
    LINUX_REPO_SNAPSHOT
        .lines()
        .filter_map(|line| {
            let kind = match line.chars().next() {
                Some('f') => EntryKind::File,
                Some('d') => EntryKind::Dir,
                _ => return None,
            };

            let entry = Entry {
                kind,
                path: Arc::from(RelPath::unix(&(line.trim_end()[2..])).unwrap()),
                id: ProjectEntryId::default(),
                size: 0,
                inode: 0,
                mtime: None,
                canonical_path: None,
                is_ignored: false,
                is_always_included: false,
                is_external: false,
                is_private: false,
                is_hidden: false,
                char_bag: Default::default(),
                is_fifo: false,
            };
            Some(GitEntry {
                entry,
                git_summary: Default::default(),
            })
        })
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let snapshot = load_linux_repo_snapshot();
    for (order, label) in [
        (DirectorySortOrder::DirectoriesFirst, "directories_first"),
        (DirectorySortOrder::Mixed, "mixed"),
        (DirectorySortOrder::DirectoriesLast, "directories_last"),
    ] {
        c.bench_function(
            &format!("Sort linux worktree snapshot/{label}"),
            |b| {
                b.iter_batched(
                    || snapshot.clone(),
                    |mut snapshot| par_sort_worktree_entries(&mut snapshot, order),
                    criterion::BatchSize::LargeInput,
                );
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
