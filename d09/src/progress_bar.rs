use indicatif::{ProgressBar, ProgressStyle};

pub fn create_pb() -> ProgressBar {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{elapsed_precise}] [{wide_bar:.green/blue}] {percent}%")
            .unwrap(),
    );

    pb.set_message("Processing...");

    return pb;
}
