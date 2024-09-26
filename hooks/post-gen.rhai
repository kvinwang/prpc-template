let app_name = variable::get("app_name");
file::rename("app.toml", `${app_name}.toml`);
file::rename("rpc/proto/rpc.proto", `rpc/proto/${app_name}_rpc.proto`);
file::delete("hooks/")
