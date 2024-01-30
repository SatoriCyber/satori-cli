#compdef satori

autoload -U is-at-least

_satori() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_satori_commands" \
"*::: :->satori" \
&& ret=0
    case $state in
    (satori)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:satori-command-$line[1]:"
        case $line[1] in
            (run)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
":: :_satori__run_commands" \
"*::: :->run" \
&& ret=0

    case $state in
    (run)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:satori-run-command-$line[1]:"
        case $line[1] in
            (dbt)
_arguments "${_arguments_options[@]}" \
'--profile-dir=[The path to the dbt profiles directory]: :_files' \
'--target=[DBT target]:PROFILE: ' \
'--domain=[INTERNAL Default to https\://app.satoricyber.com]:VALUE: ' \
'--no-launch-browser[Don'\''t launch the browser]' \
'--invalid_cert[INTERNAL disable SSL verification]' \
'--debug[Enable debug mode]' \
'--refresh[refresh the local cache files]' \
'--no-persist[Don'\''t persist the database credentials]' \
'-h[Print help]' \
'--help[Print help]' \
'::additional_args:' \
&& ret=0
;;
(psql)
_arguments "${_arguments_options[@]}" \
'--domain=[INTERNAL Default to https\://app.satoricyber.com]:VALUE: ' \
'--no-launch-browser[Don'\''t launch the browser]' \
'--invalid_cert[INTERNAL disable SSL verification]' \
'--debug[Enable debug mode]' \
'--refresh[refresh the local cache files]' \
'--no-persist[Don'\''t persist the database credentials]' \
'-h[Print help]' \
'--help[Print help]' \
':datastore_name -- The name as defined in Satori data portal:_datastores' \
':database -- Database name:_databases' \
'::additional_args:' \
&& ret=0
;;
(mongosh)
_arguments "${_arguments_options[@]}" \
'--domain=[INTERNAL Default to https\://app.satoricyber.com]:VALUE: ' \
'--no-launch-browser[Don'\''t launch the browser]' \
'--invalid_cert[INTERNAL disable SSL verification]' \
'--debug[Enable debug mode]' \
'--refresh[refresh the local cache files]' \
'--no-persist[Don'\''t persist the database credentials]' \
'-h[Print help]' \
'--help[Print help]' \
':datastore_name -- The name as defined in Satori data portal:_datastores' \
'::additional_args:' \
&& ret=0
;;
(s3)
_arguments "${_arguments_options[@]}" \
'--domain=[INTERNAL Default to https\://app.satoricyber.com]:VALUE: ' \
'--no-launch-browser[Don'\''t launch the browser]' \
'--invalid_cert[INTERNAL disable SSL verification]' \
'--debug[Enable debug mode]' \
'--refresh[refresh the local cache files]' \
'--no-persist[Don'\''t persist the database credentials]' \
'-h[Print help]' \
'--help[Print help]' \
':datastore_name -- The name as defined in Satori data portal:_datastores' \
'::additional_args:' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_satori__run__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:satori-run-help-command-$line[1]:"
        case $line[1] in
            (dbt)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(psql)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(mongosh)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(s3)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(login)
