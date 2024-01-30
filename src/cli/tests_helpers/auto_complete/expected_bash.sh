_satori() {
    local i cur prev opts cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="satori"
                ;;
            satori,auto_complete)
                cmd="satori__auto_complete"
                ;;
            satori,aws)
                cmd="satori__aws"
                ;;
            satori,help)
                cmd="satori__help"
                ;;
            satori,list)
                cmd="satori__list"
                ;;
            satori,login)
                cmd="satori__login"
                ;;
            satori,pgpass)
                cmd="satori__pgpass"
                ;;
            satori,run)
                cmd="satori__run"
                ;;
            satori__help,auto_complete)
                cmd="satori__help__auto_complete"
                ;;
            satori__help,aws)
                cmd="satori__help__aws"
                ;;
            satori__help,help)
                cmd="satori__help__help"
                ;;
            satori__help,list)
                cmd="satori__help__list"
                ;;
            satori__help,login)
                cmd="satori__help__login"
                ;;
            satori__help,pgpass)
                cmd="satori__help__pgpass"
                ;;
            satori__help,run)
                cmd="satori__help__run"
                ;;
            satori__help__run,dbt)
                cmd="satori__help__run__dbt"
                ;;
            satori__help__run,mongosh)
                cmd="satori__help__run__mongosh"
                ;;
            satori__help__run,psql)
                cmd="satori__help__run__psql"
                ;;
            satori__help__run,s3)
                cmd="satori__help__run__s3"
                ;;
            satori__run,dbt)
                cmd="satori__run__dbt"
                ;;
            satori__run,help)
                cmd="satori__run__help"
                ;;
            satori__run,mongosh)
                cmd="satori__run__mongosh"
                ;;
            satori__run,psql)
                cmd="satori__run__psql"
                ;;
            satori__run,s3)
                cmd="satori__run__s3"
                ;;
            satori__run__help,dbt)
                cmd="satori__run__help__dbt"
                ;;
            satori__run__help,help)
                cmd="satori__run__help__help"
                ;;
            satori__run__help,mongosh)
                cmd="satori__run__help__mongosh"
                ;;
            satori__run__help,psql)
                cmd="satori__run__help__psql"
                ;;
            satori__run__help,s3)
                cmd="satori__run__help__s3"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        satori)
            opts="-h -V --help --version run login auto_complete list pgpass aws help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__auto_complete)
            opts="-h -V --generate --out --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --generate)
                    COMPREPLY=($(compgen -W "bash elvish fish powershell zsh" -- "${cur}"))
                    return 0
                    ;;
                --out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__aws)
            opts="-h --no-launch-browser --domain --invalid_cert --debug --refresh --no-persist --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --domain)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__help)
            opts="run login auto_complete list pgpass aws help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__help__auto_complete)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__help__aws)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__help__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__help__login)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__help__pgpass)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__help__run)
            opts="dbt psql mongosh s3"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__help__run__dbt)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__help__run__mongosh)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__help__run__psql)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__help__run__s3)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__list)
            opts="-h -V --datastores --databases --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --databases)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__login)
            opts="-d -f -h --display --format --no-launch-browser --domain --invalid_cert --debug --refresh --no-persist --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "json yaml csv" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json yaml csv" -- "${cur}"))
                    return 0
                    ;;
                --domain)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__pgpass)
            opts="-h --no-launch-browser --domain --invalid_cert --debug --refresh --no-persist --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --domain)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__run)
            opts="-h --help dbt psql mongosh s3 help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__run__dbt)
            opts="-h --profile-dir --target --no-launch-browser --domain --invalid_cert --debug --refresh --no-persist --help [additional_args]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --profile-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --target)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --domain)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__run__help)
            opts="dbt psql mongosh s3 help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__run__help__dbt)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__run__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__run__help__mongosh)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__run__help__psql)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__run__help__s3)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__run__mongosh)
# Provide auto-complete for datastores for run commands
comp_len=${#COMP_WORDS[@]}
if ((comp_len ==4)) then 
    opts=$(list_datastores | sed 's/.*/"&"/')
    local IFS=$'\n'
    COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
    return 0
else
 
            opts="-h --no-launch-browser --domain --invalid_cert --debug --refresh --no-persist --help <datastore_name> [additional_args]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --domain)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
fi

            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__run__psql)
# Provide auto-complete for datastores for run commands
comp_len=${#COMP_WORDS[@]}
if ((comp_len ==4)) then 
    opts=$(list_datastores | sed 's/.*/"&"/')
    local IFS=$'\n'
    COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
    return 0

# Provide auto-complete for databases for run commands
elif ((comp_len == 5)) then 
    opts=$(list_databases "$prev")
    local IFS=$'\n'
    COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
    return 0
else
 
            opts="-h --no-launch-browser --domain --invalid_cert --debug --refresh --no-persist --help <datastore_name> <database> [additional_args]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --domain)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
fi

            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        satori__run__s3)
# Provide auto-complete for datastores for run commands
comp_len=${#COMP_WORDS[@]}
if ((comp_len ==4)) then 
    opts=$(list_datastores | sed 's/.*/"&"/')
    local IFS=$'\n'
    COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
    return 0
else
 
            opts="-h --no-launch-browser --domain --invalid_cert --debug --refresh --no-persist --help <datastore_name> [additional_args]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --domain)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
fi

            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _satori -o nosort -o bashdefault -o default satori

function list_datastores() {
    satori list --datastores | while IFS= read -r line; do
        echo "'$line'"
    done
}

function list_databases() {
    local datastore_name=$(echo -n $1 | sed "s/'//g")
    satori list --databases "$datastore_name"
}
