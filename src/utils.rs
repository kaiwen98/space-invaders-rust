use project_root;

pub fn get_project_root_str() -> String {
    project_root::get_project_root().unwrap().into_os_string().into_string().unwrap()
}
