use std::{path::Path, fs};

fn main() {
  let main_path = Path::new(".");
  let template_folder = &main_path.join("template");
  let copy_main_from = &template_folder.join("main.rs");
  let copy_input_from = &template_folder.join("input.txt");
  for i in 1..26 {

      let folder_name = format!("d{:0>2}", i);
      println!("{}", &folder_name);
      let sub_project_folder = main_path.join(&folder_name);
      if Path::exists(&sub_project_folder) {
        continue;
      }
      
      let input_folder = sub_project_folder.join("input");
      fs::create_dir_all(&input_folder).unwrap();
      let copy_main_to = &sub_project_folder.join("src").join("main.rs");
      let copy_input_to = &input_folder.join("input.txt");
      let copy_input_test_to = &input_folder.join("input_test.txt");


      fs::copy(copy_main_from, copy_main_to).unwrap();
      fs::copy(copy_input_from, copy_input_to).unwrap();
      fs::copy(copy_input_from, copy_input_test_to).unwrap();
  }
}
