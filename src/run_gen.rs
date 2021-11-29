
use std::collections::HashMap;

use crate::generate_js::generate_js;
use crate::utils::{get_file_name, ErrorProcess};
use crate::{open_api_spec::SpecOpenApi};
use crate::read_wanted_spec::{FixUrlParamItem, WantedMethod, WantedSource, WantedSpec, read_wanted_spec};
use crate::parse_spec::{fix_url_param, parse_spec};

pub async fn run_gen(dir_spec: String, dir_target: String, base_url: String, target_spec: String) -> Result<(), ErrorProcess> {
    let mut wanted_spec = read_wanted_spec(&dir_spec)?;

    if target_spec != "all" {
        wanted_spec.retain(|key, _| {
            *key == target_spec
        });
    }

    for (prefix, WantedSpec { source, mut methods }) in wanted_spec {
        let (mut spec, fix_url_param_opt) = get_spec(&dir_spec, &base_url, source).await?;

        if let Some(fix_url_param_list) = fix_url_param_opt {
            for FixUrlParamItem { from, to } in fix_url_param_list.into_iter() {
                fix_url_param(&mut spec, &mut methods, from, to)
            }
        }

        remove_files_from_prefix(&dir_target, &prefix).await?;
        run_gen_for_prefix(&dir_target, prefix, spec, methods).await?;
    }

    Ok(())
}

async fn get_spec(dir_spec: &String, base_url: &String, source: WantedSource) -> Result<(SpecOpenApi, Option<Vec<FixUrlParamItem>>), ErrorProcess> {
    let (spec_source, fix_url_param) = match source {
        WantedSource::File { file, fix_url_param } => {
            let file_full = format!("{dir_spec}{file}");

            log::info!("read file with spec: {file_full}");

            let content = tokio::fs::read_to_string(&file_full).await.map_err({
                let file_full = file_full.clone();
                move |err| {
                    ErrorProcess::message(format!("error read content {file_full} -> {err}"))
                }
            })?;

            (content, fix_url_param)
        },
        WantedSource::Url { url, fix_url_param } => {
            let url_full = format!("{base_url}{url}");

            log::info!("read url with spec: {url_full}");

            let response = reqwest::get(&url_full).await.map_err({
                let url_full = url_full.clone();
                move |err| {
                    ErrorProcess::message(format!("error fetch {url_full} -> {err}"))
                }
            })?;

            let text = response.text().await.map_err(move |err| {
                ErrorProcess::message(format!("error fetch(text) {url_full} -> {err}"))
            })?;

            (text, fix_url_param)
        }
    };

    let spec = parse_spec(spec_source)?;
    Ok((spec, fix_url_param))
}

async fn remove_files_from_prefix(dir_target: &String, prefix: &String) -> Result<(), ErrorProcess> {
    let start_with = format!("openapi_{prefix}");
    let mut read_dir = tokio::fs::read_dir(dir_target).await?;

    while let Ok(Some(entry)) = read_dir.next_entry().await {
        let path = entry.path();

        let file_name = get_file_name(&path)?;

        if file_name.starts_with(&start_with) {
            if path.is_file() {
                log::info!("Deleting the old file {path:#?}");
                tokio::fs::remove_file(path).await?;
            } else {
                return Err(ErrorProcess::message(format!("The file was expected -> {file_name}")));
            }
        }
    }

    Ok(())
}

async fn run_gen_for_prefix(dir_target: &String, prefix: String, spec: SpecOpenApi, methods: HashMap<String, WantedMethod>) -> Result<(), ErrorProcess> {

    for (method_name, WantedMethod { url, method }) in methods {
        let name_in_file = format!("openapi_{prefix}_{method_name}");
        let target_path = format!("{dir_target}/{name_in_file}.ts");

        let sub_sepc = match spec.paths.get(&url) {
            Some(sub_sepc ) => sub_sepc,
            None => {
                let url = &url;
                return Err(ErrorProcess::message(format!("No path in the specification {url}")));
            }
        };

        let handler = match sub_sepc.get(&method) {
            Some(handler) => handler,
            None => {
                let url = &url;
                let method = &method;
                return Err(ErrorProcess::message(format!("No method in the specification {url} {method:?}")));
            }
        };

        let content_js = generate_js(name_in_file, url, method, handler)?;
        log::info!("generate_js writh to: {target_path}");
        tokio::fs::write(target_path, content_js).await?;
    }

    Ok(())
}
