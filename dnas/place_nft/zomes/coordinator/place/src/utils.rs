use hdk::prelude::*;
use hdk::hash_path::path::{TypedPath, Component};
use place_integrity::globals::*;


//Path will be relative to a game start time = 0 with 12 subdivisions
// I.e. /root/hour/bucket e.g. /root/11/11
pub const ROOT: &'static str = "root";

/// Determine bucket path from time in sec
/// Format is Root::HourIndex::bucket_indexInHour
pub fn get_path(now: u32) -> Path {
   let sec_since_start_of_game = now - START_TIME;
   // debug!("sec_since_start_of_game is: {}", sec_since_start_of_game);
   // assert!(sec_since_start_of_game < 24 * 3600); // removed now that we can have a finished game for minting

   

   // create bucket path component
   let buckets_since_start = sec_since_start_of_game / BUCKET_SIZE_SEC;
   let subdivision_index = buckets_since_start / 12;
   let subdivision = Component::from(format!("{}", subdivision_index).as_str());
   
   let bucket_in_subdivision = buckets_since_start % 12;
   let bucket = Component::from(format!("{}", bucket_in_subdivision));
   

   let mut path = Path::from(ROOT);
   path.append_component(subdivision);
   path.append_component(bucket);
     //debug!("get_path({}); bucket_path = {}", now, path_to_str(&bucket_path));
   
   path
 }

 pub fn path_to_str(path: &TypedPath) -> String {
   let mut res = String::from("");
   let mut maybe_path: Option<TypedPath> = Some(path.to_owned());
   while maybe_path.is_some() {
      let path = maybe_path.unwrap().to_owned();
      let comp: &Component  = path.leaf().unwrap();
      res = format!("{}/{}", &String::from_utf8_lossy(comp.as_ref()), res);
      maybe_path = path.parent();
   }
   res = format!("\"//{}\"", res);

   res
}

pub fn bucket_index_to_path(bucket_index: u32) -> Path {
   let sec = bucket_index * BUCKET_SIZE_SEC + START_TIME;
   return get_path(sec);
}

/// Returns number of seconds since UNIX_EPOCH
pub fn now() -> u32 {
   let now = sys_time().expect("sys_time() should always work");
   now.as_seconds_and_nanos().0 as u32
}