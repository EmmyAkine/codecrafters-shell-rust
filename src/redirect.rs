pub struct RedirectSpec {
    pub stdout_file: Option<String>,
    pub stdout_append: bool,
    pub stderr_file: Option<String>,
    pub stderr_append: bool,
}