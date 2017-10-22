extern crate serde_json;
extern crate dumplingh;

use std::io::stderr;
use std::process::exit;
use self::dumplingh::{ops, Options, Error};


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    if let Err(err) = result_main() {
        err.print_error(&mut stderr());
        err.exit_value()
    } else {
        0
    }
}

fn result_main() -> Result<(), Error> {
    let opts = Options::parse();

    if let Some((_, out_issues)) = opts.out_issues {
        ops::save_to_file(out_issues,
                          &ops::list_issues(&opts.slug, opts.github_token.as_ref().map(String::as_str))?,
                          opts.compact,
                          "issues")?;
    }

    if let Some((_, out_pull_requests)) = opts.out_pull_requests {
        ops::save_to_file(out_pull_requests,
                          &ops::list_pull_requests(&opts.slug, opts.github_token.as_ref().map(String::as_str))?,
                          opts.compact,
                          "pull requests")?;
    }

    if let Some((_, out_labels)) = opts.out_labels {
        ops::save_to_file(out_labels,
                          &ops::list_labels(&opts.slug, opts.github_token.as_ref().map(String::as_str))?,
                          opts.compact,
                          "labels")?;
    }

    if let Some((_, out_milestones)) = opts.out_milestones {
        ops::save_to_file(out_milestones,
                          &ops::list_milestones(&opts.slug, opts.github_token.as_ref().map(String::as_str))?,
                          opts.compact,
                          "milestones")?;
    }

    if let Some((_, out_projects)) = opts.out_projects {
        ops::save_to_file(out_projects,
                          &ops::list_projects(&opts.slug, &opts.github_token.as_ref().unwrap())?,
                          opts.compact,
                          "projects")?;
    }

    Ok(())
}
