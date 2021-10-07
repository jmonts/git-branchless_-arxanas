//! The command-line options for `git-branchless`.

use clap::Clap;
use std::path::PathBuf;

/// A command wrapped by `git-branchless wrap`. The arguments are forwarded to
/// `git`.
#[derive(Clap)]
pub enum WrappedCommand {
    /// The wrapped command.
    #[clap(external_subcommand)]
    WrappedCommand(Vec<String>),
}

/// @nocommit write man page text
#[derive(Clap)]
pub enum Command {
    /// Initialize the branchless workflow for this repository.
    Init {
        /// Uninstall the branchless workflow instead of initializing it.
        #[clap(long = "uninstall")]
        uninstall: bool,
    },

    /// Display a nice graph of the commits you've recently worked on.
    Smartlog {
        /// Also show commits which have been hidden.
        #[clap(long = "hidden")]
        show_hidden_commits: bool,
    },

    /// Hide the provided commits from the smartlog.
    Hide {
        /// Zero or more commits to hide.
        ///
        /// Can either be hashes, like `abc123`, or ref-specs, like `HEAD^`.
        commits: Vec<String>,

        /// Also recursively hide all visible children commits of the provided
        /// commits.
        #[clap(short = 'r', long = "recursive")]
        recursive: bool,
    },

    /// Unhide previously-hidden commits from the smartlog.
    Unhide {
        /// Zero or more commits to unhide.
        ///
        /// Can either be hashes, like `abc123`, or ref-specs, like `HEAD^`.
        commits: Vec<String>,

        /// Also recursively unhide all children commits of the provided commits.
        #[clap(short = 'r', long = "recursive")]
        recursive: bool,
    },

    /// Move to an earlier commit in the current stack.
    Prev {
        /// The number of commits backward to go.
        num_commits: Option<isize>,
    },

    /// Move to a later commit in the current stack.
    Next {
        /// The number of commits forward to go.
        ///
        /// If not provided, defaults to 1.
        num_commits: Option<isize>,

        /// When encountering multiple next commits, choose the oldest.
        #[clap(short = 'o', long = "oldest")]
        oldest: bool,

        /// When encountering multiple next commits, choose the newest.
        #[clap(short = 'n', long = "newest", conflicts_with("oldest"))]
        newest: bool,
    },

    /// Move a subtree of commits from one location to another.
    ///
    /// By default, `git move` tries to move the entire current stack if you
    /// don't pass a `--source` or `--base` option (equivalent to writing
    /// `--base HEAD`).
    ///
    /// By default, `git move` attempts to rebase all commits in-memory. If you
    /// want to force an on-disk rebase, pass the `--on-disk` flag. Note that
    /// `post-commit` hooks are not called during in-memory rebases.
    Move {
        /// The source commit to move. This commit, and all of its descendants,
        /// will be moved.
        #[clap(short = 's', long = "source")]
        source: Option<String>,

        /// A commit inside a subtree to move. The entire subtree, starting from
        /// the main branch, will be moved, not just the commits descending from
        /// this commit.
        #[clap(short = 'b', long = "base", conflicts_with = "source")]
        base: Option<String>,

        /// The destination commit to move all source commits onto. If not
        /// provided, defaults to the current commit.
        #[clap(short = 'd', long = "dest")]
        dest: Option<String>,

        /// Only attempt to perform an in-memory rebase. If it fails, do not
        /// attempt an on-disk rebase.
        #[clap(long = "in-memory", conflicts_with = "force-on-disk")]
        force_in_memory: bool,

        /// Skip attempting to use an in-memory rebase, and try an
        /// on-disk rebase directly.
        #[clap(long = "on-disk")]
        force_on_disk: bool,

        /// Debugging option. Print the constraints used to create the rebase
        /// plan before executing it.
        #[clap(long = "debug-dump-rebase-constraints")]
        dump_rebase_constraints: bool,

        /// Debugging option. Print the rebase plan that will be executed before
        /// executing it.
        #[clap(long = "debug-dump-rebase-plan")]
        dump_rebase_plan: bool,
    },

    /// Fix up commits abandoned by a previous rewrite operation.
    Restack {
        /// The IDs of the abandoned commits whose descendants should be
        /// restacked. If not provided, all abandoned commits are restacked.
        commits: Vec<String>,

        /// Only attempt to perform an in-memory rebase. If it fails, do not
        /// attempt an on-disk rebase.
        #[clap(long = "in-memory", conflicts_with = "force-on-disk")]
        force_in_memory: bool,

        /// Skip attempting to use an in-memory rebase, and try an
        /// on-disk rebase directly.
        #[clap(long = "on-disk")]
        force_on_disk: bool,

        /// Debugging option. Print the constraints used to create the rebase
        /// plan before executing it.
        #[clap(long = "debug-dump-rebase-constraints")]
        dump_rebase_constraints: bool,

        /// Debugging option. Print the rebase plan that will be executed before
        /// executing it.
        #[clap(long = "debug-dump-rebase-plan")]
        dump_rebase_plan: bool,
    },

    /// Browse or return to a previous state of the repository.
    Undo,

    /// Run internal garbage collection.
    Gc,

    /// Wrap a Git command inside a branchless transaction.
    Wrap {
        /// The `git` executable to invoke.
        #[clap(long = "git-executable")]
        git_executable: Option<PathBuf>,

        /// The arguments to pass to `git`.
        #[clap(subcommand)]
        command: WrappedCommand,
    },

    /// Internal use.
    HookPreAutoGc,

    /// Internal use.
    HookPostRewrite {
        /// One of `amend` or `rebase`.
        rewrite_type: String,
    },

    /// Internal use.
    HookRegisterExtraPostRewriteHook,

    /// Internal use.
    HookDetectEmptyCommit {
        /// The OID of the commit currently being applied, to be checked for emptiness.
        old_commit_oid: String,
    },

    /// Internal use.
    HookSkipUpstreamAppliedCommit {
        /// The OID of the commit that was skipped.
        commit_oid: String,
    },

    /// Internal use.
    HookPostCheckout {
        /// The previous commit OID.
        previous_commit: String,

        /// The current commit OID.
        current_commit: String,

        /// Whether or not this was a branch checkout (versus a file checkout).
        is_branch_checkout: isize,
    },

    /// Internal use.
    HookPostCommit,

    /// Internal use.
    HookPostMerge {
        /// Whether or not this is a squash merge. See githooks(5).
        is_squash_merge: isize,
    },

    /// Internal use.
    HookReferenceTransaction {
        /// One of `prepared`, `committed`, or `aborted`. See githooks(5).
        transaction_state: String,
    },
}

/// Branchless workflow for Git.
///
/// See the documentation at https://github.com/arxanas/git-branchless/wiki.
#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Waleed Khan <me@waleedkhan.name>")]
pub struct Opts {
    /// Change to the given directory before executing the rest of the program.
    /// (The option is called `-C` for symmetry with Git.)
    #[clap(short = 'C')]
    pub working_directory: Option<PathBuf>,

    /// The `git-branchless` subcommand to run.
    #[clap(subcommand)]
    pub command: Command,
}