_arguments "${_arguments_options[@]}" \
'-f+[]:FORMAT:(json yaml csv)' \
'--format=[]:FORMAT:(json yaml csv)' \
'--domain=[INTERNAL Default to https\://app.satoricyber.com]:VALUE: ' \
'-d[Display the credentials or save to file]' \
'--display[Display the credentials or save to file]' \
'--no-launch-browser[Don'\''t launch the browser]' \
'--invalid_cert[INTERNAL disable SSL verification]' \
'--debug[Enable debug mode]' \
'--refresh[refresh the local cache files]' \
'--no-persist[Don'\''t persist the database credentials]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(auto_complete)
_arguments "${_arguments_options[@]}" \
'--generate=[Generate completion file]:VALUE:(bash elvish fish powershell zsh)' \
'--out=[Output file]:File:_files' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" \
'--databases=[List of databases for the datastore]:datastore_name: ' \
'--datastores[Get all available datastores]' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
&& ret=0
;;
(pgpass)
_arguments "${_arguments_options[@]}" \
'--domain=[INTERNAL Default to https\://app.satoricyber.com]:VALUE: ' \
'--no-launch-browser[Don'\''t launch the browser]' \
'--invalid_cert[INTERNAL disable SSL verification]' \
'--debug[Enable debug mode]' \
'--refresh[refresh the local cache files]' \
'--no-persist[Don'\''t persist the database credentials]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(aws)
_arguments "${_arguments_options[@]}" \
'--domain=[INTERNAL Default to https\://app.satoricyber.com]:VALUE: ' \
'--no-launch-browser[Don'\''t launch the browser]' \
'--invalid_cert[INTERNAL disable SSL verification]' \
'--debug[Enable debug mode]' \
'--refresh[refresh the local cache files]' \
'--no-persist[Don'\''t persist the database credentials]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_satori__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:satori-help-command-$line[1]:"
        case $line[1] in
            (run)
_arguments "${_arguments_options[@]}" \
":: :_satori__help__run_commands" \
"*::: :->run" \
&& ret=0

    case $state in
    (run)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:satori-help-run-command-$line[1]:"
        case $line[1] in
            (dbt)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(psql)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(mongosh)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(s3)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
