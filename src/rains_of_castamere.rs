use std::{env, fs, path::PathBuf};

fn is_snark_output(path: PathBuf) -> bool {
    let ext = path.extension();
    ext.is_some() && ext.unwrap() == "bytes"
}

pub fn kill_all_snarks() -> anyhow::Result<()> {
    let cur_dir = env::current_dir()?;
    for file in fs::read_dir(cur_dir)? {
        let path = file?.path();
        if is_snark_output(path.clone()) {
            log::info!("Removing {:?}", path);
            fs::remove_file(path)?;
        }
    }
    Ok(())
}
