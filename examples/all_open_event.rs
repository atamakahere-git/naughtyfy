use naughtyfy::api::*;
use naughtyfy::flags::*;

fn main() {
    let fd = init(FAN_CLASS_NOTIF, O_RDONLY);
    if fd.is_err() {
        eprintln!("Encountered err due to {fd:?}");
    }
    let fd = fd.unwrap();
    let status = mark(
        fd,
        FAN_MARK_ADD | FAN_MARK_MOUNT,
        FAN_OPEN | FAN_EVENT_ON_CHILD,
        AT_FDCWD,
        // Looking for whole fs.
        "/",
    );
    if status.is_err() {
        eprintln!("Encountered err due to {fd:?}");
    }
    let _status = status.unwrap();

    loop {
        // read_do(fd, print_meta).unwrap();
        let data = read(fd).unwrap();
        println!("{:#?}", data);
        data.iter().for_each(|e| {
            close(e.fd).unwrap();
        });
    }
}