(login)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(auto_complete)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(pgpass)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(aws)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_satori_commands] )) ||
_satori_commands() {
    local commands; commands=(
'run:Execute to a tool' \
'login:Login to Satori' \
'auto_complete:Generate autocomplete' \
'list:List resources' \
'pgpass:Creates a Pgpass file to be used by other apps' \
'aws:Creates a aws profile to be used with s3' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'satori commands' commands "$@"
}
(( $+functions[_satori__auto_complete_commands] )) ||
_satori__auto_complete_commands() {
    local commands; commands=()
    _describe -t commands 'satori auto_complete commands' commands "$@"
}
(( $+functions[_satori__help__auto_complete_commands] )) ||
_satori__help__auto_complete_commands() {
    local commands; commands=()
    _describe -t commands 'satori help auto_complete commands' commands "$@"
}
(( $+functions[_satori__aws_commands] )) ||
_satori__aws_commands() {
    local commands; commands=()
    _describe -t commands 'satori aws commands' commands "$@"
}
(( $+functions[_satori__help__aws_commands] )) ||
_satori__help__aws_commands() {
    local commands; commands=()
    _describe -t commands 'satori help aws commands' commands "$@"
}
(( $+functions[_satori__help__run__dbt_commands] )) ||
_satori__help__run__dbt_commands() {
    local commands; commands=()
    _describe -t commands 'satori help run dbt commands' commands "$@"
}
(( $+functions[_satori__run__dbt_commands] )) ||
_satori__run__dbt_commands() {
    local commands; commands=()
    _describe -t commands 'satori run dbt commands' commands "$@"
}
(( $+functions[_satori__run__help__dbt_commands] )) ||
_satori__run__help__dbt_commands() {
    local commands; commands=()
    _describe -t commands 'satori run help dbt commands' commands "$@"
}
(( $+functions[_satori__help_commands] )) ||
_satori__help_commands() {
    local commands; commands=(
'run:Execute to a tool' \
'login:Login to Satori' \
'auto_complete:Generate autocomplete' \
'list:List resources' \
'pgpass:Creates a Pgpass file to be used by other apps' \
'aws:Creates a aws profile to be used with s3' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'satori help commands' commands "$@"
}
(( $+functions[_satori__help__help_commands] )) ||
_satori__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'satori help help commands' commands "$@"
}
(( $+functions[_satori__run__help_commands] )) ||
_satori__run__help_commands() {
    local commands; commands=(
'dbt:dbt' \
'psql:psql' \
'mongosh:mongosh' \
's3:s3' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'satori run help commands' commands "$@"
}
(( $+functions[_satori__run__help__help_commands] )) ||
_satori__run__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'satori run help help commands' commands "$@"
}
(( $+functions[_satori__help__list_commands] )) ||
_satori__help__list_commands() {
    local commands; commands=()
    _describe -t commands 'satori help list commands' commands "$@"
}
(( $+functions[_satori__list_commands] )) ||
_satori__list_commands() {
    local commands; commands=()
    _describe -t commands 'satori list commands' commands "$@"
}
(( $+functions[_satori__help__login_commands] )) ||
_satori__help__login_commands() {
    local commands; commands=()
    _describe -t commands 'satori help login commands' commands "$@"
}
(( $+functions[_satori__login_commands] )) ||
_satori__login_commands() {
    local commands; commands=()
    _describe -t commands 'satori login commands' commands "$@"
}
(( $+functions[_satori__help__run__mongosh_commands] )) ||
_satori__help__run__mongosh_commands() {
    local commands; commands=()
    _describe -t commands 'satori help run mongosh commands' commands "$@"
}
(( $+functions[_satori__run__help__mongosh_commands] )) ||
_satori__run__help__mongosh_commands() {
    local commands; commands=()
    _describe -t commands 'satori run help mongosh commands' commands "$@"
}
(( $+functions[_satori__run__mongosh_commands] )) ||
_satori__run__mongosh_commands() {
    local commands; commands=()
    _describe -t commands 'satori run mongosh commands' commands "$@"
}
(( $+functions[_satori__help__pgpass_commands] )) ||
_satori__help__pgpass_commands() {
    local commands; commands=()
    _describe -t commands 'satori help pgpass commands' commands "$@"
}
(( $+functions[_satori__pgpass_commands] )) ||
_satori__pgpass_commands() {
    local commands; commands=()
    _describe -t commands 'satori pgpass commands' commands "$@"
}
(( $+functions[_satori__help__run__psql_commands] )) ||
_satori__help__run__psql_commands() {
    local commands; commands=()
    _describe -t commands 'satori help run psql commands' commands "$@"
}
(( $+functions[_satori__run__help__psql_commands] )) ||
_satori__run__help__psql_commands() {
    local commands; commands=()
    _describe -t commands 'satori run help psql commands' commands "$@"
}
(( $+functions[_satori__run__psql_commands] )) ||
_satori__run__psql_commands() {
    local commands; commands=()
    _describe -t commands 'satori run psql commands' commands "$@"
}
(( $+functions[_satori__help__run_commands] )) ||
_satori__help__run_commands() {
    local commands; commands=(
'dbt:dbt' \
'psql:psql' \
'mongosh:mongosh' \
's3:s3' \
    )
    _describe -t commands 'satori help run commands' commands "$@"
}
(( $+functions[_satori__run_commands] )) ||
_satori__run_commands() {
    local commands; commands=(
'dbt:dbt' \
'psql:psql' \
'mongosh:mongosh' \
's3:s3' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'satori run commands' commands "$@"
}
(( $+functions[_satori__help__run__s3_commands] )) ||
_satori__help__run__s3_commands() {
    local commands; commands=()
    _describe -t commands 'satori help run s3 commands' commands "$@"
}
(( $+functions[_satori__run__help__s3_commands] )) ||
_satori__run__help__s3_commands() {
    local commands; commands=()
    _describe -t commands 'satori run help s3 commands' commands "$@"
}
(( $+functions[_satori__run__s3_commands] )) ||
_satori__run__s3_commands() {
    local commands; commands=()
    _describe -t commands 'satori run s3 commands' commands "$@"
}

if [ "$funcstack[1]" = "_satori" ]; then
    _satori "$@"
else
    compdef _satori satori
fi
_datastores() {
    local datastore_names=()

    # Read keys line by line and populate the array
    while IFS= read -r line; do
        datastore_names+=("$line")
    done < <(satori list --datastores)

    _values datastore_name $datastore_names
}


_databases() {
    local datastore_name="$words[CURRENT-1]"  # Get the selected datastore_name
    datastore_name=${datastore_name//\\/}
    local databases=()
    while IFS= read -r line; do
        databases+=("$line")
    done < <(satori list --databases $datastore_name)
    if [[ ! -z "$databases" ]]; then
        _values database $databases
    fi
}