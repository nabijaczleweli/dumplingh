dumplingh(1) -- Issue/PR exporter for GitHub
============================================

## SYNOPSIS

`dumplingh` [OPTIONS] <REPO_SLUG>

## DESCRIPTION

Issue/PR exporter for GitHub, which is a fancy name for dump issue and
PR data to a JSON file.

Exit values and possible errors:

    1 - I/O error
    2 - parsing error

## OPTIONS

  <REPO_SLUG>

    Repository to export issues and PRs for in the form
    "<username>/<repo>".

  -i --issues <ISSUES_FILE>

    File to export issues to.

    File must either not exist or --force must be specified.

    Exclusive with --no-issues.

    Default: `./<REPO_SLUG>-issues.json`.

  -p --pulls <PULLS_FILE>

    File to export pull requests to.

    File must either not exist or --force must be specified.

    Exclusive with --no-pulls.

    Default: ./<REPO_SLUG>-pulls.json.

  --no-issues

    Don't export issues.

  --no-pulls

    Don't export pull requests.

  -f --force

    Allow for overriding <ISSUES_FILE> and <PULLS_FILE>.

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

Instigated by Vendethiel &lt;<https://github.com/vendethiel>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/dumplingh/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/dumplingh>&gt;
