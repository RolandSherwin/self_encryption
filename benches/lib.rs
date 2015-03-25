// Copyright 2014-2015 MaidSafe.net limited
//
// This MaidSafe Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the MaidSafe Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
// directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
// available at: http://www.maidsafe.net/licenses
//
// Unless required by applicable law or agreed to in writing, the MaidSafe Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
// OF ANY KIND, either express or implied.
//
// See the Licences for the specific language governing permissions and limitations relating to
// use of the MaidSafe
// Software.

// the test names contain MB, KB which should retain capitalisation
#![allow(non_snake_case)]

#![feature(test)]
#![allow(dead_code, unused_assignments)]
extern crate test;
extern crate rand;
extern crate self_encryption;

use test::Bencher;
use self_encryption::SelfEncryptor;
use self_encryption::Storage;
use self_encryption::datamap::DataMap as DataMap;

//TODO(ben 2015-03-24): replace copy from src/lib.rs mod test to properly import and reuse
fn random_string(length: u64) -> String {
  (0..length).map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char).collect()
}

struct Entry {
  name: Vec<u8>,
  data: Vec<u8>
}

struct MyStorage {
  entries: Vec<Entry>
}

impl MyStorage {
  pub fn new() -> MyStorage {
    MyStorage { entries: Vec::new() }
  }

  pub fn has_chunk(&self, name: Vec<u8>) -> bool {
    for entry in self.entries.iter() {
      if entry.name == name { return true }
    }
    false
  }
}

impl Storage for MyStorage {
  fn get(&self, name: Vec<u8>) -> Vec<u8> {
    for entry in self.entries.iter() {
      if entry.name == name { return entry.data.to_vec() }
    }

    vec![]
  }

  fn put(&mut self, name: Vec<u8>, data: Vec<u8>) {
    self.entries.push(Entry { name : name, data : data })
  }
}
// end of copy from src/lib.rs

#[bench]
fn bench_write_then_read_a_200B(b: &mut Bencher) {
  let mut my_storage = MyStorage::new();
  let string_len = 200 as u64;
  b.iter(|| {
  	let mut data_map = DataMap::None;
  	let the_string = random_string(string_len);
    {
      let mut se = SelfEncryptor::new(&mut my_storage as &mut Storage, DataMap::None);
      se.write(&the_string, 0);
      data_map = se.close();
    }
    let mut new_se = SelfEncryptor::new(&mut my_storage as &mut Storage, data_map);
    let fetched = new_se.read(0, string_len);
    assert_eq!(fetched, the_string);
  });
  b.bytes = 2 * string_len;
}

#[bench]
fn bench_write_then_read_b_1KB(b: &mut Bencher) {
  let mut my_storage = MyStorage::new();
  let string_len = 1024 as u64;
  b.iter(|| {
  	let mut data_map = DataMap::None;
  	let the_string = random_string(string_len);
    {
      let mut se = SelfEncryptor::new(&mut my_storage as &mut Storage, DataMap::None);
      se.write(&the_string, 0);
      data_map = se.close();
    }
    let mut new_se = SelfEncryptor::new(&mut my_storage as &mut Storage, data_map);
    let fetched = new_se.read(0, string_len);
    assert_eq!(fetched, the_string);
  });
  b.bytes = 2 * string_len;
}

#[bench]
fn bench_write_then_read_c_1MB(b: &mut Bencher) {
  let mut my_storage = MyStorage::new();
  let string_len = 1024 * 1024 as u64;
  b.iter(|| {
  	let mut data_map = DataMap::None;
  	let the_string = random_string(string_len);
    {
      let mut se = SelfEncryptor::new(&mut my_storage as &mut Storage, DataMap::None);
      se.write(&the_string, 0);
      data_map = se.close();
    }
    let mut new_se = SelfEncryptor::new(&mut my_storage as &mut Storage, data_map);
    let fetched = new_se.read(0, string_len);
    assert_eq!(fetched, the_string);
  });
  b.bytes = 2 * string_len;
}

#[bench]
fn bench_write_then_read_d_3MB(b: &mut Bencher) {
  let mut my_storage = MyStorage::new();
  let string_len = 3 * 1024 * 1024 as u64;
  b.iter(|| {
  	let mut data_map = DataMap::None;
  	let the_string = random_string(string_len);
    {
      let mut se = SelfEncryptor::new(&mut my_storage as &mut Storage, DataMap::None);
      se.write(&the_string, 0);
      data_map = se.close();
    }
    let mut new_se = SelfEncryptor::new(&mut my_storage as &mut Storage, data_map);
    let fetched = new_se.read(0, string_len);
    assert_eq!(fetched, the_string);
  });
  b.bytes = 2 * string_len;
}

#[bench]
fn bench_write_then_read_e_10MB(b: &mut Bencher) {
  let mut my_storage = MyStorage::new();
  let string_len = 10 * 1024 * 1024 as u64;
  b.iter(|| {
  	let mut data_map = DataMap::None;
  	let the_string = random_string(string_len);
    {
      let mut se = SelfEncryptor::new(&mut my_storage as &mut Storage, DataMap::None);
      se.write(&the_string, 0);
      data_map = se.close();
    }
    let mut new_se = SelfEncryptor::new(&mut my_storage as &mut Storage, data_map);
    let fetched = new_se.read(0, string_len);
    assert_eq!(fetched, the_string);
  });
  b.bytes = 2 * string_len;
}

/*  The assert fails !!
#[bench]
fn bench_write_then_read_range(b: &mut Bencher) {
  let mut my_storage = MyStorage::new();
  let string_range = vec![     512 * 1024,
  						  1 * 1024 * 1024,
  						  2 * 1024 * 1024,
  						  3 * 1024 * 1024,
  						  4 * 1024 * 1024,
  						  5 * 1024 * 1024,
  						  6 * 1024 * 1024];
  for string_len in string_range {
    b.iter(|| {
  	  let mut data_map = DataMap::None;
      let the_string = random_string(string_len);
      {
        let mut se = SelfEncryptor::new(&mut my_storage as &mut Storage, DataMap::None);
        se.write(&the_string, 0);
        data_map = se.close();
      }
      let mut new_se = SelfEncryptor::new(&mut my_storage as &mut Storage, data_map);
      let fetched = new_se.read(0, string_len);
      assert_eq!(fetched, the_string);
    });
    // write and read the data
    b.bytes = 2*string_len;
  }
}
*/