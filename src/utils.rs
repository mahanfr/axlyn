pub fn string_to_bool(string:&str) -> bool {
    if string == "true"{
        true
    }else if string == "false"{
        false
    }else{
        panic!("Not Parsable Authentication State")
    }
}