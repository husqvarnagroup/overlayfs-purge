use overlayfs_purge::run;
use std::path::PathBuf;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let force_flag = args[1..].iter().any(|a| a == "-f");
    if !force_flag {
        println!("Aborting. Run with `-f` if you know what you are doing.");
        std::process::exit(1);
    }

    let positional: Vec<&str> = args[1..]
        .iter()
        .filter(|a| *a != "-f")
        .map(String::as_str)
        .collect();

    let keep_file: PathBuf;
    let keep_dir: PathBuf;
    let lower_dir: PathBuf;
    let upper_dir: PathBuf;

    match positional.as_slice() {
        [] => {
            keep_file = PathBuf::from("/etc/sysupgrade.conf");
            keep_dir = PathBuf::from("/usr/lib/upgrade/keep.d");
            lower_dir = PathBuf::from("/media/rfs/ro");
            upper_dir = PathBuf::from("/media/rfs/rw/upperdir");
        }
        [kf, kd, ld, ud] => {
            keep_file = PathBuf::from(kf);
            keep_dir = PathBuf::from(kd);
            lower_dir = PathBuf::from(ld);
            upper_dir = PathBuf::from(ud);
        }
        _ => {
            eprintln!(
                "Usage: {} [-f] [<keep-file> <keep-dir> <lower-dir> <upper-dir>]",
                args[0]
            );
            std::process::exit(1);
        }
    }

    run(&keep_file, &keep_dir, &lower_dir, &upper_dir);
}
