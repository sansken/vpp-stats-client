use vpp_stat_client::*;

macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0") as *const str as *const [i8] as *const i8
    };
}

macro_rules! cstr_mut {
    ($s:expr) => {
        concat!($s, "\0") as *const str as *mut [i8] as *mut i8
    };
}

macro_rules! ucstr {
    ($s:expr) => {
        concat!($s, "\0") as *const str as *const [u8] as *const u8
    };
}

macro_rules! ucstr_mut {
    ($s:expr) => {
        concat!($s, "\0") as *const str as *mut [u8] as *mut u8
    };
}

use libc::c_char;
use std::ffi::CStr;
use std::str;

#[no_mangle]
fn check(sc: *mut stat_client_main_t, ptr: *mut u32, length: usize) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    unsafe {
        let buf: &mut [u32] = core::slice::from_raw_parts_mut(ptr, length);
        for i in 0..length {
            let name = stat_segment_index_to_name_r(buf[i], sc);
            let c_str: &CStr = unsafe { CStr::from_ptr(name) };
            let str_slice: &str = c_str.to_str().unwrap();
            let str_buf: String = str_slice.to_owned(); // if necessary
            out.push(str_buf);
        }
    }
    out
}

fn do_dump(
    sc: *mut stat_client_main_t,
    ptr: *mut stat_segment_data_t,
    length: usize,
) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    println!("test");
    unsafe {
        let buf: &mut [stat_segment_data_t] = core::slice::from_raw_parts_mut(ptr, length);
        for i in 0..length {
            let c_str: &CStr = unsafe { CStr::from_ptr(buf[i].name) };
            let str_slice: &str = c_str.to_str().unwrap();
            let str_buf: String = str_slice.to_owned(); // if necessary

            print!("Name: {} type: ", str_buf);
            match buf[i].type_ {
                stat_directory_type_t_STAT_DIR_TYPE_ILLEGAL => {
                    unimplemented!()
                }
                stat_directory_type_t_STAT_DIR_TYPE_SCALAR_INDEX => {
                    println!(
                        "SCALAR_INDEX : value {}",
                        buf[i].__bindgen_anon_1.scalar_value
                    );
                }
                stat_directory_type_t_STAT_DIR_TYPE_COUNTER_VECTOR_SIMPLE => {
                    println!("COUNTER_VECTOR_SIMPLE");
                    let vvv = buf[i].__bindgen_anon_1.simple_counter_vec;
                    let vvv_len = stat_segment_vec_len(vvv as *mut libc::c_void) as usize;
                    let vc: &[*mut u64] = core::slice::from_raw_parts_mut(vvv, vvv_len);

                    for k in 0..vvv_len {
                        let vvvj = vc[k];
                        let vvvj_len = stat_segment_vec_len(vvvj as *mut libc::c_void) as usize;
                        let vcj: &[u64] = core::slice::from_raw_parts_mut(vvvj, vvvj_len);

                        for j in 0..vvvj_len {
                            println!("     [ {} @ {} ]: {} packets", j, k, vcj[j]);
                        }
                    }
                }
                stat_directory_type_t_STAT_DIR_TYPE_COUNTER_VECTOR_COMBINED => {
                    println!("COUNTER_VECTOR_COMBINED");
                    let vvv = buf[i].__bindgen_anon_1.combined_counter_vec;
                    let vvv_len = stat_segment_vec_len(vvv as *mut libc::c_void) as usize;
                    let vc: &[*mut vlib_counter_t] = core::slice::from_raw_parts_mut(vvv, vvv_len);

                    for k in 0..vvv_len {
                        let vvvj = vc[k];
                        let vvvj_len = stat_segment_vec_len(vvvj as *mut libc::c_void) as usize;
                        let vcj: &[vlib_counter_t] =
                            core::slice::from_raw_parts_mut(vvvj, vvvj_len);

                        for j in 0..vvvj_len {
                            println!(
                                "     [ {} @ {} ]: {} packets, {} bytes",
                                j, k, vcj[j].packets, vcj[j].bytes
                            );
                        }
                    }
                }
                stat_directory_type_t_STAT_DIR_TYPE_NAME_VECTOR => {
                    println!("NAME_VECTOR");
                    let vvv = buf[i].__bindgen_anon_1.name_vector as *mut *const i8;
                    let vvv_len = stat_segment_vec_len(vvv as *mut libc::c_void) as usize;
                    let vc: &mut [*const i8] = core::slice::from_raw_parts_mut(vvv, vvv_len);

                    for k in 0..vvv_len {
                        let c_str: &CStr = unsafe { CStr::from_ptr(vc[k]) };
                        let str_slice: &str = c_str.to_str().unwrap();
                        println!("[{}]: {}", k, str_slice);
                    }
                }
                stat_directory_type_t_STAT_DIR_TYPE_EMPTY => {
                    println!("EMPTY");
                }
                stat_directory_type_t_STAT_DIR_TYPE_SYMLINK => {
                    println!("SYMLINK");
                }
                7_u32..=u32::MAX => unimplemented!(),
            }
        }
    }
    out
}

fn main() {
    unsafe {
        let data = [0u8; 128];

        clib_mem_init(std::ptr::null_mut(), 64000000);

        let sc = stat_client_get();
        let rv = stat_segment_connect_r(cstr!("/tmp/stats.sock"), sc);
        println!("Hello world! {}", rv);
        println!("running dir");
        let patterns = std::ptr::null_mut();
        let dir = stat_segment_ls_r(patterns, sc);

        let str_buf = check(
            sc,
            dir,
            stat_segment_vec_len(dir as *mut libc::c_void) as usize,
        );

        // println!("{:?}", str_buf);
        /*
        for s in str_buf {
            println!("{}", s);
        }
        */

        println!("running dump");

        let res = stat_segment_dump_r(dir, sc);

        do_dump(
            sc,
            res,
            stat_segment_vec_len(res as *mut libc::c_void) as usize,
        );

        stat_segment_data_free(res);

        stat_segment_disconnect_r(sc);
        stat_client_free(sc);
    }
}
