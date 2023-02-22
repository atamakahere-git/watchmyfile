use fanotify::high_level::*;
use nix::poll::{poll, PollFd, PollFlags};
use std::path::Path;

// Importing all flags from outer crate
pub use fanotify::high_level::{
    FAN_ACCESS, FAN_ACCESS_PERM, FAN_ATTRIB, FAN_CLOSE, FAN_CLOSE_NOWRITE, FAN_CLOSE_WRITE,
    FAN_CREATE, FAN_DELETE, FAN_DELETE_SELF, FAN_EVENT_ON_CHILD, FAN_MODIFY, FAN_MOVE,
    FAN_MOVED_FROM, FAN_MOVED_TO, FAN_MOVE_SELF, FAN_ONDIR, FAN_OPEN, FAN_OPEN_EXEC,
    FAN_OPEN_EXEC_PERM, FAN_OPEN_PERM,
};

#[derive(PartialEq)]
pub struct FileTag<'a> {
    path: &'a Path,
    eventmode: u64,
}

impl<'a> FileTag<'a> {
    pub fn new(path: &'a Path, eventmode: u64) -> Self {
        Self { path, eventmode }
    }

    pub fn chnage_path(&mut self, path: &'a Path) {
        self.path = path;
    }

    pub fn change_event(&mut self, eventmode: u64) {
        self.eventmode = eventmode;
    }

    pub fn all_event(&mut self) {
        self.eventmode = 1;
    }
}

pub struct Watcher<'a> {
    watch_points: Vec<FileTag<'a>>,
    fty: Fanotify,
}

impl<'a> Watcher<'a> {
    pub fn new() -> Self {
        Self {
            watch_points: Vec::new(),
            // Setting fanotify mode default to NOTIF. this doesnt supoort pre-content or content mode
            fty: Fanotify::new_with_nonblocking(FanotifyMode::NOTIF),
        }
    }

    pub fn add_file(&mut self, file: FileTag<'a>) -> &mut Self {
        self.watch_points.push(file);
        self
    }

    pub fn remove_file(&mut self, file: FileTag<'a>) -> &mut Self {
        self.watch_points.retain(|f| *f != file);
        self
    }

    pub fn add_files(&mut self, files: Vec<FileTag<'a>>) -> &mut Self {
        files.into_iter().for_each(|file| {
            self.add_file(file);
        });
        self
    }

    pub fn init(&self) -> &Self {
        self.watch_points.iter().for_each(|file| {
            self.fty
                .add_mountpoint(file.eventmode, file.path.to_str().unwrap())
                .unwrap();
        });
        self
    }

    pub fn watch(&self) {
        let mut fds = [PollFd::new(self.fty.as_raw_fd(), PollFlags::POLLIN)];
        loop {
            let poll_num = poll(&mut fds, -1).unwrap();
            if poll_num > 0 {
                for event in self.fty.read_event() {
                    println!("{event:?}");
                }
            } else {
                break;
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::path::Path;

//     use fanotify::low_level::FAN_ACCESS;
//     use fanotify::low_level::FAN_DELETE;
//     use fanotify::low_level::FAN_MODIFY;
//     use fanotify::low_level::FAN_OPEN;

//     use crate::FileTag;
//     use crate::Watcher;

//     #[test]
//     fn run_work() {
//         let watchflag = FAN_ACCESS | FAN_OPEN | FAN_MODIFY | FAN_DELETE;

//         let file = FileTag::new(Path::new("/"), watchflag);

//         Watcher::new().add_file(file).init().watch();
//     }
// }
