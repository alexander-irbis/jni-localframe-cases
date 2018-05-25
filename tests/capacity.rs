extern crate jni;
extern crate jni_localframe_cases;
#[macro_use]
extern crate lazy_static;
extern crate rand;

use jni::JavaVM;
use jni::objects::JObject;
use jni_localframe_cases::create_vm;
use rand::prelude::*;

lazy_static! {
    static ref JVM: JavaVM = create_vm(true);
}

#[test]
fn frame_overflow() {
    const FRAME_SIZE: i32 = 0;
    const TRIES: i32 = 4096;

    let env = JVM.attach_current_thread().unwrap();
    env.with_local_frame(FRAME_SIZE, || {
        let mut strings = Vec::new();
        for i in 1..=TRIES {
            print!("Try: {}; limit: {}. ", i, FRAME_SIZE);
            let java_string = env.new_string(format!("{}", i)).expect(
                "Can't create new local object.",
            );
            strings.push(java_string);
            println!(" Ok.");
        }
        for (i, java_string) in strings.into_iter().enumerate() {
            let java_string: String = env.get_string(java_string)
                .expect("Can't get object.")
                .into();
            let number_string = format!("{}", i + 1);
            assert_eq!(java_string, number_string);
        };
        Ok(JObject::null())
    }).unwrap();
}

#[test]
fn memory_overflow() {
    // 12 * 256Mb = 3Gb.
    // -Xmx sets the heap limit as 1Gb
    const ITER_NUM: usize = 12;
    const ARRAY_NUM: usize = 256;
    const ARRAY_SIZE: usize = 1024 * 1024;
    const FRAME_SIZE: i32 = 0;

    let mut big_array = vec![0_u8; ARRAY_SIZE];

    let env = JVM.attach_current_thread().unwrap();
    for n in 0..ITER_NUM {
        thread_rng().fill(&mut big_array[..]);
        env.with_local_frame(FRAME_SIZE, || {
            for _ in 1..=ARRAY_NUM {
                let _java_obj = env.byte_array_from_slice(&big_array).expect(
                    "Can't create new local object.",
                );
            }
            Ok(JObject::null())
        }).unwrap();
        println!("{}", n);
    }
}
