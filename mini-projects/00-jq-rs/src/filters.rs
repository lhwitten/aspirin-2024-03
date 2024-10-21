use core::panic;

pub struct Filter {
    operation_name: String,
    contents_1: String,      //could be filter
    contents_1_type: String, //could be filter
    contents_2: String,
    contents_2_type: String,
}
//name:
//obj_identify -- .<KEY_NAME>
//array_index --  .[i32]
//array_slice -- .[<start_number>:<end_number>]
//pipe -- pass output of one filter to another
// -- handle in main loop?
//array_iterator -- turn an array into an iterator
// -- effectively expands an array into multiple objects

//add -- adds all values in an array
//length -- returns the length of the input
//del -- del(.[<indexes>]) -- return everything but deleted

fn apply_single_filter(object: String, to_apply: Filter) -> String {
    match to_apply.operation_name.as_str() {
        "obj_identify" => apply_identify(object, to_apply),
        "array_index" => apply_arr_index(object, to_apply),
        _ => panic!("bad operation name"),
    }

    todo!()
}

fn apply_identify(object: &str, to_apply: Filter) {}
