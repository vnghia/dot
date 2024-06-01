use git2::RemoteCallbacks;
use indicatif::{ProgressBar, ProgressStyle};

pub struct GitProgress;

impl GitProgress {
    pub fn remote_callbacks() -> RemoteCallbacks<'static> {
        let receiving_pb = ProgressBar::new_spinner()
            .with_message("Receiving objects")
            .with_finish(indicatif::ProgressFinish::Abandon)
            .with_style(ProgressStyle::with_template("{msg}: {percent}% ({pos}/{len})").unwrap());

        let resolving_pb = ProgressBar::new_spinner()
            .with_message("Resolving deltas")
            .with_finish(indicatif::ProgressFinish::Abandon)
            .with_style(ProgressStyle::with_template("{msg}: {percent}% ({pos}/{len})").unwrap());

        let mut cb = RemoteCallbacks::new();
        cb.transfer_progress(move |progress| {
            if receiving_pb.length().is_none() {
                receiving_pb.set_length(progress.total_objects() as _);
            }
            receiving_pb.set_position(progress.received_objects() as _);

            if progress.total_deltas() > 0 {
                receiving_pb.finish();
                if resolving_pb.length().is_none() {
                    resolving_pb.set_length(progress.total_deltas() as _);
                }
                resolving_pb.set_position(progress.indexed_deltas() as _);
            }
            true
        });

        cb
    }
}
