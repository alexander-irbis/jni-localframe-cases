extern crate jni;

use jni::{InitArgsBuilder, JavaVM, JNIVersion};
use jni::objects::JObject;

const N: i32 = 0;
const R: i32 = 4096;

/// Creates a configured instance of `JavaVM`.
/// _This function should be called only *once*._
pub fn create_vm(debug: bool) -> JavaVM {
    let mut jvm_args_builder = InitArgsBuilder::new().version(JNIVersion::V8);

    if debug {
        jvm_args_builder = jvm_args_builder.option("-Xcheck:jni").option("-Xdebug");
    }

    let jvm_args = jvm_args_builder.build().unwrap_or_else(
        |e| panic!(format!("{:#?}", e)),
    );

    JavaVM::new(jvm_args).unwrap_or_else(|e| panic!(format!("{:#?}", e)))
}

fn main() {
    println!("Checking capacity overflow of LocalFrame.");

    let jvm = create_vm(true);

    let env = jvm.attach_current_thread().unwrap();
    env.with_local_frame(N, || {
        let mut strings = Vec::new();
        for i in 1..=R {
            print!("Try: {}; limit: {}. ", i, N);
            let java_string = env.new_string(format!("{}", i)).expect(
                "Can't create new local object.",
            );
            strings.push(java_string);
            println!(" Ok.");
        }
        let strings = strings
            .into_iter()
            .map(|java_string| {
                env.get_string(java_string)
                    .expect("Can't get object.")
                    .into()
            })
            .collect::<Vec<String>>()
            .join(", ");
        println!("Java strings: [{}]", strings);
        Ok(JObject::null())
    }).unwrap();

    println!("Normal exit.");
}
