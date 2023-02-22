use std::path::Path;

use fanotify::low_level::FAN_OPEN;

use nix::libc::FAN_ACCESS;
use watchmyfile::FileTag;
use watchmyfile::*;

fn main() {
    let watchflag = FAN_ACCESS | FAN_MODIFY | FAN_CLOSE | FAN_OPEN;

    let file = FileTag::new(Path::new("/"), watchflag);

    Watcher::new().add_file(file).init().watch();
}
