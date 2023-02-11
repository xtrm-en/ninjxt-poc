// use std::ffi::c_void;
// use std::io::Write;
// use std::ptr;
// use std::ptr::{null_mut};
// use jvm_rs::jni::{JavaVM, jint, JNI_GetCreatedJavaVMs, JNI_OK, JNI_VERSION_1_2, JNIEnv, jsize};

#[ctor::ctor]
unsafe fn init() {
    println!("> Hello from payload gaming!");

    /*
    let mut count: jsize = 0;
    let check = JNI_GetCreatedJavaVMs(null_mut(), 0, &mut count);
    assert_eq!(check, JNI_OK as i32);
    let mut vms = vec![null_mut(); count as usize];
    let check = JNI_GetCreatedJavaVMs(vms.as_mut_ptr(), vms.len() as i32, &mut count);
    assert_eq!(check, JNI_OK as i32);
    assert_eq!(vms.len(), count as usize);

    println!("Found {} JVMs", count);
    for vm in vms {
        println!("Found JVM: {:?}", vm);
        inject_into(&mut *vm);
    }*/
}
/*
unsafe fn inject_into(vm: &mut JavaVM) {
    println!("> Injecting into JVM: {:?}", vm);
    let vmi = **vm;

    let mut jni_env: *mut c_void = null_mut();
    if vmi.GetEnv.unwrap()(vm, &mut jni_env, JNI_VERSION_1_2 as jint) != JNI_OK as i32 {
        println!("> Failed to get JNI environment");
        return;
    }

}*/