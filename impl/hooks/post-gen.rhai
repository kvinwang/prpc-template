let app_name = variable::get("app_name");
file::rename("app.toml", `${app_name}.toml`);
file::delete("hooks/")
