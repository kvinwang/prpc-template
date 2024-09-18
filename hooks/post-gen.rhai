let service_name = variable::get("service_name");
file::rename("proto/rpc.proto", `proto/${service_name}_rpc.proto`);
file::delete("hooks/")
