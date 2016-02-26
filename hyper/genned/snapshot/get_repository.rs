//Autogenerated
pub fn get(base: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 10);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_snapshot");
    url_fmtd
}
pub fn get_repository(base: String, repository: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 11 + repository.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_snapshot/");
    url_fmtd.push_str(&repository);
    url_fmtd
}
