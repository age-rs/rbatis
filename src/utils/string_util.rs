use std::collections::{BTreeMap, HashSet};
use std::io::Read;

use serde_json::map::Map;
use serde_json::Value;

//2020-11-15 00:31:25.803227700 +08:00 INFO rbatis::plugin::log
pub const LOG_SPACE: &'static str = "                                                                ";

//find like #{*,*},${*,*} value *
pub fn find_convert_string(arg: &str) -> Vec<(String, String)> {
    let mut cache_set = HashSet::new();
    let mut results = BTreeMap::new();
    let chars: Vec<u8> = arg.bytes().collect();
    let mut item = String::new();
    let mut last_index: i32 = -1;
    let mut index: i32 = -1;
    for v in &chars {
        index = index + 1;
        if last_index == -1 && (*v == '#' as u8 || *v == '$' as u8) {
            let next = chars.get(index as usize + 1);
            let next_char = '{' as u8;
            if next.is_some() && next.unwrap().eq(&next_char) {
                last_index = index;
            }
            continue;
        }
        if *v == '}' as u8 && last_index != -1 {
            item = String::from_utf8(chars[(last_index + 2) as usize..index as usize].to_vec()).unwrap();
            if cache_set.get(&item).is_some() {
                item.clear();
                last_index = -1;
                continue;
            }
            let value = String::from_utf8(chars[last_index as usize..(index + 1) as usize].to_vec()).unwrap();
            results.insert(index, (item.clone(), value.clone()));
            cache_set.insert(item.clone());
            item.clear();
            last_index = -1;
        }
    }

    let mut array =vec![];
    for (_,(k,v)) in results {
        array.push((k,v))
    }
    return array;
}


pub fn count_string_num(s: &String, c: char) -> usize {
    let cs = s.chars();
    let mut num = 0;
    for x in cs {
        if x == c {
            num += 1;
        }
    }
    return num;
}


pub fn to_snake_name(name: &String) -> String {
    let chs = name.chars();
    let mut new_name = String::new();
    let mut index = 0;
    let chs_len = name.len();
    for x in chs {
        if x.is_uppercase() {
            if index != 0 && (index + 1) != chs_len {
                new_name.push_str("_");
            }
            new_name.push_str(x.to_lowercase().to_string().as_str());
        } else {
            new_name.push(x);
        }
        index += 1;
    }
    return new_name;
}

#[cfg(test)]
mod test {
    use crate::utils::string_util::find_convert_string;

    #[test]
    fn test_find() {
        let sql = "update user set name=#{name}, password=#{password} ,sex=#{sex}, phone=#{phone}, delete_flag=#{flag}, #{name}";
        let finds = find_convert_string(sql);
        assert_eq!(finds.len(), 5);
        let mut index = 0;
        for (k, v) in &finds {
            if index == 0 {
                assert_eq!(k, "name");
            }
            if index == 1 {
                assert_eq!(k, "password");
            }
            if index == 2 {
                assert_eq!(k, "sex");
            }
            if index == 3 {
                assert_eq!(k, "phone");
            }
            if index == 4 {
                assert_eq!(k, "flag");
            }
            index += 1;
        }
        println!("{:?}", finds);
    }

    #[test]
    fn test_find_fail() {
        let sql = "select #{column   #{  }";
        let finds = find_convert_string(sql);
        println!("{:?}", finds);
    }
}