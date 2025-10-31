use bevy::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const OUT_PATH: &str = "call_records.csv";

#[derive(Resource)]
struct CallLog {
    buffer: Vec<String>,
    file_handle: File,
    out_path: PathBuf,
}

impl CallLog {
    fn new(path: PathBuf) -> CallLog {
        let mut file_handle = File::create(&path).unwrap();
        file_handle.write_all(b"timestamp,caller,tower\n").unwrap();
        CallLog {
            buffer: vec![],
            out_path: path.clone(),
            file_handle,
        }
    }
}

pub fn data_writer_plugin(app: &mut App) {
    let mut out_path = PathBuf::new();
    out_path.push(std::env::var("PHEEPLE_OUT_DIR").unwrap());
    out_path.push(OUT_PATH);
    let out_path_str = out_path.display();
    info!("Simulated data at {out_path_str}");
    app.insert_resource(CallLog::new(out_path));
    app.add_observer(write_call_event_record);
    app.add_systems(PostUpdate, flush_call_events);
}

fn write_call_event_record(
    call: On<crate::tower::CallStarted>,
    mut call_log: ResMut<CallLog>,
    time: Res<Time>,
) {
    info!("Writing call record");
    let timestamp = time.elapsed_secs();
    let caller_id = call.caller;
    let tower_id = call.tower;
    let call_record = format!("{timestamp},{caller_id},{tower_id}");
    call_log.buffer.push(call_record.to_string())
}

fn flush_call_events(mut call_log: ResMut<CallLog>) {
    let bytes_to_write = call_log.buffer.join("\n");
    call_log.buffer = [].to_vec();
    call_log
        .file_handle
        .write_all(&bytes_to_write.into_bytes())
        .unwrap();
    call_log.file_handle.write_all(b"\n").unwrap();
}
