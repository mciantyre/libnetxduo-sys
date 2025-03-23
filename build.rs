use std::{
    collections::{HashMap, HashSet},
    env, fs,
    path::{Path, PathBuf},
};

type Error = Box<dyn std::error::Error>;

static BINARY_CFGS: &[&'static str] = &[
    "nx_enable_interface_capability",
    "nx_driver_deferred_processing",
    // dhcp-client
    "nx_dhcp_client_user_create_packet_pool",
];

fn cfgs_to_defines() -> HashMap<String, String> {
    let cfgs = BINARY_CFGS.iter();

    let cargo_env_vars = cfgs
        .clone()
        .map(|cfg| format!("CARGO_CFG_{}", cfg.to_uppercase()));
    let defines = cfgs.map(|cfg| cfg.to_uppercase());
    cargo_env_vars.zip(defines).collect()
}

fn set_cfg_defines(bld: &mut cc::Build) {
    let cfgs_to_defines = cfgs_to_defines();
    for (cfg, _) in env::vars() {
        if let Some(define) = cfgs_to_defines.get(&cfg) {
            bld.define(define, None);
        }
    }
}

fn main() -> Result<(), Error> {
    if env::var("DOCS_RS").is_ok() {
        println!("cargo::warning=Hello docs.rs! Skipping NetX Duo (cross) compile");
        return Ok(());
    }

    let port = env::var("DEP_THREADX_PORT")?;
    let threadx_common_inc = env::var("DEP_THREADX_COMMON_INCLUDE")?;
    let threadx_port_inc = env::var("DEP_THREADX_PORT_INCLUDE")?;

    let mut bld = cc::Build::new();
    set_cfg_defines(&mut bld);
    bld.include(threadx_common_inc);
    bld.include(threadx_port_inc);

    if env::var("CARGO_FEATURE_FILEX").is_ok() {
        let filex_common_inc = env::var("DEP_FILEX_COMMON_INCLUDE")?;
        let filex_port_inc = env::var("DEP_FILEX_PORT_INCLUDE")?;
        bld.include(filex_common_inc);
        bld.include(filex_port_inc);
    }

    bld.include("netxduo/common/inc");
    for src in fs::read_dir("netxduo/common/src")? {
        bld.file(src?.path());
    }

    let port_path = Path::new("netxduo/ports").join(port);
    let port_inc = port_path.join("gnu/inc");
    bld.include(&port_inc);

    let out = PathBuf::from(env::var("OUT_DIR")?);
    let common = out.join("common");
    fs::create_dir_all(&common)?;

    for inc in fs::read_dir("netxduo/common/inc")? {
        let from = inc?.path();
        let to = common.join(from.file_name().unwrap());
        fs::copy(&from, &to)?;
    }
    for addon_header in build_addons(&mut bld)? {
        let to = common.join(addon_header.file_name().unwrap());
        fs::copy(&addon_header, &to)?;
    }

    let ports = out.join("ports");
    fs::create_dir_all(&ports)?;

    fs::copy(port_inc.join("nx_port.h"), ports.join("nx_port.h"))?;

    println!("cargo::metadata=common_include={}", common.display());
    println!("cargo::metadata=port_include={}", ports.display());

    bld.compile("netxduo");
    Ok(())
}

struct Addon {
    includes: Vec<PathBuf>,
    sources: Vec<PathBuf>,
}

fn dhcp_client(addons: &Path) -> Addon {
    let dhcp = addons.join("dhcp");
    Addon {
        includes: vec![dhcp.clone()],
        sources: vec![dhcp.join("nxd_dhcp_client.c")],
    }
}

fn web(addons: &Path) -> Addon {
    let web = addons.join("web");
    Addon {
        includes: vec![web.clone()],
        sources: vec![
            web.join("nx_tcpserver.c"),
            web.join("nx_web_http_client.c"),
            web.join("nx_web_http_server.c"),
        ],
    }
}

fn features_enabled() -> HashSet<String> {
    env::vars()
        .map(|(key, _)| key)
        .flat_map(|feature| {
            feature
                .strip_prefix("CARGO_FEATURE_")
                .map(str::to_lowercase)
        })
        .collect()
}

fn build_addons(bld: &mut cc::Build) -> Result<Vec<PathBuf>, Error> {
    // Keep in mind that a '-' in a feature name becomes a '_' in the
    // environment variable.
    let mut feature_addons: HashMap<_, fn(&Path) -> Addon> = HashMap::new();
    feature_addons.insert("dhcp_client", dhcp_client);
    feature_addons.insert("web", web);

    let addons = PathBuf::new().join("netxduo").join("addons");
    let mut includes = HashSet::new();

    for feature_enabled in features_enabled() {
        if let Some(make_addon) = feature_addons.get(feature_enabled.as_str()) {
            let addon = make_addon(&addons);

            includes.extend(addon.includes);
            bld.files(addon.sources);
        }
    }

    bld.includes(&includes);

    let mut headers = Vec::new();
    for include in includes {
        for file in fs::read_dir(include)? {
            let path = file?.path();
            if let Some(ext) = path.extension() {
                if ext.to_str().unwrap() == "h" {
                    headers.push(path)
                }
            }
        }
    }

    Ok(headers)
}
