//Autogenerated
pub fn delete_id(base: String, id: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 18 + id.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_search/template/");
    url_fmtd.push_str(&id);
    url_fmtd
}
