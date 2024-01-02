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
            (connect)
_arguments "${_arguments_options[@]}" \
'--domain=[Default to https\://app.satoricyber.com]:VALUE: ' \
'--no-persist[Don'\''t persist the credentials]' \
'--no-launch-browser[Don'\''t launch the browser]' \
'--debug[Enable debug mode]' \
'--refresh[refresh the local cache files]' \
'-h[Print help]' \
'--help[Print help]' \
':tool -- Tool to connect:(psql)' \
':datastore_name -- The name of the datastore to connect:_datastores' \
'::database -- Database name:_databases' \
'::additional_args:' \
&& ret=0
;;
(login)
_arguments "${_arguments_options[@]}" \
'-f+[]:FORMAT:(json yaml csv)' \
'--format=[]:FORMAT:(json yaml csv)' \
'--domain=[Default to https\://app.satoricyber.com]:VALUE: ' \
'-d[Display the credentials or save to file]' \
'--display[Display the credentials or save to file]' \
'--no-launch-browser[Don'\''t launch the browser]' \
'--debug[Enable debug mode]' \
'--refresh[refresh the local cache files]' \
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
'--domain=[Default to https\://app.satoricyber.com]:VALUE: ' \
'--no-launch-browser[Don'\''t launch the browser]' \
'--debug[Enable debug mode]' \
'--refresh[refresh the local cache files]' \
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
            (connect)
_arguments "${_arguments_options[@]}" \
&& ret=0
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
'connect:Connect to a tool' \
'login:Login to Satori' \
'auto_complete:Generate autocomplete' \
'list:List resources' \
'pgpass:Creates a Pgpass file to be used by other apps' \
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
(( $+functions[_satori__connect_commands] )) ||
_satori__connect_commands() {
    local commands; commands=()
    _describe -t commands 'satori connect commands' commands "$@"
}
(( $+functions[_satori__help__connect_commands] )) ||
_satori__help__connect_commands() {
    local commands; commands=()
    _describe -t commands 'satori help connect commands' commands "$@"
}
(( $+functions[_satori__help_commands] )) ||
_satori__help_commands() {
    local commands; commands=(
'connect:Connect to a tool' \
'login:Login to Satori' \
'auto_complete:Generate autocomplete' \
'list:List resources' \
'pgpass:Creates a Pgpass file to be used by other apps' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'satori help commands' commands "$@"
}
(( $+functions[_satori__help__help_commands] )) ||
_satori__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'satori help help commands' commands "$@"
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
    done < <(./satori list --datastores)

    _values datastore_name $datastore_names
}


_databases() {
    local datastore_name="$words[CURRENT-1]"  # Get the selected datastore_name
    datastore_name=${datastore_name//\\/}
    local databases=()
    while IFS= read -r line; do
        databases+=("$line")
    done < <(./satori list --databases $datastore_name)
    _values database $databases
}